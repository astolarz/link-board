#[derive(Clone, Copy, PartialEq)]
pub struct Led {
    value: (u8, u8, u8)
}

impl Led {
    pub fn r(&self) -> u8 {
        self.value.0
    }

    pub fn g(&self) -> u8 {
        self.value.1
    }

    pub fn b(&self) -> u8 {
        self.value.2
    }

    #[allow(dead_code)]
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

    pub const fn red() -> Self {
        Self {
            value: (155, 0, 0)
        }
    }

    pub const fn green() -> Self {
        Self {
            value: (0, 155, 0)
        }
    }

    pub const fn dull_green() -> Self {
        Self {
            value: (0, 50, 0)
        }
    }

    pub const fn blue() -> Self {
        Self {
            value: (0, 0, 155)
        }
    }

    pub const fn dull_blue() -> Self {
        Self {
            value: (0, 0, 50)
        }
    }

    pub const fn cyan() -> Self {
        Self {
            value: (0, 155, 155)
        }
    }

    pub const fn dull_cyan() -> Self {
        Self {
            value: (0, 50, 50)
        }
    }

    pub const fn purple() -> Self {
        Self {
            value: (155, 0, 155)
        }
    }

    pub const fn orange() -> Self {
        Self {
            value: (155, 165, 0)
        }
    }

    pub const fn dull_orange() -> Self {
        Self {
            value: (50, 70, 0)
        }
    }

    pub const fn dull_yellow() -> Self {
        Self {
            value: (50, 50, 0)
        }
    }

    #[allow(dead_code)]
    pub const fn as_tuple(&self) -> (u8, u8, u8) {
        self.value
    }
}