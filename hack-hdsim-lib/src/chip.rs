#[derive(Debug)]
pub struct Chip {
    pub name: String,
    pub pinlines: ChipPinlines,
    pub parts: Vec<Child>,
    pub clocked: bool,
    pub builtin_id: Option<BuiltinChips>,
}

#[derive(Debug)]
pub struct ChipPinlines {
    pub input: Pinlines,
    pub internal: Pinlines,
    pub output: Pinlines,
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
    pub input_connections: Vec<ChildConnection>,
    pub output_connections: Vec<ChildConnection>,
}

#[derive(Debug, Clone)]
pub struct ChildConnection {
    pub own: PinlineConnection,
    pub foreign: PinlineConnection,
}

#[derive(Debug, Clone)]
pub struct PinlineConnection {
    pub name: String,
    pub indices: Vec<usize>,
}

#[derive(Debug, Clone)]
pub enum BuiltinChips {
    Nand,
    Not,
    DFF,
    Mux,
    Bit,
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
            for connection in part
                .input_connections
                .iter()
                .chain(part.output_connections.iter())
            {
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
            pinlines: ChipPinlines::new(input, internal, output),
            parts,
            clocked,
            builtin_id: None,
        }
    }
    pub fn new_builtin(id: BuiltinChips) -> Self {
        use BuiltinChips::*;
        let name: &str;
        let input: Pinlines;
        let output: Pinlines;
        let internal: Pinlines;
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
                internal = Vec::with_capacity(0);
            }
            Not => {
                name = "Not";
                input = vec![Pinline::with_capacity("in", 1)];
                output = vec![Pinline::with_capacity("out", 1)];
                clocked = false;
                internal = Vec::with_capacity(0);
            }
            DFF => {
                name = "DFF";
                input = vec![Pinline::with_capacity("in", 1)];
                output = vec![Pinline::with_capacity("out", 1)];
                clocked = true;
                internal = vec![
                    Pinline::with_capacity("buffer1", 1),
                    Pinline::with_capacity("buffer2", 1),
                ];
            }
            Mux => {
                name = "Mux";
                input = vec![
                    Pinline::with_capacity("a", 1),
                    Pinline::with_capacity("b", 1),
                    Pinline::with_capacity("sel", 1),
                ];
                output = vec![Pinline::with_capacity("out", 1)];
                clocked = false;
                internal = Vec::with_capacity(0);
            }
            Bit => {
                name = "Bit";
                input = vec![
                    Pinline::with_capacity("in", 1),
                    Pinline::with_capacity("load", 1),
                ];
                output = vec![Pinline::with_capacity("out", 1)];
                clocked = true;
                internal = vec![
                    Pinline::with_capacity("buffer1", 1),
                    Pinline::with_capacity("buffer2", 1),
                ];
            }
        }
        Self {
            name: name.to_string(),
            pinlines: ChipPinlines::new(input, internal, output),
            parts: Vec::with_capacity(0),
            clocked,
            builtin_id: Some(id),
        }
    }
    pub fn is_builtin(&self) -> bool {
        self.parts.is_empty()
    }
    /// Tick for clocked chips
    pub fn read_input(&mut self) {
        if !self.clocked {
            panic!("read_input is only for clocked chips");
        }
        if self.is_builtin() {
            return self.read_input_builtin();
        }

        // Assume unclocked children are in the right order

        // Unclocked should stabilize first
        for unclocked_part in self.parts.iter_mut().filter(|p| !p.chip.clocked)
        {
            self.pinlines.send_input(unclocked_part);
            unclocked_part.chip.evaluate();
            self.pinlines.receive_output(unclocked_part);
        }

        // Then clocked can read input
        for clocked_part in self.parts.iter_mut().filter(|p| p.chip.clocked) {
            self.pinlines.send_input(clocked_part);
            clocked_part.chip.read_input();
        }
    }
    /// Tock for clocked chips
    pub fn produce_output(&mut self) -> &Pinlines {
        if !self.clocked {
            panic!("produce_output is only for clocked chips");
        }
        if self.is_builtin() {
            return self.produce_output_builtin();
        }

        // Assume unclocked children are in the right order

        // Clocked should produce their input first
        for clocked_part in self.parts.iter_mut().filter(|p| p.chip.clocked) {
            clocked_part.chip.produce_output();
            self.pinlines.receive_output(clocked_part);
        }

        // Unclocked can then stabilize
        for unclocked_part in self.parts.iter_mut().filter(|p| !p.chip.clocked)
        {
            self.pinlines.send_input(unclocked_part);
            unclocked_part.chip.evaluate();
            self.pinlines.receive_output(unclocked_part);
        }

        &self.pinlines.output
    }
    /// For unclocked chips
    pub fn evaluate(&mut self) -> &Pinlines {
        if self.clocked {
            panic!("evaluate is only for unclocked chips");
        }
        if self.is_builtin() {
            return self.evaluate_builtin();
        }

        // Assume children are in the right order
        for part in &mut self.parts {
            self.pinlines.send_input(part);
            part.chip.evaluate();
            self.pinlines.receive_output(part);
        }
        &self.pinlines.output
    }
    fn evaluate_builtin(&mut self) -> &Pinlines {
        use BuiltinChips::*;
        match self.builtin_id.as_ref().unwrap() {
            Nand => {
                let res = !(self.pinlines.input[0].pins[0]
                    && self.pinlines.input[1].pins[0]);
                self.pinlines.output[0].pins[0] = res;
            }
            Not => {
                let res = !self.pinlines.input[0].pins[0];
                self.pinlines.output[0].pins[0] = res;
            }
            Mux => {
                let res: Pin;
                if self.pinlines.input[2].pins[0] {
                    res = self.pinlines.input[1].pins[0]
                } else {
                    res = self.pinlines.input[0].pins[0]
                }
                self.pinlines.output[0].pins[0] = res;
            }
            _ => panic!(
                "builtin chip {:?} is clocked, can't evaluate",
                self.builtin_id.clone().unwrap()
            ),
        }
        &self.pinlines.output
    }
    fn read_input_builtin(&mut self) {
        use BuiltinChips::*;
        match self.builtin_id.as_ref().unwrap() {
            DFF => {
                self.pinlines.internal[1].pins[0] =
                    self.pinlines.internal[0].pins[0];
                self.pinlines.internal[0].pins[0] =
                    self.pinlines.input[0].pins[0];
            }
            Bit => {
                self.pinlines.internal[1].pins[0] =
                    self.pinlines.internal[0].pins[0];
                if self.pinlines.input[1].pins[0] {
                    self.pinlines.internal[0].pins[0] =
                        self.pinlines.input[0].pins[0];
                }
            }
            _ => panic!(
                "builtin chip {:?} is unclocked, can't read input",
                self.builtin_id.clone().unwrap()
            ),
        }
    }
    fn produce_output_builtin(&mut self) -> &Pinlines {
        use BuiltinChips::*;
        match self.builtin_id.as_ref().unwrap() {
            DFF => {
                self.pinlines.output[0].pins[0] =
                    self.pinlines.internal[1].pins[0];
            }
            Bit => {
                self.pinlines.output[0].pins[0] =
                    self.pinlines.internal[1].pins[0];
            }
            _ => panic!(
                "builtin chip {:?} is unclocked, can't produce output",
                self.builtin_id.clone().unwrap()
            ),
        }
        &self.pinlines.output
    }
}

