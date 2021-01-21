extern crate automata;
use automata::*;

enum MyAutomata {
    State1,
    State2{ val: u8 },
    State3(u8),
}

impl Automata<'_> for MyAutomata {
    type Input = u8;
    type Output = u8;

    fn output_table(state: &Self, input: &Self::Input) -> Self::Output {
        match (state, input) {
            _ => 0
        }
    }

    fn transition_table(state: &Self, input: &Self::Input) -> Self {
        match (state, input) {
            _ => Self::State1
        }
    }
}

fn main() {
    let mut automata = MyAutomata::State2{ val: 1 };

    automata.transition(2);
}