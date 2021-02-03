/// User-defined chip
#[derive(Debug)]
pub struct Chip {
    name: String,
    input: Interface,
    output: Interface,
    parts: Parts,
}

/// Input/output of a chip
#[derive(Debug)]
pub struct Interface {
    pinlines: Vec<Pinline>,
}

/// A set of pins with a name
#[derive(Debug)]
pub struct Pinline {
    name: String,
    pin_count: usize,
    pins: Vec<Pin>,
}

/// A set of chips connected to the pins of another chip
#[derive(Debug)]
pub struct Parts {
    chips: Vec<ChipConnected>,
}

/// A chip connected to another chip
#[derive(Debug)]
pub struct ChipConnected {
    name: String,
    connections: Vec<Connection>,
}

/// A pinline of one chip going to a pinline of another
#[derive(Debug)]
pub struct Connection {
    own: PinlineConnected,
    foreign: PinlineConnected,
}

/// Name and pin indices of a pinline that connect somewhere
#[derive(Debug)]
pub struct PinlineConnected {
    name: String,
    indices: Vec<u32>,
}

/// The pin
#[derive(Debug, Clone)]
pub enum Pin {
    Set(bool),
    Unset,
}

impl Chip {
    pub fn new(
        name: &str,
        input: Interface,
        output: Interface,
        parts: Parts,
    ) -> Self {
        Self {
            name: name.to_string(),
            input,
            output,
            parts,
        }
    }
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
    pub fn input(&self) -> &Interface {
        &self.input
    }
    pub fn output(&self) -> &Interface {
        &self.output
    }
    pub fn parts(&self) -> &Parts {
        &self.parts
    }
}

impl Pinline {
    pub fn new(name: &str, pin_count: usize) -> Self {
        Self {
            name: name.to_string(),
            pin_count,
            pins: vec![Pin::Unset; pin_count],
        }
    }
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
    pub fn pin_count(&self) -> usize {
        self.pin_count
    }
}

impl Interface {
    pub fn new(pinlines: Vec<Pinline>) -> Self {
        Self { pinlines }
    }
    pub fn pinlines(&self) -> &Vec<Pinline> {
        &self.pinlines
    }
    pub fn push(&mut self, pinline: Pinline) {
        self.pinlines.push(pinline);
    }
    pub fn get(&self, name: &str) -> Option<&Pinline> {
        self.pinlines.iter().find(|p| p.name == name)
    }
}

impl PinlineConnected {
    pub fn new(name: &str, indices: Vec<u32>) -> Self {
        Self {
            name: name.to_string(),
            indices,
        }
    }
}

impl Connection {
    pub fn new(own: PinlineConnected, foreign: PinlineConnected) -> Self {
        Self { own, foreign }
    }
}

impl ChipConnected {
    pub fn new(name: &str, connections: Vec<Connection>) -> Self {
        Self {
            name: name.to_string(),
            connections,
        }
    }
}

impl Parts {
    pub fn new(chips: Vec<ChipConnected>) -> Self {
        Self { chips }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn chip_new() {
        let a_input_pinline = Pinline::new("a", 1);
        let b_input_pinline = Pinline::new("b", 1);

        let and_input = Interface::new(vec![a_input_pinline, b_input_pinline]);

        let out_output_pinline = Pinline::new("out", 1);
        let and_output = Interface::new(vec![out_output_pinline]);

        let a_connection = PinlineConnected::new("a", vec![0]);
        let b_connection = PinlineConnected::new("b", vec![0]);
        let out_connection = PinlineConnected::new("out", vec![0]);

        let a_to_a =
            Connection::new(a_connection, PinlineConnected::new("a", vec![0]));

        let b_to_b =
            Connection::new(b_connection, PinlineConnected::new("b", vec![0]));

        let out_to_out = Connection::new(
            out_connection,
            PinlineConnected::new("out", vec![0]),
        );

        let and_parts = Parts::new(vec![ChipConnected::new(
            "Nand",
            vec![a_to_a, b_to_b, out_to_out],
        )]);

        let and_chip = Chip::new("And", and_input, and_output, and_parts);

        println!("{:#?}", and_chip);

        assert!(false);
    }
}
