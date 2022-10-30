#![deny(missing_docs)]

//! ramino is a Rust engine for the traditional Italian card game Ramino.
//! It can be used both as a referee for the rules and an emulator for a game.


extern crate rand;
use std::cmp::Ordering;

use rand::{thread_rng, seq::SliceRandom};


/// The amount of cards dealt to create a Hand at the beginning of the game.
pub const HAND_SIZE: u8 = 13;


/// An enum to represent the type of a card.
///
/// The Number(u8) variant can only hold values between 2 and 10, since the ace
/// is accounted for separately.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum CardType {
    Number(u8), //TODO: make invalid state unrepresentable
    Jack,
    Queen,
    King,
    Ace,
    Joker
}

/// An enum to represent the suit of a card.
///
/// An exception is made for jokers, since they don't strictly have a suit, but
/// making the suit field on a struct an Option would be hell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
    JokerSuit
}

/// The main Card struct.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Card {
    card_type: CardType,
    suit: Suit
}

/// An enum to help with validating card runs.
///
/// The WellDefined variant contains an Ordering (from std::cmp) which makes for
/// sensible use cases. The IllDefined variant serves as a signifier for "hey,
/// this card ordering is kinda funky and will need some more fletching out" but
/// since the vast majority of card pairs don't fall into this category, the
/// enum is split up like this.
#[derive(Debug, Clone, Copy)]
pub enum CardOrdering {
    /// The WellDefined variant contains an Ordering which is used as per standard.
    WellDefined(Ordering),
    /// The IllDefined variant handles all 'edge casey' comparisons. This
    /// includes aces to twos and kings and jokers to any other cards.
    IllDefined
}

impl Card {
    /// Creates a new card. Just your standard old ::new() function.
    pub fn new(card_type: CardType, suit: Suit) -> Self {
        Card { card_type, suit }
    }

    /// Formats the card as plain ASCII text.
    pub fn format_simple(&self) -> String {
        if self.card_type == CardType::Joker { return String::from("Joker") }

        let suit = String::from(match self.suit {
            Suit::Spades    => "Spades",
            Suit::Hearts    => "Hearts",
            Suit::Diamonds  => "Diamonds",
            Suit::Clubs     => "Clubs",
            Suit::JokerSuit => unreachable!()
        });

        let value = String::from(match self.card_type {
            CardType::Ace => "Ace",
            CardType::Jack => "Jack",
            CardType::Queen => "Queen",
            CardType::King => "King",
            CardType::Joker => unreachable!(),
            CardType::Number(0 | 1) => unreachable!(),
            CardType::Number(_n) => unimplemented!() // this will match cards higher than 10! TODO: make invalid state unrepresentable
        });

        format!("{} of {}", value, suit)
    }

    /// Calculates the score for a single card. An ace is counted as 11 by
    /// default, since the "an ace counts as one if it's the only card left"
    /// rule is accounted for in the synonymous function on Hand.
    ///
    /// A numbered card is counted as its value, jokers are twenty-five, and
    /// jacks, queens and kings are all counted as ten.
    pub fn score(&self) -> u8 {
        match self.card_type {
            CardType::Number(n) => n,
            CardType::Ace => 11, // the singular ace amounting to one is accounted for in Hand
            CardType::Joker => 25,
            _ => 10
        }
    }

    /// Compares two cards and returns a CardOrdering.
    pub fn compare(&self, _other: &Self) -> CardOrdering {

        match self.card_type {
            CardType::Ace       => 1,
            CardType::Number(n) => n,
            CardType::Jack      => 11,
            CardType::Queen     => 12,
            CardType::King      => 13,
            CardType::Joker     => 99 /* { return CardOrdering::IllDefined } */
        };

        unimplemented!()
    }
}

/// A struct that represents a series of cards that a player holds.
///
/// Most functions on Hand mutate either a deck (`Vec<Card>`) or a Table instance.
pub struct Hand(Vec<Card>);
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

/// Generates a deck of 52 normal cards and 2 jokers in standard order.
///
/// If shuffled is true, shuffles the deck before returning it.
pub fn generate_single_deck(shuffled: bool) -> Vec<Card> {
    use self::{Suit::*, CardType::*};

    let mut deck: Vec<Card> = Vec::new();

    for suit in [Spades, Hearts, Diamonds, Clubs] {
        deck.push(Card::new(Ace, suit));
        deck.push(Card::new(King, suit));
        deck.push(Card::new(Queen, suit));
        deck.push(Card::new(Jack, suit));
        for i in 2..11 { deck.push(Card::new(Number(i), suit)); }
    }

    for _ in 0..2 { deck.push(Card::new(Joker, JokerSuit)) }

    if shuffled {
        deck.shuffle(&mut thread_rng());
    }

    deck
}

/// Generates the full playing deck, consisting of two normal decks.
pub fn generate_deck(shuffled: bool) -> Vec<Card> {
    let mut deck = generate_single_deck(shuffled);
    deck.append(&mut generate_single_deck(shuffled));

    deck
}

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
pub fn verify_run(cards: Vec<Card>) -> Result<Run, ()> {
    assert!(cards.len() >= 3, "A run must consist of at least three cards.");
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::{*, CardType::*, Suit::*, Run::*, RunCoercionStrategy};

    #[test]
    fn get_deck() {
        let deck = generate_single_deck(false);
        assert_eq!(deck.len(), 54);
    }

    #[test]
    fn shuffled_deck() {
        let deck = generate_single_deck(true);
        println!("{:?}", deck[0]);

        // this test always passes, but it can be
        // individually verified that it does, in fact, function
    }

    #[test]
    fn full_deck() {
        let deck = generate_deck(false);
        assert_eq!(deck.len(), 108);
    }

    #[test]
    fn generate_hand() {
        let mut deck = generate_deck(true);
        let _hand = Hand::draw(&mut deck);
    }

    #[test]
    fn hand_score() {
        let hand0 = Hand(vec![]);
        assert_eq!(hand0.score(), 0);

        let hand1 = Hand(vec![Card::new(Ace, Spades)]);
        assert_eq!(hand1.score(), 1);

        let hand2 = Hand(vec![Card::new(King, Spades), Card::new(Queen, Hearts), Card::new(Jack, Diamonds)]);
        assert_eq!(hand2.score(), 30);

        let hand3 = Hand(vec![Card::new(Number(8), Clubs), Card::new(Number(9), Spades), Card::new(Joker, JokerSuit)]);
        assert_eq!(hand3.score(), 42);
    }

    #[test]
    fn verify_runs() {
        let test_cards1 = vec![Card::new(Joker, JokerSuit), Card::new(Queen, Spades), Card::new(King, Spades)];
        assert_eq!(verify_run(test_cards1.clone()),
                   Ok(Run::Equal(test_cards1.clone())));
    }
}
