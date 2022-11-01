//! The card module contains all stuff related to cards.
//! This entails: the CardType, Suit and CardOrdering enums and the Card struct.

use std::cmp::Ordering;

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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[allow(missing_docs)]
pub enum Suit {
    Spades,
    Diamonds,
    Clubs,
    Hearts,
    JokerSuit
}

/// The main Card struct.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub struct Card {
    pub card_type: CardType,
    pub suit: Suit
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.suit == other.suit {
            match self.compare(other) {
                CardOrdering::WellDefined(o) => return Some(o),
                CardOrdering::IllDefined     => return None
            }
        } else {
            return Some(self.suit.cmp(&other.suit));
        }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Less)
    }
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

impl From<&str> for Card {
    fn from(val: &str) -> Self {
        assert!(val.len() <= 2, "cannot construct card from string with length bigger than 2");
        let mut chars = val.chars();
        let suit = match chars.next().unwrap() {
            'H' => Suit::Hearts,
            'C' => Suit::Clubs,
            'D' => Suit::Diamonds,
            'S' => Suit::Spades,
            'J' => Suit::JokerSuit,
            _   => panic!("encountered invalid character in getting suit for card")
        };
        let card_type = match chars.next().unwrap_or('?') {
            'A' => CardType::Ace,
            'J' => CardType::Jack,
            'Q' => CardType::Queen,
            'K' => CardType::King,
            '2' => CardType::Number(2),
            '3' => CardType::Number(3),
            '4' => CardType::Number(4),
            '5' => CardType::Number(5),
            '6' => CardType::Number(6),
            '7' => CardType::Number(7),
            '8' => CardType::Number(8),
            '9' => CardType::Number(9),
            'X' => CardType::Number(10),
            '?' => CardType::Joker,
            _ => panic!("encountered invalid character in getting type for card")
        };

        Card::new(card_type, suit)
    }
}

impl Card {
    /// Creates a new card. Just your standard old ::new() function.
    pub fn new(card_type: CardType, suit: Suit) -> Self {
        Card { card_type, suit }
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

    /// Gets the raw comparison value to test against.
    pub fn get_comparison_value(&self) -> u8 {
        match self.card_type {
            CardType::Ace       => 1,
            CardType::Number(n) => n,
            CardType::Jack      => 11,
            CardType::Queen     => 12,
            CardType::King      => 13,
            CardType::Joker     => 99 /* { return CardOrdering::IllDefined } */
        }
    }

    /// Gets the distance between two cards, accounting for the fact that
    /// aces and kings are also adjacent.
    ///
    /// ## Notes
    /// This function is always used in a sorted iterator, with self being the smaller.
    pub fn get_distance(&self, other: &Self) -> u8 {
        let self_val = self.get_comparison_value();
        let other_val = other.get_comparison_value();

        // this is in practice always used in sorted runs,
        // so other_val will always be a greater card.
        if self_val == 1 && other_val == 13 { 1 }
        else { other_val - self_val }
    }

    /// Compares two cards and returns a CardOrdering.
    pub fn compare(&self, other: &Self) -> CardOrdering {

        if self.suit == other.suit {
            let self_val = self.get_comparison_value();
            let other_val = other.get_comparison_value();

            if self_val == 1 && other_val == 13 {
                return CardOrdering::WellDefined(Ordering::Less);
            }

            if self_val == 13 && other_val == 1 {
                return CardOrdering::WellDefined(Ordering::Greater);
            }

            if self_val == 99 || other_val == 99 { return CardOrdering::IllDefined };
            return CardOrdering::WellDefined(self_val.cmp(&other_val));
        }

        return CardOrdering::IllDefined; // cannot *really* compare two cards of different suits

    }
}
