use rand::seq::SliceRandom;
use std::io;

#[derive(Debug, PartialEq, Clone)]
enum Color {
    White,
    Blue,
    Green,
    Red,
    Yellow,
}

#[derive(Debug, PartialEq, Clone)]
enum GameState {
    Playing,
    PlayerWon,
    ComputerWon,
    Draw,
}

struct Game {
    player: Player,
    computer: Computer,
    state: GameState,
}

impl Game {
    fn new(player: Player, computer: Computer) -> Self {
        Self {
            player,
            computer,
            state: GameState::Playing,
        }
    }

    fn start_game() -> Game {
        let mut domino_set_in_number: Vec<(i32, i32)> = Vec::new();
        let mut domino_set_in_color: Vec<Domino> = Vec::new();

        // domino set
        for i in 0..=4 {
            for j in 0..=4 {
                domino_set_in_number.push((i, j));
            }
        }

        // colored dominos
        for domino in &domino_set_in_number {
            let (left, right) = (
                Domino::generate_color_code(domino.0),
                Domino::generate_color_code(domino.1),
            );
            domino_set_in_color.push(Domino::new(left, right));
        }

        // Shuffle the dominos
        let mut random = rand::thread_rng();
        domino_set_in_color.shuffle(&mut random);

        let mid_point: usize = (domino_set_in_color.len() - 1) / 2;
        let (player_slice, mut computer_slice) =
            Domino::split_hand(&domino_set_in_color, mid_point);

        // Remove the last domino as the starting domino
        let starting_domino = computer_slice.pop().unwrap();

        // Assign 
        let player: Player = Player::new(player_slice);
        let computer: Computer = Computer::new(computer_slice);
        let game = Game::new(player, computer);

        println!("Starting Domino: {:?}", starting_domino);

        game
    }

    fn check_victory_conditions(&self) -> GameState {
        if self.player.dominos.is_empty() {
            GameState::PlayerWon
        } else if self.computer.dominos.is_empty() {
            GameState::ComputerWon
        } else {
            GameState::Playing
        }
    }

    fn end_game(&self) {
        match &self.state {
            GameState::PlayerWon => println!("Player wins!"),
            GameState::ComputerWon => println!("Computer wins!"),
            GameState::Draw => println!("The game is a draw!"),
            GameState::Playing => println!("The game is still ongoing."),
        }
    }
}

#[derive(Debug)]
struct Player {
    dominos: Vec<Domino>,
}

impl Player {
    fn new(dominos: Vec<Domino>) -> Self {
        Self { dominos }
    }

    fn remove_player_domino(&mut self, number: usize) -> Option<Domino> {
        if number < self.dominos.len() {
            Some(self.dominos.remove(number))
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Computer {
    dominos: Vec<Domino>,
}

impl Computer {
    fn new(dominos: Vec<Domino>) -> Self {
        Self { dominos }
    }

    fn remove_computer_domino(&mut self, number: usize) -> Option<Domino> {
        if number < self.dominos.len() {
            Some(self.dominos.remove(number))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
struct Domino(Color, Color);

impl Domino {
    fn new(left: Color, right: Color) -> Self {
        Self(left, right)
    }

    fn generate_color_code(number: i32) -> Color {
        match number {
            0 => Color::Blue,
            1 => Color::Green,
            2 => Color::Red,
            3 => Color::White,
            _ => Color::Yellow,
        }
    }

    fn split_hand(shuffled_dominos: &[Domino], mid_point: usize) -> (Vec<Domino>, Vec<Domino>) {
        let player_hand: Vec<Domino> = shuffled_dominos[..mid_point].to_vec();
        let computer_hand: Vec<Domino> = shuffled_dominos[mid_point..].to_vec();
        (player_hand, computer_hand)
    }

    fn count(dominos_set: &Vec<Domino>) -> i32 {
        dominos_set.len() as i32
    }

    fn display_domino(dominos: &[Domino]) {
        for (index, value) in dominos.iter().enumerate() {
            println!("{}: ({:?}, {:?})", index + 1, value.0, value.1);
        }
    }
}

fn main() {
    println!("Welcome to the mini Domino Game!");
    println!("The rule is simple: You have to match the same color on the ends of the dominoes to play them.");
    println!("Are you ready to play? \n1. Start the game\n2. Exit\nChoose an option:");

    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read input");

    let user_choice: usize = match user_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a number.");
            return;
        }
    };

    match user_choice {
        1 => {
            let game = Game::start_game();

            // println!("Player's hand:");
            // Domino::display_domino(&game.player.dominos);
            // println!("Player Count: {:?}", Domino::count(&game.player.dominos));

            // println!("Computer's hand:");
            // Domino::display_domino(&game.computer.dominos);
            // println!("Computer Count: {:?}", Domino::count(&game.computer.dominos));

        }
        2 => {
            println!("Exiting the game.");
        }
        _ => println!("Invalid option. Please choose 1 or 2."),
    }
}
