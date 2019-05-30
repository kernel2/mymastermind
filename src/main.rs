use ansi_term::Colour::{Red, Blue, Green, Yellow};

#[derive(PartialEq)]
#[derive(Debug)]
enum Color {
    Red,
    Blue,
    Green,
    Yellow
}

fn fancy_print_guess(guess: &Vec<Color>) {
    for pawn in guess {
        match pawn {
            Color::Red => print!("{}", Red.paint("R")),
            Color::Blue => print!("{}", Blue.paint("B")),
            Color::Green => print!("{}", Green.paint("G")),
            Color::Yellow => print!("{}", Yellow.paint("Y"))
        }
    }
    println!("");
}

fn main() {
    let mut turn = 0;
    let secret = vec![Color::Blue, Color::Red, Color::Green, Color::Red];

    println!("███╗   ███╗ █████╗ ███████╗████████╗███████╗██████╗ ███╗   ███╗██╗███╗   ██╗██████╗ ");
    println!("████╗ ████║██╔══██╗██╔════╝╚══██╔══╝██╔════╝██╔══██╗████╗ ████║██║████╗  ██║██╔══██╗");
    println!("██╔████╔██║███████║███████╗   ██║   █████╗  ██████╔╝██╔████╔██║██║██╔██╗ ██║██║  ██║");
    println!("██║╚██╔╝██║██╔══██║╚════██║   ██║   ██╔══╝  ██╔══██╗██║╚██╔╝██║██║██║╚██╗██║██║  ██║");
    println!("██║ ╚═╝ ██║██║  ██║███████║   ██║   ███████╗██║  ██║██║ ╚═╝ ██║██║██║ ╚████║██████╔╝");
    println!("╚═╝     ╚═╝╚═╝  ╚═╝╚══════╝   ╚═╝   ╚══════╝╚═╝  ╚═╝╚═╝     ╚═╝╚═╝╚═╝  ╚═══╝╚═════╝ ");
    println!("");
    println!("Try to guess my hidden combination of colors:");

    loop {
        let mut input = String::new();

        std::io::stdin().read_line(&mut input)
          .expect("Failed to read line");

        // Mind about the newline character.
        if input.len() != (4 + 1) {
            println!("Sorry I am hard programmed to understand input of size four, can you help me?");
            continue;
        }

        let mut guess = vec![];
        let len = input.len();
        for character in input[..len - 1].chars() {
            let pawn = match character {
                'B' => Color::Blue,
                'R' => Color::Red,
                'Y' => Color::Yellow,
                'G' => Color::Green,

                // Exaustive pattern
                _ => {
                    println!("Sorry dear human, I can see only four color please type:");
                    println!("- R for red");
                    println!("- B for blue");
                    println!("- Y for Yellow");
                    println!("- G for Green");
                    println!("Otherwise I will be confused as you are.");
                    break;
                }
            };
            guess.push(pawn);
        }
        // Encountered an error in parsing?
        if guess.len() != 4 {
            continue;
        }

        turn += 1;

        print!("Turn {}: ", turn);
        fancy_print_guess(&guess);

        if guess == secret {
            break
        }
    }

    println!("🎉🎊 Congratulation this is a huge success! 🎊🎉");
    println!("🤯 YOU are the MasterMind! 🤯");
}