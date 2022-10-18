#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::borrow::Cow;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use eframe::egui;
use crate::egui::{Color32, Vec2};
use crate::server_info_packet::server_info_packet::ServerInfo;

mod server_info_packet;

fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Option::from(Vec2::new(900.0, 800.0));
    eframe::run_native("Server Info Client", native_options, Box::new(|cc| Box::new(MyEguiApp::new(cc))));
}

#[derive(Default)]
struct MyEguiApp {
    stream: Option<TcpStream>,
    buf_vec: Vec<u8>,
    address: String,
    server_info: ServerInfo, // TODO: have the program remember previous address used, could be simple text file
    // TODO: also have the program remember the previous sessions window size, so it can copy that as well, not sure if possible due to having to load native options before egui, worth a try.
    frames: i32,
    displaying_disks: bool,
    displaying_interfaces: bool,
    displaying_cpus: bool,
    update_rate: f32,

}

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self{
            stream: None,
            buf_vec: vec![],
            address: "localhost:8111".to_string(), // TODO: have the program remember previous address used, could be simple text file
            server_info: ServerInfo::default(),
            frames: 0,
            displaying_disks: false,
            displaying_interfaces: false,
            displaying_cpus: false,
            update_rate: 0.5
        }
    }
}

// thank you online example <3
fn toggle_ui_compact(ui: &mut egui::Ui, on: &mut bool) -> egui::Response {
    let desired_size = ui.spacing().interact_size.y * egui::vec2(2.0, 1.0);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    if response.clicked() {
        *on = !*on;
        response.mark_changed();
    }
    response.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::Checkbox, *on, ""));

    if ui.is_rect_visible(rect) {
        let how_on = ui.ctx().animate_bool(response.id, *on);
        let visuals = ui.style().interact_selectable(&response, *on);
        let rect = rect.expand(visuals.expansion);
        let radius = 0.5 * rect.height();
        ui.painter()
            .rect(rect, radius, visuals.bg_fill, visuals.bg_stroke);
        let circle_x = egui::lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
        let center = egui::pos2(circle_x, rect.center().y);
        ui.painter()
            .circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);
    }
    response
}

fn deserialize_server_info(data: &String) -> Option<ServerInfo> {
     let result = serde_json::from_str(data);
    return if result.is_ok() {
        Some(result.unwrap())
    } else { None }

}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            ctx.request_repaint();
            let data: Cow<str>;

            let found_data = match self.stream {
                Some(_) => {
                    if self.frames as f32 > (60.0)/self.update_rate {
                        let mut small_buf:[u8 ; 4096] = [0 ; 4096];
                        self.stream.as_ref().unwrap().read(&mut small_buf).unwrap_or_default();

                        for value in small_buf { // make small buffer of the data into a vector sent by the server
                            if !String::from_utf8_lossy(&[value]).contains("\0") {
                                self.buf_vec.push(value);
                            }
                        }
                        let _ = self.stream.as_ref().unwrap().write(&[0]);
                        data = String::from_utf8_lossy(&*self.buf_vec); // convert the vector to a string
                        match deserialize_server_info(&data.to_string()) { // deserialize the string into a server info struct only if the data received was able to be deserialized properly
                            Some(sinfo) => self.server_info = sinfo,
                            None => (),
                        }
                    }
                    true
                }
                None => {false}
            };

            ui.text_edit_singleline(&mut self.address);

            ui.horizontal(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Display CPU info: ");
                    toggle_ui_compact(ui,&mut self.displaying_cpus);
                });

                ui.horizontal(|ui| {
                    ui.label("Display network info: ");
                    toggle_ui_compact(ui,&mut self.displaying_interfaces);
                });

                ui.horizontal(|ui| {
                    ui.label("Display disk info: ");
                    toggle_ui_compact(ui,&mut self.displaying_disks);
                });
            });

            if ui.button("Connect").clicked() {
                self.stream = match TcpStream::connect(self.address.as_str()) {
                    // TODO: probably best to save the connected session here as this runs rarely.
                    Ok(s) => {
                        Some(s)
                    }
                    Err(_) => {
                        println!("tcp stream failed to connect.");
                        None
                    }
                }
            }

            ui.horizontal(|ui| {
                ui.label("Update Rate: ");
                ui.add(egui::Slider::new(&mut self.update_rate,0.1..=2.0)).on_hover_text("Update rate per second.");
            });

            if ui.button("Disconnect").clicked() {
                match &self.stream {
                    None => {println!("failed to disconnect");}
                    Some(stream) => {
                        println!("disconnected");
                        stream.shutdown(Shutdown::Both).expect("Unable to shutdown tcp stream.");
                        self.stream = None;
                    }
                }
            }

            if found_data {
                self.buf_vec.clear();
            }

            ui.label(&self.server_info.get_date_time().to_string());

            if self.displaying_disks {
                for disk in &self.server_info.disks {
                    ui.colored_label(Color32::from_rgb(255,255,255),disk);
                }
            }

            if self.displaying_interfaces {
                for iface in &self.server_info.net_interfaces {
                    ui.colored_label(Color32::from_rgb(255,255,255),iface);
                }
            }

            if self.displaying_cpus {
                for cpu in &self.server_info.cpus {
                    ui.colored_label(Color32::from_rgb(255,255,255),cpu);
                }
            }


            ui.horizontal(|ui| {
                ui.label("Average CPU Usage: ");
                let s = format_args!("{:.2} %", &self.server_info.avg_cpu_usage).to_string();
                ui.label(s);
            });

            ui.horizontal(|ui| {
                ui.label("Total Ram: ");
                let total_ram: f64 = self.server_info.total_ram as f64 / 1000000000.0;
                let s = format_args!("{:.2} GB", total_ram).to_string();
                ui.label(s);

            });

            ui.horizontal(|ui| {
                ui.label("Used Ram: ");
                let used_ram: f64 = self.server_info.used_memory as f64 / 1000000000.0;
                let s = format_args!("{:.2} GB", used_ram).to_string();
                ui.label(s);
            });

            ui.horizontal(|ui| {
                ui.label("System Name: ");
                ui.label(&self.server_info.system_name);
            });

            ui.horizontal(|ui| {
                ui.label("Kernel Version: ");
                ui.label(&self.server_info.kernel_version);
            });

            ui.horizontal(|ui| {
                ui.label("OS Version: ");
                ui.label(&self.server_info.os_version);
            });

            ui.horizontal(|ui| {
                ui.label("Host Name: ");
                ui.label(&self.server_info.host_name);
            });

            if self.frames as f32 > ((60.0) / self.update_rate) {
                self.frames = 0;
            }
            self.frames = self.frames + 1;
        });
    }


}