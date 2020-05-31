use std::fs::File;
//use std::io::Read;
use std::env;

extern crate serde_json;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1]).expect("file should open read only");
    let json: serde_json::Value = serde_json::from_reader(file).expect("file should be proper JSON");
    println!("Please call {} at the number {}", json["magic_number"], json["instructions"][0]["opname"]);
}
