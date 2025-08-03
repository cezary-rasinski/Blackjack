use super::deck::{Deck, Card};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Hand {
    pub cards: Vec<Card>,
    total_val: u8,
}

impl Hand {
    pub fn new(deck: &mut Deck) -> Self {
        let mut cards: Vec<Card> = Vec::new();
        let mut total_val = 0;
        for _i in 0..2 {
            let card = deck.cards.pop().unwrap();
            cards.push(card);
            total_val += card.card_value();
        }
        return Hand { cards, total_val };
    }

    pub fn draw(&mut self, deck: &mut Deck) {
        let card = deck.cards.pop().unwrap();
        self.cards.push(card);
        self.total_val = self.total_val();
    }
    pub fn total_val(&self) -> u8 {
        let mut sum = 0;
        let mut aces = 0;
        for card in &self.cards {
            if card.card_value() != 11 {
                sum += card.card_value();
            } else {
                aces += 1;
                sum += 11;
            }
        }
        while sum > 21 && aces > 0 {
            sum -= 10;
            aces -= 1;
        }
        return sum;
    }
    pub fn see_hand(&self) {
        println!("\t\t---Your Hand---");

        for card in &self.cards {
            println!("\t{:?}", card);
        }

        println!("\tTotal Value: {:?}", self.total_val);
    }
}