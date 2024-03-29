use crate::solutions::SolutionCollection;

pub mod intcode;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod day_20;
mod day_21;
mod day_22;
mod day_23;
mod day_24;
mod day_25;

pub fn days() -> SolutionCollection {
    SolutionCollection::new()
        .add(1, day_01::part_one, day_01::part_two)
        .add(2, day_02::part_one, day_02::part_two)
        .add(3, day_03::part_one, day_03::part_two)
        .add(4, day_04::part_one, day_04::part_two)
        .add(5, day_05::part_one, day_05::part_two)
        .add(6, day_06::part_one, day_06::part_two)
        .add(7, day_07::part_one, day_07::part_two)
        .add(8, day_08::part_one, day_08::part_two)
        .add(9, day_09::part_one, day_09::part_two)
        .add(10, day_10::part_one, day_10::part_two)
        .add(11, day_11::part_one, day_11::part_two)
        .add(12, day_12::part_one, day_12::part_two)
        .add(13, day_13::part_one, day_13::part_two)
        .add(14, day_14::part_one, day_14::part_two)
        .add(15, day_15::part_one, day_15::part_two)
        .add(16, day_16::part_one, day_16::part_two)
        .add(17, day_17::part_one, day_17::part_two)
        .add(18, day_18::part_one, day_18::part_two)
        .add(19, day_19::part_one, day_19::part_two)
        .add(20, day_20::part_one, day_20::part_two)
        .add(21, day_21::part_one, day_21::part_two)
        .add(22, day_22::part_one, day_22::part_two)
        .add(23, day_23::part_one, day_23::part_two)
        .add(24, day_24::part_one, day_24::part_two)
        .add(25, day_25::part_one, day_25::part_two)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Point3<T> {
    x: T,
    y: T,
    z: T,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Point2<T> {
    x: T,
    y: T,
}

macro_rules! impl_point_op {
    ($name:ident, $func:ident, $op:tt) => {
        impl<T: std::ops::$name<Output = T>> std::ops::$name for Point3<T> {
            type Output = Self;
            fn $func(self, rhs: Self) -> Self::Output {
                Point3 {
                    x: self.x $op rhs.x,
                    y: self.y $op rhs.y,
                    z: self.z $op rhs.z,
                }
            }
        }

        impl<T: std::ops::$name<Output = T> + Copy> std::ops::$name<T> for Point3<T> {
            type Output = Self;
            fn $func(self, rhs: T) -> Self::Output {
                Point3 {
                    x: self.x $op rhs,
                    y: self.y $op rhs,
                    z: self.z $op rhs,
                }
            }
        }

        impl<T: std::ops::$name<Output = T>> std::ops::$name for Point2<T> {
            type Output = Self;
            fn $func(self, rhs: Self) -> Self::Output {
                Point2 {
                    x: self.x $op rhs.x,
                    y: self.y $op rhs.y,
                }
            }
        }

        impl<T: std::ops::$name<Output = T> + Copy> std::ops::$name<T> for Point2<T> {
            type Output = Self;
            fn $func(self, rhs: T) -> Self::Output {
                Point2 {
                    x: self.x $op rhs,
                    y: self.y $op rhs,
                }
            }
        }
    };
}

macro_rules! impl_point_op_assign {
    ($name:ident, $func:ident, $op:tt) => {
        impl<T: std::ops::$name> std::ops::$name for Point3<T> {
            fn $func(&mut self, rhs: Self) {
                self.x $op rhs.x;
                self.y $op rhs.y;
                self.z $op rhs.z;
            }
        }

        impl<T: std::ops::$name + Copy> std::ops::$name<T> for Point3<T> {
            fn $func(&mut self, rhs: T) {
                self.x $op rhs;
                self.y $op rhs;
                self.z $op rhs;
            }
        }

        impl<T: std::ops::$name> std::ops::$name for Point2<T> {
            fn $func(&mut self, rhs: Self) {
                self.x $op rhs.x;
                self.y $op rhs.y;
            }
        }

        impl<T: std::ops::$name + Copy> std::ops::$name<T> for Point2<T> {
            fn $func(&mut self, rhs: T) {
                self.x $op rhs;
                self.y $op rhs;
            }
        }
    };
}

impl_point_op!(Sub, sub, -);
impl_point_op!(Add, add, +);
impl_point_op!(Mul, mul, *);
impl_point_op!(Div, div, /);
impl_point_op!(Rem, rem, %);

impl_point_op_assign!(SubAssign, sub_assign, -=);
impl_point_op_assign!(AddAssign, add_assign, +=);
impl_point_op_assign!(MulAssign, mul_assign, *=);
impl_point_op_assign!(DivAssign, div_assign, /=);
impl_point_op_assign!(RemAssign, rem_assign, %=);

impl<T> Point3<T> {
    const fn new(x: T, y: T, z: T) -> Self {
        Point3 { x, y, z }
    }
}

impl<T> Point2<T> {
    const fn new(x: T, y: T) -> Self {
        Point2 { x, y }
    }
}

impl<T: std::hash::Hash> std::hash::Hash for Point3<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.z.hash(state);
    }
}

impl<T: std::hash::Hash> std::hash::Hash for Point2<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

trait Gcd<Rhs = Self> {
    type Output;
    fn gcd(self, rhs: Rhs) -> Self::Output;
}

macro_rules! impl_gcd {
    ($($type:ty )+) => {
        $(
            impl Gcd for $type {
                type Output = Self;
                fn gcd(self, rhs: Self) -> Self {
                    let mut n = self.min(rhs);
                    while n > 1 {
                        if self % n == 0 && rhs % n == 0 {
                            return n;
                        }
                        n -= 1;
                    }

                    return 1;
                }
            }
        )+
    };
}

impl_gcd!(i32 u32 i64 u64 i128 u128 isize usize);
