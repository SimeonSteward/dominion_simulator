use crate::card::Card;
use crate::kingdom::Kingdom;

use std::collections::HashMap;

pub trait CardCollectionsTrait {
    fn print_cards(&self);
}

impl CardCollectionsTrait for &Vec<&Card> {
    fn print_cards(&self) {
        for card in *self {
            print!("{}, ", card.name);
        }
    }
}
impl CardCollectionsTrait for HashMap<&Card, u16> {
    fn print_cards(&self) {
        for (card, count) in self.iter() {
            print!("{} {}s,", count, card.name);
        }
    }
}

pub fn _print_kingdom(kingdom: &Kingdom) {
    for supply_pile in &kingdom.supply_piles {
        println!(
            "Card: {}, Count: {}",
            &supply_pile.1.card.name, supply_pile.1.count
        );
    }
}
