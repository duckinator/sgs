use std::fs;

use nanoserde::{DeJson};

// TODO: Drag stuff to lib.rs and similar eventually.
//use sgs::*;

pub mod obf;
//use obf::Board;

pub mod board;
use board::Board;

fn main() {
    let path = "board.json";
    let board: Board = fs::read_to_string(path).map(|contents|
        DeJson::deserialize_json(&contents).unwrap()
    ).unwrap();

    let built = board.build().unwrap();

    for r in 0..built.layout.rows {
        print!("| ");
        for c in 0..built.layout.cols {
            let idx = (r * built.layout.cols) + c;
            //println!("({} * {}) + {} = {}", r, c, c, idx);
            let label =
                match built.buttons[idx] {
                    Some(button) => button.label.clone(),
                    _           => "".to_string(),
                };
            print!("{:^12} |", label);
        }
        println!();
    }
}
