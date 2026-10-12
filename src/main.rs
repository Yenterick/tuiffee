use dotenvy::dotenv;

mod config;
mod models;
mod repositories;

fn main() {
    dotenv().ok();

    println!("Hello, world!");
}
