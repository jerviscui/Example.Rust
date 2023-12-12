use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::io::{Error, Read};
use std::ops::Deref;
use std::path::PathBuf;
use std::vec;
use regex::Regex;

mod marco;
mod test;

fn main() {
    let a = ();
    let b = (1, "2", 3.0);

    println!("{:?}", a);
    println!("{:?}", b);

    // 中文！12345=》-》==》
    let student = Student {
        // name: String::from("aaa"),
        name: "aaa".to_string(),
        level: 1,
        remote: true,
    };

    println!("{} {} {}", student.name, student.level, student.remote);

    // let student2 = Student2 {
    //     name: "".parse().unwrap(),
    //     level: 2,
    //     remote: false,
    // };
    //
    // println!("{} {} {}", student2.name, student2.level, student2.remote);

    let student3 = Student3 {
        name: "student3".to_string(),
        level: 0,
        remote: false,
    };

    // ((((([[{{{{{{}}}}}}]])))))
    // ((((((((((((()))))))))))))
    // [[[[[[[[[[[[[]]]]]]]]]]]]]
    println!("{} {} {}", student3.name, student3.level, student3.remote);

    let s = Student3 {
        name: "student3".to_string(),
        level: 0,
        remote: false,
    };

    println!("{} {} {}", s.name, s.level, s.remote);

    let red = Color::Red;
    let white = Color::RgbColor(0, 0, 0);
    let color = Color::CmykColor {
        cyan: 0,
        magenta: 0,
        yellow: 0,
        black: 0,
    };

    // Use the {:#?} syntax to display the enum structure and data in a readable form
    println!(
        "\nColor enum structure: \n\n {:#?} \n\n {:#?} \n\n {:#?}",
        red, white, color
    );

    let mut orders = HashMap::new();

    let car = car_factory("red".to_string(), Transmission::Manual, false, 20);
    println!("Car information: \n\n {:#?}", car);
    println!("Car information: \n\n {:?}", car);
    println!(
        "Car order 1: {:?}, Hard top = {}, {:?}, {}, {} miles",
        car.age.0, car.convertible, car.transmission, car.color, car.age.1
    );

    let car2 = car_factory("silver".to_string(), Transmission::Automatic, true, 10);

    orders.insert(1, car);
    orders.insert(2, car2);

    println!("order:{} car: {:?}", 1, orders.get(&1));
    println!("order:{} car: {:?}", 2, orders.get(&2).unwrap());

    // println!("order:{} car: {:?}", 2, orders.get(&3).unwrap());
    // Option::None.unwrap();
    // Some()

    let some = Option::Some(1);
    if let Some(i) = some {
        println!("is some: {}", i);
    }
    let s = unsafe { some.unwrap_unchecked() };
    debug_assert!(some.is_some());
    match some {
        None => unreachable!(),
        Some(_) => {}
    }

    unsafe {
        // Option::<i32>::None.unwrap_unchecked();
    }

    let ss = some.expect("must not be none");
    some.unwrap_or(0);

    for i in 3..5 {
        orders.insert(i, Car::default());
    }

    println!("{:?}", orders);

    let mut arr = [1, 2, 3, 4, 5];
    // arr[7]; // this operation will panic at runtime, index out of bounds: the length is 5 but the index is 7
    println!("{:?}", arr);

    arr = [0; 5];
    println!("{:?}", arr);

    let mut vec1 = vec![1, 1, 2];
    println!("{:?}", vec1);

    vec1.insert(0, 3);
    vec1.insert(3, 4);
    println!("{:?}", vec1);

    vec1 = vec![0; 2];
    println!("{:?}", vec1);

    vec1.push(11);
    println!("{:?}", vec1);

    let option = vec1.pop();
    println!("{:?}", option);

    // let x = vec1[3];
    let mut vec2 = Vec::with_capacity(10);
    println!("{:?}", vec2.capacity());
    // vec2[0] = 1;
    vec2.push(1);
    println!("{:?}", vec2.capacity());

    let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];
    let x = fruits.get(1);
    for &index in [0, 2, 99].iter() {
        match fruits.get(index) {
            Some(&"coconut") => println!("Coconuts are awesome!!!"),
            Some(fruit_name) => println!("It's a delicious {}!", fruit_name),
            None => println!("There is no fruit! :("),
        }
    }

    let person = Person {
        first: "John".to_string(),
        middle: Some("Quincy".to_string()),
        last: "Adams".to_string(),
    };

    let name = get_name(&person);
    println!("{:?}", name);
    println!("{}", name.unwrap());

    let content = read_file_content(PathBuf::from(
        "c:\\Users\\J\\Desktop\\test\\新建文本文档.txt",
    ));
    println!("{:?}", content);

    let content = read_file_content(PathBuf::from("xxx"));
    println!("{:?}", content);

    let mut ferris = String::new();
    {
        let mascot = String::from("say hello");
        ferris = mascot;
        // println!("{}", mascot) // We'll try to use mascot after we've moved ownership of the string data from mascot to ferris.
    }
    println!("{}", ferris);

    let mut s = String::from("Hello, world!");
    // process(s); // Ownership of the string in `s` moved into `process`
    // process(s); // Error! ownership already moved.

    s = process2(s); // Ownership moved out from `process`
    process2(s);

    let i = 1;
    let j = i;
    println!("{}", i);

    let mut greeting = String::from("hello");
    let greeting_reference = &mut greeting;
    *greeting_reference = String::from("world");
    println!("Greeting: {}", greeting);

    let mut a = 1;
    let b = &a;
    println!("{}", b);
    a = 2;
    // println!("{}", b);

    let mut c = 1;
    let d = &c;
    println!("{}", c);
    println!("{}", d);

    // let x;
    // {
    //     let y = 42;
    //     x = &y; // We store a reference to `y` in `x` but `y` is about to be dropped.
    // }
    // println!("x: {}", x); // `x` refers to `y` but `y has been dropped!

    let s: String = "hello world".to_owned();
    let ss = "hello world".to_string();
    let sh = &ss;
    let sd = "hello world";

    println!("{:p}", &s);
    println!("{:p}", &ss);
    println!("{:p}", sh);
    println!("{:p}", sd);

    let mut s1: &str = s.as_str();
    let hello = strtok(&mut s1, ' ');
    println!("hello is: {}, s1: {}, s: {}", hello, s1, s);

    println!("{}", test());

    let string1 = String::from("ab");
    let string2 = "xyz"; // 'static

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // let string1 = String::from("long string is long"); // on heap
    // let result;
    // {
    //     let string2 = String::from("xyz"); // on heap
    //     let p = string2.as_str();
    //     result = longest(string1.as_str(), p);
    // }
    // println!("The longest string is {}", result);

    let string1 = String::from("long string is long"); // on heap
    let result;
    {
        let string2 = String::from("xyz"); // on heap
        let p = string2.as_str();
        result = longest(string1.as_str(), p);
        println!("The longest string is {}", result);
    }

    let string1: String = String::from("ab");
    let string2: &str = "xyz"; // 'static

    let first = get_first(&string1, string2);
    println!("first {:p}, &first {:p}", first, &first);

    // let str1 = String::from("long string is long"); // on heap
    // let result;
    // {
    //     let str2 = String::from("xyz"); // on heap
    //     result = longest3(string1.as_str(), str2.as_str());
    //     // println!("The longest string is {}", result);
    // }
    // println!("The longest string is {}", result);

    // let i;
    // {
    //     let novel = String::from("Call me Ishmael. Some years ago...");
    //     i = ImportantExcerpt { part: &novel };
    // }
    // println!("{:?}", i);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let mut excerpt = ImportantExcerpt { part: &novel };

    excerpt.set_part("hello");
    println!("{:?}", excerpt);

    {
        let mut out: &str = "";

        let s = String::from("Call me Ishmael. Some years ago...");
        let p = s.as_str();

        let ex = ImportantExcerpt { part: p };
        ex.from_part(&mut out);
        ex.from_part2(&mut out);

        println!("{}", out);
    }

    // let mut out: &str = "";
    // {
    //     let s = String::from("Call me Ishmael. Some years ago...");
    //     let p = s.as_str();
    //
    //     let ex = ImportantExcerpt { part: p };
    //     ex.from_part(&mut out); // out 的生命周期比 s 大
    //     ex.from_part2(&mut out); // out 的生命周期比 s 大
    // }
    // println!("{}", out);

    // generic type
    let p_i32: Point<i32> = Point { x: 1, y: 2 };
    let p_bool: Point<bool> = Point { x: true, y: false };
    let p_f64: Point<f64> = Point { x: 1.1, y: 2.2 };
    let p_unit: Point<()> = Point { x: (), y: () };
    let point: Point<(i32, bool)> = Point {
        x: (1, true),
        y: (2, false),
    };
    let point1: Point<i32> = Point::new(1, 10);

    let circule = Circule { radius: 1.0 };

    println!("{}", get_area(&circule));
    println!("{}", get_area2(&circule));

    let a = [1, 2];

    let mut iter = a.iter();

    // A call to next() returns the next value...
    assert_eq!(Some(&1), iter.next());
    assert_eq!(Some(&2), iter.next());

    // ... and then None once it's over.
    assert_eq!(None, iter.next());

    // More calls may or may not return `None`. Here, they always will.
    assert_eq!(None, iter.next());
    debug_assert!(iter.next().is_none());
    // assert_ne!(None, iter.next());

    let v: Vec<i32> = Vec::with_capacity(5);
    dbg!(&v);
    dbg!(&v.capacity());

    let data = vec![4, 1, 1, 2, 1, 3, 3, -2, -2, -2, 5, 5];
    // groups:     |->|---->|->|->|--->|----------->|--->|
    let group = Groups::new(data);
    let p = &group;
    assert_eq!(
        // group.collect::<Vec<Vec<_>>>(), // or this, collect auto into_iter()
        group.into_iter().collect::<Vec<Vec<_>>>(), // gourp moved
        vec![
            vec![4],
            vec![1, 1],
            vec![2],
            vec![1],
            vec![3, 3],
            vec![-2, -2, -2],
            vec![5, 5],
        ]
    );
    // dbg!(p.inner.len());

    // let u = authentication::User { name: "".to_string(), pwd: "".to_string() };
    let mut u = authentication::User::new("123".to_string(), "abc".to_string());
    u.set_pwd("ddd".to_string());
    println!("{:?}", u);

    let regex = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    println!("{}", regex.is_match("2020-01-01"));

    add(1, 2);

    eprintln!("u = {:?}", u);
}

