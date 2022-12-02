#[derive(PartialEq, Copy, Clone)]
pub enum Winning {
    YES,
    NO,
    DRAW,
}

#[derive(PartialEq, Copy, Clone)]
pub enum RPS {
    ROCK,
    PAPER,
    SCISSOR,
}

impl RPS {
    pub fn decode(ch: char) -> RPS {
        if ch == 'A' || ch == 'X' {
            RPS::ROCK
        } else if ch == 'B' || ch == 'Y' {
            RPS::PAPER
        } else if ch == 'C' || ch == 'Z' {
            RPS::SCISSOR
        } else {
            panic!("Invalid type")
        }
    }

    pub fn wins(&self, other: &RPS) -> Winning {
        if *self == *other {
            return Winning::DRAW;
        }
        if *self == RPS::ROCK && *other == RPS::SCISSOR {
            return Winning::YES;
        }
        if *self == RPS::PAPER && *other == RPS::ROCK {
            return Winning::YES;
        }
        if *self == RPS::SCISSOR && *other == RPS::PAPER {
            return Winning::YES;
        }
        return Winning::NO;
    }

    pub fn require_for_win(&self) -> RPS {
        match self {
            RPS::ROCK => RPS::PAPER,
            RPS::PAPER => RPS::SCISSOR,
            RPS::SCISSOR => RPS::ROCK,
        }
    }

    pub fn require_for_lost(&self) -> RPS {
        match self {
            RPS::ROCK => RPS::SCISSOR,
            RPS::PAPER => RPS::ROCK,
            RPS::SCISSOR => RPS::PAPER,
        }
    }
}
