#![forbid(unsafe_code)]

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RoundOutcome {
    BothCooperated,
    LeftCheated,
    RightCheated,
    BothCheated,
}

#[derive(PartialEq, Copy, Clone)]
pub enum Bet {
    Cooperate,
    Cheat,
}

pub struct Game {
    left: Box<dyn TrustAgent>,
    right: Box<dyn TrustAgent>,
    rounds_passed: usize,
}

impl Game {
    pub fn new(left: Box<dyn TrustAgent>, right: Box<dyn TrustAgent>) -> Self
    {
        Self {
            left: left,
            right: right,
            rounds_passed: 0,
        }
    }

    pub fn left_score(&self) -> i32 {
        self.left.score()
    }

    pub fn right_score(&self) -> i32 {
        self.right.score()
    }

    pub fn play_round(&mut self) -> RoundOutcome {
        self.rounds_passed += 1;
        let left_bet = self.left.bet();
        let right_bet = self.right.bet();
        if left_bet == Bet::Cooperate && right_bet == Bet::Cooperate {
            self.left.update(2, Bet::Cooperate);
            self.right.update(2, Bet::Cooperate);
            return RoundOutcome::BothCooperated;
        } else if left_bet == Bet::Cooperate && right_bet == Bet::Cheat {
            self.left.update(-1, Bet::Cheat);
            self.right.update(3, Bet::Cooperate);
            return RoundOutcome::RightCheated;
        } else if left_bet == Bet::Cheat && right_bet == Bet::Cooperate {
            self.left.update(3, Bet::Cooperate);
            self.right.update(-1, Bet::Cheat);
            return RoundOutcome::LeftCheated;
        } else {
            self.left.update(0, Bet::Cheat);
            self.right.update(0, Bet::Cheat);
            RoundOutcome::BothCheated
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

pub trait TrustAgent {
    fn bet(&self) -> Bet;
    fn update(&mut self, points: i32, opponent_bet: Bet);
    fn score(&self) -> i32;
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct CheatingAgent {
    score: i32,
}

impl TrustAgent for CheatingAgent {
    fn bet(&self) -> Bet {
        Bet::Cheat
    }
    fn update(&mut self, points: i32, _: Bet) {
        self.score += points;
    }
    fn score(&self) -> i32 {
        self.score
    }
}


////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct CooperatingAgent {
    score: i32,
}

impl TrustAgent for CooperatingAgent {
    fn bet(&self) -> Bet {
        Bet::Cooperate
    }
    fn update(&mut self, points: i32, _: Bet) {
        self.score += points;
    }
    fn score(&self) -> i32 {
        self.score
    }

}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct GrudgerAgent {
    score: i32,
    is_disappointed: bool,
}

impl TrustAgent for GrudgerAgent {
    fn bet(&self) -> Bet {
        if self.is_disappointed {
            Bet::Cheat
        } else {
            Bet::Cooperate
        }
    }
    fn update(&mut self, points: i32, _: Bet) {
        if points == -1 {
            self.is_disappointed = true;
        }
        self.score += points;
    }
    fn score(&self) -> i32 {
        self.score
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct CopycatAgent {
    score: i32,
    last_opponent_action: Bet,
}

impl Default for CopycatAgent {
    fn default() -> Self {
        CopycatAgent { score: 0, last_opponent_action: Bet::Cooperate }
    }
}

impl TrustAgent for CopycatAgent {
    fn bet(&self) -> Bet {
        self.last_opponent_action
    }
    fn update(&mut self, points: i32, opponent_action: Bet) {
        self.score += points;
        self.last_opponent_action = opponent_action;
    }
    fn score(&self) -> i32 {
        self.score
    }
}


////////////////////////////////////////////////////////////////////////////////

pub struct DetectiveAgent {
    score: i32,
    current_round: usize,
    last_opponent_action: Bet,
    opponent_cheated: bool,
}

impl Default for DetectiveAgent {
    fn default() -> Self {
        DetectiveAgent { score: 0, current_round: 0, last_opponent_action: Bet::Cooperate, opponent_cheated: false }
    }
}

impl TrustAgent for DetectiveAgent {
    fn bet(&self) -> Bet {
        if self.current_round == 0 {
            Bet::Cooperate
        } else if self.current_round == 1 {
            Bet::Cheat
        } else if self.current_round < 4 {
            Bet::Cooperate
        } else {
            if self.opponent_cheated {
                self.last_opponent_action
            } else {
                Bet::Cheat
            }
        }
    }
    fn update(&mut self, points: i32, opponent_action: Bet) {
        self.score += points;
        self.current_round += 1;
        self.last_opponent_action = opponent_action;
        if self.current_round < 4 && opponent_action == Bet::Cheat {
            self.opponent_cheated = true;
        }
    }
    fn score(&self) -> i32 {
        self.score
    }
}

// TODO: your code goes here.
