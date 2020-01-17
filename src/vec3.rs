use std::ops::{
    Add, AddAssign,
    Neg,
    Sub, SubAssign,
    Div, DivAssign,
    Mul, MulAssign,
    Index
};
use rand::Rng;
use num_traits::Float;

type InternalVec3<T> = [T; 3];

#[derive(Copy,Clone)]
pub struct Vec3<T: Float + MulAssign> {
    pub e: InternalVec3<T>,
}

impl<T: Float + MulAssign> Vec3<T> {
    pub fn new(e: Option<InternalVec3<T>>) -> Vec3<T> {
        match e {
            Some(internalvec3) => Vec3 { e: internalvec3 },
            None => Vec3 { e: [T::zero() , T::zero(), T::zero()] },
        }
    }

    pub fn random_in_unit_sphere() -> Vec3<T> {
        let mut rng = rand::thread_rng();
        let one = T::one();
        let two = T::one() + T::one();
        let vec3_unit = Vec3::new(Some([one, one, one]));
    
        let result = loop {
            let iv = [
                T::from(rng.gen::<f32>()),
                T::from(rng.gen::<f32>()),
                T::from(rng.gen::<f32>()),
            ];
            if let None = iv[0] {
                continue;
            }
            if let None = iv[1] {
                continue;
            }
            if let None = iv[2] {
                continue;
            }

            let p = Vec3::new(Some([iv[0].unwrap(), iv[1].unwrap(), iv[2].unwrap()])) * two - vec3_unit;
            if p.squared_length() <= one {
                break p;
            }
        };
    
        result
    }

    pub fn get_x(&self) -> T {
        self.e[0]
    }

    pub fn get_y(&self) -> T {
        self.e[1]
    }

    pub fn get_z(&self) -> T {
        self.e[2]
    }

    pub fn get_r(&self) -> T {
        self.e[0]
    }

    pub fn get_g(&self) -> T {
        self.e[1]
    }

    pub fn get_b(&self) -> T {
        self.e[2]
    }

    pub fn make_unit_vector(&mut self) {
        let sum: T = self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2];
        let k: T = T::one() / sum.sqrt();

        self.e[0] *= k;
        self.e[1] *= k;
        self.e[2] *= k;
    }

    pub fn dot(a: &Vec3<T>, b: &Vec3<T>) -> T {
        a.e[0]*b.e[0] + a.e[1]*b.e[1] + a.e[2]*b.e[2]
    }

    pub fn cross(a: &Vec3<T>, b: &Vec3<T>) -> Vec3<T> {
        Vec3 {
            e: [
                a.e[1]*b.e[2] - a.e[2]*b.e[1],
                -(a.e[0]*b.e[2] - a.e[2]*b.e[0]),
                a.e[0]*b.e[1] - a.e[1]*b.e[0]
            ]
        }
    }

    pub fn length(&self) -> T {
        (self.e[0]*self.e[0] + self.e[1]*self.e[1] + self.e[2]*self.e[2]).sqrt()
    }

    pub fn squared_length(&self) -> T {
        (self.e[0]*self.e[0] + self.e[1]*self.e[1] + self.e[2]*self.e[2])
    }

    pub fn unit_vector(v: Vec3<T>) -> Vec3<T> {
        let k = T::one() / v.length();
        v * k
    }

    pub fn gamma2_correct(&mut self) {
        self.e[0] = self.e[0].sqrt();
        self.e[1] = self.e[1].sqrt();
        self.e[2] = self.e[2].sqrt();
    }
}

impl<T: Float + MulAssign> Add for Vec3<T> {
    type Output = Vec3<T>;

    fn add(self, other: Vec3<T>) -> Vec3<T> {
        Vec3 {
            e: [self.e[0] + other.e[0], self.e[1] + other.e[1], self.e[2] + other.e[2]],
        }
    }
}

impl<T: Float + MulAssign> Sub for Vec3<T> {
    type Output = Vec3<T>;

    fn sub(self, other: Vec3<T>) -> Vec3<T> {
        Vec3 {
            e: [self.e[0] - other.e[0], self.e[1] - other.e[1], self.e[2] - other.e[2]],
        }
    }
}

impl<T: Float + MulAssign> Mul for Vec3<T> {
    type Output = Vec3<T>;

    fn mul(self, other: Vec3<T>) -> Vec3<T> {
        Vec3 {
            e: [self.e[0] * other.e[0], self.e[1] * other.e[1], self.e[2] * other.e[2]],
        }
    }
}

impl<T: Float + MulAssign> Div for Vec3<T> {
    type Output = Vec3<T>;

    fn div(self, other: Vec3<T>) -> Vec3<T> {
        Vec3 {
            e: [self.e[0] / other.e[0], self.e[1] / other.e[1], self.e[2] / other.e[2]],
        }
    }
}

impl<T: Float + MulAssign> Mul<T> for Vec3<T> {
    type Output = Vec3<T>;

    fn mul(self, other: T) -> Vec3<T> {
        Vec3 {
            e: [self.e[0] * other, self.e[1] * other, self.e[2] * other],
        }
    }
}

impl<T: Float + MulAssign> Div<T> for Vec3<T> {
    type Output = Vec3<T>;

    fn div(self, other: T) -> Vec3<T> {
        Vec3 {
            e: [self.e[0] / other, self.e[1] / other, self.e[2] / other],
        }
    }
}

impl<T: Float + MulAssign> AddAssign for Vec3<T> {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            e: [self.e[0] + other.e[0], self.e[1] + other.e[1], self.e[2] + other.e[2]],
        }
    }
}

impl<T: Float + MulAssign> SubAssign for Vec3<T> {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            e: [self.e[0] - other.e[0], self.e[1] - other.e[1], self.e[2] - other.e[2]],
        }
    }
}

impl<T: Float + MulAssign> MulAssign for Vec3<T> {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            e: [self.e[0] * other.e[0], self.e[1] * other.e[1], self.e[2] * other.e[2]],
        }
    }
}

impl<T: Float + MulAssign> DivAssign for Vec3<T> {
    fn div_assign(&mut self, other: Self) {
        *self = Self {
            e: [self.e[0] / other.e[0], self.e[1] / other.e[1], self.e[2] / other.e[2]],
        }
    }
}

impl<T: Float + MulAssign> MulAssign<T> for Vec3<T> {
    fn mul_assign(&mut self, other: T) {
        *self = Self {
            e: [self.e[0] * other, self.e[1] * other, self.e[2] * other],
        }
    }
}

impl<T: Float + MulAssign> DivAssign<T> for Vec3<T> {
    fn div_assign(&mut self, other: T) {
        *self = Self {
            e: [self.e[0] / other, self.e[1] / other, self.e[2] / other],
        }
    }
}

impl<T: Float + MulAssign> Neg for Vec3<T> {
    type Output = Vec3<T>;

    fn neg(self) -> Vec3<T> {
        Vec3 { e: [-self.e[0], -self.e[1], -self.e[2]] }
    }
}

impl<T: Float + MulAssign> Index<usize> for Vec3<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        &self.e[index]
    }
}
