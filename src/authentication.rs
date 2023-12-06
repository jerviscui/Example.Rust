#[derive(Debug)]
pub struct User {
    name: String,
    pwd: String,
}

impl User {
    pub fn new(name: String, pwd: String) -> Self {
        User { name: "".to_string(), pwd: "".to_string() }
    }

    pub fn get_name(&self) -> &str{
        self.name.as_str()
    }

    pub fn set_pwd(&mut self, pwd: String){
        self.pwd = pwd;
    }
}
