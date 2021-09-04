use std::env;
use std::process;
use std::fs::File;
use std::io::{self, BufRead};

fn read_file_lines(filename: &String) -> Result<Vec<String>, io::Error> {
    let file = File::open(filename)?;
    let mut v = vec![];

    for line in io::BufReader::new(file).lines() {
        let line_str = line?;
        v.push(line_str);
    }

    Ok(v)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename = &args[1];
    let lines: Vec<String> = read_file_lines(filename).expect("Can not open file");
    println!("Number of lines = {}", lines.len());

    let mut cnt_word: usize = 0;
    let mut cnt_char: usize = 0;

    for line in lines.iter() {
        for word in line.split(" ") {
            cnt_word += 1;
            cnt_char += word.len();
        }
    }

    println!("Nuber of words = {}", cnt_word);
    println!("Nuber of characters = {}", cnt_char);
}
