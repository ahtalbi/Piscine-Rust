#[derive(Debug, PartialEq)]
pub struct Member {
    pub role: Role,
    pub age: u32,
}

#[derive(Debug, PartialEq)]
pub enum Role {
    Underboss,
    Caporegime,
    Soldier,
    Associate,
}

impl Role {
    pub fn get_power(&self) -> usize {
        match self {
            Role::Associate => 1,
            Role::Soldier => 2,
            Role::Caporegime => 3,
            Role::Underboss => 4,
        }
    }
}

impl Member {
    pub fn new(age: u32) -> Member {
        Member { role: Role::Associate, age }
    }

    pub fn get_promotion(&mut self) {
        match self.role {
            Role::Associate => self.role = Role::Soldier,
            Role::Soldier => self.role = Role::Caporegime,
            Role::Caporegime => self.role = Role::Underboss,
            _ => panic!("Impossibility"),
        }
    }
}