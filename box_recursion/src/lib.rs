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
        let mut c = &mut self.grade;
        if c.is_none() {*c = Some(Box::new(Worker {
            role: Role::from(role),
            name: name.to_string(),
            next: None,
        }));
        return;}
        while let Some(worker) = c {
            if worker.next == None {
                worker.next = Some(Box::new(Worker {
                    role: Role::from(role),
                    name: name.to_string(),
                    next: None,
                }));
                break;
            }
        
            c = &mut worker.next;
        }
    }

    pub fn remove_worker(&mut self) -> Option<String> {
        let mut c = &mut self.grade;
        let mut name : String = String::new();
        loop {
            if c.is_none() {
                break;
            }
            if c.as_ref().unwrap().next.is_none() {
                name = c.as_ref().unwrap().name.clone();
                break;
            }
        
            c = &mut c.as_mut().unwrap().next;
        }
        *c = None;
        Some("".to_string())
    }

    pub fn last_worker(&self) -> Option<(String, Role)> {
        println!("{:#?}", &self);
        let mut c = &self.grade;
        while let Some(worker) = c {
            if worker.next == None {
                return Some((worker.name.clone(), worker.role.clone()));
            }
            c = &worker.next;
        }
        None
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
