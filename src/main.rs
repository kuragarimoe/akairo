use sekkei::game::Mods;
use std::{
        env,
        path::{Path, PathBuf},
};

mod game;
mod calculator;

/// UTIL FUNCTIONS ///

fn get_path<P: AsRef<Path>>(file: P) -> PathBuf {
    let dir = env::current_dir().unwrap();

    dir.join(file)
}

fn without_first(string: &str) -> &str {
    string
        .char_indices()
        .next()
        .and_then(|(i, _)| string.get(i + 1..))
        .unwrap_or("")
}

fn split_by(string: &str, n: usize) -> Vec<String> {
    string
        .chars()
        .collect::<Vec<char>>()
        .chunks(n)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<String>>()
}

/// MAIN ///
fn main() {
    // args
    let args: Vec<String> = env::args().collect();

    // data
    let mut mods = Mods::NoMod as i32;
    let mut acc: f32 = 100.0;
    let mut combo: i32 = -1;
    let mut misses: i32 = 0;

    // for loop possible args
    let mut iter = args.iter();
    let arg_type;

    // ignore first result
    &iter.next();

    arg_type = match iter.next().map(|s| &**s) {
        Some("difficulty") => "d",
        Some("performance") => "p",
        Some(_) => {
                eprintln!("Invalid calculator type");
                return;
        },
        None => {
                eprintln!("Too few arguments");
                return;
        },
    };

    println!("{}", arg_type);

    let file_path = if let Some(path) = iter.next() {
        get_path(path)
    } else {
        eprintln!("Too few arguments");
        return;
    };

    for item in iter {
        if item.starts_with("+") {
            // mods
            let string = without_first(item.as_str());

            for modstr in split_by(string, 2) {
                mods += Mods::from_str(modstr) as i32;
            }
        } else if item.ends_with("%") {
            if arg_type == "d" {
                // ignore because type is difficulty
                continue;
            }

            // accuracy
            // remove first char
            let mut string = item.clone();
            string.pop();

            // parse
            acc = string.parse().unwrap_or(100.0);
        } else if item.ends_with("x") {
            if arg_type == "d" {
                // ignore because type is difficulty
                continue;
            }

            // combo
            // remove first char
            let mut string = item.clone();
            string.pop();

            // parse
            combo = string.parse().unwrap_or(-1);
        } else if item.ends_with("m") {
            if arg_type == "d" {
                // ignore because type is difficulty
                continue;
            }

            // misses
            // remove first char
            let mut string = item.clone();
            string.pop();

            // parse
            misses = string.parse().unwrap_or(0);
        }
    }

    println!("{:?} {} {} {} {}", file_path, mods, acc, combo, misses);
}
