extern crate automata_macro;
pub use automata_macro::*;

pub trait Automata<'a> {
  type Input;
  type Output;

  fn output_table(state: &Self, input: &Self::Input) -> Self::Output;

  fn transition_table(state: &mut Self, input: &Self::Input);

  fn transition(&'a mut self, x: Self::Input) -> Self::Output {
    Self::transition_table(self, &x);

    Self::output_table(self, &x)
  }
}