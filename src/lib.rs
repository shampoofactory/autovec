use std::ops;

/// Pass by val, return by ref: 4 * f32, addition.
pub fn case_4_f32_add(a: [f32; 4], b: [f32; 4]) -> [f32; 4] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2], a[3] + b[3]]
}

/// Pass by val, return by ref: 4 * f32, subtraction.
pub fn case_4_f32_sub(a: [f32; 4], b: [f32; 4]) -> [f32; 4] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2], a[3] - b[3]]
}

/// Pass by val, return by ref: 4 * f32, multiplication.
pub fn case_4_f32_mul(a: [f32; 4], b: [f32; 4]) -> [f32; 4] {
    [a[0] * b[0], a[1] * b[1], a[2] * b[2], a[3] * b[3]]
}

/// Pass by val, return by ref: 4 * f32, division.
pub fn case_4_f32_div(a: [f32; 4], b: [f32; 4]) -> [f32; 4] {
    [a[0] / b[0], a[1] / b[1], a[2] / b[2], a[3] / b[3]]
}

/// Pass by val, return by ref: 4 * i32, addition.
pub fn case_4_i32_add(a: [i32; 4], b: [i32; 4]) -> [i32; 4] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2], a[3] + b[3]]
}

/// Pass by val, return by ref: 4 * i32, subtraction.
pub fn case_4_i32_sub(a: [i32; 4], b: [i32; 4]) -> [i32; 4] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2], a[3] - b[3]]
}

/// Pass by val, return by ref: 4 * i32, multiplication.
pub fn case_4_i32_mul(a: [i32; 4], b: [i32; 4]) -> [i32; 4] {
    [a[0] * b[0], a[1] * b[1], a[2] * b[2], a[3] * b[3]]
}

// Ignored, the zero division checks overwhelm output.
// /// Pass by val, return by ref: 4 * i32, division.
// pub fn case_4_i32_div(a: [i32; 4], b: [i32; 4]) -> [i32; 4] {
//     [a[0] / b[0], a[1] / b[1], a[2] / b[2], a[3] / b[3]]
// }

/// Pass by val, return by ref: 4 * i32, bitwise and.
pub fn case_4_i32_and(a: [i32; 4], b: [i32; 4]) -> [i32; 4] {
    [a[0] & b[0], a[1] & b[1], a[2] & b[2], a[3] & b[3]]
}

/// Pass by val, return by ref: 4 * i32, bitwise or.
pub fn case_4_i32_xor(a: [i32; 4], b: [i32; 4]) -> [i32; 4] {
    [a[0] | b[0], a[1] | b[1], a[2] | b[2], a[3] | b[3]]
}

/// Pass by val, return by ref: 4 * i32, bitwise xor.
pub fn case_4_i32_or(a: [i32; 4], b: [i32; 4]) -> [i32; 4] {
    [a[0] ^ b[0], a[1] ^ b[1], a[2] ^ b[2], a[3] ^ b[3]]
}

/// Pass by val, return by ref: 4 * i32, bitwise shift left.
pub fn case_4_i32_shl(a: [i32; 4], b: [i32; 4]) -> [i32; 4] {
    [a[0] << b[0], a[1] << b[1], a[2] << b[2], a[3] << b[3]]
}

/// Pass by val, return by ref: 4 * i32, bitwise shift right.
pub fn case_4_i32_shr(a: [i32; 4], b: [i32; 4]) -> [i32; 4] {
    [a[0] >> b[0], a[1] >> b[1], a[2] >> b[2], a[3] >> b[3]]
}

/// Pass by ref, return by val: 4 * f32, addition.
pub fn case_4_f32_add_pass_ref(a: &[f32; 4], b: &[f32; 4]) -> [f32; 4] {
    let mut c = [0.0; 4];
    for i in 0..4 {
        c[i] = a[i] + b[i];
    }
    c
}

