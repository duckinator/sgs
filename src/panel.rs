use crate::button::Button;
use crate::system::System;

#[derive(Default)]
pub struct Panel {
    pub entries: Vec<Button>,
}

impl Panel {
    pub fn add_entry(&mut self, button: &Button) {
        self.entries.push(button.clone());
    }

    pub fn last_entry(&mut self) -> Option<&Button> {
        if self.entries.len() == 0 {
            None
        } else {
            let idx = self.entries.len() - 1;
            Some(&self.entries[idx])
        }
    }

    pub fn last_entry_label(&mut self) -> Option<String> {
        if self.entries.len() == 0 {
            None
        } else {
            Some(self.entries[self.entries.len() - 1].label.clone())
        }
    }

    pub fn last_entry_related_label(&mut self, system: &System) -> Option<String> {
        if self.entries.len() == 0 {
            None
        } else {
            Some(self.entries[self.entries.len() - 1].get_related_word_label(system).clone())
        }
    }

    pub fn set_last_entry_variant(&mut self, variant: usize) {
        if self.entries.len() == 0 {
            return;
        }

        let last_idx = self.entries.len() - 1;

        self.entries[last_idx].set_variant(variant);
    }

    pub fn clear_last_entry_variant(&mut self) {
        if self.entries.len() == 0 {
            return;
        }

        let last_idx = self.entries.len() - 1;

        self.entries[last_idx].clear_variant();
    }


    pub fn set_last_entry_related(&mut self, related: usize) {
        if self.entries.len() == 0 {
            return;
        }

        let last_idx = self.entries.len() - 1;

        self.entries[last_idx].set_related(related);
    }


    pub fn remove_last_entry(&mut self) {
        self.entries.pop();
    }

    pub fn replace_last_entry(&mut self, replacement: &Button) {
        self.remove_last_entry();
        self.add_entry(replacement);
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn get_text(&self) -> String {
        self.entries.iter().map(|e| e.label.clone()).collect::<Vec<_>>().join(" ")
    }

    pub fn get_pronouncible_text(&self, system: &System) -> String {
        self.entries.iter().map(|e| e.get_pronouncible_text(system)).collect::<Vec<_>>().join(" ")
    }
}

#[test]
fn test_panel() {
    let foo = Button { label: "foo".to_string(), pronunciation: None, image: None };
    let bar = Button { label: "bar".to_string(), pronunciation: None, image: None };
    let baz = Button { label: "baz".to_string(), pronunciation: None, image: None };
    let exc = Button { label: "!".to_string(), pronunciation: None, image: None };

    let mut panel = Panel::default();

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
