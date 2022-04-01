use test_proc_macros::add_tuple;

#[add_tuple]
#[derive(Debug)]
pub struct Table;

impl Table {
    pub fn round(&self) {
        println!("This is round table! {}", self.0);
    }
}

#[cfg(test)]
mod tests {
    use super::Table;

    #[test]
    fn test() {
        let t = Table("se".to_string());
        t.round();
    }
}
