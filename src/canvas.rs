use crate::vec3::TypedVec;
use std::fmt::Write;

#[derive(Debug, Clone)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pixels: Vec<TypedVec>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let mut pixels = Vec::new();
        let p = TypedVec::colour(0.0, 0.0, 0.0);
        (0..width * height).for_each(|_| pixels.push(p));
        Self {
            width,
            height,
            pixels,
        }
    }

    pub fn save(&self) -> String {
        let mut s = format!("P3\n{} {}\n255\n", self.width, self.height);
        for y in 0..self.height {
            let mut l = String::new();
            for x in 0..self.width {
                if l.len() >= 58 {
                    s.push_str(l.as_str());
                    s.push('\n');
                    l.truncate(0)
                }
                write!(l, "{} ", self.pixels[(x + y * self.width) as usize]).unwrap();
            }
            s.push_str(l.as_str());
            s.push('\n');
        }

        s.push('\n');
        s
    }

    pub fn fill(&mut self, colour: TypedVec) {
        (0..self.width * self.height).for_each(|n| self.pixels[n as usize] = colour);
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, colour: TypedVec) {
        if (x >= self.width) || (y >= self.height) {
            return;
        }
        self.pixels[(x + y * self.width) as usize] = colour;
    }

    pub fn get(&self, x: usize, y: usize) -> Option<TypedVec> {
        Some(self.pixels[(x + y * self.width) as usize])
    }
}

#[cfg(test)]
mod tests {
    use crate::canvas::Canvas;
    use crate::vec3::TypedVec;

    #[test]
    fn test_create() {
        let c = Canvas::new(10, 20);
        assert_eq!(c.get(3, 4).unwrap(), TypedVec::colour(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_set_pixel() {
        let mut c = Canvas::new(10, 20);
        c.write_pixel(1, 0, TypedVec::colour(1.0, 0.0, 0.0));
        assert_eq!(c.get(1, 0).unwrap(), TypedVec::colour(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_save_blank() {
        let mut c = Canvas::new(10, 2);

        c.fill(TypedVec::colour(1.0, 0.8, 0.6));
        dbg!(&c.save());
    }
}
