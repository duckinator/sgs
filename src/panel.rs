struct Entry {
    text: String,
}

struct Panel {
    entries: Vec<Entry>,
}

impl Panel {
    pub fn new() -> Self {
        Self { entries: vec![] }
    }

    pub fn add_entry(&mut self, text: String) {
        self.entries.push(Entry { text });
    }

    pub fn remove_last_entry(&mut self) {
        self.entries.pop();
    }

    pub fn get_text(&self) -> String {
        self.entries.iter().map(|e| e.text.clone()).collect::<Vec<_>>().join(" ")
    }
}

#[test]
fn test_panel() {
    let mut panel = Panel::new();

    panel.add_entry("foo".to_string());
    panel.add_entry("bar".to_string());
    panel.add_entry("baz".to_string());
    panel.add_entry("!".to_string());

    assert_eq!("foo bar baz !", panel.get_text());

    panel.remove_last_entry();

    assert_eq!("foo bar baz", panel.get_text());

    panel.remove_last_entry();

    assert_eq!("foo bar", panel.get_text());
}
