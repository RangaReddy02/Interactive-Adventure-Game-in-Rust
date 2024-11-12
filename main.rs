use macroquad::prelude::*;

// Define the direction enum with only North and South.
#[derive(Debug, PartialEq)]
enum Direction {
    North,
    South,
}

// Define the Item struct.
#[derive(Debug)]
struct Item {
    name: String,
}

// Define the Room struct.
struct Room {
    description: String,
    exits: Vec<Direction>,  // Directions available to exit from this room.
    items: Vec<Item>,       // Items in the room.
}

impl Room {
    fn new(description: &str, exits: Vec<Direction>, items: Vec<Item>) -> Self {
        Room {
            description: description.to_string(),
            exits,
            items,
        }
    }

    fn display_info(&self) {
        println!("{}", self.description);
        print!("Exits: ");
        for exit in &self.exits {
            print!("{:?} ", exit);
        }
        println!();
        println!("Items: ");
        for item in &self.items {
            println!("{}", item.name);
        }
    }

    fn take_item(&mut self) -> Option<Item> {
        self.items.pop()
    }
}

// Game state struct.
struct Game {
    player_inventory: Vec<Item>,
    current_room: Room,
}

impl Game {
    fn new() -> Self {
        let starting_room = Room::new(
            "You are in a  room with Bright lighting.",
            vec![Direction::North, Direction::South],
            vec![Item {
                name: "Old Key".to_string(),
            }],
        );

        Game {
            player_inventory: Vec::new(),
            current_room: starting_room,
        }
    }

    fn handle_input(&mut self) {
        let mut input = String::new();

        // Wait for key input (W, S, E)
        if is_key_pressed(KeyCode::W) {
            input.push('W');
        } else if is_key_pressed(KeyCode::S) {
            input.push('S');
        } else if is_key_pressed(KeyCode::E) {
            input.push('E');
        }

        match input.as_str() {
            "W" => {
                if self.current_room.exits.contains(&Direction::North) {
                    println!("You move North.");
                    // You could add logic for moving to another room here.
                }
            }
            "S" => {
                if self.current_room.exits.contains(&Direction::South) {
                    println!("You move South.");
                    // You could add logic for moving to another room here.
                }
            }
            "E" => {
                if let Some(item) = self.current_room.take_item() {
                    self.player_inventory.push(item);
                    println!("You picked up an item!");
                } else {
                    println!("No items to pick up.");
                }
            }
            _ => {
                println!("Invalid input.");
            }
        }
    }

    fn display_info(&self) {
        self.current_room.display_info();
        println!("\nInventory: ");
        for item in &self.player_inventory {
            println!("{}", item.name);
        }
    }

    // Make this function async
    async fn run(&mut self) {
        loop {
            clear_background(WHITE); // Clear background to white

            // Draw some basic info to confirm the loop runs
            draw_text("Welcome to the Adventure!", 10.0, 20.0, 30.0, DARKGRAY);
            self.display_info(); // Print room and inventory info in the console

            // Handle input and move the game state forward
            self.handle_input();

            // Update the screen and wait for the next frame
            next_frame().await;
        }
    }
}

#[macroquad::main("Adventure")]
async fn main() {
    let mut game = Game::new();
    game.run().await;  // Correctly await the run function
}
