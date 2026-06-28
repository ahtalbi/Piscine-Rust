use rand::Rng;

#[derive(Debug)]
pub enum Suit {
    Heart,
    Diamond,
    Club,
    Spade,
}

#[derive(Debug)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Suit {
    pub fn random() -> Suit {
        match rand::thread_rng().gen_range(0..4){
            0=> return Suit::Heart,
            1=> return Suit::Diamond,
            2=> return Suit::Club,
            3=> return Suit::Spade,
            _ => Suit::Spade,
        }
    }

    pub fn translate(value: u8) -> Suit {
        match value{
            0=> return Suit::Heart,
            1=> return Suit::Diamond,
            2=> return Suit::Club,
            3=> return Suit::Spade,
            _ => Suit::Spade,
        }
    }
}

impl Rank {
    pub fn random() -> Rank {
        match rand::thread_rng().gen_range(0..4){
            0=> return Rank::Ace,
            1=> return Rank::Two,
            2=> return Rank::Three,
            3=> return Rank::Four,
            4=> return Rank::Five,
            5=> return Rank::Six,
            6=> return Rank::Seven,
            7=> return Rank::Eight,
            8=> return Rank::Nine,
            9=> return Rank::Ten,
            10=> return Rank::Jack,
            11=> return Rank::Queen,
            12=> return Rank::King,
            _=> return Rank::Ace,
        }
        
    }

    pub fn translate(value: u8) -> Rank {
        match value{
            0=> return Rank::Ace,
            1=> return Rank::Two,
            2=> return Rank::Three,
            3=> return Rank::Four,
            4=> return Rank::Five,
            5=> return Rank::Six,
            6=> return Rank::Seven,
            7=> return Rank::Eight,
            8=> return Rank::Nine,
            9=> return Rank::Ten,
            10=> return Rank::Jack,
            11=> return Rank::Queen,
            12=> return Rank::King,
            _ => return Rank::Ace,
        }

    }
}

#[derive(Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    match (&card.suit, &card.rank) {
        (Suit::Spade, Rank::Ace) => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let your_card = Card {
            rank: Rank::random(),
            suit: Suit::random(),
        };

        println!("Your card is {:?}", &your_card);

        if winner_card(&your_card) {
            println!("You are the winner!");
        }
    }
}
