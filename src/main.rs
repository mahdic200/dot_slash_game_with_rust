use app::App;

use crate::app::{player::Player, enums::GameType};

pub mod app;

fn main() {
    print!("\x1B[2J\x1B[1;1H");
    
    let mut difficulty: usize;
    loop {
        println!("Select game difficulty : ");
        println!("(1) => normal");
        println!("(2) => professional");
        difficulty = App::get_user_input("\nEnter (1 or 2) : ", "invalid input !");
        println!("\n******\nselected game difficulty : {}\n******\n", difficulty);
        match difficulty {
            x if x == 1 || x == 2 => break,
            _ => {println!("\ninvalid input !\n");continue;},
        }
    }
    print!("\x1B[2J\x1B[1;1H");
    let game_type: GameType = if difficulty == 1 {
        GameType::Normal
    } else {
        GameType::Professional
    };
    print!("\x1B[2J\x1B[1;1H");
    let mut game_mode: usize;
    loop {
        println!("Select game mode :");
        println!("(1) => two players");
        println!("(2) => play with computer");
        game_mode = App::get_user_input("\nEnter (1 or 2) : : ", "invalid input !");
        println!("\n******\nselected game mode : {}\n******\n", game_mode );
        match game_mode {
            x if x == 1 || x == 2 => break,
            _ => {println!("\ninvalid input !\n");continue;},
        }
    }
    print!("\x1B[2J\x1B[1;1H");

    let player_1: Player;
    let player_2: Player;

    println!("Enter player 1 name: ");
    let mut pl1_name: String = String::new();
    std::io::stdin().read_line(&mut pl1_name).unwrap();
    pl1_name = String::from(pl1_name.trim());
    player_1 = Player::new(pl1_name, app::enums::PlayerType::User);

    if game_mode == 1 {
        println!("Enter player 2 name: ");
        let mut pl2_name: String = String::new();
        std::io::stdin().read_line(&mut pl2_name).unwrap();
        pl2_name = String::from(pl2_name.trim());
        player_2 = Player::new(pl2_name, app::enums::PlayerType::User);
    } else {
        player_2 = Player::new(String::from("computer"), app::enums::PlayerType::Computer);
    }

    let app = App::new((player_1, player_2), game_type);
    app.start();


}
