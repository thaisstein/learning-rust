struct Player {
    user: String,
    character: String,
    health: u8,
    level: u32,
}

fn main() { 
    let player1 = Player {
        user: String::from("Thais"),
        character: String::from("Luigi"),
        health: 100,
        level: 8
    };
}