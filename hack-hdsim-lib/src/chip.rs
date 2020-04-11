pub type Pin = bool;

pub type PinLine = Vec<Pin>;

pub struct Chip {
    _input: Vec<PinLine>,
    _output: Vec<PinLine>,
}

impl Chip {
    pub fn new(_input: Vec<PinLine>, _output: Vec<PinLine>) -> Self {
        Self { _input, _output }
    }
}
