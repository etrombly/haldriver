use hal::pin::{Pin, State};
pub use ::stepper::Stepper as halStepper;
pub use ::stepper::Direction;

/*
const STEPS:[[State; 4]; 8] = [[State::HIGH,State::HIGH,State::LOW,State::LOW],
                [State::LOW,State::HIGH,State::LOW,State::LOW],
                [State::LOW,State::HIGH,State::HIGH,State::LOW],
                [State::LOW,State::LOW,State::HIGH,State::LOW],
                [State::LOW,State::LOW,State::HIGH,State::HIGH],
                [State::LOW,State::LOW,State::LOW,State::HIGH],
                [State::HIGH,State::LOW,State::LOW,State::HIGH],
                [State::HIGH,State::LOW,State::LOW,State::LOW],];
*/

const STEPS:[[State; 4]; 4] = [[State::HIGH,State::HIGH,State::LOW,State::LOW],
                [State::LOW,State::HIGH,State::HIGH,State::LOW],
                [State::LOW,State::LOW,State::HIGH,State::HIGH],
                [State::HIGH,State::LOW,State::LOW,State::HIGH],];

pub struct Stepper<'a, T> 
where T: 'a{
    pub direction: Direction,
    pub index: u8,
    pub pin1: &'a Pin<T>,
    pub pin2: &'a Pin<T>,
    pub pin3: &'a Pin<T>,
    pub pin4: &'a Pin<T>,
}

impl<'a, T> halStepper for Stepper<'a, T>{
    fn direction(&mut self, direction: Direction){
        self.direction = direction;
    }

    fn step(&mut self){
        self.pin1.digital_write(&STEPS[index as usize][0]);
        self.pin2.digital_write(&STEPS[index as usize][1]);
        self.pin3.digital_write(&STEPS[index as usize][2]);
        self.pin4.digital_write(&STEPS[index as usize][3]);

        self.index = match self.direction {
            Direction::RIGHT => {if self.index < 3 {
                    self.index + 1
                } else {
                    0
                }},
            Direction::LEFT => {if self.index > 0 {
                    self.index - 1
                } else {
                    3
                }},
        }
    }
}