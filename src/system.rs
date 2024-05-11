use crate::button::Button;

use nanoserde::{DeJson, DeJsonErr, SerJson};

use std::collections::hash_map::HashMap;

#[derive(Clone, Debug, Default, DeJson, SerJson, PartialEq)]
pub struct Folder {
    pub name: String,
    pub toplevel: bool,
    pub immediate: bool,
    pub rows: usize,
    pub cols: usize,
    pub buttons: Vec<Option<Button>>,
}

#[derive(Clone, Debug, Default, DeJson, SerJson, PartialEq)]
pub struct Hotbar {
    pub rows: usize,
    pub cols: usize,
    pub buttons: Vec<Option<Button>>,
}

#[derive(Clone, Debug, Default, DeJson, SerJson)]
pub struct System {
    pub name: String,
    pub description: String,
    pub folders: Vec<Folder>,
    pub hotbar: Hotbar,
    pub variants: HashMap<String, Vec<Button>>,
    pub related: HashMap<String, Vec<Button>>,
}

impl Folder {
    pub fn get_button(&self, col: usize, row: usize) -> Option<&Button> {
        if let Some(btn) = self.buttons.get(row * self.cols + col) {
            return btn.as_ref();
        } else {
            return None;
        }
    }
}

impl System {
    pub fn load_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = std::fs::read_to_string(path)?;
        Ok(Self::load_str(&contents)?)
    }

    pub fn load_str(json: &str) -> Result<System, DeJsonErr> {
        DeJson::deserialize_json(json)
    }

    pub fn toplevel_folders(&self) -> Vec<&Folder> {
        let mut folders = vec![];
        for folder in &self.folders {
            if folder.toplevel {
                folders.push(folder);
            }
        }

        folders
    }
}

#[test]
fn test_system() {
    let json = r#"{
        "name": "Example System",
        "description": "This is an example system.",
        "folders": [
            {
                "name": "Home",
                "toplevel": true,
                "immediate": false,
                "rows": 4,
                "cols": 6,
                "buttons": [
                    {"label": "hello"}, {"label": "world"}, null, null, null, null,
                    null, null, null, null, null, null,
                    null, null, null, null, null, null,
                    null, null, null, null, null, {"label": "what"},
                ]
            },
            {
                "name": "Quick Response",
                "toplevel": true,
                "immediate": true,
                "rows": 1,
                "cols": 1,
                "buttons": [
                    {"label": "Hey!"}
                ]
            },
            {
                "name": "Hidden Folder",
                "toplevel": false,
                "immediate": false,
                "rows": 1,
                "cols": 1,
                "buttons": [
                    {"label": "this folder is hidden"},
                ]
            }
        ],
        "hotbar": {
            "rows": 2,
            "cols": 2,
            "buttons": [
                {"label": "a"}, {"label": "the"},
                null, null
            ]
        },
        "variants": {
            "hello": [
                {"label": "hello"},
                {"label": "Hello!"}
            ],
            "hey": [
                {"label": "hey"},
                {"label": "Hey!"}
            ]
        },
        "related": {
            "hello": [
                {"label": "hello"},
                {"label": "hi"},
                {"label": "hey"}
            ],
            "world": [
                {"label": "world"},
                {"label": "planet"}
            ]
        }
    }"#;

    let system: System = DeJson::deserialize_json(json).unwrap();

    assert_eq!(true, system.folders[0].toplevel);
    assert_eq!(true, system.folders[1].toplevel);
    assert_eq!(system.toplevel_folders(), vec![&system.folders[0], &system.folders[1]]);

    let folder = &system.folders[0];
    assert_eq!("Home", folder.name);
    assert_eq!(false, folder.immediate);
    assert_eq!("hello", folder.buttons[0].as_ref().unwrap().label);
    assert_eq!("world", folder.buttons[1].as_ref().unwrap().label);
    assert_eq!("what", folder.buttons[4 * 6 - 1].as_ref().unwrap().label);

    let quick_folder = &system.folders[1];
    assert_eq!("Quick Response", quick_folder.name);
    assert_eq!(true, quick_folder.immediate);
    assert_eq!("Hey!", quick_folder.buttons[0].as_ref().unwrap().label);

    let hotbar = &system.hotbar;
    assert_eq!(2, hotbar.rows);
    assert_eq!(2, hotbar.cols);
    assert_eq!("a", hotbar.buttons[0].as_ref().unwrap().label);

    assert_eq!("Hello!", &system.variants["hello"][1].label);
    assert_eq!("hey", &system.related["hello"][2].label);
}
