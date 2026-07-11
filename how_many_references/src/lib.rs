use std::rc::Rc;

pub struct Node {
    pub ref_list: Vec<Rc<String>>,
}

impl Node {
    pub fn new(ref_list: Vec<Rc<String>>) -> Self {
        Self {
            ref_list: ref_list,
        }
    }

    pub fn add_element(&mut self, element: Rc<String>) {
        self.ref_list.push(element);
    }

    pub fn rm_all_ref(&mut self, element: Rc<String>) {
        let mut i = 0;
        while i < self.ref_list.len() {
            let ele = &self.ref_list[i as usize];
            if *ele == element {
                self.ref_list.remove(i);
                continue;
            }
            i = i + 1;
        }
    }
}

pub fn how_many_references(ref_list: &Rc<String>) -> usize {
    Rc::strong_count(ref_list)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a = Rc::new("a".to_owned());
        let b = Rc::new("b".to_owned());
        let c = Rc::new("c".to_owned());
        
        let a1 = Rc::new("a".to_owned());
        
        let mut new_node = Node::new(vec![a.clone()]);
        new_node.add_element(b.clone());
        new_node.add_element(a.clone());
        new_node.add_element(c.clone());
        new_node.add_element(a.clone());
        
        println!("a: {:?}", how_many_references(&a));
        println!("b: {:?}", how_many_references(&b));
        println!("c: {:?}", how_many_references(&c));
        new_node.rm_all_ref(a1.clone());
        new_node.rm_all_ref(a.clone());
        
        println!("a: {:?}", how_many_references(&a));
        println!("b: {:?}", how_many_references(&b));
        println!("c: {:?}", how_many_references(&c));
    }
}
