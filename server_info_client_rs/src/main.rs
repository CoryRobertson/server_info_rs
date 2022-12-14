#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate core;

use crate::egui::{Color32, Vec2};
use crate::last_session::LastSession;
use eframe::egui;
use eframe::egui::{Pos2, Rounding};
use eframe::epaint::Rect;
use server_info_packets::server_info_packet::ServerInfo;
use std::borrow::Cow;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};

mod last_session;

static LAST_SESSION_FILE_NAME: &str = "server_info_last_session.sav";

fn main() {
    let native_options = eframe::NativeOptions {
        initial_window_size: Option::from(Vec2::new(900.0, 800.0)),
        ..Default::default()
    };
    //native_options.initial_window_size = Option::from(Vec2::new(900.0, 800.0));
    eframe::run_native(
        "Server Info Client",
        native_options,
        Box::new(|cc| Box::new(MyEguiApp::new(cc))),
    );
}

#[derive(Default)]
struct MyEguiApp {
    stream: Option<TcpStream>,
    buf_vec: Vec<u8>,
    address: String,
    server_info: ServerInfo,
    frames: i32,
    displaying_disks: bool,
    displaying_interfaces: bool,
    displaying_cpus: bool,
    update_rate: f32,
    first_run: bool,
}

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            stream: None,
            buf_vec: vec![],
            address: "localhost:8111".to_string(),
            server_info: ServerInfo::default(),
            frames: 0,
            displaying_disks: false,
            displaying_interfaces: false,
            displaying_cpus: false,
            update_rate: 0.5,
            first_run: true,
        }
    }
}

/// thank you online example <3
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

/// Deserializes a given string into a ServerInfo struct, returns none if data is invalid
fn deserialize_server_info(data: &str) -> Option<ServerInfo> {
    let result = serde_json::from_str(data);
    // if result.is_ok() {
    //     Some(result.unwrap())
    // } else {
    //     None
    // }
    if let Ok(res) = result {
        return Some(res);
    }
    None
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if self.first_run {
            self.first_run = false;

            let ls = match last_session::read_from_file(LAST_SESSION_FILE_NAME) {
                Ok(f) => f,
                Err(_) => LastSession {
                    address: "localhost:8111".to_string(),
                    screen_dimension: (900.0, 900.0),
                },
            };

            self.address = ls.address;
            let size = Vec2 {
                x: ls.screen_dimension.0,
                y: ls.screen_dimension.1,
            };
            frame.set_window_size(size);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ctx.request_repaint();
            let data: Cow<str>;

            let found_data = match &self.stream {
                Some(_) => {
                    if self.frames as f32 > (60.0) / self.update_rate {
                        let mut small_buf: [u8; 4096] = [0; 4096];
                        match self.stream.as_ref().unwrap().read(&mut small_buf) {
                            // try to read stream into a buffer
                            Ok(_) => {
                                // data was able to be read

                                for value in small_buf {
                                    // make small buffer of the data into a vector sent by the server
                                    if !String::from_utf8_lossy(&[value]).contains('\0') {
                                        self.buf_vec.push(value);
                                    }
                                }
                                let _ = self.stream.as_ref().unwrap().write(&[0]);
                                data = String::from_utf8_lossy(&self.buf_vec); // convert the vector to a string
                                if let Some(sinfo) = deserialize_server_info(&data) {
                                    self.server_info = sinfo
                                }
                            }
                            Err(_) => {
                                // data was not able to be read, because of this, remove the stream
                                self.stream = None;
                            }
                        };
                    }
                    true
                }
                None => false,
            };

            ui.text_edit_singleline(&mut self.address);

            ui.horizontal(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Display CPU info: ");
                    toggle_ui_compact(ui, &mut self.displaying_cpus);
                });

                ui.horizontal(|ui| {
                    ui.label("Display network info: ");
                    toggle_ui_compact(ui, &mut self.displaying_interfaces);
                });

                ui.horizontal(|ui| {
                    ui.label("Display disk info: ");
                    toggle_ui_compact(ui, &mut self.displaying_disks);
                });
            });

            if ui.button("Connect").clicked() {
                self.stream = match TcpStream::connect(self.address.as_str()) {
                    Ok(s) => {
                        s.set_read_timeout(Some(core::time::Duration::from_secs(5)))
                            .unwrap();
                        s.set_write_timeout(Some(core::time::Duration::from_secs(5)))
                            .unwrap();
                        let size = frame.info().window_info.size;
                        let ls = LastSession {
                            address: self.address.to_string(),
                            screen_dimension: (size.x, size.y),
                        };
                        last_session::write_to_file(LAST_SESSION_FILE_NAME, ls)
                            .expect("Unable to write to file.");

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
                ui.add(egui::Slider::new(&mut self.update_rate, 0.1..=2.0))
                    .on_hover_text("Update rate per second.");
            });

            if ui.button("Disconnect").clicked() {
                match &self.stream {
                    None => {
                        println!("failed to disconnect");
                    }
                    Some(stream) => {
                        println!("disconnected");
                        stream
                            .shutdown(Shutdown::Both)
                            .expect("Unable to shutdown tcp stream.");
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
                    ui.colored_label(Color32::from_rgb(255, 255, 255), disk);
                }
            }

            if self.displaying_interfaces {
                for interface in &self.server_info.net_interfaces {
                    ui.colored_label(Color32::from_rgb(255, 255, 255), interface);
                }
            }

            if self.displaying_cpus {
                for cpu in &self.server_info.cpus {
                    ui.colored_label(Color32::from_rgb(255, 255, 255), cpu);
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
                let s = format_args!("{total_ram:.2} GB").to_string();
                ui.label(s);
            });

            ui.horizontal(|ui| {
                ui.label("Used Ram: ");
                let used_ram: f64 = self.server_info.used_memory as f64 / 1000000000.0;
                let s = format_args!("{used_ram:.2} GB").to_string();
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

            let indicator_rect_color = {
                if found_data {
                    Color32::from_rgb(50, 255, 50)
                } else {
                    Color32::from_rgb(255, 50, 50)
                }
            };
            ui.painter().rect_filled(
                Rect::from_two_pos(Pos2::new(245.0, 50.0), Pos2::new(245.0 + 50.0, 50.0 + 50.0)),
                Rounding::none(),
                indicator_rect_color,
            );

            #[cfg(debug_assertions)]
            {
                let mousepos = match ctx.pointer_hover_pos() {
                    None => Pos2::new(0.0, 0.0),
                    Some(a) => a,
                };
                ui.label(format!("DEBUG mouse pos: {},{}", mousepos.x, mousepos.y));
            }

            egui::warn_if_debug_build(ui);

            if self.frames as f32 > ((60.0) / self.update_rate) {
                self.frames = 0;
            }
            self.frames += 1;
        });
    }
}
