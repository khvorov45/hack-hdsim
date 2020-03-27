pub type Pin = bool;

pub struct And2 {
    in_a: Pin,
    in_b: Pin,
    out: Pin,
}

impl And2 {
    pub fn new(in_a: Pin, in_b: Pin) -> And2 {
        And2 {
            in_a,
            in_b,
            out: in_a && in_b,
        }
    }
    pub fn in_a(&self) -> Pin {
        self.in_a
    }
    pub fn in_b(&self) -> Pin {
        self.in_b
    }
    pub fn out(&self) -> Pin {
        self.out
    }
    pub fn set_input(&mut self, a: Pin, b: Pin) -> Pin {
        self.in_a = a;
        self.in_b = b;
        self.out = a && b;
        self.out
    }
}
