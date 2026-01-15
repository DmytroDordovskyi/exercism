#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    rolls: Vec<u16>,
    next_roll_completes_frame: bool,
}

const MAX_PINS: u16 = 10;
const MAX_FRAMES: usize = 10;

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame { rolls: vec![], next_roll_completes_frame: false }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.score().is_some() { return Err(Error::GameComplete) };
        if (pins > MAX_PINS) || (self.next_roll_completes_frame && self.rolls.last().unwrap() + pins > MAX_PINS) {
            return Err(Error::NotEnoughPinsLeft)
        };

        self.rolls.push(pins);

        if self.next_roll_completes_frame {
            self.next_roll_completes_frame = false;
        } else {
            self.next_roll_completes_frame = pins != MAX_PINS;
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        let rolls_count = self.rolls.len();
        let mut completed_frames = 0;
        let mut total: u16 = 0;

        let mut i = 0;

        while completed_frames < MAX_FRAMES - 1 && i + 1 < rolls_count {
            if self.rolls[i] == MAX_PINS {
                if i + 2 >= rolls_count { return None };
                total += self.rolls[i] + self.rolls[i+1] + self.rolls[i+2];
                i += 1;
            } else if self.rolls[i] + self.rolls[i+1] == MAX_PINS {
                if i + 2 >= rolls_count { return None };
                total += self.rolls[i] + self.rolls[i+1] + self.rolls[i+2];
                i += 2;
            } else {
                total += self.rolls[i] + self.rolls[i+1];
                i += 2;
            }
            completed_frames += 1;
        }

        if completed_frames != MAX_FRAMES - 1 { return None };

        let last_frame: Vec<u16> = self.rolls[i..].to_vec();

        let is_complete = match last_frame.len() {
            0 | 1 => false,
            2 => last_frame[0] + last_frame[1] < MAX_PINS,
            _ => true,
        };

        if is_complete {
            total += last_frame.iter().sum::<u16>();
            Some(total)
        } else {
            None
        }
    }
}
