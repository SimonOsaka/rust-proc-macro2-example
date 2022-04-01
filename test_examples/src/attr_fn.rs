use test_proc_macros::my_attribute_fn;

#[derive(Debug)]
pub struct Animal {
    pub t1: String,
}
impl Animal {
    #[my_attribute_fn]
    fn roll(&self) -> String {
        println!("roll and roll");
        "monkey".to_string()
    }
}
#[cfg(test)]
mod tests {
    use super::Animal;

    #[test]
    fn test_attribute_fn() {
        let r = || {
            return Animal {
                t1: "dog".to_string(),
            };
        };
        r().roll();
    }
}
