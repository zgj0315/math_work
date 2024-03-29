use chrono::prelude::*;
use clap::{arg, command};
use math_work::{Difficulty, Formula, Symbol};
use std::{env, fs, io::Write};
fn main() {
    let matchs = command!()
        .arg(arg!(-l --level <VALUE>"The level to math").required(true))
        .get_matches();
    let mut difficulty = Difficulty::Normal;
    if let Some(level) = matchs.get_one::<String>("level") {
        println!("level: {}", &level);
        if level == "easy" {
            difficulty = Difficulty::Easy;
        } else if level == "hard" {
            difficulty = Difficulty::Hard;
        }
    }

    fs::create_dir_all("./data").unwrap();
    let local: DateTime<Local> = Local::now();
    let file_path_formula = format!(
        "./data/formula_{}{:02}{:02}.csv",
        local.year(),
        local.month(),
        local.day()
    );
    let file_path_formula_result = format!(
        "./data/formula_{}{:02}{:02}_result.csv",
        local.year(),
        local.month(),
        local.day()
    );
    let mut file_formula = fs::File::create(file_path_formula).expect("create failed");
    let mut file_formula_result =
        fs::File::create(file_path_formula_result).expect("create failed");

    for _n in 1..14 {
        let add = Formula::new(Symbol::Add, &difficulty);
        let subtract = Formula::new(Symbol::Subtract, &difficulty);
        let multiply = Formula::new(Symbol::Multiply, &difficulty);
        let divide = Formula::new(Symbol::Divide, &difficulty);
        let line = format!(
            "{} + {} =,{} - {} =,{} * {} =,{} / {} =\n",
            add.x, add.y, subtract.z, subtract.x, multiply.x, multiply.y, divide.z, divide.x,
        );
        let line_result = format!(
            "{} + {} = {},{} - {} = {},{} * {} = {},{} / {} = {}\n",
            add.x,
            add.y,
            add.z,
            subtract.z,
            subtract.x,
            subtract.y,
            multiply.x,
            multiply.y,
            multiply.z,
            divide.z,
            divide.x,
            divide.y
        );
        file_formula
            .write_all(line.as_bytes())
            .expect("write failed");
        file_formula_result
            .write_all(line_result.as_bytes())
            .expect("write failed");
    }
}
