pub type Pin = bool;

#[derive(Debug, PartialEq)]
pub struct Pinline {
    name: String,
    pins: Vec<Pin>,
}

impl Pinline {
    pub fn new(name: &str, capacity: usize) -> Self {
        Self {
            name: name.to_string(),
            pins: Vec::<Pin>::with_capacity(capacity),
        }
    }
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
    pub fn pins(&self) -> &Vec<Pin> {
        &self.pins
    }
}

pub type Interface = Vec<Pinline>;

#[derive(Debug)]
pub struct Chip {
    input: Interface,
    output: Interface,
}

impl Chip {
    pub fn new(input: Interface, output: Interface) -> Self {
        Self { input, output }
    }
    pub fn input(&self) -> &Interface {
        &self.input
    }
    pub fn output(&self) -> &Interface {
        &self.output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn chip_new() {
        let _chip = Chip::new(
            vec![Pinline::new("a", 1), Pinline::new("b", 1)],
            vec![Pinline::new("c", 1)],
        );
    }
}
