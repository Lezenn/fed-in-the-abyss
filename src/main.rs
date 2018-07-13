extern crate noise;

use noise::{Perlin, NoiseFn};
use std::fs::File;
use std::io::Write;
use std::error::Error;
use std::fs::OpenOptions;
use std::path::Path;
use std::env;

fn write_in_file(file_name: String, content: String) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(Path::new(&file_name.trim()))
        .unwrap();
    if let Err(why) = writeln!(file, "{}", content) {
        eprintln!("Could not write to file : {}", why.description());
    }
}

fn generate_map(file: String, seed: i32, size: i32) {
    let perlin = Perlin::new();

    let _file = File::create(file.as_str());

    let mut line: String = String::new();

    for i in 0..size {
        for j in 0..size {
            let x = i as f64 / 10.0;
            let y = j as f64 / 10.0;
            let z = seed as f64 / 10.0;
            let value = (perlin.get([x, y, z])).abs();

            line.push(parser(value));
            line.push(' ');
        }
        write_in_file(file.clone(), line);
        line = String::new();
    }
}

fn parser(value: f64) -> char {
    if value < 0.025 {
        '~' // Water
    } else if value >= 0.025 && value <= 0.7 {
        '.' // Dirt
    } else {
        '%' // Grass
    }
}

fn main() {

    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);

    println!("\n  *****************************");
    println!("  -===== Level Generator =====-");
    println!("  *****************************\n");
    /*
    println!("Enter file path, seed and size : ");

    let mut file = String::new();
    let mut seed = String::new();
    let mut size = String::new();

    std::io::stdin().read_line(&mut file).expect("Could not read line");
    std::io::stdin().read_line(&mut seed).expect("Could not read line");
    std::io::stdin().read_line(&mut size).expect("Could not read line");
    let passed_seed: i32 = seed.trim().parse().expect("Couldn't parse from String to Int");
    let passed_size: i32 = size.trim().parse().expect("Couldn't parse from String to Int");

    generate_map(file, passed_seed, passed_size);*/
}