impl ChipPinlines {
    pub fn new(input: Pinlines, internal: Pinlines, output: Pinlines) -> Self {
        Self {
            input,
            internal,
            output,
        }
    }
    pub fn send_input(&self, part: &mut Child) {
        for connection in &part.input_connections {
            let name_to_find = connection.foreign.name.as_str();
            let relevant_pinline = self
                .input
                .iter()
                .chain(self.internal.iter())
                .find(|p| p.name == name_to_find)
                .unwrap();
            let input_pinline = relevant_pinline.clone().into_own(connection);
            part.chip.pinlines.input.set_pinline(input_pinline);
        }
    }
    pub fn receive_output(&mut self, part: &Child) {
        for connection in &part.output_connections {
            let relevant_pinline = part
                .chip
                .pinlines
                .output
                .get_pinline(connection.own.name.as_str())
                .unwrap();
            let our_pinline = relevant_pinline.clone().into_foreign(connection);
            let pinline_to_replace = self
                .internal
                .iter_mut()
                .chain(self.output.iter_mut())
                .find(|p| p.name == connection.foreign.name.as_str())
                .unwrap();
            let _ = std::mem::replace(pinline_to_replace, our_pinline);
        }
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
    pub fn into_own(mut self: Pinline, connection: &ChildConnection) -> Self {
        self.name = connection.own.name.clone();
        let mut new_pins = vec![false; connection.own.indices.len()];
        for (own_i, foreing_i) in connection
            .own
            .indices
            .iter()
            .zip(connection.foreign.indices.iter())
        {
            new_pins[*own_i] = self.pins[*foreing_i];
        }
        self.pins = new_pins;
        self
    }
    pub fn into_foreign(
        mut self: Pinline,
        connection: &ChildConnection,
    ) -> Self {
        self.name = connection.foreign.name.clone();
        let mut new_pins = vec![false; connection.foreign.indices.len()];
        for (own_i, foreing_i) in connection
            .own
            .indices
            .iter()
            .zip(connection.foreign.indices.iter())
        {
            new_pins[*foreing_i] = self.pins[*own_i];
        }
        self.pins = new_pins;
        self
    }
    pub fn get_pin(&self, index: usize) -> Pin {
        // Bounds check here?
        self.pins[index]
    }
}

impl Child {
    pub fn new(chip: Chip, connections: Vec<ChildConnection>) -> Self {
        // Verify that connection names make sense?
        let input_connections = connections
            .iter()
            .filter(|c| {
                chip.pinlines
                    .input
                    .get_pinline(c.own.name.as_str())
                    .is_some()
            })
            .cloned()
            .collect();
        let output_connections = connections
            .iter()
            .filter(|c| {
                chip.pinlines
                    .output
                    .get_pinline(c.own.name.as_str())
                    .is_some()
            })
            .cloned()
            .collect();
        Self {
            chip,
            input_connections,
            output_connections,
        }
    }
}

impl ChildConnection {
    pub fn new(own: PinlineConnection, foreign: PinlineConnection) -> Self {
        Self { own, foreign }
    }
}

impl PinlineConnection {
    pub fn new(name: &str, indices: Vec<usize>) -> Self {
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

        chip.pinlines
            .input
            .set_pinline(Pinline::new("a", vec![true]));
        res_actual = chip.evaluate();
        assert_eq!(res_actual, &res_expected);

        res_expected[0].pins[0] = false;

        chip.pinlines
            .input
            .set_pinline(Pinline::new("b", vec![true]));
        res_actual = chip.evaluate();
        assert_eq!(res_actual, &res_expected);

        res_expected[0].pins[0] = true;

        chip.pinlines
            .input
            .set_pinline(Pinline::new("a", vec![false]));
        res_actual = chip.evaluate();
        assert_eq!(res_actual, &res_expected);
    }
    #[test]
    fn not() {
        let mut chip = Chip::new_builtin(BuiltinChips::Not);
        let mut res_expected = vec![Pinline::new("out", vec![true])];
        let mut res_actual = chip.evaluate();
        assert_eq!(res_actual, &res_expected);

        res_expected[0].pins[0] = false;

        chip.pinlines
            .input
            .set_pinline(Pinline::new("in", vec![true]));
        res_actual = chip.evaluate();
        assert_eq!(res_actual, &res_expected);
    }
    #[test]
    fn dff() {
        let mut chip = Chip::new_builtin(BuiltinChips::DFF);
        let mut res_expected = vec![Pinline::new("out", vec![false])];
        chip.pinlines
            .input
            .set_pinline(Pinline::new("in", vec![true]));
        chip.read_input();
        let mut res_actual = chip.produce_output();
        assert_eq!(res_actual, &res_expected);

        res_expected[0].pins[0] = true;

        chip.pinlines
            .input
            .set_pinline(Pinline::new("in", vec![false]));
        chip.read_input();
        res_actual = chip.produce_output();
        assert_eq!(res_actual, &res_expected);

        res_expected[0].pins[0] = false;
        chip.read_input();
        res_actual = chip.produce_output();
        assert_eq!(res_actual, &res_expected);
    }
    #[test]
    fn mux() {
        let mut chip = Chip::new_builtin(BuiltinChips::Mux);
        let mut res_expected = vec![Pinline::new("out", vec![false])];
        chip.pinlines
            .input
            .set_pinline(Pinline::new("b", vec![true]));
        let mut res_actual = chip.evaluate();
        assert_eq!(res_actual, &res_expected);

        res_expected[0].pins[0] = true;

        chip.pinlines
            .input
            .set_pinline(Pinline::new("sel", vec![true]));
        res_actual = chip.evaluate();
        assert_eq!(res_actual, &res_expected);

        chip.pinlines.input.set_pinlines(vec![
            Pinline::new("a", vec![true]),
            Pinline::new("b", vec![false]),
            Pinline::new("sel", vec![false]),
        ]);
        res_actual = chip.evaluate();
        assert_eq!(res_actual, &res_expected);
    }
    fn test_bit(mut chip: Chip) {
        let mut res_expected = vec![Pinline::new("out", vec![false])];
        chip.pinlines.input.set_pinlines(vec![
            Pinline::new("in", vec![true]),
            Pinline::new("load", vec![true]),
        ]);
        chip.read_input();
        let mut res_actual = chip.produce_output();
        assert_eq!(res_actual, &res_expected);

        res_expected[0].pins[0] = true;

        chip.pinlines
            .input
            .set_pinline(Pinline::new("in", vec![false]));

        chip.read_input();
        res_actual = chip.produce_output();
        assert_eq!(res_actual, &res_expected);

        res_expected[0].pins[0] = false;

        chip.read_input();
        res_actual = chip.produce_output();
        assert_eq!(res_actual, &res_expected);

        chip.pinlines.input.set_pinlines(vec![
            Pinline::new("in", vec![true]),
            Pinline::new("load", vec![false]),
        ]);

        chip.read_input();
        res_actual = chip.produce_output();
        assert_eq!(res_actual, &res_expected);

        chip.read_input();
        res_actual = chip.produce_output();
        assert_eq!(res_actual, &res_expected);
    }
    #[test]
    fn bit() {
        let chip = Chip::new_builtin(BuiltinChips::Bit);
        test_bit(chip);
    }

