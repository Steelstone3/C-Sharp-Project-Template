use presenters::presenter::{create_cylinders, select_dive_model, welcome_message};

mod commands;
mod controllers;
mod models;
mod presenters;

fn main() {
    welcome_message();
    select_dive_model();
    create_cylinders();
}
