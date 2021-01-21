extern crate automata_macro;
pub use automata_macro::*;

pub trait Automata<'a> {
  type Input;
  type Output;

  fn output_table(state: &Self, input: &Self::Input) -> Self::Output;

  fn transition_table(state: &Self, input: &Self::Input) -> Self;

  fn transition(&'a mut self, x: Self::Input) -> Self::Output where Self: Sized {
    *self = Self::transition_table(self, &x);

    Self::output_table(self, &x)
  }
}