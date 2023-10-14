use crate::button::Button;
use crate::speech::SpeechEngine;

pub struct Panel {
    pub entries: Vec<Button>,
}

impl Panel {
    pub fn new() -> Self {
        Self { entries: vec![] }
    }

    pub fn add_entry(&mut self, button: &Button) {
        self.entries.push(button.clone());
    }

    pub fn remove_last_entry(&mut self) {
        self.entries.pop();
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn get_text(&self) -> String {
        self.entries.iter().map(|e| e.label.clone()).collect::<Vec<_>>().join(" ")
    }

    pub fn get_pronouncible_text(&self) -> String {
        self.entries.iter().map(|e| e.get_pronouncible_text()).collect::<Vec<_>>().join(" ")
    }

    pub fn speak(&mut self, speech_engine: &mut SpeechEngine) -> Result<(), String> {
        speech_engine.speak(self.get_pronouncible_text())?;
        self.clear();
        Ok(())
    }
}

#[test]
fn test_panel() {
    let foo = Button { label: "foo".to_string(), pronunciation: None, image: None };
    let bar = Button { label: "bar".to_string(), pronunciation: None, image: None };
    let baz = Button { label: "baz".to_string(), pronunciation: None, image: None };
    let exc = Button { label: "!".to_string(), pronunciation: None, image: None };

    let mut panel = Panel::new();

    panel.add_entry(&foo);
    panel.add_entry(&bar);
    panel.add_entry(&baz);
    panel.add_entry(&exc);
    panel.add_entry(&exc);
    assert_eq!("foo bar baz ! !", panel.get_text());

    panel.remove_last_entry();
    panel.remove_last_entry();
    assert_eq!("foo bar baz", panel.get_text());

    panel.remove_last_entry();
    assert_eq!("foo bar", panel.get_text());
}
