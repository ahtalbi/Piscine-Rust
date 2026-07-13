use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
    Positive,
    Negative,
}

#[derive(Clone, PartialEq, Eq, PartialOrd)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

use std::cmp::{ Ord, Ordering };

impl FromStr for Antigen {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Antigen::A),
            "AB" => Ok(Antigen::AB),
            "B" => Ok(Antigen::B),
            "O" => Ok(Antigen::O), // not Ordering
            _ => Err("Invalid antigen type".to_string()),
        }
    }
}

impl FromStr for RhFactor {
    type Err = String;

    fn from_str(s: &str) -> Result<RhFactor, Self::Err> {
        match s {
            "-" => Ok(RhFactor::Negative),
            "+" => Ok(RhFactor::Positive),
            _ => Err("Invalid type".to_string()),
        }
    }
}

impl Ord for BloodType {
    fn cmp(&self, other: &Self) -> Ordering {
        let antigen_order = self.antigen.cmp(&other.antigen);
        if antigen_order != Ordering::Equal {
            return antigen_order;
        }

        self.rh_factor.cmp(&other.rh_factor)
    }
}

impl FromStr for BloodType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "O-" => Ok(BloodType { antigen: Antigen::O, rh_factor: RhFactor::Negative }),
            "O+" => Ok(BloodType { antigen: Antigen::O, rh_factor: RhFactor::Positive }),
            "A-" => Ok(BloodType { antigen: Antigen::A, rh_factor: RhFactor::Negative }),
            "A+" => Ok(BloodType { antigen: Antigen::A, rh_factor: RhFactor::Positive }),
            "B-" => Ok(BloodType { antigen: Antigen::B, rh_factor: RhFactor::Negative }),
            "B+" => Ok(BloodType { antigen: Antigen::B, rh_factor: RhFactor::Positive }),
            "AB-" => Ok(BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Negative }),
            "AB+" => Ok(BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Positive }),
            _ => Err("Invalid blood type".to_string()),
        }
    }
}
use std::fmt::Formatter;
use std::fmt::{ self, Debug };

impl Debug for BloodType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut s: String = String::new();
        s.push_str(match self.antigen {
            Antigen::AB => "AB",
            Antigen::B => "B",
            Antigen::A => "A",
            Antigen::O => "O",
        });
        s.push(match self.rh_factor {
            RhFactor::Positive => '+',
            RhFactor::Negative => '-',
        });
        write!(f, "{}", s)
    }
}

const ORDERED_BLOOD: [BloodType; 8] = [
    BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Positive },
    BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Negative },
    BloodType { antigen: Antigen::O, rh_factor: RhFactor::Positive },
    BloodType { antigen: Antigen::O, rh_factor: RhFactor::Negative },
    BloodType { antigen: Antigen::A, rh_factor: RhFactor::Positive },
    BloodType { antigen: Antigen::A, rh_factor: RhFactor::Negative },
    BloodType { antigen: Antigen::B, rh_factor: RhFactor::Positive },
    BloodType { antigen: Antigen::B, rh_factor: RhFactor::Negative },
];

impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        if self.rh_factor == RhFactor::Negative && other.rh_factor == RhFactor::Positive {
            return false;
        }

        match self.antigen {
            Antigen::O => other.antigen == Antigen::O,
            Antigen::A => other.antigen == Antigen::A || other.antigen == Antigen::O,
            Antigen::B => other.antigen == Antigen::B || other.antigen == Antigen::O,
            Antigen::AB => true,
        }
    }

    pub fn donors(&self) -> Vec<Self> {
        ORDERED_BLOOD.iter()
            .cloned()
            .filter(|donor| self.can_receive_from(donor))
            .collect()
    }

    pub fn recipients(&self) -> Vec<Self> {
        ORDERED_BLOOD.iter()
            .cloned()
            .filter(|recipient| recipient.can_receive_from(self))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let blood_type = "O+".parse::<BloodType>().unwrap();
	    println!("recipients of O+ {:?}", blood_type.recipients());
	    println!("donors of O+ {:?}", blood_type.donors());

	    let another_blood_type = "A-".parse::<BloodType>().unwrap();
	    println!(
	    	"donors of O+ can receive from {:?} {:?}",
	    	another_blood_type,
	    	blood_type.can_receive_from(&another_blood_type)
	    );
    }
}