/// Generally, the first line is a brief summary describing the function.
///
/// The next lines present detailed documentation.
/// Code blocks start with triple backticks. The code has an implicit `fn main()` inside and `extern crate <cratename>`,
/// which means you can just start writing code.
///
/// # Examples
///
/// ```
/// let result = basic_math::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod add_function_tests{
    // use crate::add;
    use super::*;

    #[test]
    fn add_works() {
        assert_eq!(add(1, 2), 3);
        assert_eq!(add(10, 12), 22);
        assert_eq!(add(5, -2), 3);
    }

    #[test]
    #[should_panic]
    fn add_fails() {
        assert_eq!(add(2, 2), 7);
    }
}

mod math_even;
mod authentication;

struct Groups<T> {
    inner: Vec<T>,
}

impl<T> Groups<T> {
    fn new(inner: Vec<T>) -> Self {
        Groups {
            inner
        }
    }
}

impl<T: PartialEq+Debug+Copy> Iterator for Groups<T> {
    type Item = Vec<T>;

    // fn next(&mut self) -> Option<Self::Item> {
    //     if  self.inner.is_empty() {
    //         return None;
    //     }
    //
    //     let inner = &self.inner;
    //
    //     let t = inner.get(0);
    //
    //     // let x1 = t.unwrap();
    //     // let x2 = (*x1).copy();
    //
    //     let x:T = (*(t.unwrap())).clone();
    //     dbg!(x);
    //
    //     inner.drain()
    //     let mut result = Vec::with_capacity(inner.len() / 2);
    //     result.push(x);
    //
    //     for i in 1..inner.len() {
    //         let item = inner.get(i);
    //         if item == t {
    //             dbg!("push");
    //             result.push((*(item.unwrap())).clone());
    //         } else {
    //             break;
    //         }
    //     }
    //
    //     Some(result)
    // }

