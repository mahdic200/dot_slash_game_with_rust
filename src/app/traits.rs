pub trait Draw {
    fn draw(&self);
}

pub trait Owner {
    fn has_owner(&self) -> bool {
        false
    }
    fn owner_name(&self) -> String {
        format!("  ")
    }
}

pub trait Player {
    fn name(&self) -> String;
    // fn color(&self) -> String;
}