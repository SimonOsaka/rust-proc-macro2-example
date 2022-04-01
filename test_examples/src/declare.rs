use test_proc_macros::my_macro;

fn do_test() {
    my_macro!(name = "Sam", call = "09");
}

#[cfg(test)]
mod tests {
    use crate::declare::hello;
    #[test]
    fn test() {
        hello();
    }
}
