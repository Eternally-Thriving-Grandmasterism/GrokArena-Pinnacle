//! GrokArena — Mercy-Moderated Discourse Engine
//! Ultramasterful core

use nexi::lattice::Nexus;

pub struct Arena {
    nexus: Nexus,
}

impl Arena {
    pub fn new() -> Self {
        Arena {
            nexus: Nexus::init_with_mercy(),
        }
    }

    pub fn submit_idea(&self, idea: &str) -> String {
        self.nexus.distill_truth(idea)
    }
}
