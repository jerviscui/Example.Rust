pub trait Draw {
    fn draw(&self);
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Button todo")
    }
}

pub struct Text {
    pub content: String,
}

impl Draw for Text {
    fn draw(&self) {
        println!("Text todo")
    }
}

pub struct ScreenGeneric<T: Draw> {
    pub components: Vec<T>,
}

impl<T> ScreenGeneric<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }

    fn returns_summarizable() -> Box<dyn Draw> {
        if true {
            Box::new(Button {
                width: 1,
                height: 2,
                label: "a button".to_string(),
            })
        } else {
            Box::new(Text { content: todo!() })
        }
    }
}

#[test]
fn generic_test() {
    // error[E0282]: type annotations needed for `trait_object_tests::ScreenGeneric<T>`
    // let screen = ScreenGeneric {
    //     components: todo!(),
    // };

    let screen = ScreenGeneric::<Button> {
        components: vec![Button {
            width: 1,
            height: 2,
            label: "a button".to_string(),
        }],
    };

    let screen = ScreenGeneric::<Text> {
        components: vec![Text {
            content: "a text".to_string(),
        }],
    };

    screen.run();
}

pub struct Screen {
    // error[E0782]: trait objects must include the `dyn` keyword
    // pub components: Vec<Draw>,
    // error[E0277]: the size for values of type `(dyn trait_object_tests::Draw + 'static)` cannot be known at compilation time
    // pub components: Vec<dyn Draw>,
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

#[test]
fn test() {
    let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 1,
                height: 2,
                label: "a button".to_string(),
            }),
            Box::new(Text {
                content: "a text".to_string(),
            }),
        ],
    };

    screen.run();
}
