use std::io::stdin;

struct Game {
    room: usize,
    inventory: Vec<Item>,
    rooms: Vec<Room>
}

impl Game {
    fn room(&self) -> &Room {
        &self.rooms[self.room]
    }

    fn room_mut(&mut self) -> &mut Room {
        &mut self.rooms[self.room]
    }

    fn exits(&self) {
        let mut index = 0;
        let mut s = String::from(
            format!("{} has {} exits:", &self.room().name, &self.room().exits.len())
        );

        for exit in &self.room().exits {
            s = format!("{}\n({}) {}", s, index, self.rooms[*exit].name);
            index += 1;
        }

        println!("{}", s);
    }

    fn view_inventory(&self) {
        let mut index = 0;
        let mut s = String::from(
            format!("You have {} items:", self.inventory.len())
        );

        for item in &self.inventory {
            s = format!("{}\n({}) {}", s, index, item.name);
            index += 1;
        }

        println!("{}", s);
    }

    fn move_room(&mut self, room: usize) {
        self.room = self.room().exits[room];
    }

    fn take(&mut self, item: usize) -> &Item {
        let item = self.room_mut().items.remove(item);
        self.inventory.push(item);

        &self.inventory[self.inventory.len() - 1]
    }
}

struct Item {
    name: String,
    description: String
}

struct Room {
    name: String,
    description: String,
    exits: Vec<usize>,
    items: Vec<Item>
}

impl Room {
    fn look(&self) {
        println!("{}", self.description)
    }

    fn inspect(&self) {
        let mut index = 0;
        let mut s = String::from(
            format!("{} has {} items:", &self.name, &self.items.len())
        );

        for item in &self.items {
            s = format!("{}\n({}) {}", s, index, item.name);
            index += 1;
        }

        println!("{}", s);
    }
}

fn main() {
    let mut rooms = vec![
        Room {
            name: String::from("Bedroom"),
            description: String::from("A tidy, clean bedroom with 1 door and a balcony"),
            exits: vec![1, 2],
            items: vec![ Item {
                name: String::from("Key"),
                description: String::from("A golden key")
            }]
        },

        Room {
            name: String::from("Balcony"),
            description: String::from("An outdoor balcony that overlooks a gray garden"),
            exits: vec![0],
            items: vec![]
        },

        Room {
            name: String::from("Landing"),
            description: String::from("A carpetted landing with doors leading off it. It overlooks a large living space. A set of stairs leads down"),
            exits: vec![0],
            items: vec![]
        },
    ];

    let mut player = Game {
        room: 0,
        rooms: rooms,
        inventory: vec![]
    };

    println!("Type `look' to look around. Type `move <room no>' to switch room");

    loop {
        let mut input = String::new();

        match stdin().read_line(&mut input) {
            Ok(_) => {
                let mut commands = input.trim().split_whitespace();

                match commands.next() {
                    Some("look") => {
                        player.room().look();
                        player.exits();
                    }

                    Some("move") => {

                        let args: Vec<&str> = commands.collect();

                        if args.len() != 1 {
                            println!("Incorrect args.");
                            continue;
                        }

                        let room_no: usize = match args[0].parse() {
                            Ok(a) => {a},

                            Err(e) => {
                                println!("{}", e);
                                continue
                            }
                        };

                        player.move_room(room_no);

                        println!("You moved to {}", player.room().name);
                    }

                    Some("inventory") => {
                        player.view_inventory();
                    }

                    Some("inspect") => {
                        player.room().inspect();
                    }

                    Some("take") => {
                        let args: Vec<&str> = commands.collect();

                        if args.len() != 1 {
                            println!("Incorrect args.");
                            continue;
                        }

                        let item_no: usize = match args[0].parse() {
                            Ok(a) => {a},

                            Err(e) => {
                                println!("{}", e);
                                continue
                            }
                        };

                        let item = player.take(item_no);

                        println!("You collected {}", item.name);
                    }

                    None => {},

                    _ => {},
                }
            }
            Err(error) => panic!("Error occured reading stdin: {}", error),
        }
    }
}
