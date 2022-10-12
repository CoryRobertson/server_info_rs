use std::io::{Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::time::Duration;
use chrono::Utc;
use serde::Serialize;
use crate::server_info_packet::server_info_packet::ServerInfo;

mod server_info_packet;


fn main() {
    println!("I am the server!");

    let listener = TcpListener::bind("localhost:8111").unwrap();
    let mut count = 0;

    loop {
        let stream = listener.accept().unwrap().0;

        handle_client(&stream, ServerInfo{ date: Utc::now().timestamp() });
        println!("handled client, waiting...");
        //thread::sleep(Duration::from_secs(1));
        count = count + 1;

    }



}

fn handle_client(mut stream: &TcpStream, info: ServerInfo) {

    //let data = format!("hello this is a test! {}", info.date);
    //stream.write(data.as_bytes()).expect("TODO: panic message");
    let ser = serde_json::to_string(&info).unwrap();

    stream.write_all(ser.as_bytes()).expect("TODO: panic message");
    stream.shutdown(Shutdown::Both).expect("TODO: panic message");

}
