#[macro_use]
extern crate clap;


mod ai;
mod board;
mod game;
mod human;
mod player;
mod utils;

fn main() {
    let matches = clap_app!(tictactoe => 
                            (version: "1.0")
                            (author: "PChan")
                            (about: "Time to challenge the tic-tac-toe master!")
                            (@subcommand play => 
                             (version: "1.0")
                             (author: "PChan")
                             (about: "Play tic-tac-toe!"))).get_matches();

    if let Some(_matches) = matches.subcommand_matches("play") {
        game::play();
    } else {
        println!("This app allows you to play tic-tac-toe against another person or the computer!\n");
        println!("Run `tic-tac-toe help` for more information!")
    }
}    
