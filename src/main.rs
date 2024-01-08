mod game_of_life;
//use egui;
use game_of_life::game::*;
use std::{thread::sleep, time::Duration};

fn main() {
    let mut ctx = egui::Context::default();
    loop {
        egui::Window::new("My Window").show(&ctx, |ui| {
            ui.label("Hello World!");
        });
    }
    let mut universe = Universe::new(40, 40);

    universe.edit_cell((20, 20), Cell::Alive);
    universe.edit_cell((21, 21), Cell::Alive);
    universe.edit_cell((22, 21), Cell::Alive);
    universe.edit_cell((22, 20), Cell::Alive);
    universe.edit_cell((22, 22), Cell::Alive);

    let mut i = 0;
    loop {
        println!("{i}");
        //println!("{universe}");
        universe = universe.iterate();
        i += 1;
        sleep(Duration::from_millis(200));
    }
}