    fn construct_custom_and() -> Chip {
        Chip::new_custom(
            "And",
            vec![
                Pinline::with_capacity("a", 1),
                Pinline::with_capacity("b", 1),
            ],
            vec![Pinline::with_capacity("out", 1)],
            vec![
                Child::new(
                    Chip::new_builtin(BuiltinChips::Nand),
                    vec![
                        ChildConnection::new(
                            PinlineConnection::new("a", vec![0]),
                            PinlineConnection::new("a", vec![0]),
                        ),
                        ChildConnection::new(
                            PinlineConnection::new("b", vec![0]),
                            PinlineConnection::new("b", vec![0]),
                        ),
                        ChildConnection::new(
                            PinlineConnection::new("out", vec![0]),
                            PinlineConnection::new("c", vec![0]),
                        ),
                    ],
                ),
                Child::new(
                    Chip::new_builtin(BuiltinChips::Nand),
                    vec![
                        ChildConnection::new(
                            PinlineConnection::new("a", vec![0]),
                            PinlineConnection::new("c", vec![0]),
                        ),
                        ChildConnection::new(
                            PinlineConnection::new("b", vec![0]),
                            PinlineConnection::new("c", vec![0]),
                        ),
                        ChildConnection::new(
                            PinlineConnection::new("out", vec![0]),
                            PinlineConnection::new("out", vec![0]),
                        ),
                    ],
                ),
            ],
        )
    }

