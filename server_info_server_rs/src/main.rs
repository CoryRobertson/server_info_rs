use std::io::{Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::time::Duration;

fn main() {
    println!("I am the server!");

    let listener = TcpListener::bind("localhost:8111").unwrap();
    let mut count = 0;

    loop {
        let stream = listener.accept().unwrap().0;

        handle_client(&stream, &count);
        println!("handled client, waiting...");
        //thread::sleep(Duration::from_secs(1));
        count = count + 1;

    }



}

fn handle_client(mut stream: &TcpStream, count: &i32) {

    let data = format!("hello this is a test! {}", count);
    //stream.write(data.as_bytes()).expect("TODO: panic message");
    stream.write_all(data.as_bytes()).expect("TODO: panic message");
    stream.shutdown(Shutdown::Both).expect("TODO: panic message");

}
