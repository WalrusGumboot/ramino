//! Tbh I only put this in a separate module because it didn't fit anywhere else.

use crate::HAND_SIZE;
use crate::card::{Card, CardType};

/// A struct that represents a series of cards that a player holds.
///
/// Most functions on Hand mutate either a deck (`Vec<Card>`) or a Table instance.
pub struct Hand(pub Vec<Card>);
impl Hand {
    /// Creates a new hand of thirteen cards by popping them off of a mutable deck.
    pub fn draw(deck: &mut Vec<Card>) -> Self {
        assert!(deck.len() >= HAND_SIZE.into());
        Hand((0..HAND_SIZE).map(|_| deck.pop().unwrap()).collect())
    }

    /// Calculates the total score.
    ///
    /// If this hand still contains thirteen cards by the time the function gets
    /// called, it will return 100. If the Hand holds only a single ace, it'll
    /// return 1.
    pub fn score(&self) -> u8 {
        if self.0.len() == HAND_SIZE.into() { 100 }
        else if self.0.len() == 1 && self.0[0].card_type == CardType::Ace { 1 }
        else { self.0.iter().fold(0u8, |acc, c| acc + c.score()) }
    }
}
