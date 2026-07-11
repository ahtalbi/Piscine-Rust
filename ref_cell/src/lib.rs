use std::rc::Rc;
use std::borrow::Borrow;
use std::cell::RefCell;

pub struct Tracker {
    pub messages: RefCell<Vec<String>>,
    pub value: RefCell<usize>,
    pub max: usize,
}

impl Tracker {
    pub fn new(max: usize) -> Self {
        Self {
            messages: Vec::new().into(),
            value: 0.into(),
            max: max,
        }
    }

    pub fn set_value(&self, value: &Rc<usize>) {
        let refs = Rc::strong_count(value);
        *self.value.borrow_mut() = refs;
        if refs > self.max {
            self.messages.borrow_mut().push("Error: You can't go over your quota!".to_string());
        } else if refs >= self.max * 70 / 100 {
            self.messages.borrow_mut().push("Warning: You have used up over 70% of your quota!".to_string());
        }
    }

    fn peek(&self, value: &Rc<usize>) {
        let refs = Rc::strong_count(value);
        let p = refs * 100 / self.max;
        self.messages.borrow_mut().push(format!("Info: This value would use {}% of your quota", p));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let v = Rc::new(1); // we have one reference to this Rc

        // initialize the tracker, with the max number of
        // called references as 10
        let track = Tracker::new(10);

        let _v = Rc::clone(&v); // |\
        let _v = Rc::clone(&v); // | -> increase the Rc to 4 references
        let _v = Rc::clone(&v); // |/

        // take a peek of how much we already used from our quota
        track.peek(&v);

        let _v = Rc::clone(&v); // |\
        let _v = Rc::clone(&v); // |  -> increase the Rc to 8 references
        let _v = Rc::clone(&v); // | /
        let _v = Rc::clone(&v); // |/

        // this will change the tracker's inner value
        // and make a verification of how much we already used of our quota
        track.set_value(&v);

        let _v = Rc::clone(&v); // increase the Rc to 9 references
        let _v = Rc::clone(&v); // increase the Rc to 10 references, the maximum we allow

        track.set_value(&v);

        let _v = Rc::clone(&v); // surpass the maximum allowed references

        track.peek(&v);
        track.set_value(&v);

        track.messages.borrow().iter().for_each(|msg| println!("{}", msg));
    }
}
