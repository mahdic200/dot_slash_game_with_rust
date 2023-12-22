pub struct Screen {
    // drawer: T,
}

impl Screen {
    pub fn new() -> Screen {
        Screen {
            // drawer,
        }
    }

    pub fn print(&self) {
        for i in 0..7 {
            for j in 0..9 {
                match j {
                    j if i % 2 == 0 => match j {
                        j if j % 2 == 0 => print!("*"),
                        j if j % 2 != 0 => print!("--"),
                        _ => (),
                    }
                    j if i % 2 == 1 => match j {
                        j if j % 2 == 0 => print!("|"),
                        j if j % 2 != 0 => print!("A "),
                        _ => (),
                    }
                    _ => (),
                }
            }
            println!("");
        }
    }

}
