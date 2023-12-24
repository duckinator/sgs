use crate::button::Button;

use nanoserde::{DeJson, DeJsonErr, SerJson};

#[derive(Clone, Debug, Default, DeJson, SerJson, PartialEq)]
pub struct Folder {
    pub name: String,
    pub default: bool,
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
}

impl Folder {
    pub fn get_button(&self, col: usize, row: usize) -> Option<&Button> {
        self.buttons[row * self.cols + col].as_ref()
    }
}

impl System {
    pub fn load_file(path: &str) -> Self {
        std::fs::read_to_string(path).map(|contents|
            Self::load_str(&contents).unwrap()
        ).unwrap()
    }

    pub fn load_str(json: &str) -> Result<System, DeJsonErr> {
        DeJson::deserialize_json(json)
    }

    pub fn default_folder(&self) -> usize {
        self.folders.iter().position(|folder| folder.default).expect("No default Folder defined.")
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
                "default": true,
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
                "default": false,
                "immediate": true,
                "rows": 1,
                "cols": 1,
                "buttons": [
                    {"label": "Hey!"}
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
        }
    }"#;

    let system: System = DeJson::deserialize_json(json).unwrap();
    assert_eq!(0, system.default_folder());
    let folder = &system.folders[system.default_folder()];

    assert_eq!("Home", folder.name);
    assert_eq!(true, folder.default);
    assert_eq!(false, folder.immediate);

    assert_eq!("hello", folder.buttons[0].as_ref().unwrap().label);
    assert_eq!("world", folder.buttons[1].as_ref().unwrap().label);
    assert_eq!("what", folder.buttons[4 * 6 - 1].as_ref().unwrap().label);

    let quick_folder = &system.folders[1];
    assert_eq!("Quick Response", quick_folder.name);
    assert_eq!(false, quick_folder.default);
    assert_eq!(true, quick_folder.immediate);
    assert_eq!("Hey!", quick_folder.buttons[0].as_ref().unwrap().label);

    let hotbar = &system.hotbar;
    assert_eq!(2, hotbar.rows);
    assert_eq!(2, hotbar.cols);
    assert_eq!("a", hotbar.buttons[0].as_ref().unwrap().label);
}
