#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    frames: Vec<Vec<u16>>,
    finished: bool
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame { frames: vec![], finished: false }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }
        if self.finished {
            return Err(Error::GameComplete);
        }

        match self.frames.len() {
            0 => {
                self.frames.push(vec![pins]);
                Ok(())
            },
            10 => {
                let last_frame = self.frames.last_mut().unwrap();
                if last_frame.len() == 1 {
                    let value = last_frame[0];
                    if value == 10 {
                        last_frame.push(pins);
                        Ok(())
                    } else if value + pins < 10 {
                        last_frame.push(pins);
                        self.finished = true;
                        Ok(())
                    } else if value + pins == 10 {
                        last_frame.push(pins);
                        Ok(())
                    } else {
                        Err(Error::NotEnoughPinsLeft)
                    }
                } else {
                    let value1 = last_frame[0];
                    let value2 = last_frame[1];
                    if (value2 + pins <= 10 || value2 == 10) && value1 == 10 || (value1 + value2 == 10){
                        last_frame.push(pins);
                        self.finished = true;
                        Ok(())
                    } else {
                        Err(Error::NotEnoughPinsLeft)
                    }
                }
            },
            _ => {
                let last_frame = self.frames.last_mut().unwrap();
                let value = last_frame[0];
                if last_frame.len() == 2 || (last_frame.len() == 1 && value == 10) {
                    self.frames.push(vec![pins]);
                    Ok(())
                } else if value + pins <= 10 {
                    last_frame.push(pins);
                    Ok(())
                } else {
                    Err(Error::NotEnoughPinsLeft)
                }
            }
        }
    }

    pub fn score(&self) -> Option<u16> {
        if self.finished {
            let mut total = 0;
            for i in 0..9 {
                let current_frame = &self.frames[i];
                let frame_total = current_frame.iter().sum::<u16>();

                if current_frame.len() == 1 {
                    let next_frame = &self.frames[i+1];
                    if next_frame.len() == 1 {
                        let next_next_frame = &self.frames[i+2];
                        total += frame_total + next_frame[0] + next_next_frame[0];
                    } else {
                        total += frame_total  + next_frame[0] + next_frame[1]
                    }
                } else if frame_total == 10 {
                    let next_frame = &self.frames[i+1];
                    total += frame_total + next_frame[0];
                } else {
                    total += frame_total;
                }
            }

            let final_frame = &self.frames[9];
            let final_frame_total = final_frame.iter().sum::<u16>();
            total += final_frame_total;

            Some(total)
        } else { None }
    }
}
