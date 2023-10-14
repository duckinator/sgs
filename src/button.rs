use std::str::FromStr;
use nanoserde::{DeJson, SerJson};

#[derive(Clone, Copy, Debug, DeJson, SerJson, PartialEq)]
pub enum Action {
    Speak,
    Append,
}

impl FromStr for Action {
    type Err = String;

    fn from_str(input: &str) -> Result<Action, Self::Err> {
        match input {
            "Speak" => Ok(Action::Speak),
            "Append" => Ok(Action::Append),
            _ => Err(format!("unknown Action: {}", input)),
        }
    }
}


#[derive(Clone, Debug, DeJson, SerJson, PartialEq)]
pub struct Button {
    pub label: String,
    pub pronunciation: Option<String>,
    pub image: Option<String>,
    pub action: Action,
}

impl Button {
    pub fn get_pronouncible_text(&self) -> String {
        if let Some(pronunciation) = self.pronunciation.clone() {
            pronunciation
        } else {
            self.label.clone()
        }
    }
}
