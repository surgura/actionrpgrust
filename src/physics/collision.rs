#[derive(Eq, Hash)]
pub struct Collision {
    pub a: u32, // smalest is always a
    pub b: u32,
}

impl Collision {
    pub fn new(a: u32, b: u32) -> Self {
        if a < b {
            Self { a, b }
        } else {
            Self { b, a }
        }
    }
}

impl PartialEq for Collision {
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b
    }
}
