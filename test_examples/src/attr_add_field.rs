use test_proc_macros::add_field;

#[add_field]
#[derive(Debug)]
pub struct Toy {
    pub a: String,
    pub b: i32,
    pub c: bool,
}

impl Toy {
    pub fn build(&self) {
        println!("build completed! {}", self.d);
    }
}

#[cfg(test)]
mod tests {
    use crate::attr_add_field::Toy;

    #[test]
    fn test() {
        let t = Toy {
            a: "a".to_string(),
            b: 1,
            c: false,
            d: "d".to_string(),
        };
        t.build();

        println!("{:?}", t);
    }
}
