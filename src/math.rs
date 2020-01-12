#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32
}


pub fn wrap_angle(a: f32) -> f32 {
    (a + std::f32::consts::PI).rem_euclid(std::f32::consts::PI * 2.0) - std::f32::consts::PI
}


pub fn vec2(x: f32, y: f32) -> Vec2 {
    Vec2 { x, y }
}


impl Vec2 {
    pub fn from_angle(angle: f32) -> Vec2 {
        vec2(angle.cos(), angle.sin())
    }


    pub fn to_angle(self) -> f32 {
        self.y.atan2(self.x)
    }


    pub fn length(self) -> f32 {
        self.x.hypot(self.y)
    }


    pub fn length_squared(self) -> f32 {
        self.x * self.x + self.y * self.y
    }


    pub fn dot(self, other: Vec2) -> f32 {
        self.x * other.x + self.y * other.y
    }


    pub fn normalized(self) -> Vec2 {
        self / self.length()
    }


    pub fn rotated(self, direction: Vec2) -> Vec2 {
        vec2(
            self.x * direction.x - self.y * direction.y,
            self.x * direction.y + self.y * direction.x
        )
    }


    pub fn unrotated(self, direction: Vec2) -> Vec2 {
        vec2(
            self.x * direction.x + self.y * direction.y,
            self.y * direction.x - self.x * direction.y
        )
    }
}


impl std::ops::Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Vec2 {
        vec2(-self.x, -self.y)
    }
}


impl std::ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Vec2 {
        vec2(self.x + rhs.x, self.y + rhs.y)
    }
}


impl std::ops::Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Vec2 {
        vec2(self.x - rhs.x, self.y - rhs.y)
    }
}


impl std::ops::Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f32) -> Vec2 {
        vec2(self.x * rhs, self.y * rhs)
    }
}


impl std::ops::Mul<Vec2> for f32 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Vec2 {
        rhs * self
    }
}


impl std::ops::Div<f32> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: f32) -> Vec2 {
        vec2(self.x / rhs, self.y / rhs)
    }
}


impl std::ops::AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        *self = *self + rhs
    }
}


impl std::ops::SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Vec2) {
        *self = *self - rhs
    }
}


impl std::ops::MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs
    }
}


impl std::ops::DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs
    }
}
