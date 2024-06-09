/* 
* 
* Use rocket as an event system
* Use egui for window
*
* */

use eframe::egui::{self, CentralPanel, Context, Label};
use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct EngineFrontend {
    label_text: String,
}

impl EngineFrontend {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl Default for EngineFrontend {
    fn default() -> Self {
        Self {
            label_text: "Hello World!".to_owned(),
        }
    }
}
impl eframe::App for EngineFrontend {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

    }
}

pub fn run() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0]),
        ..Default::default()
    };

    eframe::run_native(
        "EngineFrontend Example",
        native_options,
        Box::new(|cc| Box::new(EngineFrontend::new(cc))),
    )
}

pub struct EngineBackend {

}

impl EngineBackend {

}
