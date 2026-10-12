use dotenvy::dotenv;

mod config;
mod models;

fn main() {
    dotenv().ok();

    println!("Hello, world!");
}
