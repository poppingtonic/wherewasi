use std::env;
use std::fs::OpenOptions;
use std::io::{stdin};
use std::io::prelude::*;

fn main() {
    println!("Remember to do this when you restart your machine \n\n (Press {} when finished)\n", EOF);
    let mut note = String::new();
    stdin().read_line(&mut note).unwrap();

    let note = &note[..(note.len() - 1)];
    let home_dir = env::home_dir().expect("could not determine a home directory");
    let path = home_dir.join(".wherewasi");
    println!("The file is now: {}", &path.display());

    let mut file =
        OpenOptions::new()
        .append(true)
        .create(true)
        .open(path.as_os_str())
        .unwrap();

    if let Err(e) = writeln!(file, "{}", note) {
        println!("{}", e);
    }
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";