    fn next(&mut self) -> Option<Self::Item> {
        // if the inner vector is empty, we are done
        if self.inner.is_empty() {
            return None;
        }

        // lets check the span of equal items
        let mut cursor = 1;
        let first = &self.inner[0];

        dbg!(first);

        for element in &self.inner[1..] {
            if element == first {
                cursor += 1;
            } else {
                break;
            }
        }

        // we use the `Vec::drain` to extract items up until the cursor
        let items = self.inner.drain(0..cursor).collect();

        // return the extracted items
        Some(items)
    }
}

fn get_area(area: &impl Area) -> f64 {
    area.area()
}

fn get_area2<T: Area>(area: &T) -> f64 {
    area.area()
}

struct Circule {
    radius: f64,
}

impl Area for Circule {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

trait Area {
    fn area(&self) -> f64;
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}

// enum E { A(f64), B(HashMap), C(Result, String>),}
// // 这是一个声明宏，它会打印各种数据结构本身的大小，在 Option 中的大小，以及在 Result 中的大小
// macro_rules! show_size { (header) => { println!( "{:<24} {:>4} {} {}", "Type", "T", "Option", "Result" ); println!("{}", "-".repeat(64)); }; ($t:ty) => { println!( "{:<24} {:4} {:8} {:12}", stringify!($t), size_of::<$t>(), size_of::>(), size_of::>(), ) };}
// fn aaa() { show_size!(header); show_size!(u8); show_size!(f64); show_size!(&u8); show_size!(Box); show_size!(&[u8]); show_size!(String); show_size!(Vec); show_size!(HashMap); show_size!(E);}

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn set_part(&mut self, announcement: &'a str) -> &str {
        self.part = announcement;

        self.part
    }

