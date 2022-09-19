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
        let mut buf_reader = BufReader::new(&mut stream);

        let mut request_line = String::new();
        buf_reader.read_line(&mut request_line).await.unwrap();
        //let good_line = request_line.clone();
        let good_line = request_line.clone();
        // let mut k: u32 = 0;
        // while true {
        //     println!("{} {}", k, request_line);
        //     buf_reader.read_line(&mut request_line).await.unwrap();
        //     if request_line.contains("Cache-Control"){
        //         break;
        //     }
        //     k+=1;
        // }

        // println!("here");
        //let mut str_lines: Vec<String> = Vec::new();

        // buf_reader.read_line(buf)

        // lines.for_each(|l| {
        //     match l {
        //         Ok(s) => println!("{}", s),
        //         _=> (),
        //     }
        // }).await;

        // println!("here")

        //let request_line :String = lines.next().await.unwrap().unwrap();

        // while !request_line.is_empty()  {
        //     print!("{}", request_line);
        //     buf_reader.read_line(&mut request_line);
        // }

        println!("{}", request_line);
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