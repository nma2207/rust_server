use async_std::io::prelude::BufReadExt;
use async_std::net::{SocketAddr, TcpListener, TcpStream};
use async_std::io::{BufReader, WriteExt};
use async_std::fs;
use futures::StreamExt;


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
        
        let (status_line, filename) = if request_line.trim() == "GET / HTTP/1.1" {
            ("HTTP/1.1 200 OK", "templates/index.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND", "templates/404.html")
        };

        
        let content = fs::read_to_string(filename).await.unwrap();
        let length = content.len();
        let responce = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");
        stream.write_all(responce.as_bytes()).await.unwrap();
        stream.flush().await.unwrap();
    }
}