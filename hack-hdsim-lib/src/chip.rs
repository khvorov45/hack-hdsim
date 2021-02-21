/// Applies to both user and built-in chips
pub trait Chip {
    fn get_name(&self) -> &str;
    fn process_input(&self, input: ChipIO) -> ChipIO;
}

pub type ChipsAvailable = Vec<Box<dyn Chip>>;

/// Input/Output for any chip
#[derive(Debug)]
pub struct ChipIO {
    pub pinlines: Vec<PinlineIO>,
}

/// One pinline's pins
#[derive(Debug)]
pub struct PinlineIO {
    pub name: String,
    pub pins: Vec<Pin>,
}

/// User-defined chip
#[derive(Debug)]
pub struct UserChipSpec {
    pub name: String,
    pub input: ChipIOSpec,
    pub output: ChipIOSpec,
    pub parts: ChildrenSpec,
    pub internal: ChipIOSpec,
}

/// Input/output of a chip
#[derive(Debug)]
pub struct ChipIOSpec {
    pub pinlines: Vec<PinlineIOSpec>,
}

/// A set of pins with a name
#[derive(Debug)]
pub struct PinlineIOSpec {
    pub name: String,
    pub pin_count: usize,
}

/// A set of chips connected to the pins of another chip
#[derive(Debug)]
pub struct ChildrenSpec {
    pub children: Vec<ChildSpec>,
}

/// A chip connected to another chip
pub struct ChildSpec {
    pub chip: Box<dyn Chip>,
    pub connections: Vec<ChildConnectionSpec>,
}

/// A pinline of one chip going to a pinline of another
#[derive(Debug)]
pub struct ChildConnectionSpec {
    pub own: PinlineConnectionSpec,
    pub foreign: PinlineConnectionSpec,
}

/// Name and pin indices of a pinline that connect somewhere
#[derive(Debug, Clone)]
pub struct PinlineConnectionSpec {
    pub name: String,
    pub indices: Vec<u32>,
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
        let mut internal = Vec::<PinlineIOSpec>::new();
        let mut exposed_names = input.get_names();
        exposed_names.append(&mut output.get_names());
        for part in &parts.children {
            for connection in &part.connections {
                if exposed_names
                    .iter()
                    // I don't know how we got to a triple reference here
                    .find(|n| n == &&connection.foreign.name)
                    .is_none()
                    && internal
                        .iter()
                        .find(|p| p.name == connection.foreign.name)
                        .is_none()
                {
                    internal.push(PinlineIOSpec::new(
                        connection.foreign.name.as_str(),
                        connection.foreign.get_pin_count(),
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
    /// All pinline names are unique, this will search through input, internal
    /// and output in that order
    pub fn get_pinline(&self, name: &str) -> Option<&PinlineIOSpec> {
        match self.input.get_pinline(name) {
            Some(o) => Some(o),
            None => match self.internal.get_pinline(name) {
                Some(o) => Some(o),
                None => match self.output.get_pinline(name) {
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
        self.pinlines.iter().find(|p| p.name.as_str() == name)
    }
}

impl PinlineIO {
    pub fn new(name: &str, pins: Vec<Pin>) -> Self {
        Self {
            name: name.to_string(),
            pins,
        }
    }
    pub fn get_pin(&self, index: usize) -> Pin {
        // Bounds check here?
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
}

impl ChipIOSpec {
    pub fn new(pinlines: Vec<PinlineIOSpec>) -> Self {
        Self { pinlines }
    }
    pub fn push(&mut self, pinline: PinlineIOSpec) {
        self.pinlines.push(pinline);
    }
    pub fn get_pinline(&self, name: &str) -> Option<&PinlineIOSpec> {
        self.pinlines.iter().find(|p| p.name == name)
    }
    pub fn get_names(&self) -> Vec<&str> {
        self.pinlines.iter().map(|p| p.name.as_str()).collect()
    }
}

impl ChildrenSpec {
    pub fn new(children: Vec<ChildSpec>) -> Self {
        Self { children }
    }
    pub fn get_child(&self, name: &str) -> Option<&ChildSpec> {
        self.children.iter().find(|c| c.chip.get_name() == name)
    }
}

impl ChildSpec {
    pub fn new(
        chip: Box<dyn Chip>,
        connections: Vec<ChildConnectionSpec>,
    ) -> Self {
        Self { chip, connections }
    }
    pub fn get_chip(&self) -> &dyn Chip {
        self.chip.as_ref()
    }
}

impl std::fmt::Debug for ChildSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ChildSpec")
            .field("name", &self.chip.get_name())
            .field("connections", &self.connections)
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
            a_connection.clone(),
            PinlineConnectionSpec::new("a", vec![0]),
        );

        let b_to_b = ChildConnectionSpec::new(
            b_connection.clone(),
            PinlineConnectionSpec::new("b", vec![0]),
        );

        let out_to_c = ChildConnectionSpec::new(
            out_connection.clone(),
            PinlineConnectionSpec::new("c", vec![0]),
        );

        let first_child = ChildSpec::new(
            Box::new(Nand::new()),
            vec![a_to_a, b_to_b, out_to_c],
        );

        let a_to_c = ChildConnectionSpec::new(
            a_connection,
            PinlineConnectionSpec::new("c", vec![0]),
        );

        let b_to_c = ChildConnectionSpec::new(
            b_connection,
            PinlineConnectionSpec::new("c", vec![0]),
        );

        let out_to_out = ChildConnectionSpec::new(
            out_connection,
            PinlineConnectionSpec::new("out", vec![0]),
        );

        let second_child = ChildSpec::new(
            Box::new(Nand::new()),
            vec![a_to_c, b_to_c, out_to_out],
        );

        let and_parts = ChildrenSpec::new(vec![first_child, second_child]);

        let and_chip =
            UserChipSpec::new("And", and_input, and_output, and_parts);

        println!("{:#?}", and_chip);

        assert!(false);
    }
}
