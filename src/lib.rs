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

#[derive(Debug)]
pub struct Formula {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Formula {
    pub fn new(symbol: Symbol, difficulty: &Difficulty) -> Formula {
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
            match difficulty {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let symbol = Symbol::Add;
        let difficulty = Difficulty::Normal;
        let formual = Formula::new(symbol, &difficulty);
        println!("formual: {:?}", formual);
    }
}
