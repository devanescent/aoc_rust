#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Point { x, y }
    }

    pub fn steps_to(&self, other: &Point) -> u64 {
        u64::try_from((other.x - self.x).abs()).unwrap() + 
        u64::try_from((other.y - self.y).abs()).unwrap()
    }

    pub fn distance_to(&self, other: &Point) -> Point {
        Point { x: other.x - self.x, y: other.y - self.y }
    }

    pub fn add(&self, other: &Point) -> Point {
        Point { x: self.x + other.x, y: self.y + other.y }
    }

    pub fn move_by(&self, x: i64, y: i64) -> Point {
        Point { x: self.x + x, y: self.y + y }
    }
}

impl From<(usize, usize)> for Point {
    fn from((x,y): (usize, usize)) -> Self {
        Point {
          x: i64::try_from(x).unwrap(),
          y: i64::try_from(y).unwrap()
        }
    }
}