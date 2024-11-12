mod room;
mod player;
mod item;
mod npc;
mod quest;

use room::Room;
use player::Player;
use item::Item;
use npc::NPC;
use quest::Quest;
use std::collections::HashMap;
use std::io::{self, Write};
use rand::Rng;

fn main() {
    // Define the game world with rooms
    let mut rooms: HashMap<String, Room> = HashMap::new();

    // Create rooms with descriptions, exits, and items
    rooms.insert("start".to_string(), Room::new(
        "You are in a small, dimly lit room. There is a door to the north.".to_string(),
        HashMap::from([("north".to_string(), "forest".to_string())]),
        vec![Item::new("rusty key", "A key with rust marks.")],
        None,
    ));

    rooms.insert("forest".to_string(), Room::new(
        "You are in a forest, surrounded by tall trees. There is a path to the east.".to_string(),
        HashMap::from([("east".to_string(), "river".to_string()), ("south".to_string(), "start".to_string())]),
        vec![Item::new("stone", "A smooth stone, could be useful.")],
        Some(NPC::new("Old Man", "I used to be an adventurer like you.")),
    ));

    rooms.insert("river".to_string(), Room::new(
        "You are at the edge of a river, the sound of flowing water fills the air.".to_string(),
        HashMap::from([("west".to_string(), "forest".to_string())]),
        vec![Item::new("fishing rod", "A long, sturdy fishing rod.")],
        None,
    ));

    // Create quests
    let quest = Quest::new(
        "Find the Ancient Artifact",
        "An old legend speaks of an ancient artifact hidden deep in the forest.",
        "forest",
    );

    // Create the player
    let mut player = Player::new("Adventurer", "start", quest);

    // Main game loop
    loop {
        let current_room = rooms.get_mut(&player.current_room).unwrap();
        println!("\nYou are in: {}", player.current_room);
        println!("{}", current_room.description);
        println!("Items in the room: {:?}", current_room.items);
        if let Some(npc) = &current_room.npc {
            println!("There is a person here: {}", npc.name);
        }
        println!("Exits: {:?}", current_room.exits);

        print!("\nWhat do you want to do? ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let command = input.trim().to_lowercase();

        match command.as_str() {
            "look" => println!("{}", current_room.description),
            "inventory" => println!("{:?}", player.inventory),
            "quit" => {
                println!("Thanks for playing!");
                break;
            }
            _ if command.starts_with("take") => {
                let item_name = command.split_whitespace().nth(1).unwrap_or("");
                let item_index = current_room.items.iter().position(|item| item.name == item_name);
                if let Some(index) = item_index {
                    let item = current_room.items.remove(index);
                    player.take_item(item.clone());
                    println!("You took the {}.", item.name);
                } else {
                    println!("There is no {} here.", item_name);
                }
            }
            _ if command.starts_with("move") => {
                let direction = command.split_whitespace().nth(1).unwrap_or("");
                if let Some(new_room) = current_room.exits.get(direction) {
                    player.move_to(new_room);
                    println!("You move to the {}.", new_room);
                    // Random encounter logic
                    if rand::thread_rng().gen_bool(0.2) { // 20% chance for a random event
                        println!("A wild animal appears! Prepare for battle.");
                        // Combat logic can be implemented here later
                    }
                } else {
                    println!("You can't go that way.");
                }
            }
            _ if command.starts_with("talk") => {
                if let Some(npc) = &current_room.npc {
                    println!("{} says: {}", npc.name, npc.dialogue);
                } else {
                    println!("There is no one to talk to.");
                }
            }
            _ if command.starts_with("use") => {
                let item_name = command.split_whitespace().nth(1).unwrap_or("");
                if player.inventory.iter().any(|item| item.name == item_name) {
                    println!("You use the {}.", item_name);
                    // Implement item usage here (e.g., use a key to unlock something)
                } else {
                    println!("You don't have a {} in your inventory.", item_name);
                }
            }
            "quest" => {
                if let Some(q) = &player.quest {
                    println!("Quest: {}", q.name);
                    println!("Description: {}", q.description);
                    if player.current_room == q.target_room {
                        println!("You found the artifact! Quest completed.");
                        // Update quest status
                    }
                } else {
                    println!("You have no active quests.");
                }
            }
            _ => println!("I don't understand that command."),
        }
    }
}
