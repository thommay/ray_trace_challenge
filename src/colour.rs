use lazy_static::lazy_static;
use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Colour {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Colour {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Colour { red, green, blue }
    }

    pub fn is_colour(&self) -> bool {
        true
    }

    #[cfg(test)]
    pub(crate) fn round(&self, factor: f64) -> Self {
        Self {
            red: { (self.red * factor).round() / factor },
            green: { (self.green * factor).round() / factor },
            blue: { (self.blue * factor).round() / factor },
        }
    }
}

impl Default for Colour {
    fn default() -> Self {
        Self {
            red: 0f64,
            green: 0f64,
            blue: 0f64,
        }
    }
}

impl std::fmt::Display for Colour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn clamp(val: f64) -> f64 {
            if val < 0f64 {
                0.0
            } else if val > 1f64 {
                1.0
            } else {
                val.into()
            }
        }

        write!(
            f,
            "{} {} {}",
            (clamp(self.red) * 255f64).round(),
            (clamp(self.green) * 255f64).round(),
            (clamp(self.blue) * 255f64).round()
        )
    }
}

impl Add<Colour> for Colour {
    type Output = Colour;

    fn add(self, rhs: Colour) -> Self::Output {
        Self::Output {
            red: { self.red + rhs.red },
            green: { self.green + rhs.green },
            blue: { self.blue + rhs.blue },
        }
    }
}

impl Sub<Colour> for Colour {
    type Output = Colour;

    fn sub(self, rhs: Colour) -> Self::Output {
        Self::Output {
            red: { self.red - rhs.red },
            green: { self.green - rhs.green },
            blue: { self.blue - rhs.blue },
        }
    }
}

impl Div<Colour> for Colour {
    type Output = Colour;

    fn div(self, rhs: Colour) -> Self::Output {
        Self::Output {
            red: { self.red / rhs.red },
            green: { self.green / rhs.green },
            blue: { self.blue / rhs.blue },
        }
    }
}

impl Mul<f64> for Colour {
    type Output = Colour;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output {
            red: { self.red * rhs },
            green: { self.green * rhs },
            blue: { self.blue * rhs },
        }
    }
}

impl Mul<Colour> for Colour {
    type Output = Colour;

    fn mul(self, rhs: Colour) -> Self::Output {
        Self::Output {
            red: { self.red * rhs.red },
            green: { self.green * rhs.green },
            blue: { self.blue * rhs.blue },
        }
    }
}

lazy_static! {
    pub static ref BLACK: Colour = Colour::new(0f64, 0f64, 0f64);
    pub static ref WHITE: Colour = Colour::new(1f64, 1f64, 1f64);
}

#[cfg(test)]
mod test {
    use crate::colour::Colour;

    #[test]
    fn add_colours() {
        let c1 = Colour::new(0.9, 0.6, 0.75);
        let c2 = Colour::new(0.7, 0.1, 0.25);
        assert_eq!(c1 + c2, Colour::new(1.6, 0.7, 1.0))
    }
}
