use std::borrow::Cow;

use constants::PADDING;
use egui::{
    CentralPanel, Color32, FontData, FontDefinitions, Label, Layout, RichText, ScrollArea,
    Separator, Ui,
};
use macroquad::prelude::*;
mod constants;
struct Lista {
    items: Vec<CardData>,
}

impl Lista {
    fn new() -> Self {
        let iter = (0..20).map(|x| CardData {
            title: format!("{} title", x),
            desc: format!("{} description", x),
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
    let mut font_def = FontDefinitions::default();
    font_def.font_data.insert(
        "atari".into(),
        FontData {
            font: Cow::Borrowed(include_bytes!("../resources/atari_games.ttf")),
            index: 0,
            tweak: Default::default(),
        },
    );

    font_def
        .families
        .insert(egui::FontFamily::Proportional, vec!["atari".to_owned()]);
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
            ctx.set_fonts(font_def.clone());

            CentralPanel::default().show(ctx, |ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    ui.add(Label::new("Hello World!"));

                    for i in &lista.items {
                        ui.add_space(PADDING);
                        ui.label(
                            RichText::new(format!("> {}", &i.title))
                                .size(35.0)
                                .color(Color32::KHAKI),
                        );

                        ui.add_space(PADDING);
                        ui.label(RichText::new(&i.desc).size(30.0));
                        ui.add_space(PADDING);
                        ui.with_layout(Layout::right_to_left(egui::Align::Min), |ui| {
                            ui.hyperlink_to(RichText::new("read more >").size(30.0), &i.url);
                        });
                        ui.add_space(PADDING);
                        ui.add(Separator::default());
                    }

                    ui_counter(ui, &mut c);
                })
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
