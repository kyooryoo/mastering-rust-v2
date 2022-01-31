#[allow(dead_code)]

struct Player {
    name: String,
    iq: u8,
    friends: u8
}

impl Player {
    // one kind of struct method like OOP static method
    // does not use &self as the first method argument
    // can be used by the struct without init any obj
    fn with_name(name: &str) -> Player {
        Player {
            name: name.to_string(),
            iq: 100,
            friends: 100
        }
    }

    // another kind of struct method uses &self argument
    // it can only be called by initialized struct object
    fn get_friends(&self) -> u8 {
        self.friends
    }

    fn set_friends(&mut self, count: u8) {
        self.friends = count;
    }
}

fn main() {
    let mut player = Player::with_name("Dave");
    player.set_friends(23);
    println!("{}'s friends count:: {}", player.name, Player::get_friends(&player));
}