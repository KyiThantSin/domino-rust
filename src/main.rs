use rand::Rng;

#[derive(Debug, PartialEq)]
enum Color {
    White,
    Blue,
    Green,
    Red,
    Yellow,
}

#[derive(Debug)]
struct Domino {
    left: Color,
    right: Color,
}

impl Domino {
    // create a new domino tile
    fn new(left: Color, right: Color) -> Self {
        Self {
            left: left,
            right: right,
        }
    }

    // get color from random no
    fn generate_color_code(number: i32) -> Color {
        match number {
            0 => Color::Blue,
            1 => Color::Green,
            2 => Color::Red,
            3 => Color::White,
            _ => Color::Yellow,
        }
    }

    // check getting same domino tile
    fn is_duplicate(first_item: &Domino, second_item: &Domino) -> bool {
        first_item.left == second_item.left
    }

    fn display_domino(&self) {
        println!("{:?}", self)
    }
}

fn generate_random_number() {
    let mut random = rand::thread_rng();
    let number: i32 = random.gen_range(0..4);
    println!("{}", &number);
}
fn main() {
    let first: Domino = Domino::new(
        Domino::generate_color_code(3),
        Domino::generate_color_code(2),
    );
    let second: Domino = Domino::new(
        Domino::generate_color_code(3),
        Domino::generate_color_code(2),
    );
    Domino::display_domino(&first);
    let same = Domino::is_duplicate(&first, &second);
    println!("{:?}", same);
    generate_random_number();
}