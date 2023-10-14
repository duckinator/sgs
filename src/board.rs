use crate::button::Button;

use std::collections::HashMap;
use nanoserde::{DeJson, DeJsonErr, SerJson};

#[derive(Clone, Debug, Default, DeJson, SerJson, PartialEq)]
pub struct Layout {
    pub name: String,
    pub default: bool,
    pub immediate: bool,
    pub rows: usize,
    pub cols: usize,
    pub buttons: Vec<Option<Button>>,
}

#[derive(Clone, Debug, Default, DeJson, SerJson)]
pub struct Board {
    #[nserde(default = "sgs-board")]
    format: String,
    #[nserde(default = 0)]
    format_version: usize,
    pub name: String,
    pub description: String,
    pub images: HashMap<String, String>,
    pub layouts: Vec<Layout>,
}

impl Layout {
    pub fn get_button(&self, col: usize, row: usize) -> Option<&Button> {
        self.buttons[row * self.cols + col].as_ref()
    }
}

impl Board {
    /*pub fn load_file(path: &str) -> Board {
        std::fs::read_to_string(path).map(|contents|
            Board::load_str(&contents).unwrap()
        ).unwrap()
    }*/

    pub fn load_str(json: &str) -> Result<Board, DeJsonErr> {
        DeJson::deserialize_json(json)
    }

    pub fn get_image_location(&self, name: &str) -> Option<&String> {
        self.images.get(name)
    }

    pub fn default_layout(&self) -> usize {
        self.layouts.iter().position(|layout| layout.default).expect("No default layout defined.")
    }
}

#[test]
fn test_board() {
    let json = r#"{
        "format": "sgs-board",
        "format_version": 0,
        "name": "Example Board",
        "description": "This is an example board.",
        "images": {
            "what-img": "what.png"
        },
        "layouts": [
            {
                "name": "Home",
                "default": true,
                "immediate": false,
                "rows": 4,
                "cols": 6,
                "buttons": [
                    {"label": "hello"}, {"label": "world"}, null, null, null, null,
                    null, null, null, null, null, null,
                    null, null, null, null, null, null,
                    null, null, null, null, null, {"label": "what", "image": "what-img"},
                ]
            },
            {
                "name": "Quick Response",
                "default": false,
                "immediate": true,
                "rows": 1,
                "cols": 1,
                "buttons": [
                    {"label": "Hey!"}
                ]
            }
        ]
    }"#;

    let board: Board = DeJson::deserialize_json(json).unwrap();
    assert_eq!(0, board.default_layout());
    let layout = &board.layouts[board.default_layout()];

    assert_eq!("Home", layout.name);
    assert_eq!(true, layout.default);
    assert_eq!(false, layout.immediate);

    assert_eq!("hello", layout.buttons[0].as_ref().unwrap().label);
    assert_eq!("world", layout.buttons[1].as_ref().unwrap().label);
    assert_eq!("what", layout.buttons[4 * 6 - 1].as_ref().unwrap().label);
    assert_eq!(Some("what-img".to_string()), layout.buttons[4 * 6 - 1].as_ref().unwrap().image);

    let quick_layout = &board.layouts[1];
    assert_eq!("Quick Response", quick_layout.name);
    assert_eq!(false, quick_layout.default);
    assert_eq!(true, quick_layout.immediate);
    assert_eq!("Hey!", quick_layout.buttons[0].as_ref().unwrap().label);
}
