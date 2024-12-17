pub struct User {
    name: String,
    pass: String,
}

impl User {
    pub fn new(username: &str, password: &str) -> User {
        User {
            name: username.to_string(),
            pass: password.to_string(),
        }
    }

    pub fn get_username(&self) -> &String {
        &self.name
    }

    pub fn set_password(&mut self, new_password: &str) {
        self.pass = new_password.to_string();
    }
    pub fn login(&self) {
        println!("User {} login", self.name);
    }
}
