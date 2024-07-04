mod models;
use models::Applicant::*;
use std::env::args;

fn main() {
    let comm: Vec<String> = args().collect();
    println!("Hello, world! {comm:#?}");
}
