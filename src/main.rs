use std::fs;

use clearscreen;

use sgs::board::Board;
use sgs::panel::Panel;
use sgs::speech::SpeechEngine;

fn main() {
    let path = "board.json";
    let mut board: Board = fs::read_to_string(path).map(|contents|
        Board::load_str(&contents).unwrap()
    ).unwrap();

    let speech_engine = SpeechEngine::new();
    let mut panel = Panel::new();
    let mut input = String::new();

    loop {
        render(&panel, &board);
        read(&mut input);

        if input.trim() == "q" { return; }

        match process(&mut panel, &speach_engine, &board, &input) {
            Ok(b) => { board = b },
            Err(s) => println!("ERROR: {}", s),
        }

        input.clear()
    }
}

fn render(panel: &Panel, board: &Board) {
    let built = board.build().unwrap();

    clearscreen::clear().unwrap();

    println!("TEXT: {}", panel.get_text());
    println!();

    print!("{:^12} |", "");
    for c in 0..built.layout.cols {
        print!("{:^12} |", c);
    }
    println!();

    for r in 0..built.layout.rows {
        print!("{:^12} |", r);
        for c in 0..built.layout.cols {
            let idx = (r * built.layout.cols) + c;
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

fn read(input: &mut String) {
    std::io::stdin().read_line(input).unwrap();
}

fn process<'a>(panel: &'a mut Panel, speach_engine: &SpeachEngine, board: &'a Board, input: &'a String) -> Result<Board, &'static str> {
    let built = board.build().unwrap();

    let words: Vec<&str> = input.split_whitespace().collect();
    let len = words.len();

    if len != 0 && len != 2 {
        return Err("expected `<col> <row>`, e.g. `2 3`")
    }

    if len != 0 {
        let col_str = words[0];
        let row_str = words[1];

        let col: usize = col_str.parse().map_err(|_| "column (first argument) was not a number")?;
        let row: usize = row_str.parse().map_err(|_| "row (second argument) was not a number")?;

        let btn = built.get_button(col, row).ok_or("no such button")?;

        panel.apply_button(btn, speech_engine);
    }

    Ok(board.clone())
}
