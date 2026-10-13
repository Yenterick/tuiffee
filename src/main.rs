use dotenvy::dotenv;

mod config;
mod models;
mod presentation;
mod repositories;
mod services;

fn main() {
    dotenv().ok();

    println!("Hello, world!");
}
