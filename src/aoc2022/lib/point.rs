use std::ops::Add;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct Point2D {
    pub x: i32,
    pub y: i32,
}

impl ToString for Point2D {
    fn to_string(&self) -> String {
        format!("({}/{})", self.x, self.y)
    }
}

impl Point2D {
    pub fn create(x: i32, y: i32) -> Self {
        Point2D {
            x,
            y,
        }
    }

    pub fn x(&self) -> i32 {
        self.x
    }
    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn left(&self) -> Self {
        Point2D::create(self.x() - 1, self.y())
    }

    pub fn right(&self) -> Self {
        Point2D::create(self.x() + 1, self.y())
    }

    pub fn top(&self) -> Self {
        Point2D::create(self.x(), self.y() - 1)
    }

    pub fn bottom(&self) -> Self {
        Point2D::create(self.x(), self.y() + 1)
    }

    pub fn adjacents(&self) -> Vec<Self> {
        let x = self.x();
        let y = self.y();
        vec![
            Point2D::create(x - 1, y - 1),
            Point2D::create(x, y - 1),
            Point2D::create(x + 1, y - 1),
            Point2D::create(x - 1, y),
            Point2D::create(x + 1, y),
            Point2D::create(x - 1, y + 1),
            Point2D::create(x, y + 1),
            Point2D::create(x + 1, y + 1),
        ]
    }

    pub fn adjacents4(&self) -> Vec<Self> {
        let x = self.x();
        let y = self.y();
        vec![
            Point2D::create(x, y - 1),
            Point2D::create(x - 1, y),
            Point2D::create(x + 1, y),
            Point2D::create(x, y + 1),
        ]
    }

    pub fn manhatten_distance(&self, other: &Self) -> usize {
        (self.x - other.x).abs() as usize + (self.y - other.y).abs() as usize
    }
}

impl Add for Point2D {
    type Output = Point2D;

    fn add(self, rhs: Self) -> Point2D {
        Point2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

pub struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

impl Point3D {
    pub fn create(x: i32, y: i32, z: i32) -> Point3D {
        Point3D {
            x,
            y,
            z,
        }
    }

    pub fn x(&self) -> i32 {
        self.x
    }
    pub fn y(&self) -> i32 {
        self.y
    }
    pub fn z(&self) -> i32 {
        self.z
    }
}