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
    internal: ChipIOSpec,
}

/// Input/output of a chip
#[derive(Debug)]
pub struct ChipIOSpec {
    pinlines: Vec<PinlineIOSpec>,
}

/// A set of pins with a name
#[derive(Debug)]
pub struct PinlineIOSpec {
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
        // Figure out what the internal pins are
        let mut internal = Vec::new();
        let mut exposed_names = input.get_names();
        exposed_names.append(&mut output.get_names());
        for part in parts.get_children() {
            for connection in part.get_connections() {
                let foreign = connection.get_foreign();
                if exposed_names
                    .iter()
                    // I don't know how we got to a triple reference here
                    .find(|n| n == &&foreign.get_name())
                    .is_none()
                {
                    internal.push(PinlineIOSpec::new(
                        foreign.get_name(),
                        foreign.get_pin_count(),
                    ));
                }
            }
        }
        Self {
            name: name.to_string(),
            input,
            output,
            parts,
            internal: ChipIOSpec::new(internal),
        }
    }
    pub fn get_input(&self) -> &ChipIOSpec {
        &self.input
    }
    pub fn get_input_pinline(&self, name: &str) -> Option<&PinlineIOSpec> {
        self.input.get_pinline(name)
    }
    pub fn get_output(&self) -> &ChipIOSpec {
        &self.output
    }
    pub fn get_output_pinline(&self, name: &str) -> Option<&PinlineIOSpec> {
        self.output.get_pinline(name)
    }
    pub fn get_parts(&self) -> &ChildrenSpec {
        &self.parts
    }
    pub fn get_child(&self, name: &str) -> Option<&ChildSpec> {
        self.parts.get_child(name)
    }
    pub fn get_internal(&self) -> &ChipIOSpec {
        &self.internal
    }
    pub fn get_internal_pinline(&self, name: &str) -> Option<&PinlineIOSpec> {
        self.internal.get_pinline(name)
    }
    /// All pinline names are unique, this will search through input, internal
    /// and output in that order
    pub fn get_pinline(&self, name: &str) -> Option<&PinlineIOSpec> {
        match self.get_input_pinline(name) {
            Some(o) => Some(o),
            None => match self.get_internal_pinline(name) {
                Some(o) => Some(o),
                None => match self.get_output_pinline(name) {
                    Some(o) => Some(o),
                    None => None,
                },
            },
        }
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

impl PinlineIOSpec {
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
    pub fn new(pinlines: Vec<PinlineIOSpec>) -> Self {
        Self { pinlines }
    }
    pub fn get_pinlines(&self) -> &Vec<PinlineIOSpec> {
        &self.pinlines
    }
    pub fn push(&mut self, pinline: PinlineIOSpec) {
        self.pinlines.push(pinline);
    }
    pub fn get_pinline(&self, name: &str) -> Option<&PinlineIOSpec> {
        self.pinlines.iter().find(|p| p.name == name)
    }
    pub fn get_names(&self) -> Vec<&str> {
        self.pinlines.iter().map(|p| p.get_name()).collect()
    }
}

impl ChildrenSpec {
    pub fn new(children: Vec<ChildSpec>) -> Self {
        Self { children }
    }
    pub fn get_children(&self) -> &Vec<ChildSpec> {
        &self.children
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
    pub fn get_connections(&self) -> &Vec<ChildConnectionSpec> {
        &self.connections
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
    pub fn get_foreign(&self) -> &PinlineConnectionSpec {
        &self.foreign
    }
}

impl PinlineConnectionSpec {
    pub fn new(name: &str, indices: Vec<u32>) -> Self {
        Self {
            name: name.to_string(),
            indices,
        }
    }
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }
    pub fn get_pin_count(&self) -> usize {
        self.indices.len()
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
        let a_input_pinline = PinlineIOSpec::new("a", 1);
        let b_input_pinline = PinlineIOSpec::new("b", 1);

        let and_input = ChipIOSpec::new(vec![a_input_pinline, b_input_pinline]);

        let out_output_pinline = PinlineIOSpec::new("out", 1);
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