    fn from_part(&self, announcement: &mut &'a str) -> &str {
        *announcement = self.part;

        *announcement
    }

    fn from_part2<'b>(&self, announcement: &mut &'b str) -> &'b str
        where
            'a: 'b,
    {
        *announcement = self.part;

        *announcement
    }

    fn from_part3<'b>(&'a self, announcement: &mut &'b str) -> &'b str
        where
            'a: 'b,
    {
        longest(self.part, announcement);

        *announcement
    }
}

fn longest3<'a: 'b, 'b>(x: &'a str, y: &'b str) -> &'b str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn get_first<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    println!("x {:p}, y {:p}", x, y);
    println!("&x {:p}, &y {:p}", &x, &y);

    x
}

fn test3(p: &i32) -> &i32 {
    let x = &p;
    *x
}

fn test2() -> &'static str {
    let s: &'static str = "abc";

    s
}

fn test<'a>() -> &'a str {
    let s: &str = "abc";

    s
}

// fn test_int() -> &'static i32 {
//     let i = 1;
//
//     &i
// }

// fn test_int<'a>() -> &'a i32 {
//     let i = 1;
//
//     &i
// }

fn test_int() -> i32 {
    let i = 1;

    i
}

fn test_str() -> &'static str {
    let s = "abc";

    &s
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest2(x: &str) -> &str {
    x
}

pub fn strtok<'a>(s: &mut &'a str, delimiter: char) -> &'a str {
    if let Some(i) = s.find(delimiter) {
        let prefix = &s[..i];
        // 由于 delimiter 可以是 utf8，所以我们需要获得其 utf8 长度，
        // 直接使用 len 返回的是字节长度，会有问题
        let suffix = &s[(i + delimiter.len_utf8())..];
        *s = suffix;
        prefix
    } else {
        // 如果没找到，返回整个字符串，把原字符串指针 s 指向空串
        let prefix = *s;
        *s = "";
        prefix
    }
}

fn process(input: String) {}

fn process2(input: String) -> String {
    input
}

fn read_file_content(path: PathBuf) -> Result<String, Error> {
    let mut s = String::new();

    // let file = File::open(path);
    // if let Err(err) = file {
    //     return Err(err);
    // }

    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(err) => {
            return Err(err);
        }
    };

    match file.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(err) => Err(err),
    }
}

#[derive(Debug)]
struct NameErr;

fn get_name(person: &Person) -> Result<String, NameErr> {
    let mut name = String::new();
    name.push_str(person.first.as_str());

    name.push(' ');

    match &person.middle {
        Some(s) => {
            name.push_str(&s);
            name.push(' ');
        }
        _ => {}
    }

    name.push_str(&person.last);

    Result::Ok(name)
}

#[derive(Debug)]
struct Person {
    first: String,
    middle: Option<String>,
    last: String,
}

struct Student3 {
    name: String,
    level: u8,
    remote: bool,
}

struct Student2 {
    name: String,
    level: u8,
    remote: bool,
}

struct Student {
    name: String,
    level: u8,
    remote: bool,
}

impl Clone for Student {
    fn clone(&self) -> Self {
        todo!()
    }
}

#[derive(Debug)]
enum Color {
    Red,
    RgbColor(u8, u8, u8),
    // tuple
    CmykColor {
        cyan: u8,
        magenta: u8,
        yellow: u8,
        black: u8,
    }, // struct
}

fn car_factory(color: String, transmission: Transmission, convertible: bool, miles: u32) -> Car {
    let car = Car {
        color,
        transmission,
        convertible,
        age: car_quality(miles),
    };

    return car;
}

fn car_quality(miles: u32) -> (Age, u32) {
    if miles > 10 {
        return (Age::Used, miles);
    }

    return (Age::New, miles);
}

#[derive(Debug)]
struct Car {
    color: String,
    transmission: Transmission,
    convertible: bool,
    age: (Age, u32),
}

impl Car {
    fn default() -> Car {
        Car {
            color: "".to_string(),
            transmission: Transmission::Manual,
            convertible: false,
            age: (Age::New, 0),
        }
    }

    fn test(&self, s: &str) -> &String {
        &self.color
    }
}

#[derive(PartialEq, Debug)]
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

#[derive(Debug)]
enum Age {
    New,
    Used,
}
