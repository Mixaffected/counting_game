use rand::Rng;
use std::io;

fn main() {
    println!("===Counting Game===");
    let answere_number: i32 = rand::thread_rng().gen_range(0..=100);
    let mut tries: u32 = 0;

    loop {
        println!("\nInput a number from 0 to 100!");

        let mut player_answere: String = String::new();
        io::stdin()
            .read_line(&mut player_answere)
            .expect("Could not read keyboard input!");
        let player_answere = player_answere.trim().parse::<i32>();

        let player_answere: i32 = match player_answere {
            Ok(player_answere) => player_answere,
            Err(_) => {
                println!("Please input a valid number!");
                continue;
            }
        };

        if player_answere > 100 || player_answere < 0 {
            println!("Please input a valid number!");
            continue;
        }

        if player_answere == answere_number {
            println!(
                "You won! The number was {}.\nYou needed {} tries.",
                answere_number, tries
            );
            break;
        } else if player_answere > answere_number {
            tries += 1;
            println!("Your number is too high.")
        } else if player_answere < answere_number {
            tries += 1;
            println!("Your number is too low.")
        }
    }
}
