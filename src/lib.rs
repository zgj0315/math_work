use rand::prelude::*;

pub enum Difficulty {
    Easy,
    Normal,
    Hard,
}
pub enum Symbol {
    Add,
    Subtract,
    Multiply,
    Divide,
}

pub struct Formula {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

pub fn make_formula(symbol: Symbol, _difficulty: &Difficulty) -> Formula {
    let mut rng = thread_rng();
    let mut x: i32;
    let mut y: i32;
    loop {
        x = rng.gen_range(10..99);
        y = rng.gen_range(10..99);
        let x_tens = x / 10;
        let x_ones = x % 10;
        let y_tens = y / 10;
        let y_ones = y % 10;
        match _difficulty {
            Difficulty::Easy => match symbol {
                Symbol::Add | Symbol::Subtract => {
                    if (x_ones + y_ones) < 10 && (x_tens + y_tens) < 10 {
                        break;
                    }
                }
                Symbol::Multiply | Symbol::Divide => {
                    if (x_ones * y_ones) < 10 && (x_tens * y_tens) < 10 {
                        break;
                    }
                }
            },
            Difficulty::Normal => {
                break;
            }
            Difficulty::Hard => match symbol {
                Symbol::Add | Symbol::Subtract => {
                    if (x_ones + y_ones) >= 10 && (x_tens + y_tens) >= 10 {
                        break;
                    }
                }
                Symbol::Multiply | Symbol::Divide => {
                    if (x_ones * y_ones) >= 10 && (x_tens * y_tens) >= 10 {
                        break;
                    }
                }
            },
        }
    }
    match symbol {
        Symbol::Add | Symbol::Subtract => {
            let z = x + y;
            Formula { x, y, z }
        }
        Symbol::Multiply | Symbol::Divide => {
            let z = x * y;
            Formula { x, y, z }
        }
    }
}
