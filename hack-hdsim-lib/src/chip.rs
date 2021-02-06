/// Applies to both user and built-in chips
pub trait Chip {
    fn get_name(&self) -> &str;
    fn process_input(&self, input: ChipIO) -> ChipIO;
}

pub type ChipsAvailable = Vec<Box<dyn Chip>>;

/// Input/Output for any chip
#[derive(Debug)]
pub struct ChipIO {
    pinlines: Vec<PinlineIO>,
}

/// One pinline's pins
#[derive(Debug)]
pub struct PinlineIO {
    name: String,
    pins: Vec<Pin>,
}

/// User-defined chip
#[derive(Debug)]
pub struct UserChipSpec {
    name: String,
    input: ChipIOSpec,
    output: ChipIOSpec,
    parts: ChildrenSpec,
}

/// Input/output of a chip
#[derive(Debug)]
pub struct ChipIOSpec {
    pinlines: Vec<PinlineSpec>,
}

/// A set of pins with a name
#[derive(Debug)]
pub struct PinlineSpec {
    name: String,
    pin_count: usize,
}

/// A set of chips connected to the pins of another chip
#[derive(Debug)]
pub struct ChildrenSpec {
    children: Vec<ChildSpec>,
}

/// A chip connected to another chip
pub struct ChildSpec {
    chip: Box<dyn Chip>,
    connections: Vec<ChildConnectionSpec>,
}

/// A pinline of one chip going to a pinline of another
#[derive(Debug)]
pub struct ChildConnectionSpec {
    own: PinlineConnectionSpec,
    foreign: PinlineConnectionSpec,
}

/// Name and pin indices of a pinline that connect somewhere
#[derive(Debug)]
pub struct PinlineConnectionSpec {
    name: String,
    indices: Vec<u32>,
}

/// The pin
pub type Pin = bool;

impl UserChipSpec {
    pub fn new(
        name: &str,
        input: ChipIOSpec,
        output: ChipIOSpec,
        parts: ChildrenSpec,
    ) -> Self {
        Self {
            name: name.to_string(),
            input,
            output,
            parts,
        }
    }
    pub fn get_input(&self) -> &ChipIOSpec {
        &self.input
    }
    pub fn get_output(&self) -> &ChipIOSpec {
        &self.output
    }
    pub fn get_parts(&self) -> &ChildrenSpec {
        &self.parts
    }
}

impl Chip for UserChipSpec {
    fn get_name(&self) -> &str {
        self.name.as_str()
    }
    fn process_input(&self, input: ChipIO) -> ChipIO {
        // Compare input to spec here presumably
        for part in &self.parts.children {
            // Find an available chip with this name
            part.get_name();
            // Call the `process_input` function on it with the appropriate
            // arguments
            // Take the input and create the appropriate set of pins out of it
        }
        ChipIO::new(vec![PinlineIO::new("a", vec![true])])
    }
}

impl ChipIO {
    pub fn new(pinlines: Vec<PinlineIO>) -> Self {
        Self { pinlines }
    }
    pub fn get_pinline(&self, name: &str) -> Option<&PinlineIO> {
        self.pinlines.iter().find(|p| p.get_name() == name)
    }
}

impl PinlineIO {
    pub fn new(name: &str, pins: Vec<Pin>) -> Self {
        Self {
            name: name.to_string(),
            pins,
        }
    }
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }
    pub fn get_pin(&self, index: usize) -> Pin {
        self.pins[index]
    }
}

impl PinlineSpec {
    pub fn new(name: &str, pin_count: usize) -> Self {
        Self {
            name: name.to_string(),
            pin_count,
        }
    }
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }
    pub fn get_pin_count(&self) -> usize {
        self.pin_count
    }
}

impl ChipIOSpec {
    pub fn new(pinlines: Vec<PinlineSpec>) -> Self {
        Self { pinlines }
    }
    pub fn get_pinlines(&self) -> &Vec<PinlineSpec> {
        &self.pinlines
    }
    pub fn push(&mut self, pinline: PinlineSpec) {
        self.pinlines.push(pinline);
    }
    pub fn get_pinline(&self, name: &str) -> Option<&PinlineSpec> {
        self.pinlines.iter().find(|p| p.name == name)
    }
}

impl ChildrenSpec {
    pub fn new(children: Vec<ChildSpec>) -> Self {
        Self { children }
    }
}

impl ChildSpec {
    pub fn new(
        chip: Box<dyn Chip>,
        connections: Vec<ChildConnectionSpec>,
    ) -> Self {
        Self { chip, connections }
    }
    pub fn get_name(&self) -> &str {
        self.chip.get_name()
    }
}

impl std::fmt::Debug for ChildSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ChildSpec")
            .field("name", &self.get_name())
            .finish()
    }
}

impl ChildConnectionSpec {
    pub fn new(
        own: PinlineConnectionSpec,
        foreign: PinlineConnectionSpec,
    ) -> Self {
        Self { own, foreign }
    }
}

impl PinlineConnectionSpec {
    pub fn new(name: &str, indices: Vec<u32>) -> Self {
        Self {
            name: name.to_string(),
            indices,
        }
    }
}

pub struct Nand {}

impl Nand {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Nand {
    fn default() -> Self {
        Nand::new()
    }
}

impl Chip for Nand {
    fn get_name(&self) -> &str {
        "Nand"
    }
    fn process_input(&self, input: ChipIO) -> ChipIO {
        // Validate input I guess
        let res = !(input.get_pinline("a").unwrap().get_pin(0)
            && input.get_pinline("b").unwrap().get_pin(0));
        ChipIO::new(vec![PinlineIO::new("out", vec![res])])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn chip_new() {
        let a_input_pinline = PinlineSpec::new("a", 1);
        let b_input_pinline = PinlineSpec::new("b", 1);

        let and_input = ChipIOSpec::new(vec![a_input_pinline, b_input_pinline]);

        let out_output_pinline = PinlineSpec::new("out", 1);
        let and_output = ChipIOSpec::new(vec![out_output_pinline]);

        let a_connection = PinlineConnectionSpec::new("a", vec![0]);
        let b_connection = PinlineConnectionSpec::new("b", vec![0]);
        let out_connection = PinlineConnectionSpec::new("out", vec![0]);

        let a_to_a = ChildConnectionSpec::new(
            a_connection,
            PinlineConnectionSpec::new("a", vec![0]),
        );

        let b_to_b = ChildConnectionSpec::new(
            b_connection,
            PinlineConnectionSpec::new("b", vec![0]),
        );

        let out_to_out = ChildConnectionSpec::new(
            out_connection,
            PinlineConnectionSpec::new("out", vec![0]),
        );

        let and_parts = ChildrenSpec::new(vec![ChildSpec::new(
            Box::new(Nand::new()),
            vec![a_to_a, b_to_b, out_to_out],
        )]);

        let and_chip =
            UserChipSpec::new("And", and_input, and_output, and_parts);

        println!("{:#?}", and_chip);

        assert!(false);
    }
}
