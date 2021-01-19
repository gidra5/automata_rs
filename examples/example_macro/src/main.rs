extern crate automata;
use automata::*;

// enum MyAutomata {
//     State1,
//     State2{ val: u8 },
//     State3(u8),
// }

// impl Automata<'_> for MyAutomata {
//     type Input = u8;
//     type Output = u8;

//     fn output_table(state: &Self, input: &Self::Input) -> Self::Output {
//         match (state, input) {
//             _ => 0
//         }
//     }

//     fn transition_table(state: &mut Self, input: &Self::Input) {
//         *state = match (&state, input) {
//             _ => Self::State1
//         }
//     }
// }

// automata!{
//     MyAutomata {
//         state: enum {
//             State1,
//             State2{ val: u8 },
//             State3(u8),
//         },
//         input_type: u8,
//         output_type: u8,
//         output_table {
//             _ => 0
//         },
//         transition_table {
//             _ => Self::State::State1
//         }
//     }
// }

fn main() {
    // let mut automata = MyAutomata::new(MyAutomata::State::State2{ val: 1 });

    // automata.transition(2);
}