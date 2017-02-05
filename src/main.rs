use std::fs::OpenOptions;
use std::io::{stdin};
use std::io::prelude::*;

fn main() {
    println!("Remember to do this when you restart your machine \n\n (Press {} when finished)\n", EOF);
    let mut note = String::new();
    stdin().read_line(&mut note).unwrap();

    let note = &note[..(note.len() - 1)];

    let mut file =
        OpenOptions::new()
        .append(true)
        .open("/home/brian/.wherewasi")
        .unwrap();

    if let Err(e) = writeln!(file, "{}", note) {
        println!("{}", e);
    }
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";
