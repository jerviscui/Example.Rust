use std::ops::Deref;

pub struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// error[E0119]: conflicting implementations of trait `Deref` for type `MyBox<MyBox<_>>`
// impl<T> Deref for MyBox<MyBox<T>> {
//     type Target = T;
//
//     fn deref(&self) -> &Self::Target {
//         &self.0.0
//     }
// }

impl<T> MyBox<T> {
    pub fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
