pub mod player;
pub mod point;
pub mod screen;
pub mod traits;
pub mod enums;

use std::cell::RefCell;

use point::Point;
use enums::GameType;
use player::Player;
use point::Points;


#[allow(unused)]
pub struct App {
    player_1: Player,
    player_2: Player,
    score_1: usize,
    score_2: usize,
    stars: u8,
    game_type: GameType,
    points: RefCell<Vec<Point>>,
}

impl App {
    pub fn new(players: (Player, Player), game_type: GameType) -> App {
        App {
            player_1: players.0,
            player_2: players.1,
            score_1: 0,
            score_2: 0,
            stars: App::rec_stars(&game_type),
            game_type: game_type.clone(),
            points: RefCell::new(App::init_points(&game_type)),
        }
    }

    fn rec_stars(game_type: &GameType) -> u8 {
        match game_type {
            GameType::Normal => 4,
            GameType::Professional => 6,
        }
    }

    fn rows(&self) -> u8 {
        2 * &self.stars - 1
    }

    fn cols(&self) -> u8 {
        2 * &self.stars - 1
    }

    fn recognize_point_type(row_num: &u8, col_num: &u8) -> Points {
        let i = row_num;
        let j = col_num;
        match j {
            j if i % 2 == 0 => match j {
                j if j % 2 == 0 => Points::Star,
                j if j % 2 == 1 => Points::HorLine,
                _ => Points::Star,
            },
            j if i % 2 == 1 => match j {
                j if j % 2 == 0 => Points::VerLine,
                j if j % 2 == 1 => Points::SqCenter,
                _ => Points::VerLine,
            },
            _ => Points::Star,
        }
    }

    fn init_points(game_type: &GameType) -> Vec<Point> {
        let rows = 2 * App::rec_stars(game_type) - 1;
        let cols = rows;
        let mut points: Vec<Point> = Vec::new();

        for i in 0..rows {
            for j in 0..cols {
                let type_ = App::recognize_point_type(&i, &j);
                points.push(Point::new(type_));
            }
        }
        points
    }

    pub fn draw(&self, points: &Vec<Point>) {
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                let index = (i * self.cols() + j) as usize;
                let points = points;
                let point = &points[index];
                point.get_symbol();
                if point.is_valid_line() {
                    if point.owner_name() == self.player_1.name() {
                        print!("\u{1b}[31m{}\u{1b}[39m", point.get_symbol());
                    } else if point.owner_name() == self.player_2.name() {
                        print!("\u{1b}[34m{}\u{1b}[39m", point.get_symbol());
                    } else {
                        point.draw();
                    }
                } else {
                    point.draw();
                }
            }
            println!("");
        }
        println!("");
    }

    fn get_index(&self, i: u8, j: u8) -> usize {
        (i * &self.cols() + j) as usize
    }

    pub fn get_user_input(msg: &str, err: &str) -> usize {
        let mut result: usize = 0;
        #[allow(while_true)]
        while true {
            let mut user_input: String = String::new();
            println!("{msg}");
            std::io::stdin().read_line(&mut user_input).unwrap();
            match user_input.trim().parse::<usize>() {
                Ok(user_input) => {result = user_input;break;},
                Err(_) => println!("\n{err}\n"),
            }
        };
        return result;
    }

    pub fn start(&self) {
        let points = &mut *self.points.borrow_mut();
        let mut turn = 0;
        let (mut sc1, mut sc2): (usize, usize) = (0, 0);
        loop {
            print!("\x1B[2J\x1B[1;1H");
            if self.is_done(points) {break;};
            (sc1, sc2) = self.sq_check(points);
            self.draw(points);
            println!("{}'s Score : {}", self.player_1.name(), sc1);
            println!("{}'s Score : {}", self.player_2.name(), sc2);
            let row: usize;
            let col: usize;
            if turn == 0 {
                println!("{}'s turn", &self.player_1.name());
                (row, col) = self.player_1.get_player_input(self.stars as usize, points);
            } else {
                println!("{}'s turn", &self.player_2.name());
                (row, col) = self.player_2.get_player_input(self.stars as usize, points);
            }
            
            println!("row : {}, col : {}", row, col);
            
            turn = if turn == 1 {0} else {1};
        }
        print!("\x1B[2J\x1B[1;1H");
        self.draw(points);
        println!("\n");
        println!("{}'s scores : {}", self.player_1.name(), sc1);
        println!("{}'s scores : {}", self.player_2.name(), sc2);
        if sc1 > sc2 {
            println!("{} is winner !", self.player_1.name());
        } else if sc2 > sc1 {
            println!("{} is winner !", self.player_2.name());
        } else {
            println!("draw !");
        }
    }

    pub fn is_square(&self, i: u8, j: u8, points: &mut Vec<Point>, p_name: &str) -> bool {
        if points[self.get_index(i - 1, j)].owner_name() == p_name
        && points[self.get_index(i, j - 1)].owner_name() == p_name
        && points[self.get_index(i, j + 1)].owner_name() == p_name
        && points[self.get_index(i + 1, j)].owner_name() == p_name {
            true
        } else {
            false
        }
    }

    pub fn sq_check(&self, points: &mut Vec<Point>) -> (usize, usize) {
        let mut sc1 = 0;
        let mut sc2 = 0;
        for i in (1..self.rows()).step_by(2) {
            for j in (1..self.cols()).step_by(2) {
                if self.is_square(i, j, points, &self.player_1.name()) {
                    points[self.get_index(i, j)].set_owner(format!("P1"));
                    sc1 += 1;
                } else if self.is_square(i, j, points, &self.player_2.name()){
                    points[self.get_index(i, j)].set_owner(format!("P2"));
                    sc2 += 1;
                }
            }
        }
        (sc1, sc2)
    }

    pub fn is_done(&self, points: &mut Vec<Point>) -> bool {
        let mut count = self.stars * self.stars + (self.stars - 1) * (self.stars - 1);
        for item in &*points {
            match item {
                point => match point.get_point_type() {
                    Points::HorLine if point.has_owner() => {count += 1},
                    Points::VerLine if point.has_owner() => {count += 1},
                    _ => {},
                },
            }
        }
        if count as usize == points.len() {
            true
        } else {
            false
        }
    }
}