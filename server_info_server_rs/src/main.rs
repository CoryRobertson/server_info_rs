use std::fmt::Write;
use std::io::{Write as OtherWrite};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;
use std::time::Duration;
use chrono::Utc;
use serde::Serialize;
use std::string::String;
use sysinfo::{CpuExt, NetworkExt, ProcessExt, System, SystemExt};
use crate::server_info_packet::server_info_packet::ServerInfo;

mod server_info_packet;


fn main() {
    println!("I am the server!");

    let listener = TcpListener::bind("localhost:8111").unwrap();
    let mut count = 0;

    loop {
        let stream = listener.accept().unwrap().0;

        handle_client(&stream, generate_server_info_packet());
        println!("sent client data, waiting...");
        //thread::sleep(Duration::from_secs(1));
        count = count + 1;

        //let _ = generate_server_info_packet();

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

    // println!("{:?}", disks);

    let mut net_interfaces: Vec<String> = vec![];
    
    for (interface_name, data) in sys.networks() {
        net_interfaces.push(format_args!("{}: {}/{} B", interface_name, data.received(), data.transmitted()).to_string());
    }

    // println!("{:?}", net_interfaces);

    let mut components: Vec<String> = vec![];

    for component in sys.components() {
        // println!("{:?}", component);
        //components.write_fmt(format_args!("{:?} \n", component));
        components.push(format_args!("{:?}", component).to_string());
    }

    // println!("{:?}", components);

    let total_ram = sys.total_memory();
    let used_memory = sys.used_memory();

    // println!("total memory: {} bytes", sys.total_memory()); // keep
    // println!("used memory : {} bytes", sys.used_memory()); // keep
    // println!("total swap  : {} bytes", sys.total_swap());
    // println!("used swap   : {} bytes", sys.used_swap());

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

    // println!("Average Cpu Usage: {} over {} cores", avg_cpu_usage, total_cpus);

    ServerInfo{ date: Utc::now().timestamp(), disks, net_interfaces, components, total_ram, used_memory, system_name, kernel_version, os_version, host_name, total_cpus, cpus, avg_cpu_usage }
}

fn handle_client(mut stream: &TcpStream, info: ServerInfo) {

    //let data = format!("hello this is a test! {}", info.date);
    //stream.write(data.as_bytes()).expect("TODO: panic message");
    let ser = serde_json::to_string(&info).unwrap();

    stream.write_all(ser.as_bytes()).expect("TODO: panic message");
    stream.shutdown(Shutdown::Both).expect("TODO: panic message");

}
