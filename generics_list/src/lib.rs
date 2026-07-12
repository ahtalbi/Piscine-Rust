#[derive(Clone, Debug, PartialEq)]
pub struct List<T> {
    pub head: Option<Node<T>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self {
            head: None,
        }
    }

    pub fn push(&mut self, value: T) {
        if self.head.is_none() {
            self.head = Some(Node{value: value, next: None});
            return;
        }
        let mut c = self.head.as_mut();
        loop {
            match c {
                Some(node) => {
                    if node.next.is_none() {
                        node.next = Some(Box::new(Node {
                            value,
                            next: None,
                        }));
                        break;
                    }
                
                    c = node.next.as_deref_mut();
                }
                None => break,
            }
        }
    }

    pub fn pop(&mut self) {
        match self.head.as_mut() {
            None => return,
            Some(head) if head.next.is_none() => {
                self.head = None;
            },
            Some(_) => {
                let mut c = self.head.as_mut().unwrap();
                while c.next.as_ref().unwrap().next.is_some() {
                    c = c.next.as_mut().unwrap();
                }
                c.next = None;
            }
        }
    }

    pub fn len(&self) -> usize {
        let mut res : usize = 0;
        let mut c = self.head.as_ref();
        loop {
            match c {
                Some(node) => {
                    res += 1;
                    if node.next.is_none() {
                        break;
                    }
                    c = node.next.as_deref();
                },
                None => break,
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut new_list_str = List::new();
        new_list_str.push("String Test 1");
        println!("The size of the list is {}", new_list_str.len());

        new_list_str.push("String Test 2");
        println!("The size of the list is {}", new_list_str.len());

        new_list_str.push("String Test 3");
        println!("The size of the list is {}", new_list_str.len());

        new_list_str.pop();
        println!("The size of the list is {}", new_list_str.len());
    }
}