/// Pass by val, return by ref: 4 * f32, addition.
pub fn case_4_f32_add_ret_ref(a: &[f32; 4], b: &[f32; 4], c: &mut [f32; 4]) {
    for i in 0..4 {
        c[i] = a[i] + b[i];
    }
}

/// Pass by val, return by val: 4 * i32, addition and subtraction.
/// Not essential, we can work around this.
pub fn case_4_i32_add_sub(a: [i32; 4], b: [i32; 4], c: [i32; 4]) -> [i32; 4] {
    [a[0] + b[0] - c[0], a[1] + b[1] - c[1], b[2] + a[2] - c[2], a[3] + b[3] - c[3]]
}

/// Pass by val, return by val: 4 * i32, addition and subtraction, commutative.
/// Not essential, we can work around this.
pub fn case_4_i32_add_sub_com(a: [i32; 4], b: [i32; 4], c: [i32; 4]) -> [i32; 4] {
    [a[0] + b[0] - c[0], -c[1] + a[1] + b[1], b[2] - c[2] + a[2], a[3] - c[3] + b[3]]
}

/// Pass by ref, return by val: 4 * f32, addition loop.
pub fn case_4_f32_add_loop(a: [f32; 4], b: [f32; 4]) -> [f32; 4] {
    let mut c = [0.0; 4];
    for i in 0..4 {
        c[i] = a[i] + b[i];
    }
    c
}

/// Pass by ref, return by ref: 4 * f32, addition loop.
pub fn case_4_f32_add_ret_ref_loop(a: [f32; 4], b: [f32; 4], c: &mut [f32; 4]) {
    for i in 0..4 {
        c[i] = a[i] + b[i];
    }
}

/// Pass by val, return by val: Vec4, addition and subtraction.
pub fn case_vec4_add_sub(a: Vec4, b: Vec4, c: Vec4) -> Vec4 {
    a + b - c
}

/// Pass by val, return by val: Vec4, addition cancelling c.
/// Not essential, we can attempt to work around this.
pub fn case_vec4_add_sub_to_cancel_c(a: Vec4, b: Vec4, c: Vec4) -> Vec4 {
    a + b - c + c
}

/// Pass by val, return by val: Vec4, addition simplify with multiply 16.
/// Not essential, we can attempt to  work around this.
pub fn case_vec4_add_to_mul_16(a: Vec4, b: Vec4) -> Vec4 {
    a + b + b + b + b + b + b + b + b + b + b + b + b + b + b + b + b
}

/// Pass by val, return by val: Vec4, addition simplify with multiply 15.
/// Not essential, we can attempt to  work around this.
pub fn case_vec4_add_to_mul_15(a: Vec4, b: Vec4) -> Vec4 {
    a + b + b + b + b + b + b + b + b + b + b + b + b + b + b + b
}

/// Vec 4 * f32
#[derive(Copy, Clone)]
pub struct Vec4 {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl ops::AddAssign for Vec4 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
    }
}

impl ops::Add for Vec4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z, w: self.w + rhs.w }
    }
}

impl ops::SubAssign for Vec4 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self.w -= rhs.w;
    }
}

impl ops::Sub for Vec4 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z, w: self.w - rhs.w }
    }
}

impl ops::DivAssign for Vec4 {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
        self.w /= rhs.w;
    }
}

impl ops::Div for Vec4 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self { x: self.x / rhs.x, y: self.y / rhs.y, z: self.z / rhs.z, w: self.w / rhs.w }
    }
}

impl ops::MulAssign for Vec4 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
        self.w *= rhs.w;
    }
}

impl ops::Mul for Vec4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self { x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z, w: self.w * rhs.w }
    }
}

/// Vec 3 * f32
#[derive(Copy, Clone)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}

impl ops::DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

impl ops::Div for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self { x: self.x / rhs.x, y: self.y / rhs.y, z: self.z / rhs.z }
    }
}

impl ops::MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self { x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z }
    }
}
