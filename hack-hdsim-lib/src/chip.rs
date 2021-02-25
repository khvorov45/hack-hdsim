#[derive(Debug)]
pub struct Chip {
    pub name: String,
    pub input: Pinlines,
    pub output: Pinlines,
    pub internal: Pinlines,
    pub parts: Vec<Child>,
    pub clocked: bool,
}

pub type Pinlines = Vec<Pinline>;

#[derive(Debug)]
pub struct Pinline {
    pub name: String,
    pub pins: Vec<Pin>,
}

pub type Pin = bool;

#[derive(Debug)]
pub struct Child {
    pub chip: Chip,
    pub connections: Vec<ChildConnection>,
}

#[derive(Debug)]
pub struct ChildConnection {
    pub own: PinlineConnection,
    pub foreign: PinlineConnection,
}

#[derive(Debug, Clone)]
pub struct PinlineConnection {
    pub name: String,
    pub indices: Vec<u32>,
}

#[derive(Debug)]
pub enum BuiltinChips {
    Nand,
    Not,
}

// ============================================================================

impl Chip {
    pub fn new_custom(
        name: &str,
        input: Pinlines,
        output: Pinlines,
        parts: Vec<Child>,
    ) -> Self {
        if parts.is_empty() {
            panic!(
                "chips with no children must be built-in, so call Chip::builtin"
            )
        }
        let mut clocked = false;
        let mut internal = Vec::<Pinline>::new();
        for part in &parts {
            // We are clocked if any child is clocked
            if !clocked && part.chip.clocked {
                clocked = true
            }
            // Figure out what the internal pins are
            for connection in &part.connections {
                // Any name not already present somewhere should be added
                let name = connection.foreign.name.as_str();
                if input.get_pinline(name).is_none()
                    && output.get_pinline(name).is_none()
                    && internal.get_pinline(name).is_none()
                {
                    internal.push(Pinline::new(
                        name,
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
            internal,
            clocked,
        }
    }
    pub fn new_builtin(id: BuiltinChips) -> Self {
        use BuiltinChips::*;
        let name: &str;
        let input: Pinlines;
        let output: Pinlines;
        let clocked: bool;
        match id {
            Nand => {
                name = "Nand";
                input = vec![Pinline::new("a", 1), Pinline::new("b", 1)];
                output = vec![Pinline::new("out", 1)];
                clocked = false;
            }
            Not => {
                name = "Not";
                input = vec![Pinline::new("in", 1)];
                output = vec![Pinline::new("out", 1)];
                clocked = false;
            }
        }
        Self {
            name: name.to_string(),
            input,
            output,
            parts: Vec::with_capacity(0),
            internal: Vec::with_capacity(0),
            clocked,
        }
    }
    /// All pinline names are unique, this will search through input, internal
    /// and output in that order
    pub fn get_pinline(&self, name: &str) -> Option<&Pinline> {
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
    /// Tick for clocked chips
    pub fn read_input(&mut self, input: Pinlines) {
        if !self.clocked {
            panic!("read_input is only for clocked chips")
        }
        // Compare input to spec here presumably

        self.input = input;

        // Set input of all the children

        // We also need to go through the children and actually run those
        // that are unclocked
    }
    /// Tock for clocked chips
    pub fn produce_output(&self) -> Pinlines {
        if !self.clocked {
            panic!("produce_output is only for clocked chips")
        }
        for part in &self.parts {
            // Go through each pinline and see if we've got its foreign name
            // in our input or internal pins
            // (output isn't plugged into anything).
            // Then get the correspoding value in input.
            // We should have the full input by the end of that.

            let out = &part.chip; // .process_input(input: ChipIO);

            // Take the output and create the appropriate set of pins out of it.
            // That is, go through the pins and see if we  have their foreign
            // name somewhere, if so - set the appropriate value.
        }
        // We probably need to rerun all the unclocked chips here as well
        // Placeholder
        vec![Pinline::new("a", 1)]
    }
    /// For unclocked chips
    pub fn evaluate(&self) -> Pinlines {
        if self.clocked {
            panic!("evaluate is only for unclocked chips")
        }
        // Placeholder
        vec![Pinline::new("a", 1)]
    }
}

pub trait GetPinline {
    fn get_pinline(&self, name: &str) -> Option<&Pinline>;
}

impl GetPinline for Pinlines {
    fn get_pinline(&self, name: &str) -> Option<&Pinline> {
        self.iter().find(|p| p.name == name)
    }
}

impl Pinline {
    pub fn new(name: &str, size: usize) -> Self {
        Self {
            name: name.to_string(),
            pins: vec![false; size],
        }
    }
    pub fn get_pin(&self, index: usize) -> Pin {
        // Bounds check here?
        self.pins[index]
    }
}

impl Child {
    pub fn new(chip: Chip, connections: Vec<ChildConnection>) -> Self {
        Self { chip, connections }
    }
}

impl ChildConnection {
    pub fn new(own: PinlineConnection, foreign: PinlineConnection) -> Self {
        Self { own, foreign }
    }
}

impl PinlineConnection {
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

// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn chip_new() {
        let a_input_pinline = Pinline::new("a", 1);
        let b_input_pinline = Pinline::new("b", 1);

        let and_input: Pinlines = vec![a_input_pinline, b_input_pinline];

        let out_output_pinline = Pinline::new("out", 1);
        let and_output = vec![out_output_pinline];

        let a_connection = PinlineConnection::new("a", vec![0]);
        let b_connection = PinlineConnection::new("b", vec![0]);
        let out_connection = PinlineConnection::new("out", vec![0]);

        let a_to_a = ChildConnection::new(
            a_connection.clone(),
            PinlineConnection::new("a", vec![0]),
        );

        let b_to_b = ChildConnection::new(
            b_connection.clone(),
            PinlineConnection::new("b", vec![0]),
        );

        let out_to_c = ChildConnection::new(
            out_connection.clone(),
            PinlineConnection::new("c", vec![0]),
        );

        let first_child = Child::new(
            Chip::new_builtin(BuiltinChips::Nand),
            vec![a_to_a, b_to_b, out_to_c],
        );

        let a_to_c = ChildConnection::new(
            a_connection,
            PinlineConnection::new("c", vec![0]),
        );

        let b_to_c = ChildConnection::new(
            b_connection,
            PinlineConnection::new("c", vec![0]),
        );

        let out_to_out = ChildConnection::new(
            out_connection,
            PinlineConnection::new("out", vec![0]),
        );

        let second_child = Child::new(
            Chip::new_builtin(BuiltinChips::Nand),
            vec![a_to_c, b_to_c, out_to_out],
        );

        let and_parts = vec![first_child, second_child];

        let and_chip =
            Chip::new_custom("And", and_input, and_output, and_parts);

        println!("{:#?}", and_chip);

        assert!(false);
    }
}
