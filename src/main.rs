mod cli_style;
mod csv_format;
use crate::cli_style::{print_info, Args};
use crate::csv_format::{read_csv, write_csv};
use clap::Parser;
use std::io;
mod modules;

fn main() {
    let args = Args::parse();
    let mut buf = String::new();
    let mut filep = String::new();
    println!("Write your .csv file-name: ");
    io::stdin()
        .read_line(&mut filep)
        .expect("Error reading input!");

    loop {
        buf.clear();
        println!("Enter your array elements (To stop type this: END): ");
        io::stdin()
            .read_line(&mut buf)
            .expect("Error reading input!");
        if buf.trim() == "END" {
            std::process::exit(0);
        }
        if buf.trim() == "\n" || filep.trim() == "\n" {
            panic!("Write NOT empty data!");
        }

        let vec: Vec<f64> = buf
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Please enter valid numbers"))
            .collect();

        let info = print_info(vec, &args);

        if let Err(e) = write_csv(&filep, info) {
            println!("Operation failed: {}", e);
        } else {
            println!("Operation succeeded\n");
        }

        if let Err(e) = read_csv(&filep) {
            println!("Operation failed: {}", e);
        } else {
            println!("Operation succeeded. Writed message to csv.\n");
        }
    }
}
