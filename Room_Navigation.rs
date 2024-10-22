
use std::io::{self, Write};

struct Room {
    description: String,
    exits: Vec<String>,  // Directions like "north", "south", etc.
}

struct Player {
    current_room: usize,  // Index of the current room
}

fn main() {
    let rooms = setup_rooms();
    let mut player = Player {
        current_room: 0,
    };

    println!("Welcome to the Interactive Adventure Game!");
    loop {
        display_room(&rooms[player.current_room]);
        let input = get_player_input();
        if input == "quit" {
            println!("Thank you for playing!");
            break;
        }
        process_command(input, &mut player, &rooms);
    }
}

fn setup_rooms() -> Vec<Room> {
    vec![
        Room {
            description: String::from("You are in a small, dimly lit room."),
            exits: vec![String::from("north")],
        },
        Room {
            description: String::from("You are in a bright, open room."),
            exits: vec![String::from("south"), String::from("east")],
        },
    ]
}

fn display_room(room: &Room) {
    println!("
{}", room.description);
    println!("You can move: {:?}", room.exits);
}

fn get_player_input() -> String {
    print!("> ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_lowercase()
}

fn process_command(command: String, player: &mut Player, rooms: &Vec<Room>) {
    match command.as_str() {
        "move north" if rooms[player.current_room].exits.contains(&"north".to_string()) => {
            player.current_room = 1;  // Move to the second room
            println!("You move north.");
        }
        "move south" if rooms[player.current_room].exits.contains(&"south".to_string()) => {
            player.current_room = 0;  // Move back to the first room
            println!("You move south.");
        }
        _ => println!("Invalid command."),
    }
}