    fn construct_custom_bit() -> Chip {
        Chip::new_custom(
            "Bit",
            vec![
                Pinline::with_capacity("in", 1),
                Pinline::with_capacity("load", 1),
            ],
            vec![Pinline::with_capacity("out", 1)],
            vec![
                Child::new(
                    Chip::new_builtin(BuiltinChips::Mux),
                    vec![
                        ChildConnection::new(
                            PinlineConnection::new("b", vec![0]),
                            PinlineConnection::new("in", vec![0]),
                        ),
                        ChildConnection::new(
                            PinlineConnection::new("a", vec![0]),
                            PinlineConnection::new("checkout", vec![0]),
                        ),
                        ChildConnection::new(
                            PinlineConnection::new("sel", vec![0]),
                            PinlineConnection::new("load", vec![0]),
                        ),
                        ChildConnection::new(
                            PinlineConnection::new("out", vec![0]),
                            PinlineConnection::new("int", vec![0]),
                        ),
                    ],
                ),
                Child::new(
                    Chip::new_builtin(BuiltinChips::DFF),
                    vec![
                        ChildConnection::new(
                            PinlineConnection::new("in", vec![0]),
                            PinlineConnection::new("int", vec![0]),
                        ),
                        ChildConnection::new(
                            PinlineConnection::new("out", vec![0]),
                            PinlineConnection::new("out", vec![0]),
                        ),
                        ChildConnection::new(
                            PinlineConnection::new("out", vec![0]),
                            PinlineConnection::new("checkout", vec![0]),
                        ),
                    ],
                ),
            ],
        )
    }

    #[test]
    fn internal_pins() {
        let and = construct_custom_and();
        assert_eq!(and.pinlines.internal, vec![Pinline::with_capacity("c", 1)]);
    }
    #[test]
    fn custom_unclocked() {
        let mut and = construct_custom_and();
        let mut res_expected = vec![Pinline::new("out", vec![false])];
        let mut res_actual = and.evaluate();
        assert_eq!(&res_expected, res_actual);

        and.pinlines
            .input
            .set_pinline(Pinline::new("a", vec![true]));
        res_actual = and.evaluate();
        assert_eq!(&res_expected, res_actual);

        res_expected[0].pins[0] = true;

        and.pinlines
            .input
            .set_pinline(Pinline::new("b", vec![true]));
        res_actual = and.evaluate();
        assert_eq!(&res_expected, res_actual);

        res_expected[0].pins[0] = false;

        and.pinlines
            .input
            .set_pinline(Pinline::new("a", vec![false]));
        res_actual = and.evaluate();
        assert_eq!(&res_expected, res_actual);
    }
    #[test]
    fn custom_clocked() {
        let bit = construct_custom_bit();
        test_bit(bit)
    }
}
