use crate::system::System;
use nanoserde::{DeJson, SerJson};

#[derive(Clone, Debug, DeJson, SerJson, PartialEq)]
pub struct Button {
    pub label: String,
    pub pronunciation: Option<String>,
    pub image: Option<String>,
    related: Option<usize>,
    variant: Option<usize>,
}

impl Button {
    pub fn new(label: String, pronunciation: Option<String>, image: Option<String>) -> Self {
        let related = None;
        let variant = None;
        Self { label, pronunciation, image, related, variant }
    }

    pub fn get_related_word_label(&self, system: &System) -> String {
        if let Some(related) = self.related {
            if let Some(relateds) = system.related.get(&self.label) {
                return relateds.get(related).unwrap_or(self).label.clone();
            }
        }

        // No related words exist.
        self.label.clone()
    }

    pub fn get_label(&self, system: &System) -> String {
        let btn =
            if let Some(related) = self.related {
                if let Some(relateds) = system.related.get(&self.label) {
                    relateds.get(related).unwrap_or(self)
                } else {
                    // No related words exist.
                    self
                }
            } else {
                self
            };

        let btn =
            if let Some(variant) = self.variant {
                if let Some(variants) = system.variants.get(&btn.label) {
                    variants.get(variant).unwrap_or(btn)
                } else {
                    // No variants exist.
                    btn
                }
            } else {
                // No variant specified.
                btn
            };

        btn.label.clone()
    }

    pub fn get_pronouncible_text(&self, system: &System) -> String {
        let btn =
            if let Some(related) = self.related {
                if let Some(relateds) = system.related.get(&self.label) {
                    relateds.get(related).unwrap_or(self)
                } else {
                    // No related words exist.
                    self
                }
            } else {
                self
            };

        let btn =
            if let Some(variant) = self.variant {
                if let Some(variants) = system.variants.get(&btn.label) {
                    variants.get(variant).unwrap_or(btn)
                } else {
                    // No variants exist.
                    btn
                }
            } else {
                // No variant specified.
                btn
            };

        if let Some(pronunciation) = btn.pronunciation.clone() {
            pronunciation
        } else {
            btn.label.clone()
        }
    }

    pub fn variant(&self) ->  usize {
        self.variant.unwrap_or(0)
    }

    pub fn set_variant(&mut self, variant: usize) {
        self.variant = Some(variant);
    }

    pub fn clear_variant(&mut self) {
        self.variant = None;
    }

    pub fn related(&self) -> usize {
        self.related.unwrap_or(0)
    }

    pub fn set_related(&mut self, related: usize) {
        self.related = Some(related);
    }
}
