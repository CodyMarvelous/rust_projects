use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("172.19.191.192:7878").unwrap();
    for stream in listener.incoming(){
        let _stream = stream.unwrap();
        println!("Connection established");
    }

    
}
