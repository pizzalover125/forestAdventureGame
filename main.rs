use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use std::process::Command;

struct Player {
    name: String,
    inventory: Vec<String>,
    is_alive: bool,
    has_won: bool,
}

impl Player {
    fn new(name: String) -> Self {
        Player {
            name,
            inventory: Vec::new(),
            is_alive: true,
            has_won: false,
        }
    }

    fn has_item(&self, item: &str) -> bool {
        self.inventory.contains(&item.to_string())
    }

    fn add_item(&mut self, item: String) {
        if !self.has_item(&item) {
            self.inventory.push(item.clone());
            println!("You have obtained: {}", item);
            thread::sleep(Duration::from_secs(1));
        } else {
            println!("You already have the {}", item);
            thread::sleep(Duration::from_secs(1));
        }
    }
}

fn clear_screen() {
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(&["clear", "cls"]).status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    }
}

fn main() {
    println!("Welcome to the Forest Adventure!");
    thread::sleep(Duration::from_secs(1));
    print!("Enter your name: ");
    io::stdout().flush().unwrap();

    let mut player_name = String::new();
    io::stdin().read_line(&mut player_name).unwrap();
    let player_name = player_name.trim().to_string();

    let mut player = Player::new(player_name);

    println!("\nHello, {}! You find yourself in the middle of a vast and dangerous forest.\n", player.name);
    thread::sleep(Duration::from_secs(1));

    while player.is_alive && !player.has_won {
        clear_screen();  

        println!("\nYou are in a central clearing surrounded by towering trees. Paths lead in various directions:");
        thread::sleep(Duration::from_secs(1));
        println!("1. North - A dense, shadowy grove.🌲");
        thread::sleep(Duration::from_secs(1));
        println!("2. East - A path lined with glowing mushrooms.🍄");
        thread::sleep(Duration::from_secs(1));
        println!("3. South - A riverbank with a rickety bridge.🌉");
        thread::sleep(Duration::from_secs(1));
        println!("4. West - A meadow full of wildflowers.🌻");
        thread::sleep(Duration::from_secs(1));
        println!("5. Northeast - A crumbling stone tower.🪨");
        thread::sleep(Duration::from_secs(1));
        println!("6. Southwest - A cave entrance shrouded in mist.🌫️");
        thread::sleep(Duration::from_secs(1));
        println!("7. Inventory - Check your items.");
        thread::sleep(Duration::from_secs(1));
        println!("8. Quit - Leave the adventure.\n");
        thread::sleep(Duration::from_secs(1));

        print!("Choose a direction or action: ");
        io::stdout().flush().unwrap();
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim().to_lowercase();

        match choice.as_str() {
            "1" | "north" => {
                clear_screen();  // Clear screen before entering the area
                println!("\nYou head north into the shadowy grove. A sinister presence surrounds you.");
                thread::sleep(Duration::from_secs(1));
                if player.has_item("Ancient Talisman") {
                    println!("The talisman glows and protects you from the dark energy. You safely leave the grove.");
                    thread::sleep(Duration::from_secs(1));
                } else {
                    println!("The darkness consumes you. You have perished.");
                    player.is_alive = false;
                    thread::sleep(Duration::from_secs(1));
                }
            }
            "2" | "east" => {
                clear_screen();
                println!("\nYou walk east along the path lined with glowing mushrooms. A forest spirit appears and offers you a magical herb.");
                thread::sleep(Duration::from_secs(1));
                if !player.has_item("Magical Herb") {
                    player.add_item("Magical Herb".to_string());
                } else {
                    println!("The mushrooms glow peacefully, but nothing new happens.");
                    thread::sleep(Duration::from_secs(1));
                }
            }
            "3" | "south" => {
                clear_screen();
                println!("\nYou approach the riverbank. The rickety bridge creaks dangerously.");
                thread::sleep(Duration::from_secs(1));
                if player.has_item("Silver Feather") {
                    println!("The feather grants you safe passage across the bridge. You find a treasure chest filled with gold!");
                    player.has_won = true;
                    thread::sleep(Duration::from_secs(1));
                } else {
                    println!("The bridge collapses, and you fall into the river. You have drowned.");
                    player.is_alive = false;
                    thread::sleep(Duration::from_secs(1));
                }
            }
            "4" | "west" => {
                clear_screen();
                println!("\nYou walk into the meadow, where bees buzz lazily. A fairy offers you a Silver Feather for protection.");
                thread::sleep(Duration::from_secs(1));
                if !player.has_item("Silver Feather") {
                    player.add_item("Silver Feather".to_string());
                } else {
                    println!("The meadow is peaceful, but nothing new happens.");
                    thread::sleep(Duration::from_secs(1));
                }
            }
            "5" | "northeast" => {
                clear_screen();
                println!("\nYou cautiously approach the crumbling stone tower. The air is thick with danger.");
                thread::sleep(Duration::from_secs(1));
                println!("Inside, you find a sleeping dragon. Will you sneak past it or run away?");
                print!("Choose: sneak/run: ");
                io::stdout().flush().unwrap();
                let mut decision = String::new();
                io::stdin().read_line(&mut decision).unwrap();
                let decision = decision.trim().to_lowercase();

                match decision.as_str() {
                    "sneak" => {
                        if player.has_item("Magical Herb") {
                            println!("The herb masks your scent, and you safely sneak past the dragon. You find a legendary sword!");
                            player.add_item("Legendary Sword".to_string());
                        } else {
                            println!("The dragon awakens and incinerates you. You have perished.");
                            player.is_alive = false;
                        }
                        thread::sleep(Duration::from_secs(1));
                    }
                    "run" => {
                        println!("You run away safely but find nothing useful.");
                        thread::sleep(Duration::from_secs(1));
                    }
                    _ => {
                        println!("Invalid choice. The dragon awakens and incinerates you. You have perished.");
                        player.is_alive = false;
                        thread::sleep(Duration::from_secs(1));
                    }
                }
            }
            "6" | "southwest" => {
                clear_screen();
                println!("\nYou enter the misty cave. It's cold and damp, with the sound of dripping water echoing.");
                thread::sleep(Duration::from_secs(1));
                if player.has_item("Legendary Sword") {
                    println!("You use the sword to slay a monstrous spider lurking in the cave and escape victorious!");
                    player.has_won = true;
                } else {
                    println!("The spider catches you in its web. You have perished.");
                    player.is_alive = false;
                }
                thread::sleep(Duration::from_secs(1));
            }
            "7" | "inventory" => {
                clear_screen();
                println!("\nYour Inventory: {:?}", player.inventory);
                if player.inventory.is_empty() {
                    println!("Your inventory is empty.");
                }
                thread::sleep(Duration::from_secs(1));
            }
            "8" | "quit" => {
                println!("\nThank you for playing, {}! Farewell and may the forest remember you.", player.name);
                break;
            }
            _ => {
                println!("\nInvalid choice. Please try again.");
                thread::sleep(Duration::from_secs(1));
            }
        }
    }

    if player.has_won {
        println!("\nCongratulations, {}! You have won the game and emerged victorious from the forest!", player.name);
    } else if !player.is_alive {
        println!("\nGame Over, {}. You have met a tragic end in the forest.", player.name);
    }
}
