mod day_01;
mod day_02;
use std::{fs, str};

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Day of the program
    #[arg(short, long)]
    pub day: u8,

    /// Exercice number
    #[arg(short, long)]
    pub exercice: u8,

    /// path to the file
    #[arg(short, long)]
    pub filename: String,
}

fn main() {
    let args = Args::parse();

    let file = fs::read(args.filename).expect("cannot open file");

    let content = str::from_utf8(&file).expect("cannot convert to utf8");

    match (args.day, args.exercice) {
        (0, 0) => day_01::max_calories(content),
        (0, 1) => day_01::top_three(content),
        (1, 0) => day_02::count_points(content),
        (1, 1) => day_02::count_points_with_result(content),
        _ => unreachable!(),
    }
}
