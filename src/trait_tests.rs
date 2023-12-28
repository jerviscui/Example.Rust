// pub trait Associated {
//     type ReturnType;
//     fn parse(s: &str) -> Result<Self, &Self::ReturnType>
//     where
//         Self: std::marker::Sized;
// }

use std::fmt::Error;

pub trait Associated {
    type ReturnType;
    fn parse(s: &str) -> Result<Self::ReturnType, Error>
    where
        Self: std::marker::Sized;
}

impl Associated for i32 {
    type ReturnType = i64;
    fn parse(s: &str) -> Result<Self::ReturnType, Error> {
        Err(Error)
    }
}

#[test]
fn parse_test() {
    assert!(i32::parse("123").is_err());
}

use std::ops::Add;

#[derive(Debug)]
pub struct Complex {
    pub real: f64,
    pub imagine: f64,
}

impl Add for Complex {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            real: self.real + rhs.real,
            imagine: self.imagine + rhs.imagine,
        }
    }
}

impl Add<f64> for Complex {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        Complex {
            real: self.real + rhs,
            imagine: self.imagine,
        }
    }
}

impl Add for &Complex {
    type Output = Complex;

    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            real: self.real + rhs.real,
            imagine: self.imagine + rhs.imagine,
        }
    }
}
