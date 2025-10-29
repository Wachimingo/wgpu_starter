use crate::logic::run::run;

mod graphics;
mod logic;
mod window;

fn main() {
    pollster::block_on(run());
}
