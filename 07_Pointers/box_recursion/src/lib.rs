#[derive(Debug, PartialEq, Clone)]
pub enum Role {
    CEO,
    Manager,
    Worker,
}

impl From<&str> for Role {
    fn from(value: &str) -> Self {
        match value {
            "CEO" => Role::CEO,
            "Manager" => Role::Manager,
            _ => Role::Worker,
        }
    }
}

#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link = Option<Box<Worker>>;

#[derive(Debug, PartialEq)]
pub struct Worker {
    pub role: Role,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    pub fn new() -> Self {
        Self {
            grade: None,
        }
    }

    pub fn add_worker(&mut self, name: &str, role: &str) {
        self.grade = Some(Box::new(Worker {
            role: Role::from(role),
            name: name.to_string(),
            next: self.grade.take(),
        }));
    }

    pub fn last_worker(&self) -> Option<(String, Role)> {
        match &self.grade {
            Some(w) => Some((w.name.clone(), w.role.clone())),
            None => None,
        }
    }

    pub fn remove_worker(&mut self) -> Option<(String, Role)> {
        let f = self.grade.take()?;
        self.grade = f.next;
        Some((f.name, f.role))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut list = WorkEnvironment::new();

        list.add_worker("Marie", "CEO");
        list.add_worker("Monica", "Manager");
        list.add_worker("Ana", "Normal Worker");
        list.add_worker("Alice", "Normal Worker");

        println!("{:#?}", list);

        // println!("{:?}", list.last_worker());
        list.remove_worker();
        list.remove_worker();
        list.remove_worker();

        println!("{:?}", list);

        list.remove_worker();

        println!("{:?}", list);
    }
}
