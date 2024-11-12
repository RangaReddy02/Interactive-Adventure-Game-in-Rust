pub struct NPC {
    pub name: String,
    pub dialogue: String,
}

impl NPC {
    pub fn new(name: &str, dialogue: &str) -> Self {
        NPC {
            name: name.to_string(),
            dialogue: dialogue.to_string(),
        }
    }
}
