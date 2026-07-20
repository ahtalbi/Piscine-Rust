#[derive(Clone, Debug, PartialEq)]
pub struct List<T> {
    pub head: Option<Node<T>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T: std::clone::Clone> List<T> {
    pub fn new() -> Self {
        Self {
            head: None,
        }
    }

    pub fn push(&mut self, value: T) {
        let h : Option<Node<T>> = self.head.take();
        self.head = Some(Node{
            value: value,
            next: h.map(Box::new),
        });
    }

    pub fn pop(&mut self) {
        if let Some(node) = self.head.take() {
            if let Some(next) = node.next {
                self.head = Some(*next);
            } else {
                self.head = None;
            }
        }
    }

    pub fn len(&self) -> usize {
        let mut res : usize = 0;
        let mut c = self.head.as_ref();
        while let Some(node) = c {
            res += 1;
            c = c.unwrap().next.as_deref();
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
        println!("{:?}", new_list_str);
        println!("The size of the list is {}", new_list_str.len());

        new_list_str.push("String Test 2");
        println!("The size of the list is {}", new_list_str.len());

        new_list_str.push("String Test 3");
        println!("The size of the list is {}", new_list_str.len());

        new_list_str.pop();
        println!("The size of the list is {}", new_list_str.len());
    }
}
