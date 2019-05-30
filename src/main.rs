#[derive(Debug)]
enum Color {
    Red,
    Blue,
    Green,
    Yellow
}

fn fancy_print_guess(guess: Vec<Color>) {
    for pawn in guess {
        match pawn {
            Color::Red => print!("R"),
            Color::Blue => print!("B"),
            Color::Green => print!("G"),
            Color::Yellow => print!("Y")
        }
    }
}

fn main() {
    let guess = vec![Color::Blue, Color::Red, Color::Green, Color::Red];
    fancy_print_guess(guess);
}
