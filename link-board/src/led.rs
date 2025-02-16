#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Led {
    value: (u8, u8, u8)
}

const ULTRA_DIM: u8 = 2;
const DIM_MAJOR: u8 = 20;
const DIM_MINOR: u8 = 10;
const DIM_EQ_MIX: u8 = DIM_MAJOR;
const REG_MAJOR: u8 = 30;
const REG_MINOR: u8 = 20;
const REG_EQ_MIX: u8 = REG_MAJOR;


impl Led {
    pub fn from(r: u8, g: u8, b: u8) -> Self {
        Self {
            value: (r, g, b)
        }
    }

    pub fn r(&self) -> u8 {
        self.value.0
    }

    pub fn g(&self) -> u8 {
        self.value.1
    }

    pub fn b(&self) -> u8 {
        self.value.2
    }

    pub fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }

    pub fn add_tuple(&mut self, rgb: (u8, u8, u8)) {
        self.value.0 = self.value.0.saturating_add(rgb.0);
        self.value.1 = self.value.1.saturating_add(rgb.1);
        self.value.2 = self.value.2.saturating_add(rgb.2);
    }

    pub const fn off() -> Self {
        Self {
            value: (0, 0, 0)
        }
    }

    pub const fn empty_station() -> Self {
        Self::dull_white()
    }

    pub const fn ln_1_at_station() -> Self {
        Self::green()
    }

    pub const fn ln_1_between_stations() -> Self {
        Self::dull_yellow()
    }

    pub const fn ln_2_at_station() -> Self {
        Self::blue()
    }

    pub const fn ln_2_between_stations() -> Self {
        Self::dull_purple()
    }

    pub const fn at_station_mixed() -> Self {
        Self::purple()
    }

    pub const fn between_stations_mixed() -> Self {
        Self::dull_orange()
    }

    pub const fn red() -> Self {
        Self {
            value: (REG_MAJOR, 0, 0)
        }
    }

    pub const fn green() -> Self {
        Self {
            value: (0, REG_MAJOR, 0)
        }
    }

    pub const fn dull_green() -> Self {
        Self {
            value: (0, DIM_MAJOR, 0)
        }
    }

    pub const fn blue() -> Self {
        Self {
            value: (0, 0, REG_MAJOR)
        }
    }

    pub const fn dull_blue() -> Self {
        Self {
            value: (0, 0, DIM_MAJOR)
        }
    }

    pub const fn cyan() -> Self {
        Self {
            value: (0, REG_EQ_MIX, REG_EQ_MIX)
        }
    }

    pub const fn dull_cyan() -> Self {
        Self {
            value: (5, DIM_MINOR, DIM_MAJOR)
        }
    }

    pub const fn purple() -> Self {
        Self {
            value: (REG_EQ_MIX, 0, REG_EQ_MIX)
        }
    }

    pub const fn dull_purple() -> Self {
        Self {
            value: (DIM_EQ_MIX, 0, DIM_EQ_MIX)
        }
    }

    pub const fn orange() -> Self {
        Self {
            value: (REG_MINOR, REG_MAJOR, 0)
        }
    }

    pub const fn dull_orange() -> Self {
        Self {
            value: (DIM_MINOR, DIM_MAJOR, 0)
        }
    }

    pub const fn dull_yellow() -> Self {
        Self {
            value: (DIM_EQ_MIX, DIM_EQ_MIX, 0)
        }
    }

    pub const fn dull_white() -> Self {
        Self {
            value: (ULTRA_DIM, ULTRA_DIM, ULTRA_DIM)
        }
    }

    pub const fn as_tuple(&self) -> (u8, u8, u8) {
        self.value
    }
}