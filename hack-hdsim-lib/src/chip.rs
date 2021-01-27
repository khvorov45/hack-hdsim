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
    pub fn connect(&self, indices: Vec<u32>) -> PinlineConnected {
        PinlineConnected::new(self.name.as_str(), indices)
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
pub struct PinlineConnected {
    name: String,
    indices: Vec<u32>,
}

impl PinlineConnected {
    pub fn new(name: &str, indices: Vec<u32>) -> Self {
        Self {
            name: name.to_string(),
            indices,
        }
    }
}

#[derive(Debug)]
pub struct Connection {
    from: PinlineConnected,
    to: PinlineConnected,
}

impl Connection {
    pub fn new(from: PinlineConnected, to: PinlineConnected) -> Self {
        Self { from, to }
    }
}

#[derive(Debug)]
pub struct Chip {
    name: String,
    input: Interface,
    output: Interface,
    parts: Vec<ChipConnected>,
}

#[derive(Debug)]
pub struct ChipConnected {
    chip: Chip,
    connections: Vec<Connection>,
}

impl Chip {
    pub fn new(
        name: &str,
        input: Interface,
        output: Interface,
        parts: Vec<ChipConnected>,
    ) -> Self {
        Self {
            name: name.to_string(),
            input,
            output,
            parts,
        }
    }
    pub fn builtin(name: &str) -> Self {
        if name == "Nand" {
            let mut input = Interface::new(2);
            input.push(Pinline::new("a", 1));
            input.push(Pinline::new("b", 1));

            let mut output = Interface::new(1);
            output.push(Pinline::new("out", 1));

            return Self {
                name: "Nand".to_string(),
                input,
                output,
                parts: Vec::<ChipConnected>::with_capacity(0),
            };
        }
        panic!("No such built-in chip: {}", name)
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
        let a_input_pinline = Pinline::new("a", 1);
        let b_input_pinline = Pinline::new("b", 1);

        let mut and_input = Interface::new(2);
        and_input.push(a_input_pinline);
        and_input.push(b_input_pinline);

        let out_output_pinline = Pinline::new("out", 1);
        let mut and_output = Interface::new(1);
        and_output.push(out_output_pinline);

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

        let mut and_parts = Vec::<ChipConnected>::with_capacity(1);
        and_parts.push(ChipConnected {
            chip: Chip::builtin("Nand"),
            connections: vec![a_to_a, b_to_b, out_to_out],
        });

        let and_chip = Chip::new("And", and_input, and_output, and_parts);

        println!("{:#?}", and_chip);

        assert!(false);
    }
}
