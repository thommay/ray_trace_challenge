use crate::colour::Colour;
use crate::vec3::TypedVec;

#[derive(Copy, Clone, PartialEq, Debug, PartialOrd)]
pub struct Point {
    pub(crate) intensity: Colour,
    pub(crate) position: TypedVec,
}

impl Point {
    pub fn new(position: TypedVec, intensity: Colour) -> Self {
        Point {
            intensity,
            position,
        }
    }
}
