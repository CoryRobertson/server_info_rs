use std::io::{Read, Write as OtherWrite};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;
use std::time::Duration;
use chrono::Utc;
use std::string::String;
use std::thread::JoinHandle;
use sysinfo::{CpuExt, NetworkExt, System, SystemExt};
use crate::server_info_packet::server_info_packet::ServerInfo;

mod server_info_packet;


fn main() {
    println!("Listening for connections on port 8111!");

    let listener = TcpListener::bind("0.0.0.0:8111").unwrap();

    let mut thread_vec:Vec<JoinHandle<()>> = vec![];

    for incomming in listener.incoming() {

        for i in 0..thread_vec.len() {
            match thread_vec.get(i) {
                None => {}
                Some(t) => {
                    if t.is_finished() {
                        //println!("Thread finished: {:?}", thread_vec.get(i).unwrap());
                        thread_vec.remove(i);
                    }
                }
            }
        }

        let handle = thread::spawn(move || {
            let stream = incomming.expect("failed to handle");
            println!("Client connected: {:?}", stream);
            loop {
                if !handle_client(&stream,generate_server_info_packet()) {
                    println!("Client disconnected: {:?}", stream);
                    break;
                }
            }
        });

        thread_vec.push(handle);
        println!("Number of currently connected clients: {}",thread_vec.len());

    }
    
    for handle in thread_vec {
        handle.join().expect("TODO: panic message");
    }

}

fn generate_server_info_packet() -> ServerInfo {
    let mut sys = System::new_all();
    sys.refresh_all();

    thread::sleep(Duration::from_millis(250));
    sys.refresh_cpu();

    let mut disks: Vec<String> = vec![];
    
    for disk in sys.disks() {
        disks.push(format_args!("{:?}", disk).to_string());
    }

    let mut net_interfaces: Vec<String> = vec![];
    
    for (interface_name, data) in sys.networks() {
        net_interfaces.push(format_args!("{}: {}/{} B", interface_name, data.received(), data.transmitted()).to_string());
    }


    let mut components: Vec<String> = vec![];

    for component in sys.components() {
        components.push(format_args!("{:?}", component).to_string());
    }

    let total_ram = sys.total_memory();
    let used_memory = sys.used_memory();

    let system_name = sys.name().unwrap_or_default();
    let kernel_version = sys.kernel_version().unwrap_or_default();
    let os_version = sys.os_version().unwrap_or_default();
    let host_name = sys.host_name().unwrap_or_default();

    let total_cpus = sys.cpus().len();
    let mut avg_cpu_usage = 0.0;

    thread::sleep(Duration::from_millis(250));
    sys.refresh_cpu();

    let mut cpus: Vec<String> = vec![];

    for cpu in sys.cpus() {
        cpus.push(format_args!("{:?}", cpu).to_string());
        avg_cpu_usage = avg_cpu_usage + cpu.cpu_usage();
    }
    avg_cpu_usage = avg_cpu_usage / total_cpus as f32;

    ServerInfo{ date: Utc::now().timestamp(), disks, net_interfaces, components, total_ram, used_memory, system_name, kernel_version, os_version, host_name, total_cpus, cpus, avg_cpu_usage }
}

fn handle_client(mut stream: &TcpStream, info: ServerInfo) -> bool {

    let ser = serde_json::to_string(&info).unwrap_or_default();

    let write = stream.write(ser.as_bytes());

    let flush = stream.flush();

    let read = stream.read(&mut [0;128]);

    if write.is_err() || flush.is_err() || read.is_err() {
        
        stream.shutdown(Shutdown::Both).expect("Unable to shutdown client stream.");
        return false;
    }
    return true;
}
