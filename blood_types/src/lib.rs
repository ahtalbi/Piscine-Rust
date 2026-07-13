use std::{fmt, str::FromStr};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum RhFactor {
    Positive,
    Negative,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}


impl FromStr for BloodType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A+" => Ok(BloodType {
                antigen: Antigen::A,
                rh_factor: RhFactor::Positive,
            }),
            "A-" => Ok(BloodType {
                antigen: Antigen::A,
                rh_factor: RhFactor::Negative,
            }),
            "B+" => Ok(BloodType {
                antigen: Antigen::B,
                rh_factor: RhFactor::Positive,
            }),
            "B-" => Ok(BloodType {
                antigen: Antigen::B,
                rh_factor: RhFactor::Negative,
            }),
            "AB+" => Ok(BloodType {
                antigen: Antigen::AB,
                rh_factor: RhFactor::Positive,
            }),
            "AB-" => Ok(BloodType {
                antigen: Antigen::AB,
                rh_factor: RhFactor::Negative,
            }),
            "O+" => Ok(BloodType {
                antigen: Antigen::O,
                rh_factor: RhFactor::Positive,
            }),
            "O-" => Ok(BloodType {
                antigen: Antigen::O,
                rh_factor: RhFactor::Negative,
            }),
            _ => Err(()),
        }
    }
}


impl fmt::Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let antigen = match self.antigen {
            Antigen::A => "A",
            Antigen::B => "B",
            Antigen::AB => "AB",
            Antigen::O => "O",
        };

        let rh = match self.rh_factor {
            RhFactor::Positive => "+",
            RhFactor::Negative => "-",
        };

        write!(f, "{}{}", antigen, rh)
    }
}

impl BloodType {
    pub fn can_receive_from(self, other: Self) -> bool {
        let antigen_ok = match self.antigen {
            Antigen::O => other.antigen == Antigen::O,

            Antigen::A => other.antigen == Antigen::A || other.antigen == Antigen::O,

            Antigen::B => other.antigen == Antigen::B || other.antigen == Antigen::O,

            Antigen::AB => true,
        };

        let rh_ok = match self.rh_factor {
            RhFactor::Positive => true,
            RhFactor::Negative => other.rh_factor == RhFactor::Negative,
        };

        antigen_ok && rh_ok
    }

    pub fn donors(self) -> Vec<Self> {
        let all = vec![
            "A+".parse().unwrap(),
            "A-".parse().unwrap(),
            "B+".parse().unwrap(),
            "B-".parse().unwrap(),
            "AB+".parse().unwrap(),
            "AB-".parse().unwrap(),
            "O+".parse().unwrap(),
            "O-".parse().unwrap(),
        ];

        all.into_iter()
            .filter(|b| self.can_receive_from(*b))
            .collect()
    }

    pub fn recipients(self) -> Vec<Self> {
        let all: Vec<BloodType> = vec![
            "A+".parse().unwrap(),
            "A-".parse().unwrap(),
            "B+".parse().unwrap(),
            "B-".parse().unwrap(),
            "AB+".parse().unwrap(),
            "AB-".parse().unwrap(),
            "O+".parse().unwrap(),
            "O-".parse().unwrap(),
        ];

        all.into_iter()
            .filter(|b| b.can_receive_from(self))
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
	    	blood_type.can_receive_from(another_blood_type)
	    );
    }
}
