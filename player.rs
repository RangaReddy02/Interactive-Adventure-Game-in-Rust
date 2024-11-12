use crate::item::Item;
use crate::quest::Quest;

#[allow(dead_code)]
pub struct Player {
    pub name: String,
    pub current_room: String,
    pub inventory: Vec<Item>,
    pub quest: Option<Quest>,
}

impl Player {
    pub fn new(name: &str, current_room: &str, quest: Quest) -> Self {
        Player {
            name: name.to_string(),
            current_room: current_room.to_string(),
            inventory: Vec::new(),
            quest: Some(quest),
        }
    }

    pub fn move_to(&mut self, new_room: &str) {
        self.current_room = new_room.to_string();
    }

    pub fn take_item(&mut self, item: Item) {
        self.inventory.push(item.clone());
    }
}
