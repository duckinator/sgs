use crate::button::{Button, Action};
use crate::speech::SpeechEngine;

pub struct Panel {
    entries: Vec<Button>,
}

impl Panel {
    pub fn new() -> Self {
        Self { entries: vec![] }
    }

   pub fn add_entry(&mut self, button: &Button) {
        if button.action != Action::Append {
            panic!("Panel::add_entry() should only be used with buttons that have the type of Action::Append");
        }

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

    // FIXME: Where the fuck should this go?
    // It modifies a Panel, but also uses a Button and a SpeechEngine.
    pub fn apply_button(&mut self, button: &Button, speech_engine: &mut SpeechEngine) {
        match &button.action {
            Action::Speak => speech_engine.speak(button.get_pronouncible_text()),
            Action::SpeakBuiltPhrase => {
                speech_engine.speak(self.get_pronouncible_text());
                self.clear();
            },
            Action::Append => self.add_entry(button),
            Action::SelectBoard => panic!("board selection not implemented"),
            Action::RemoveLast => self.remove_last_entry(),
        }
    }

}

#[test]
fn test_panel() {
    let foo = Button { label: "foo".to_string(), pronunciation: None, image: None, action: Action::Append };
    let bar = Button { label: "bar".to_string(), pronunciation: None, image: None, action: Action::Append };
    let baz = Button { label: "baz".to_string(), pronunciation: None, image: None, action: Action::Append };
    let exc = Button { label: "!".to_string(), pronunciation: None, image: None, action: Action::Append };
    let delete = Button { label: "[Backspace]".to_string(), pronunciation: None, image: None, action: Action::RemoveLast };

    let mut panel = Panel::new();

    panel.apply_button(&foo);
    panel.apply_button(&bar);
    panel.apply_button(&baz);
    panel.apply_button(&exc);
    panel.apply_button(&exc);
    assert_eq!("foo bar baz ! !", panel.get_text());

    panel.apply_button(&delete);
    assert_eq!("foo bar baz !", panel.get_text());

    panel.remove_last_entry();
    assert_eq!("foo bar baz", panel.get_text());

    panel.remove_last_entry();
    assert_eq!("foo bar", panel.get_text());
}
