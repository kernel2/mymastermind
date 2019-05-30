use ansi_term::Colour::{Red, Blue, Green, Yellow};

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
            Color::Red => print!("{}", Red.paint("R")),
            Color::Blue => print!("{}", Blue.paint("B")),
            Color::Green => print!("{}", Green.paint("G")),
            Color::Yellow => print!("{}", Yellow.paint("Y"))
        }
    }
}

fn main() {
    let guess = vec![Color::Blue, Color::Red, Color::Green, Color::Red];

    println!("███╗   ███╗ █████╗ ███████╗████████╗███████╗██████╗ ███╗   ███╗██╗███╗   ██╗██████╗ ");
    println!("████╗ ████║██╔══██╗██╔════╝╚══██╔══╝██╔════╝██╔══██╗████╗ ████║██║████╗  ██║██╔══██╗");
    println!("██╔████╔██║███████║███████╗   ██║   █████╗  ██████╔╝██╔████╔██║██║██╔██╗ ██║██║  ██║");
    println!("██║╚██╔╝██║██╔══██║╚════██║   ██║   ██╔══╝  ██╔══██╗██║╚██╔╝██║██║██║╚██╗██║██║  ██║");
    println!("██║ ╚═╝ ██║██║  ██║███████║   ██║   ███████╗██║  ██║██║ ╚═╝ ██║██║██║ ╚████║██████╔╝");
    println!("╚═╝     ╚═╝╚═╝  ╚═╝╚══════╝   ╚═╝   ╚══════╝╚═╝  ╚═╝╚═╝     ╚═╝╚═╝╚═╝  ╚═══╝╚═════╝ ");
    println!("");
    //println!("Secret combination: ");
    //fancy_print_guess(guess);

    loop {
        let mut guess = String::new();

        std::io::stdin().read_line(&mut guess)
          .expect("Failed to read line");
        
        println!("{}", guess);
    }
}
