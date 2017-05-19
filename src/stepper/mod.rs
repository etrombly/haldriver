pub trait Stepper {
    fn direction(&mut self, direction: Direction);
    fn step(&mut self);
}

pub enum Direction{
    RIGHT,
    LEFT,
}

pub mod ulnXXXX;