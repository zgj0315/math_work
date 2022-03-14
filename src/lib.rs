use rand::prelude::*;
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

pub fn make_formula(symbol: Symbol) -> Formula {
    let mut rng = thread_rng();
    let x = rng.gen_range(10..99);
    let y = rng.gen_range(10..99);
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
