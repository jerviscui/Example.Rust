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

pub struct GenericComplex<T> {
    pub real: T,
    pub imagine: T,
}

impl<T> Add for GenericComplex<T>
where
    T: Add<Output = T>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        GenericComplex {
            real: self.real + rhs.real,
            imagine: self.imagine + rhs.imagine,
        }
    }
}

impl<T, RT> Add<RT> for GenericComplex<T>
where
    T: Add<RT, Output = T>,
    RT: Copy,
{
    type Output = Self;
    fn add(self, rhs: RT) -> Self::Output {
        GenericComplex {
            real: self.real + rhs,
            imagine: self.imagine,
        }
    }
}

#[test]
fn impl_trait_generic_struct() {
    let c = GenericComplex::<f64> {
        real: 1.0,
        imagine: 2.0,
    };

    assert_eq!(c.add(1.0).real, 2.0);

    let a = GenericComplex::<f64> {
        real: 1.0,
        imagine: 2.0,
    };
    let b = GenericComplex::<f64> {
        real: 2.0,
        imagine: 2.2,
    };

    assert_eq!(a.add(b).imagine, 4.2);

    let d = GenericComplex::<i32> {
        real: 1,
        imagine: 2,
    };

    assert_eq!((d + 1).real, 2);
}

// impl<T> Add<T> for GenericComplex<T>
// where
//     T: Add<Output = T>,
// {
//     type Output = Self;
//     fn add(self, rhs: T) -> Self::Output {
//         GenericComplex {
//             real: self.real + rhs,
//             imagine: self.imagine,
//         }
//     }
// }

// #[test]
// fn impl_trait_generic_struct_add_f64() {
//     let c = GenericComplex::<f64> {
//         real: 1.0,
//         imagine: 2.0,
//     };

//     assert_eq!(c.add(1.0).real, 2.0);

//     let d = GenericComplex::<i32> {
//         real: 1,
//         imagine: 2,
//     };

//     // error[E0308]: mismatched types
//     assert_eq!(d.add(1).real, 2);
// }
