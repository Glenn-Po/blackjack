use rand::{seq::SliceRandom, thread_rng};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Clone, Copy, Debug, EnumIter)]
enum Suit {
    HEARTS,
    DIAMONDS,
    CLUBS,
    SPADES,
}

#[derive(Clone, Copy, Debug, EnumIter, PartialEq)]
enum Rank {
    ACE = 1,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
    TEN,
    JACK,
    QUEEN,
    KING,
}

impl Rank {
    pub fn value(&self) -> i16 {
        *self as i16
    }
}

#[derive(Clone, Copy, Debug)]
struct Card {
    suit: Suit,
    rank: Rank,
}

impl Card {
    fn new(suit: Suit, rank: Rank) -> Self {
        Self { suit, rank }
    }
}

struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn new() -> Self {
        let mut cards: Vec<Card> = Vec::new();

        for suit in Suit::iter() {
            for rank in Rank::iter() {
                cards.push(Card::new(suit.clone(), rank.clone()))
            }
        }

        Self { cards }
    }

    fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng())
    }

    fn draw_card(&mut self) -> Option<Card> {
        if self.cards.len() == 0 {
            ()
        }
        {
            self.cards.pop()
        }
    }
}

struct Player {
    hand: Vec<Card>,
}

impl Player {
    fn new(self: Self) -> Self {
        let hand = Vec::new();
        Self { hand }
    }
    fn add_card(&mut self, card: Card) {
        self.hand.push(card)
    }

    fn get_hand_value(self: Self) -> i16 {
        let mut value: i16 = 0;
        let mut aces: i16 = 0;

        for card in self.hand {
            let mut card_rank = card.rank;
            let card_value = if card_rank.value() >= 10 {
                10
            } else if card_rank == Rank::ACE {
                aces += 1;
                11
            } else {
                card_rank.value()
            };
            value += card_value;
        }

        while value > 21 && aces > 0 {
            value -= 10;
            aces -= 1;
        }
        value
    }
}
