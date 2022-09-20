use async_std::io::prelude::BufReadExt;
use async_std::net::{SocketAddr, TcpListener, TcpStream};
use async_std::io::{BufReader, WriteExt, ReadExt};
use async_std::fs;
use futures::{StreamExt};


pub struct HttpServer {
    pub addr : SocketAddr
}

impl HttpServer {
    pub fn new(addr: SocketAddr) -> HttpServer {
        println!("Create a server");
        HttpServer{addr}
    }

    pub async fn run(&self)
    {
        let listener=TcpListener::bind(self.addr).await.unwrap();
        listener
            .incoming()
            .for_each_concurrent(None, |stream| async move {
                    let stream = stream.unwrap();

                    HttpServer::handle_connection(stream).await;
                }
            ).await;
    }

    pub fn display(&self) {
        println!("Server at {:?}", self.addr)
    }

    async fn handle_connection(mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);


        let mut lines = buf_reader.lines();
        
        let mut count = 0;
        let mut lines_vec: Vec<String> = Vec::new();

        while let Some(line) = lines.next().await {
            count += 1;
            let s = line.unwrap();
            println!("{} {}", count, s);
            if s.len() == 0 {
                break;
            }
            lines_vec.push(s);
        }
        
        let good_line = &lines_vec[0];

        let mut status_line = "HTTP/1.1 404 NOT FOUND";
        let mut filename = "templates/404.html";
        
        if good_line.trim() == "GET / HTTP/1.1" {
            status_line = "HTTP/1.1 200 OK";
            filename = "templates/index.html";
        }
        else if good_line.starts_with("GET /greeting.js HTTP/1.1") {
            status_line = "HTTP/1.1 200 OK";
            filename = "templates/greeting.js";
        }
        else if good_line.starts_with("GET /favicon.ico HTTP/1.1") {
            println!("get favicon");
        }
        
        let content = fs::read_to_string(filename).await.unwrap();
        let length = content.len();
        let responce = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");
        stream.write_all(responce.as_bytes()).await.unwrap();
        stream.flush().await.unwrap();
    }
}