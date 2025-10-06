use crate::logic::run::run;

mod window;
mod logic;

fn main() {
    pollster::block_on(run());
}
