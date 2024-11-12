pub struct Quest {
    pub name: String,
    pub description: String,
    pub target_room: String,
}

impl Quest {
    pub fn new(name: &str, description: &str, target_room: &str) -> Self {
        Quest {
            name: name.to_string(),
            description: description.to_string(),
            target_room: target_room.to_string(),
        }
    }
}
