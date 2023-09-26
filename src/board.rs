use std::collections::HashMap;
use nanoserde::{DeJson, SerJson};

#[derive(Clone, Debug, DeJson, SerJson, PartialEq)]
pub struct Button {
    pub label: String,
    pub pronunciation: Option<String>,
    pub image: Option<String>,
    pub action: Option<String>,
}

#[derive(Clone, Debug, Default, DeJson, SerJson)]
pub struct Layout {
    pub rows: usize,
    pub cols: usize,
    pub order: Vec<Option<String>>,
}

#[derive(Clone, Debug, Default, DeJson, SerJson)]
pub struct Board {
    #[nserde(default = "sgs-board")]
    format: String,
    #[nserde(default = 0)]
    format_version: usize,
    pub name: String,
    pub description: String,
    pub buttons: HashMap<String, Button>,
    pub images: HashMap<String, String>,
    pub layout: Layout,
}

pub struct PopulatedLayout<'a> {
    pub layout: &'a Layout,
    pub buttons: Vec<Option<&'a Button>>
}

impl PopulatedLayout<'_> {
    pub fn get_button(&self, col: usize, row: usize) -> Option<&Button> {
        self.buttons[row * self.layout.cols + col]
    }
}

impl Board {
    pub fn get_button(&self, name: &str) -> Option<&Button> {
        self.buttons.get(name)
    }

    pub fn get_image_location(&self, name: &str) -> Option<&String> {
        self.images.get(name)
    }

    pub fn get_button_image(&self, name: &str) -> Option<&String> {
        let button = self.get_button(name)?;
        let image_name: &String = button.image.as_ref()?;

        self.get_image_location(image_name)
    }

    fn get_button_from_layout(&self, index: usize) -> Option<&Button> {
        let name = &self.layout.order[index];

        self.get_button(name.as_ref()?)
    }

    pub fn build(&self) -> Option<PopulatedLayout> {
        let layout = &self.layout;
        let size: usize = layout.rows as usize * layout.cols as usize;
        let mut buttons = vec![None; size];

        assert!(layout.order.len() == size);
        assert!(layout.order.len() > 0);

        for r in 0..layout.rows {
            for c in 0..layout.cols {
                let index = ((r * layout.cols) + c) as usize;
                buttons[index] = self.get_button_from_layout(index);
            }
        }

        Some(PopulatedLayout { layout, buttons })
    }
}

#[test]
fn test_board() {
    let json = r#"{
        "format": "sgs-board",
        "format_version": 0,
        "name": "Example Board",
        "description": "This is an example board.",
        "buttons": {
            "happy": {
                "label": "happy",
                "image": null
            },
            "what": {
                "label": "what",
                "image": "what"
            }
        },
        "images": {
            "what": "what.png"
        },
        "layout": {
            "rows": 4,
            "cols": 6,
            "order": [
                "happy", "what", null, null, null, null,
                null, null, null, null, null, null,
                null, null, null, null, null, null,
                null, null, null, null, null, "what"
            ]
        }
    }"#;

    let board: Board = DeJson::deserialize_json(json).unwrap();
    assert_eq!(board.name, "Example Board");
    assert_eq!(board.buttons["happy"].label, "happy");
    assert_eq!(board.buttons["what"].label, "what");
    assert_eq!(board.buttons["what"].image, Some("what".to_string()));
    assert_eq!(board.images["what"], "what.png");

    let built = board.build().unwrap();
    assert_eq!(built.buttons[0], Some(&board.buttons["happy"]));
    assert_eq!(built.buttons[1], Some(&board.buttons["what"]));
    assert_eq!(built.buttons[4 * 6 - 1], Some(&board.buttons["what"]));
}

#[test]
pub fn test_populated_layout() {
    let a = Some(Button { label: "a".to_string(), pronunciation: None, image: None, action: None });
    let c = Some(Button { label: "c".to_string(), pronunciation: None, image: None, action: None });
    let d = Some(Button { label: "d".to_string(), pronunciation: None, image: None, action: None });
    let e = Some(Button { label: "e".to_string(), pronunciation: None, image: None, action: None });

    let layout = &Layout {
        rows: 2,
        cols: 3,
        order: vec![Some("a".to_string()), None, Some("c".to_string()),
                    Some("d".to_string()), Some("e".to_string()), None],
    };
    let buttons = vec![a.as_ref(), None, c.as_ref(), d.as_ref(), e.as_ref(), None];
    let built = PopulatedLayout { layout, buttons };

    assert_eq!(built.buttons.len(), (built.layout.rows * built.layout.cols).try_into().unwrap());

    assert_eq!(&built.buttons[0..built.layout.cols], vec![a.as_ref(), None, c.as_ref()]);
}
