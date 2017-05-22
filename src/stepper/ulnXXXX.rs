use hal::pin::{Pin, State};
pub use ::stepper::Stepper as halStepper;
pub use ::stepper::Direction;

const STEPS:[[State; 4]; 8] = [[State::HIGH,State::HIGH,State::LOW,State::LOW],
                [State::LOW,State::HIGH,State::LOW,State::LOW],
                [State::LOW,State::HIGH,State::HIGH,State::LOW],
                [State::LOW,State::LOW,State::HIGH,State::LOW],
                [State::LOW,State::LOW,State::HIGH,State::HIGH],
                [State::LOW,State::LOW,State::LOW,State::HIGH],
                [State::HIGH,State::LOW,State::LOW,State::HIGH],
                [State::HIGH,State::LOW,State::LOW,State::LOW],];

pub struct Stepper<'a> {
    pub direction: Direction,
    pub index: u8,
    pub pin1: &'a Pin,
    pub pin2: &'a Pin,
    pub pin3: &'a Pin,
    pub pin4: &'a Pin,
}

impl<'a> halStepper for Stepper<'a>{
    fn direction(&mut self, direction: Direction){
        self.direction = direction;
    }

    fn step(&mut self){
        let index = self.index as usize;
        match self.direction {
            Direction::RIGHT => self.index += 1,
            Direction::LEFT => self.index -= 1,
        }
        self.pin1.digital_write(&STEPS[index][0]);
        self.pin2.digital_write(&STEPS[index][1]);
        self.pin3.digital_write(&STEPS[index][2]);
        self.pin4.digital_write(&STEPS[index][3]);
    }
}