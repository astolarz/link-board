use crate::Direction;

#[derive(Debug, Clone)]
pub struct Train {
    // id: String,
    // next_stop_id: String,
    pub next_stop_name: String,
    // time_until: std:time::duration,
    direction: Direction,
    pub at_station: bool,
}

impl Train {
    pub fn new(
        // id: String,
        // next_stop_id: String,
        next_stop_name: String,
        direction: Direction,
        at_station: bool) -> Self {
        Self {
            // id,
            // next_stop_id,
            next_stop_name,
            direction,
            at_station,
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