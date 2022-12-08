use std::fmt::{Display, Formatter};
use std::ops::Add;

pub struct Point2D {
    x: i32,
    y: i32,
}

impl Display for Point2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}/{})", self.x, self.y)
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