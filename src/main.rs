extern crate serde_json;

use serde_json::Value;

use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {

    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("No json file given as argument");
        return;
    }

    let filename : &str = &args[1];

    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let obj: Value = serde_json::from_str(&contents).expect("could not parse json");

    println!("{}",serde_json::to_string_pretty(&obj).expect("could not pretty print json"));

}
