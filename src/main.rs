use presenters::presenter::{welcome_message, select_dive_model, create_cylinder};

mod commands;
mod controllers;
mod models;
mod presenters;

fn main() {
    welcome_message();
    select_dive_model();
    create_cylinder();

}
