use test_proc_macros::MyDerive;

#[derive(MyDerive, Debug)]
pub struct Person {
    pub name: String,
}
impl Person {
    pub fn jump(&self) {
        println!("{} jump high", self.name);
    }
}

#[cfg(test)]
mod tests {
    use super::Person;

    #[test]
    fn test_derive() {
        let p = Person {
            name: "I".to_string(),
        };
        p.ok();
        p.jump();
    }
}
