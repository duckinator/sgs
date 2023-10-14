use nanoserde::{DeJson, SerJson};



#[derive(Clone, Debug, DeJson, SerJson, PartialEq)]
pub struct Button {
    pub label: String,
    pub pronunciation: Option<String>,
    pub image: Option<String>,
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
