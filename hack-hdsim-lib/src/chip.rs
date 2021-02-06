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
    input: ChipIO,
    output: ChipIO,
    parts: ChildrenSpec,
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
        input: ChipIO,
        output: ChipIO,
        parts: ChildrenSpec,
    ) -> Self {
        Self {
            name: name.to_string(),
            input,
            output,
            parts,
        }
    }
    pub fn get_input(&self) -> &ChipIO {
        &self.input
    }
    pub fn get_output(&self) -> &ChipIO {
        &self.output
    }
    pub fn get_parts(&self) -> &ChildrenSpec {
        &self.parts
    }
    pub fn get_child(&self, name: &str) -> Option<&ChildSpec> {
        self.parts.get_child(name)
    }
}

impl Chip for UserChipSpec {
    fn get_name(&self) -> &str {
        self.name.as_str()
    }
    fn process_input(&self, input: ChipIO) -> ChipIO {
        // Compare input to spec here presumably
        for part in &self.parts.children {
            // Have to create the appropriate input per specification somehow
            let out = part.get_chip(); // .process_input(input: ChipIO);

            // Take the output and create the appropriate set of pins out of it
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

impl ChildrenSpec {
    pub fn new(children: Vec<ChildSpec>) -> Self {
        Self { children }
    }
    pub fn get_child(&self, name: &str) -> Option<&ChildSpec> {
        self.children.iter().find(|c| c.get_name() == name)
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
    pub fn get_chip(&self) -> &dyn Chip {
        self.chip.as_ref()
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
        let a_input_pinline = PinlineIO::new("a", vec![true]);
        let b_input_pinline = PinlineIO::new("b", vec![true]);

        let and_input = ChipIO::new(vec![a_input_pinline, b_input_pinline]);

        let out_output_pinline = PinlineIO::new("out", vec![true]);
        let and_output = ChipIO::new(vec![out_output_pinline]);

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
