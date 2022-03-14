use chrono::prelude::*;
use math_work::{make_formula, Symbol};
use std::{fs, io::Write};
fn main() {
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
        let add = make_formula(Symbol::Add);
        let subtract = make_formula(Symbol::Subtract);
        let multiply = make_formula(Symbol::Multiply);
        let divide = make_formula(Symbol::Divide);
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
