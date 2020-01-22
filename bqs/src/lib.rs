pub struct Item {
    pub name: &'static str,
    pub value: i32,
}

pub struct Bag<T> {
    contents: Vec<T>,
}

impl<T> Bag<T> {
    fn add(&mut self, item: T) {
        self.contents.push(item)
    }

    fn is_empty(&self) -> bool {
        self.size() == 0
    }

    fn size(&self) -> usize {
        self.contents.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_empty_bag() {
        let contents: Vec<Box<Item>> = vec![];
        let bag = Bag { contents };
        assert_eq!(0, bag.size());
        assert_eq!(true, bag.is_empty());
    }

    #[test]
    fn add_one_item() {
        let contents: Vec<Box<Item>> = vec![];
        let mut bag = Bag { contents };
        assert_eq!(0, bag.size());
        assert_eq!(true, bag.is_empty());

        let item = Item {
            name: "ok",
            value: 77,
        };
        bag.add(Box::from(item));
        assert_eq!(1, bag.size());
        assert_eq!(false, bag.is_empty());
    }
}
