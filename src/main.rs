use egui::{CentralPanel, Label, Ui};
use macroquad::prelude::*;

struct Lista {
    items: Vec<CardData>,
}

impl Lista {
    fn new() -> Self {
        let iter = (0..20).map(|x| CardData {
            title: format!("{} título", x),
            desc: format!("{} desc", x),
            url: format!("https://example.com/{}", x),
        });
        Self {
            items: Vec::from_iter(iter),
        }
    }
}

struct CardData {
    title: String,
    desc: String,
    url: String,
}

#[macroquad::main("egui")]
async fn main() {
    let mut c = 0;
    let lista = Lista::new();
    loop {
        clear_background(DARKBLUE);

        // Process keys, mouse etc.
        //? Conventions❗
        //? https://docs.rs/egui/latest/egui/#conventions
        //? angles are in radians
        //? Vec2::X is right and Vec2::Y is down.
        //? Pos2::ZERO is left top.
        //? Positions and sizes are measured in points. Each point may consist of many physical pixels.
        egui_macroquad::ui(|ctx| {
            CentralPanel::default().show(ctx, |ui| {
                ui.add(Label::new("Hello World!"));

                for i in &lista.items {
                    ui.label(&i.title);
                    ui.label(&i.desc);
                    ui.label(&i.url);
                }

                ui.label("A shorter and more convenient way to add a label. 😊");
                if ui.button("Click me").clicked() {
                    // take some action here
                }

                ui_counter(ui, &mut c);
            });
        });

        // * Draw things before egui
        egui_macroquad::draw();
        // * Draw things after egui
        next_frame().await;
    }
}

fn ui_counter(ui: &mut Ui, counter: &mut i32) {
    // Put the buttons and label on the same row:
    ui.horizontal(|ui| {
        if ui.button("-").clicked() {
            *counter -= 1;
        }
        ui.label(counter.to_string());
        if ui.button("+").clicked() {
            *counter += 1;
        }
    });
}
