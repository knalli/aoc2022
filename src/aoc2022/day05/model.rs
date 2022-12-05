pub struct Instruction {
    from: i32,
    to: i32,
    amount: i32,
}

impl Instruction {
    pub fn new(amount: i32, from: i32, to: i32) -> Instruction {
        return Instruction { from, to, amount };
    }

    pub fn from(&self) -> i32 {
        self.from
    }
    pub fn to(&self) -> i32 {
        self.to
    }
    pub fn amount(&self) -> i32 {
        self.amount
    }
}