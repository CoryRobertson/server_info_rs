use std::borrow::Cow;
use std::io::{Read};
use std::net::{Shutdown, TcpStream};
use eframe::egui;
use crate::server_info_packet::server_info_packet::ServerInfo;

mod server_info_packet;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Server Info Client", native_options, Box::new(|cc| Box::new(MyEguiApp::new(cc))));
}

#[derive(Default)]
struct MyEguiApp {
    stream: Option<TcpStream>,
    buf_vec: Vec<u8>,
    address: String,
    server_info: ServerInfo,
    auto_refresh: bool,
    frames: i32,

}

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self{
            stream: None,
            buf_vec: vec![],
            address: "localhost:8111".to_string(),
            server_info: ServerInfo::default(),
            auto_refresh: false,
            frames: 0
        }
    }
}

fn deserialize_server_info(data: &String) -> ServerInfo {
    return serde_json::from_str(data).unwrap_or_default();
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            ctx.request_repaint();
            let mut data= Cow::default();

            let found_data = match self.stream {
                Some(_) => {
                    self.stream.as_ref().unwrap().read_to_end(&mut self.buf_vec).unwrap();
                    data = String::from_utf8_lossy(&*self.buf_vec);
                    self.server_info = deserialize_server_info(&data.to_string());
                    data.len() != 0
                }
                None => {false}
            };


            ui.text_edit_singleline(&mut self.address);

            if ui.button("refresh?").clicked() {
                //self.connecting = true;
                self.stream = match TcpStream::connect(self.address.as_str()) {
                    Ok(s) => {
                        Some(s)
                    }
                    Err(_) => {
                        println!("tcp stream failed to connect.");
                        None
                    }
                }
            }

            ui.checkbox(&mut self.auto_refresh,"auto-refresh");


            
            if ui.button("disconnect").clicked() {

                match &self.stream {
                    None => {println!("failed to disconnect");}
                    Some(strm) => {
                        println!("disconnected");
                        strm.shutdown(Shutdown::Both).unwrap();
                        self.stream = None;
                        //self.connecting = false;
                    }
                }
                    
            }

            if found_data {
                println!("{}", data);
                println!("{}", self.server_info);
                self.buf_vec.clear();

            }



            self.frames = self.frames + 1;

            if self.frames > 120 {
                self.frames = 0;
                if self.auto_refresh {
                    self.stream = match TcpStream::connect(self.address.as_str()) {
                        Ok(s) => {
                            Some(s)
                        }
                        Err(_) => {
                            println!("tcp stream failed to connect.");
                            None
                        }
                    }
                }
            }
        });
    }
}