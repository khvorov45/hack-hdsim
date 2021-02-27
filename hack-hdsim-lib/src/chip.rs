#[derive(Debug)]
pub struct Chip {
    pub name: String,
    pub input: Pinlines,
    pub output: Pinlines,
    pub internal: Pinlines,
    pub parts: Vec<Child>,
    pub clocked: bool,
    pub builtin_id: Option<BuiltinChips>,
}

pub type Pinlines = Vec<Pinline>;

#[derive(Debug, Clone, PartialEq)]
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
                    internal.push(Pinline::with_capacity(
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
            builtin_id: None,
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
                input = vec![
                    Pinline::with_capacity("a", 1),
                    Pinline::with_capacity("b", 1),
                ];
                output = vec![Pinline::with_capacity("out", 1)];
                clocked = false;
            }
            Not => {
                name = "Not";
                input = vec![Pinline::with_capacity("in", 1)];
                output = vec![Pinline::with_capacity("out", 1)];
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
            builtin_id: Some(id),
        }
    }
    pub fn is_builtin(&self) -> bool {
        self.parts.is_empty()
    }
    pub fn set_input(&mut self, input: Pinlines) {
        self.input.set_pinlines(input);
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
    pub fn produce_output(&self) -> &Pinlines {
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
        &self.output
    }
    /// For unclocked chips
    pub fn evaluate(&mut self) -> &Pinlines {
        if self.clocked {
            panic!("evaluate is only for unclocked chips")
        }
        if self.is_builtin() {
            return self.evaluate_builtin();
        }

        // If we force children to be in the right order then we should just
        // be able to go through them in order and have all the internal pins
        // set to the appropriate value before we get there

        // Input is set at this point, need to verify that internal pins and
        // output pins are set before actually using them
        let mut pins_set: Vec<String> =
            Vec::with_capacity(self.internal.len() + self.output.len());
        for part in &mut self.parts {
            // Construct input
            let mut part_input = Pinlines::with_capacity(part.chip.input.len());
            for connection in part.get_input_connections() {
                let name_to_find = connection.foreign.name.as_str();
                let mut search_in =
                    self.input.iter().chain(self.internal.iter());
                let pinline = match search_in.find(|p| p.name == name_to_find) {
                    Some(p) => {
                        if pins_set
                                .iter()
                                .find(|p| p == &name_to_find)
                                .is_none()
                            {
                                panic!(
                                    "want to use pin `{}` in chip `{}` for input into chip `{}` but nothing set it (children are in the wrong order)", name_to_find, self.name, part.chip.name
                                );
                            }
                        p
                    },
                    None => panic!(
                        "want to use pin `{}` in chip `{}` for input into chip `{}` but we don't have it in input nor internal pins", name_to_find, self.name, part.chip.name
                    )
                };
                part_input.push(pinline.clone());
            }

            // Then generate output
            part.chip.set_input(part_input);
            part.chip.evaluate();

            // Set own output and internal pins accordingly
            for connection in part.get_output_connections() {
                let name_to_find = connection.foreign.name.as_str();
                let mut search_in =
                    self.internal.iter().chain(self.output.iter());
                match search_in.position(|p| p.name == name_to_find) {
                    Some(i) => {
                        if pins_set
                                .iter()
                                .any(|p| p == name_to_find)
                            {
                                panic!(
                                    "want to set pin `{}` in chip `{}` from output of chip `{}` but it's already set", name_to_find, self.name, part.chip.name
                                );
                            }
                        let to_set = part.chip.output
                            .iter()
                            .find(|o| o.name == name_to_find)
                            .unwrap()
                            .clone();
                        pins_set.push(to_set.name.clone());
                        if i < self.internal.len() {
                            self.internal[i] = to_set;
                        } else {
                            self.output[i] = to_set;
                        }
                    },
                    None => panic!(
                        "want to set pin `{}` in chip `{}` from output of chip `{}` but we don't have it in output nor internal pins", name_to_find, self.name, part.chip.name
                    )
                };
            }
        }
        &self.output
    }
    fn evaluate_builtin(&mut self) -> &Pinlines {
        use BuiltinChips::*;
        match self.builtin_id.as_ref().unwrap() {
            Nand => {
                let res = !self.input[0].pins[0] && !self.input[1].pins[0];
                self.output[0].pins = vec![res];
            }
            Not => {
                let res = !self.input[0].pins[0];
                self.output[0].pins = vec![res];
            }
        }
        &self.output
    }
}

pub trait PinlinesMethods {
    fn get_pinline(&self, name: &str) -> Option<&Pinline>;
    fn set_pinline(&mut self, pinline: Pinline);
    fn set_pinlines(&mut self, pinline: Pinlines);
}

impl PinlinesMethods for Pinlines {
    fn get_pinline(&self, name: &str) -> Option<&Pinline> {
        self.iter().find(|p| p.name == name)
    }
    fn set_pinline(&mut self, pinline: Pinline) {
        let i = self.iter().position(|p| p.name == pinline.name).unwrap();
        self[i].pins = pinline.pins;
    }
    fn set_pinlines(&mut self, pinlines: Pinlines) {
        for pinline in pinlines {
            self.set_pinline(pinline)
        }
    }
}

impl Pinline {
    pub fn new(name: &str, pins: Vec<Pin>) -> Self {
        Self {
            name: name.to_string(),
            pins,
        }
    }
    pub fn with_capacity(name: &str, size: usize) -> Self {
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
        // Verify that connection names make sense?
        Self { chip, connections }
    }
    pub fn get_input_connections(&self) -> Vec<&ChildConnection> {
        self.connections
            .iter()
            .filter(|c| {
                self.chip.input.get_pinline(c.own.name.as_str()).is_some()
            })
            .collect()
    }
    pub fn get_output_connections(&self) -> Vec<&ChildConnection> {
        self.connections
            .iter()
            .filter(|c| {
                self.chip.output.get_pinline(c.own.name.as_str()).is_some()
            })
            .collect()
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
    fn nand() {
        let mut chip = Chip::new_builtin(BuiltinChips::Nand);
        let mut res_expected = vec![Pinline::new("out", vec![true])];
        let mut res_actual = chip.evaluate();
        assert_eq!(res_actual, &res_expected);

        res_expected[0].pins[0] = false;

        chip.set_input(vec![Pinline::new("a", vec![true])]);
        res_actual = chip.evaluate();
        assert_eq!(res_actual, &res_expected);

        chip.set_input(vec![Pinline::new("b", vec![true])]);
        res_actual = chip.evaluate();
        assert_eq!(res_actual, &res_expected);

        chip.set_input(vec![Pinline::new("a", vec![false])]);
        res_actual = chip.evaluate();
        assert_eq!(res_actual, &res_expected);
    }
}
