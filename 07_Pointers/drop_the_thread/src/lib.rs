use std::cell::{Cell, RefCell};
use std::rc::Rc;

#[derive(Debug)]
pub struct ThreadPool {
    pub drops: Cell<usize>,
    pub states: RefCell<Vec<bool>>,
}

impl ThreadPool {
    pub fn new() -> Self {
        Self {
            drops: Cell::new(0),
            states: RefCell::new(Vec::new()),
        }
    }

    pub fn new_thread(&self, c: String) -> (usize, Thread) {
        let id = self.thread_len();
        self.states.borrow_mut().push(false);
        (id, Thread::new(id, c, self))
    }

    pub fn thread_len(&self) -> usize {
        self.states.borrow().len()
    }

    pub fn is_dropped(&self, id: usize) -> bool {
        self.states.borrow().get(id).copied().unwrap_or(false)
    }

    pub fn drop_thread(&self, id: usize) {
        if self.is_dropped(id) {
            panic!("{} is already dropped", id);
        }

        self.states.borrow_mut()[id] = true;
        self.drops.set(self.drops.get() + 1);
    }
}

#[derive(Debug)]
pub struct Thread<'a> {
    pub pid: usize,
    pub cmd: String,
    pub parent: &'a ThreadPool,
}

impl<'a> Thread<'a> {
    pub fn new(p: usize, c: String, t: &'a ThreadPool) -> Self {
        Self {
            pid: p,
            cmd: c,
            parent: t,
        }
    }

    pub fn skill(self) {
        drop(self)
    }
}

impl Drop for Thread<'_> {
    fn drop(&mut self) {
        self.parent.drop_thread(self.pid);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let pool = ThreadPool::new();
        let (id, thread) = pool.new_thread(String::from("command"));
        let (id1, thread1) = pool.new_thread(String::from("command1"));

        thread.skill();

        println!("{:?}", (pool.is_dropped(id), id, &pool.drops));

        thread1.skill();
        println!("{:?}", (pool.is_dropped(id1), id1, &pool.drops));

        let (id2, thread2) = pool.new_thread(String::from("command2"));
        let thread2 = Rc::new(thread2);
        let thread2_clone = thread2.clone();

        drop(thread2_clone);

        println!("{:?}", (pool.is_dropped(id2), id2, &pool.drops, Rc::strong_count(&thread2)));
    }
}
