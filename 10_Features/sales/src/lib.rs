#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}

impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    pub items: Vec<(String, f32)>,
    pub receipt: Vec<f32>,
}

impl Cart {
    pub fn new() -> Cart {
        Cart {
            items: Vec::new(),
            receipt: Vec::new(),
        }
    }

    pub fn insert_item(&mut self, store: &Store, item: String) {
        let prix = store
            .products
            .iter()
            .find(|(name, _)| *name == item)
            .map(|(_, prix)| *prix)
            .unwrap_or(0.0);

        if let Some((_, total)) = self.items.iter_mut().find(|(name, _)| *name == item) {
            *total += prix;
        } else {
            self.items.push((item, prix));
        }
    }

    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let mut res: Vec<f32> = self.items.iter().map(|(_, prix)| *prix).collect();

        res.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let discount = res.len() / 3;

        let total: f32 = res.iter().sum();
        let cheap: f32 = res.iter().take(discount).sum();
        let percentage = cheap / total;

        res.iter_mut().for_each(|prix| {
            *prix -= *prix * percentage;
            *prix = (*prix * 100.0).round() / 100.0;
        });

        self.receipt = res.clone();
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let store = Store::new(vec![
            (String::from("product A"), 1.23),
            (String::from("product B"), 23.1),
            (String::from("product C"), 3.12)]);
        
        println!("{:?}", store);
        
        let mut cart = Cart::new();
        cart.insert_item(&store, String::from("product A"));
        cart.insert_item(&store, String::from("product B"));
        cart.insert_item(&store, String::from("product C"));
        
        println!("{:?}", cart.generate_receipt());
        
        println!("{:?}", cart);
    }
}
