use eframe::egui;
use pulsectl::controllers::{types::DeviceInfo, DeviceControl, SinkController};

use std::process::Command;

fn main() -> Result<(), eframe::Error> {
    let mut options = eframe::NativeOptions::default();

    options.persist_window = true;

    eframe::run_native("sinker", options, Box::new(|_cc| Box::new(App::default())))
}

struct App {
    controller: SinkController,
    sink_name: String,
}

impl Default for App {
    fn default() -> Self {
        App {
            controller: SinkController::create().unwrap(),
            sink_name: String::new(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(_ctx, |ui| {
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.sink_name);
                if ui.button("add sink").clicked() {
                    let sink_name = format!("'sinker-{}'", self.sink_name.trim());
                    let _output = Command::new("pactl")
                        .args([
                            "load-module",
                            "module-null-sink",
                            "media.class=Audio/Sink",
                            format!("sink_name=sinker-{}", sink_name).as_str(),
                            "channel_map=stereo",
                        ])
                        .output()
                        .expect("could not create sink");
                }
            });

            let sinks: Vec<DeviceInfo> = self
                .controller
                .list_devices()
                .unwrap()
                .into_iter()
                .filter(|sink| {
                    if let Some(name) = &sink.name {
                        return name.contains("sinker-");
                    }
                    false
                })
                .collect();

            if sinks.is_empty() {
                ui.label("No sinks found");
            }

            for sink in sinks {
                let name = sink.name.unwrap();
                let module_index = sink.owner_module.unwrap();

                ui.horizontal(|ui| {
                    ui.label(name);

                    if ui.button("remove").clicked() {
                        let _output = Command::new("pactl")
                            .args(["unload-module", module_index.to_string().as_str()])
                            .output()
                            .expect("could not remove sink");
                    }
                });
            }
        });
    }
}
