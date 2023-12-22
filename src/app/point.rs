#[derive(Debug)]
pub enum Points {
    Star,
    SqCenter,
    HorLine,
    VerLine,
}

#[derive(Debug)]
#[allow(unused)]
pub struct Point {
    symbol: String,
    point_type: Points,
    owner: String,
}

impl Point {
    pub fn new(point_type: Points) -> Point {
        Point {
            symbol: Point::rec_symbol(&point_type),
            point_type,
            owner: String::from(""),
        }
    }

    fn rec_symbol(_type: &Points) -> String {
        match _type {
            Points::Star => String::from("*"),
            Points::VerLine => String::from(" "),
            Points::HorLine => String::from("  "),
            Points::SqCenter => String::from("  "),
        }
    }

    pub fn draw(&self) {
        match &self.point_type {
            Points::Star => print!("{}", &self.symbol),
            Points::SqCenter => print!("{}", &self.owner_name()),
            _ => print!("{}", &self.symbol),
        }
    }

    pub fn has_owner(&self) -> bool {
        match &self.point_type {
            Points::Star => false,
            _ => match &self.owner {
                name if name == "" || name == " " || name == "  " => false,
                _ => true,
            }
        }
    }

    pub fn owner_name(&self) -> String {
        match &self.owner {
            owner if owner == "" => String::from("  "),
            owner => owner.clone(),
        }
    }

    pub fn set_symbol(&mut self, symbol: String) {
        self.symbol = symbol;
    }

    pub fn set_owner(&mut self, owner: String) {
        match &self.point_type {
            Points::VerLine => {self.symbol = String::from("|");self.owner = owner;},
            Points::HorLine => {self.symbol = String::from("--");self.owner = owner;},
            Points::SqCenter => {self.symbol = String::from("SS");self.owner = owner;},
            _ => (),
        };
    }

    pub fn get_symbol(&self) -> &String {
        &self.symbol
    }

    pub fn get_point_type(&self) -> &Points {
        &self.point_type
    }

    pub fn is_valid_line(&self) -> bool {
        match self.point_type {
            Points::HorLine => true,
            Points::VerLine => true,
            _ => false,
        }
    }
}

