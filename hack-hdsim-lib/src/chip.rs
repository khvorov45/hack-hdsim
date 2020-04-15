pub type Pin = bool;

#[derive(Debug)]
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
    pub fn set(&mut self, vals: Vec<Pin>) {
        self.pins = vals;
    }
}

#[derive(Debug)]
pub struct Interface {
    pinlines: Vec<Pinline>,
}

impl Interface {
    pub fn new(lines: &[(&str, usize)]) -> Self {
        let mut pinlines = Vec::<Pinline>::with_capacity(lines.len());
        for line in lines {
            pinlines.push(Pinline::new(line.0, line.1))
        }
        Self { pinlines }
    }
    pub fn set(&mut self, name: &str, vals: Vec<Pin>) {
        for pinline in &mut self.pinlines {
            if pinline.name.as_str() == name {
                pinline.pins = vals;
                return;
            }
        }
        panic!("No such name {}", name);
    }
    pub fn get(&self, name: &str) -> &Vec<Pin> {
        for pinline in &self.pinlines {
            if pinline.name.as_str() == name {
                return &pinline.pins;
            }
        }
        panic!("No such name {}", name);
    }
}

#[derive(Debug)]
pub struct Chip<F: Fn(Interface, Interface) -> ()> {
    input: Interface,
    output: Interface,
    process: F,
}

impl<F: Fn(Interface, Interface) -> ()> Chip<F> {
    pub fn new(input: Interface, output: Interface, process: F) -> Self {
        Self {
            input,
            output,
            process,
        }
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
        let and_chip = Chip::new(
            Interface::new(&[("a", 1), ("b", 1)]),
            Interface::new(&[("c", 1)]),
            |input, mut output| {
                output.set("c", vec![input.get("a")[1] && input.get("b")[1]]);
            },
        );
        println!("{:#?}", and_chip.input);
        println!("{:#?}", and_chip.output);
        // assert!(false);
    }
}
