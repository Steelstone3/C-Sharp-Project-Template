use presenters::dive_setup::{welcome_message, select_dive_model, create_cylinders};

mod commands;
mod controllers;
mod models;
mod presenters;

fn main() {
    welcome_message();
    select_dive_model();
    create_cylinders();
}
