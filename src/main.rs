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
struct Domino(Color,Color);

impl Domino {
    // create a new domino tile
    fn new(left: Color, right: Color) -> Self {
        Self (left, right)
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

    // check whether player gets same domino or not
    fn is_duplicate(first_item: i32, second_item: i32) -> bool {
        first_item == second_item
    }

    fn display_domino(dominos: &Vec<Domino>){
        for value in dominos.iter(){
            println!("({:?}, {:?})", value.0, value.1)
        }
    }
  
}

fn generate_random_number() {
    let mut random: rand::rngs::ThreadRng = rand::thread_rng();
    let number: i32 = random.gen_range(0..4);
    println!("{}", &number);
}

fn main() {
    let mut domino_set_in_number: Vec<(i32,i32)> = Vec::new();
    let mut domino_set_in_color: Vec<Domino> = Vec::new();

    for i in 0..=4 {
        for j in 0..=4 {
            domino_set_in_number.push((i, j));
        }
    }

    for domino in &domino_set_in_number {
        let (left, right) = (Domino::generate_color_code(domino.0),Domino::generate_color_code(domino.1));
        domino_set_in_color.push(Domino::new(left, right));
    }

    Domino::display_domino(&domino_set_in_color);
    println!("length : {:?}", domino_set_in_color.len());
    // println!("{:?}", domino_set_in_number);
}