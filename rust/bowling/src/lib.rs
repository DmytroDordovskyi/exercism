#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

enum Frame {
    Strike,
    Spare(u16, u16),
    Regular(u16, u16),
    Tenth(Vec<u16>),
}

pub struct BowlingGame {
    completed_frames: Vec<Frame>,
    current_frame: Vec<u16>,
}

const MAX_PINS: u16 = 10;
const MAX_FRAMES: usize = 10;

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame { completed_frames: vec![], current_frame: vec![] }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.completed_frames.len() == MAX_FRAMES { return Err(Error::GameComplete); }
        if pins > MAX_PINS {
            return Err(Error::NotEnoughPinsLeft);
        }

        match (self.current_frame.is_empty(), self.completed_frames.len() < MAX_FRAMES - 1) {
            (true, true) => {
                if pins == MAX_PINS {
                    self.completed_frames.push(Frame::Strike);
                } else {
                    self.current_frame.push(pins);
                }
            },
            (true, false) => { self.current_frame.push(pins) },
            (false, true) => {
                let first_roll = self.current_frame[0];
                if first_roll + pins == MAX_PINS {
                    self.completed_frames.push(Frame::Spare(first_roll, pins));
                } else if first_roll + pins < MAX_PINS {
                    self.completed_frames.push(Frame::Regular(first_roll, pins));
                } else {
                    return Err(Error::NotEnoughPinsLeft);
                }
                self.current_frame.clear();
            },
            _ => {
                let first_roll = self.current_frame[0];

                if self.current_frame.len() == 2 {
                    let second_roll = self.current_frame[1];
                    if first_roll == MAX_PINS && second_roll != MAX_PINS && second_roll + pins > MAX_PINS {
                        return Err(Error::NotEnoughPinsLeft);
                    }
                }

                self.current_frame.push(pins);
                if self.current_frame.len() == 3 || (self.current_frame.len() == 2 && first_roll != MAX_PINS && first_roll + pins != MAX_PINS) {
                    self.completed_frames.push(Frame::Tenth(self.current_frame.clone()));
                    self.current_frame.clear();
                }
            }
        }

        Ok(())
    }

    fn get_roll(&self, frame_idx: usize, roll_idx: usize) -> u16 {
        match &self.completed_frames[frame_idx] {
            Frame::Strike => {
                if roll_idx == 0 {
                    10
                } else {
                    self.get_roll(frame_idx + 1, 0)
                }
            },
            Frame::Spare(first, second) | Frame::Regular(first, second) => {
                if roll_idx == 0 { *first } else { *second }
            },
            Frame::Tenth(v) => v[roll_idx],
        }
    }

    pub fn score(&self) -> Option<u16> {
        if self.completed_frames.len() != MAX_FRAMES {
            return None;
        }

        let mut total = 0;
        for i in 0..9 {
            match &self.completed_frames[i] {
                Frame::Strike => {
                    total += 10 + self.get_roll(i + 1, 0) + self.get_roll(i + 1, 1);
                },
                Frame::Spare(first, second) => {
                    total += first + second + self.get_roll(i + 1, 0);
                },
                Frame::Regular(first, second) => {
                    total += first + second;
                },
                Frame::Tenth(_) => unreachable!("10th frame can't be in first 9"),
            }
        }

        if let Frame::Tenth(rolls) = &self.completed_frames[9] {
            total += rolls.iter().sum::<u16>();
        }

        Some(total)
    }
}
