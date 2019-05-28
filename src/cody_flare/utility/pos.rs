use std::ops;
//use std::f64;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Pos {
    pub x: f64,
    pub y: f64,
}

impl ops::Add for Pos {
    type Output = Pos;
    fn add(self, others: Pos) -> Pos {
        Pos {x: self.x + others.x, y: self.y + others.y}
    }
}

impl ops::Add for &Pos {
    type Output = Pos;
    fn add(self, others: &Pos) -> Pos {
        Pos {x: self.x + others.x, y: self.y + others.y}
    }
}

impl ops::Sub for Pos {
    type Output = Pos;
    fn sub(self, others: Pos) -> Pos {
        Pos {x: self.x - others.x, y: self.y - others.y}
    }
}

impl ops::Sub for &Pos {
    type Output = Pos;
    fn sub(self, others: &Pos) -> Pos {
        Pos {x: self.x - others.x, y: self.y - others.y}
    }
}

impl ops::Neg for Pos {
    type Output = Pos;
    fn neg(self) -> Pos {
        Pos {x: -self.x, y: -self.y}
    }
}

impl ops::Neg for &Pos {
    type Output = Pos;
    fn neg(self) -> Pos {
        Pos {x: -self.x, y: -self.y}
    }
}

impl ops::MulAssign<f64> for Pos {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl ops::Mul for Pos {
    type Output = f64;
    fn mul(self, others: Pos) -> f64 {
        self.x * others.x + self.y * others.y
    }
}

impl ops::Mul for &Pos {
    type Output = f64;
    fn mul(self, others: &Pos) -> f64 {
        self.x * others.x + self.y * others.y
    }
}

impl Pos {
    pub fn new(x: f64, y: f64) -> Pos {
        Pos {x, y}
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.x * self.x + self.y * self.y)
    }

    pub fn arg(&self) -> f64 {
        f64::atan2(self.y, self.x)
    }

    pub fn rect_to_polar(&self) -> Pos {
        Pos {x: self.length(), y: self.arg()}
    }

    pub fn polar_to_rect(&self) -> Pos {
        Pos {x: self.x * f64::cos(self.y), y: self.x * f64::sin(self.y)}
    }

}
