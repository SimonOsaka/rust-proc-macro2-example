use test_proc_macros::my_attribute_struct;

#[my_attribute_struct]
#[derive(Debug)]
pub struct Car {
    pub no: String,
}

#[cfg(test)]
mod tests {
    use super::Car;

    #[test]
    fn test_attribute_struct() {
        let c = Car {
            no: "阿D 12345".to_string(),
        };
        c.drive();
    }
}
