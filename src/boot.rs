use crate::app::App;
use super::app::player::Player;
use crate::app::enums::{GameType, PlayerType};

pub struct Boot {

}

impl Boot {
    pub fn new() -> Boot {
        Boot {}
    }

    fn clear_screen(&self) {
        print!("\x1B[2J\x1B[1;1H");
    }

    fn config_difficulty(&self) -> GameType {
        let mut difficulty: usize;
        loop {
            println!("Select game difficulty : ");
            println!("(1) => normal");
            println!("(2) => professional");
            difficulty = App::get_user_input("\nEnter (1 or 2) : ", "invalid input !");
            match difficulty {
                x if x == 1 || x == 2 => break,
                _ => {println!("\ninvalid input !\n");continue;},
            }
        }
        if difficulty == 1 {
            GameType::Normal
        } else {
            GameType::Professional
        }
    }

    fn config_game_mode(&self) -> usize {
        let mut game_mode: usize;
        loop {
            println!("Select game mode :");
            println!("(1) => two players");
            println!("(2) => play with computer");
            game_mode = App::get_user_input("\nEnter (1 or 2) : ", "invalid input !");
            match game_mode {
                x if x == 1 || x == 2 => break,
                _ => {println!("\ninvalid input !\n");continue;},
            }
        }
        game_mode
    }

    fn set_players(&self, mode: usize) -> (Player, Player) {
        let player_1: Player;
        let player_2: Player;

        println!("Enter player 1 name: ");
        let mut pl1_name: String = String::new();
        std::io::stdin().read_line(&mut pl1_name).unwrap();
        pl1_name = String::from(pl1_name.trim());
        player_1 = Player::new(pl1_name, PlayerType::User);
    
        if mode == 1 {
            println!("Enter player 2 name: ");
            let mut pl2_name: String = String::new();
            std::io::stdin().read_line(&mut pl2_name).unwrap();
            pl2_name = String::from(pl2_name.trim());
            player_2 = Player::new(pl2_name, PlayerType::User);
        } else {
            player_2 = Player::new(String::from("computer"), PlayerType::Computer);
        }

        (player_1, player_2)
    }

    fn initializer(&self) -> App {
        self.clear_screen();
        let difficulty: GameType = self.config_difficulty();
        self.clear_screen();
        let game_mode: usize = self.config_game_mode();
        self.clear_screen();
        let (player_1, player_2) = self.set_players(game_mode);

        let app = App::new((player_1, player_2), difficulty);
        app

    }

    pub fn boot(&self) {
        self.initializer()
            .start();
    }
}