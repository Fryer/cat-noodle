#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32
}


impl Vec2 {
    pub fn length(self) -> f32 {
        self.x.hypot(self.y)
    }


    pub fn normalized(self) -> Self {
        self / self.length()
    }


    pub fn rotated(self, direction: Self) -> Self {
        Self {
            x: self.x * direction.x - self.y * direction.y,
            y: self.x * direction.y + self.y * direction.x
        }
    }
}


impl std::ops::Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}


impl std::ops::Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}


impl std::ops::Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self { x: self.x * rhs, y: self.y * rhs }
    }
}


impl std::ops::Div<f32> for Vec2 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Self { x: self.x / rhs, y: self.y / rhs }
    }
}
