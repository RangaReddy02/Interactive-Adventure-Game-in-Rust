use crate::item::Item;
use crate::npc::NPC;
use std::collections::HashMap;

pub struct Room {
    pub description: String,
    pub exits: HashMap<String, String>,
    pub items: Vec<Item>,
    pub npc: Option<NPC>,
}

impl Room {
    pub fn new(description: String, exits: HashMap<String, String>, items: Vec<Item>, npc: Option<NPC>) -> Self {
        Room { description, exits, items, npc }
    }
}
