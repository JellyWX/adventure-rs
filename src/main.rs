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

    fn move_room(&mut self, room: usize) {
        self.room = self.room().exits[room];
    }
}

struct Item {
    name: String,
    description: String
}

struct Room {
    name: String,
    description: String,
    exits: Vec<usize>
}

impl Room {
    fn look(&self) {
        println!("{}", self.description)
    }
}

fn main() {
    let rooms = vec![
        Room {
            name: String::from("Bedroom"),
            description: String::from("A tidy, clean bedroom with 1 door and a balcony"),
            exits: vec![1, 2]
        },

        Room {
            name: String::from("Balcony"),
            description: String::from("An outdoor balcony that overlooks a gray garden"),
            exits: vec![0]
        },

        Room {
            name: String::from("Landing"),
            description: String::from("A carpetted landing with doors leading off it. It overlooks a large living space. A set of stairs leads down"),
            exits: vec![0]
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
                    },
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
                    },

                    None => {},

                    _ => {},
                }
            }
            Err(error) => panic!("Error occured reading stdin: {}", error),
        }
    }
}
