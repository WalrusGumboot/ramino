//! Everything that has to do with runs of cards. This includes the Run enum,
//! the RunCoercionStrategy struct and the verify_run function.

use crate::card::{Card, Suit};

/// A Run of cards describes a sequence of cards as it could appear on the table.
///
/// Note that upon creation, this can be an invalid sequence (e.g. [♤2, ♤3, ♤5]).
/// The verify function needs to be called to ensure validity.
#[derive(PartialEq, Eq, Debug)]
pub enum Run {
    /// An Ascending run is one that takes cards of the same suit but sequentially higher cards.
    Ascending(Vec<Card>),
    /// An Equal run is one that takes (at most four) cards of the same value but differing suits.
    Equal(Vec<Card>)
}

/// A struct to determine how Runs should be coerced.
///
/// ### Fields
/// * `prefer_ascending: bool` - determines whether to fit the run ascendingly
/// * `highest_possible: bool` - tries to fit the highest possible score
/// * `suit_preference: [Suit, 4]` - order of preference for suits, when applicable
///
/// ### Examples
///
/// **For `prefer_ascending = true`:**
///
/// [JOKER, ♤3, ♤4] → [♤2, ♤3, ♤4] when `highest_possible` is set to false.
/// [JOKER, ♤3, ♤4] → [♤3, ♤4, ♤5] when `highest_possible` is set to true.
///
/// **With `prefer_ascending = false` and `suit_preference = [Clubs, Diamonds, Spades, Hearts]`:**
///
/// [JOKER, ♧Q, ♡Q] → [♢Q, ♧Q, ♡Q]
pub struct RunCoercionStrategy {
    prefer_ascending: bool,
    highest_possible: bool,
    suit_preference: [Suit; 4]
}

impl Run {
    /// Calculates the score that this run stands for, taking jokers into account.
    pub fn get_score(&self) -> u8 {
        unimplemented!("Calculating run score is unimplemented");
    }

    /// Returns a new Run identical to `self`, but with all jokers replaced by
    /// the cards they actually stand for according to the given strategy.
    pub fn coerce_to_real(&self, strategy: RunCoercionStrategy) -> Run {
        unimplemented!("Run coercion isn't yet implemented");
    }
}


/// A function used to either construct a Run instance from the given cards,
/// or return an Error if this isn't possible. This is the only way to directly
/// create Runs.
pub fn verify_run(mut cards: Vec<Card>) -> Result<Run, ()> {
    assert!(cards.len() >= 3, "A run must consist of at least three cards.");
    let mut dedupped = cards.clone(); dedupped.dedup();
    assert!(dedupped == cards, "A run cannot contain duplicate cards.");

    // The first check is trivial: checking if all card types are the same and all suits are different.
    if cards.iter().all(|&c| c.card_type == cards[0].card_type) {
        let mut suits_seen_so_far: Vec<Suit> = Vec::new();
        for c in cards.iter() {
            if suits_seen_so_far.contains(&c.suit) { break; }
            else { suits_seen_so_far.push(c.suit); }
        }

        return Ok(Run::Equal(cards))
    }

    // If that check failed, the series of sorted cards may only have a maximum distance of one.
    cards.sort();
    if cards.windows(2).fold(0u8, |acc, cards| acc + (cards[0].get_distance(&cards[1]))) > cards.len() as u8 {
        return Err(());
    }

    return Ok(Run::Ascending(cards));
}
