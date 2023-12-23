pub mod boot;
pub mod app;

use boot::Boot;

fn main() {
    let boot = Boot::new();
    boot.boot();
}
