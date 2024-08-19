use rand::seq::SliceRandom;

#[derive(Debug, PartialEq, Clone)]
enum Color {
    White,
    Blue,
    Green,
    Red,
    Yellow,
}
#[derive(Debug)]
struct Player {
    dominos: Vec<Domino>,
}

impl Player{
    // assign dominos
    fn new(dominos: Vec<Domino>) -> Self{
        Self { dominos: dominos }
    }
}
#[derive(Debug)]
struct Computer{
    dominos: Vec<Domino>,
}

impl Computer{
    // assign dominos
    fn new(dominos: Vec<Domino>) -> Self{
        Self { dominos: dominos }
    }
}

#[derive(Debug, Clone)]
struct Domino(Color, Color);

impl Domino {
    // Create a new domino tile
    fn new(left: Color, right: Color) -> Self {
        Self(left, right)
    }
    // Get color from random number
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
        let computer_hand = shuffled_dominos[mid_point..].to_vec();
        (player_hand, computer_hand)
    }

    //count domino
    fn count(domios_set:&Vec<Domino>) -> i32{
        domios_set.len() as i32
    }

    fn display_domino(dominos: &[Domino]) {
        for value in dominos.iter() {
            println!("({:?}, {:?})", value.0, value.1);
        }
    }
}

fn main() {
    let mut domino_set_in_number: Vec<(i32, i32)> = Vec::new();
    let mut domino_set_in_color: Vec<Domino> = Vec::new();

    for i in 0..=4 {
        for j in 0..=4 {
            domino_set_in_number.push((i, j));
        }
    }

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
    let (player_slice, computer_slice) = Domino::split_hand(&domino_set_in_color, mid_point);

    //assign 
    let player: Player = Player::new(player_slice);
    let computer: Computer = Computer { dominos: computer_slice };

    println!("Player's hand:");
    Domino::display_domino(&player.dominos);
    println!("Player Count: {:?}", Domino::count(&player.dominos));

    println!("Computer's hand:");
    Domino::display_domino(&computer.dominos);
    println!("Computer Count: {:?}", Domino::count(&computer.dominos))

}
