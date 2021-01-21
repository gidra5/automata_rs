extern crate automata;
use automata::*;

automata!{
    MyAutomata {
        state: enum {
            State1,
            State2{ val: u8 },
            State3(u8),
        } 
        output_table: {
            match (state, input) {
                _ => 0
            }
        } 
        input: u8 
        output: u8 
        transition_table: {
            match (state, input) {
                _ => Self::State1
            }
        }
    }
}

fn main() {
    let mut automata = MyAutomata::State2{ val: 1 };

    automata.transition(2);
}