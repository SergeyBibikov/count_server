use std::net::TcpListener;
use std::io::{Read,Write};
use std::sync::mpsc;

fn main() {
    std::io::stdout().write_all(b"Starting the server.\n\
    To exit type 'exit'.\n\
    To get the count type 'count'\n\
    To drop the count type 'drop'\n").unwrap();
    std::io::stdout().flush().unwrap();
    let (sender,receiver) = mpsc::channel();
    run(sender);
    let stdin = std::io::stdin();
    let mut total_count = 0;
    loop{
        let mut resp = String::new();
        stdin.read_line(&mut resp).unwrap();
        match resp.trim(){
            "exit"=>break,
            "count"=>{total_count+=receiver.try_iter().count(); println!("{}",total_count)},
            "drop"=>total_count=0,
            _=>{continue}
        }
    }
}

fn run(sender: mpsc::Sender<i32>){
    std::thread::spawn(move || {
    let n = TcpListener::bind("127.0.0.1:8080").unwrap();
    for i in n.incoming(){
        let sender = sender.clone();
        std::thread::spawn(move || {
            handle_connection(i.unwrap(),sender)
        });        
    }});
}

fn handle_connection(mut stream: std::net::TcpStream,
                    sender: mpsc::Sender<i32>){
    let mut buffer = [0;1024];
    stream.read(&mut buffer).unwrap();
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let (status_line,filename) = if buffer.starts_with(get){
            let _ = sender.send(1);
            ("HTTP/1.1 200 OK\r\n\r\n","hello.html")                    
        }else if buffer.starts_with(sleep){
            std::thread::sleep(std::time::Duration::from_secs(10));
            ("HTTP/1.1 200 OK\r\n\r\n","hello.html") 
        }else {
            ("HTTP/1.1 404 NOT FOUND\r\n\r\n","404.html") 
        };
    let content = std::fs::read_to_string(filename).unwrap();
    let response = format!("{}{}",status_line,content);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}