pub type Pin = bool;

pub type Pinline = Vec<Pin>;

pub type Interface = std::collections::HashMap<String, Pinline>;

pub fn add_line(
    interface: &mut Interface,
    line_name: &str,
    line_capacity: usize,
) {
    interface.insert(
        String::from(line_name),
        Pinline::with_capacity(line_capacity),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_line_test() {
        let mut input = Interface::new();
        add_line(&mut input, "a", 1);
        add_line(&mut input, "b", 16);
        assert!(input.contains_key("a"));
        assert!(input.contains_key("b"));
        assert_eq!(input["a"].capacity(), 1);
        assert_eq!(input["b"].capacity(), 16);
    }
}
