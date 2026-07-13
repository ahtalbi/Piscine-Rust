use std::fmt;

#[derive(Debug)]
pub struct Player<'a> {
	pub name: &'a str,
	pub strength: f64,
	pub score: u32,
	pub money: u32,
	pub weapons: Vec<&'a str>,
}

pub struct Fruit {
	pub weight_in_kg: f64,
}

pub struct Meat {
	pub weight_in_kg: f64,
	pub fat_content: f64,
}

impl Player<'_> {
	pub fn eat(&mut self, food: impl Food) {
		self.strength += food.gives();
	}
}

impl fmt::Display for Player<'_> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		writeln!(f, "{}", self.name);
		writeln!(f, "Strength: {}, Score: {}, Money: {}", self.strength, self.score, self.money);
		writeln!(f, "Weapons: {:?}", self.weapons);
        Ok(())
    }
}

pub trait Food {
	fn gives(&self) -> f64;
}

impl Food for Fruit {
	fn gives(&self) -> f64 {
		self.weight_in_kg * 4.0
	}
}

impl Food for Meat {
	fn gives(&self) -> f64 {
        let mut fat = self.fat_content;
		let protein = self.weight_in_kg - fat;
        (protein * 4.0) + (fat * 9.0)
	}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
	    let apple = Fruit { weight_in_kg: 1.0 };

	    println!("this apple gives {} units of strength", apple.gives());

	    let steak = Meat {
	    	weight_in_kg: 1.0,
	    	fat_content: 1.0,
	    };

	    let mut player1 = Player {
	    	name: "player1",
	    	strength: 1.0,
	    	score: 0,
	    	money: 0,
	    	weapons: vec!["knife"],
	    };

	    println!("Before eating {:?}", player1);

	    player1.eat(apple);
	    println!("After eating an apple\n{}", player1);

	    player1.eat(steak);
	    println!("After eating a steak\n{}", player1);
    }
}
