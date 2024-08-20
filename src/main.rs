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
    starting_domino: Domino
}

impl Game {
    fn new(player: Player, computer: Computer,starting_domino: Domino) -> Self {
        Self {
            player,
            computer,
            state: GameState::Playing,
            starting_domino,
        }
    }

    fn start_game() -> Game {
        let mut domino_set_in_number: Vec<(i32, i32)> = Vec::new();
        let mut domino_set_in_color: Vec<Domino> = Vec::new();

        // Create domino set
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

        // Shuffle
        let mut random = rand::thread_rng();
        domino_set_in_color.shuffle(&mut random);

        let mid_point: usize = (domino_set_in_color.len() - 1) / 2;
        let (player_slice, mut computer_slice) =
            Domino::split_hand(&domino_set_in_color, mid_point);

        // starting domino
        let starting_domino = computer_slice.pop().unwrap();

        // Initialize
        let player = Player::new(player_slice);
        let computer = Computer::new(computer_slice);
        let game = Game::new(player, computer, starting_domino);

        println!("Starting Domino: {:?}", (&game.starting_domino.0, &game.starting_domino.1));

        game
    }

    fn is_valid_move(&self, domino: &Domino) -> bool {
        domino.0 == self.starting_domino.1 || domino.1 == self.starting_domino.1
    }
    
    fn play_turn(&mut self) {
        self.choose_domino();
        self.computer_choose_domino();

        self.state = self.check_victory_conditions();
        self.end_game();
    }

    fn choose_domino(&mut self) {
        loop {
            println!("Player's Box:");
            Domino::display_domino(&self.player.dominos);
            println!("You have {:?} dominoes left.", Domino::count(&self.player.dominos));
            println!("Your turn. Choose a domino to play:");

            let mut player_input = String::new();
            io::stdin()
                .read_line(&mut player_input)
                .expect("Failed to read input");

            let chosen_index: usize = match player_input.trim().parse::<usize>() {
                Ok(num) => num - 1, // users enter number starting from 1
                Err(_) => {
                    println!("Invalid input. Please enter a valid number.");
                    continue;
                }
            };
            
            if chosen_index < self.player.dominos.len() {
                let chosen_domino = self.player.dominos.remove(chosen_index);
                if self.is_valid_move(&chosen_domino) {
                    println!("You played: {:?}", chosen_domino);
                    self.starting_domino = chosen_domino;
                    println!(
                        "Starting Domino: {:?}", (&self.starting_domino.0,&self.starting_domino.1)
                    );
                    break;
                } else {
                    println!(
                        "Invalid move. The domino doesn't match the starting domino. Try again."
                    );
                    println!(
                        "Starting Domino: {:?}", (&self.starting_domino.0,&self.starting_domino.1)
                    );
                    self.player.dominos.insert(chosen_index, chosen_domino);
                }
            } else {
                println!("Invalid move. Try again.");
            }
        }
    }

    fn computer_choose_domino(&mut self) {
        if !self.computer.dominos.is_empty() {
            // let mut rng = rand::thread_rng();
            // let chosen_index = rng.gen_range(0..self.computer.dominos.len());
            // let chosen_domino = self.computer.remove_computer_domino(chosen_index).unwrap();
            // println!("Computer played: {:?}", (chosen_domino.0, chosen_domino.1));
        }
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
            GameState::Playing => (),
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
            let mut game = Game::start_game();
            while game.state == GameState::Playing {
                game.play_turn();
            }
        }
        2 => {
            println!("Exiting the game.");
        }
        _ => println!("Invalid option. Please choose 1 or 2."),
    }
}