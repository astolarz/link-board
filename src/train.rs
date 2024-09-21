use crate::Direction;

#[derive(Debug)]
pub struct Train {
    id: u16,
    next_stop_id: String,
    next_stop_name: String,
    // time_until: std:time::duration,
    direction: Direction,
}

impl Train {
    pub fn new(id: u16, next_stop_id: String, next_stop_name: String, direction: Direction) -> Self {
        Self {
            id,
            next_stop_id,
            next_stop_name,
            direction,
        }
    }

    pub fn add_dir(&mut self, direction: Direction) -> &mut Self {
        self.direction = direction;
        self
    }

    pub fn direction(&self) -> Direction {
        self.direction
    }
}