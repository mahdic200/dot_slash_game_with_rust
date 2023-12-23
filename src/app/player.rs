use super::{enums::PlayerType, point::Point};
use crate::app::App;


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

    fn get_user_input(&self, edge_size: usize, points: &mut Vec<Point>) {
        let mut row: usize;
        let mut col: usize;
        loop {
            row = App::get_user_input("Enter row number :", "invalid row !");
            println!("selected row : {}", row);
            row -= 1;
            match row {
                row if (row < edge_size) => {},
                _ => {println!("\ninvalid input !\n");continue;},
            }
            col = App::get_user_input("Enter col number :", "invalid col !");
            col = match col {
                col if row % 2 == 0 => 2 * col - 1,
                col if row % 2 == 1 => 2 * (col - 1),
                _ => col,
            };
            match col {
                x if x < edge_size => {
                    match &mut points[row * (edge_size) + col] {
                        point if point.is_valid_line() && !point.has_owner() => {
                            point.set_owner(String::from(&self.name));
                            break;
                        },
                        point => {
                            println!("\n\n******\ninvalid choice !\n******");
                            if point.has_owner() {println!("\ncell is taken !\n")};
                            continue;
                        },
                    }
                },
                _ => {println!("\ninvalid input !\n");continue;},
            }
        }
    }
    
    fn get_computer_input(&self, _: usize, points: &mut Vec<Point>) {
        use rand::Rng;
        loop {
            let mut index = rand::thread_rng().gen_range(0..=500);
            index = index % points.len();
            match &mut points[index] {
                point if point.is_valid_line() && !point.has_owner() => {
                    point.set_owner(String::from(&self.name));
                    break
                },
                _ => continue,
            }
        }
    }

    pub fn get_player_input(&self, edge_size: usize, points: &mut Vec<Point>) {
        match &self.player_type {
            PlayerType::User => self.get_user_input(edge_size, points),
            PlayerType::Computer => self.get_computer_input(edge_size, points),
        }
    }
}