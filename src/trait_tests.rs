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
        Err(Error::default())
    }
}

#[test]
fn parse_test() {
    assert!(i32::parse("123").is_err());
}
