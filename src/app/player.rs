use super::{enums::PlayerType, point::Point};
use crate::App;


#[derive(Debug)]
#[allow(unused)]
pub struct Player {
    name: String,
    player_type: PlayerType,
}

impl Player {
    pub fn new(name: String, player_type: PlayerType) -> Player {
        Player {
            name,
            player_type,
        }
    }

    pub fn name(&self) -> String {
        String::from(&self.name)
    }

    fn get_user_input(&self, stars: usize, points: &mut Vec<Point>) -> (usize, usize) {
        let mut row: usize;
        let mut col: usize;
        loop {
            row = App::get_user_input("Enter row number :", "invalid row !");
            println!("selected row : {}", row);
            match row {
                row if (row < 2 * stars - 1) => {},
                _ => {println!("\ninvalid input !\n");continue;},
            }
            col = App::get_user_input("Enter col number :", "invalid col !");
            println!("selected row : {}", col);
            match col {
                x if x < 2 * stars => {
                    match &mut points[row * (2 * stars - 1) + col] {
                        point if !point.is_valid_line() => {println!("\ninvalid line !\n");continue;},
                        point if point.has_owner() => {println!("\nline is taken !\n");continue;},
                        point => {
                            point.set_owner(String::from(&self.name));
                            break;
                        },
                    }
                },
                _ => {println!("\ninvalid input !\n");continue;},
            }
        }
        (row, col)
    }
    
    fn get_computer_input(&self, stars: usize, points: &mut Vec<Point>) -> (usize, usize) {
        use rand::Rng;
        let row : usize;
        let col : usize;
        loop {
            let mut index = rand::thread_rng().gen_range(0..=500);
            index = index % points.len();
            match &mut points[index] {
                point if !point.is_valid_line() => continue,
                point if point.has_owner() => continue,
                point => {
                    point.set_owner(String::from(&self.name));
                    row = (index % (2 * stars)) - 1;
                    col = index % (2 * stars - 1);
                    break;
                },
            }

        }
        println!("computer selected row : {} and col : {}", row, col);
        (row, col)
    }

    pub fn get_player_input(&self, stars: usize, points: &mut Vec<Point>) -> (usize, usize) {
        match &self.player_type {
            PlayerType::User => self.get_user_input(stars, points),
            PlayerType::Computer => self.get_computer_input(stars, points),
        }
    }
}