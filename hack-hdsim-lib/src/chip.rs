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
    pub fn new(capacity: usize) -> Self {
        Self {
            pinlines: Vec::<Pinline>::with_capacity(capacity),
        }
    }
    pub fn push(&mut self, pinline: Pinline) {
        self.pinlines.push(pinline);
    }
    pub fn set(&mut self, name: &str, vals: Vec<Pin>) {
        for pinline in &mut self.pinlines {
            if pinline.name.as_str() == name {
                pinline.set(vals);
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
pub struct Chip {
    name: String,
    input: Interface,
    output: Interface,
}

impl Chip {
    pub fn new(name: &str, input: Interface, output: Interface) -> Self {
        Self {
            name: name.to_string(),
            input,
            output,
        }
    }
    pub fn input(&self) -> &Interface {
        &self.input
    }
    pub fn set_input(&mut self, name: &str, vals: Vec<Pin>) {
        self.input.set(name, vals);
    }
    pub fn output(&self) -> &Interface {
        &self.output
    }
    fn _call() {
        // this needs to be able to call other chips somehow
    }
}

#[derive(Debug)]
pub struct Nand {
    name: String,
    a: Pin,
    b: Pin,
    out: Pin,
}

impl Nand {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            a: true,
            b: true,
            out: false,
        }
    }
    pub fn process_input(&mut self) {
        self.out = !(self.a && self.b);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn chip_new() {
        let mut and_input = Interface::new(2);
        and_input.push(Pinline::new("a", 1));
        and_input.push(Pinline::new("b", 1));
        let mut and_output = Interface::new(1);
        and_output.push(Pinline::new("out", 1));
        let mut and_chip = Chip::new("And", and_input, and_output);
        // Need to connect to other chips somehow
        println!("{:#?}", and_chip.input);
        println!("{:#?}", and_chip.output);
        and_chip.set_input("a", vec![true]);
        and_chip.set_input("b", vec![true]);
        // assert!(false);
    }
}
