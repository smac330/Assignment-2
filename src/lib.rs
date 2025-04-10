#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Debug)]
pub struct DynamicLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T: PartialEq + Clone> DynamicLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn insert(&mut self, data: T) {
        let mut new_node = Box::new(Node { data, next: None });
        let mut curr = &mut self.head;

        while let Some(node) = curr {
            curr = &mut node.next;
        }

        *curr = Some(new_node);
    }

    pub fn insert_at_index(&mut self, index: usize, data: T) {
        if index == 0 {
            let new_node = Box::new(Node {
                data,
                next: self.head.take(),
            });
            self.head = Some(new_node);
            return;
        }

        let mut curr = &mut self.head;
        for _ in 0..index - 1 {
            match curr {
                Some(node) => curr = &mut node.next,
                None => return, // index out of bounds
            }
        }
    }

    
}

mod test; 