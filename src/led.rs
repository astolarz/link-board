#[derive(Clone, Copy)]
#[derive(PartialEq)]
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

    pub fn add_tuple(&mut self, rgb: (u8, u8, u8)) {
        self.value.0 += rgb.0;
        self.value.1 += rgb.1;
        self.value.2 += rgb.2;
    }

    pub const fn off() -> Self {
        Self {
            value: (0, 0, 0)
        }
    }

    pub const fn red() -> Self {
        Self {
            value: (255, 0, 0)
        }
    }

    pub const fn green() -> Self {
        Self {
            value: (0, 255, 0)
        }
    }

    pub const fn blue() -> Self {
        Self {
            value: (0, 0, 255)
        }
    }

    pub const fn purple() -> Self {
        Self {
            value: (255, 0, 255)
        }
    }

    pub const fn orange() -> Self {
        Self {
            value: (255, 165, 0)
        }
    }

    pub const fn dull_yellow() -> Self {
        Self {
            value: (100, 100, 0)
        }
    }

    pub const fn as_tuple(&self) -> (u8, u8, u8) {
        self.value
    }
}