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
        let new_node = Box::new(Node { data, next: None });
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
                None => return,
            }
        }

        if let Some(node) = curr {
            let new_node = Box::new(Node {
                data,
                next: node.next.take(),
            });
            node.next = Some(new_node);
        }
    }

    pub fn delete_element(&mut self, data: T) -> bool {
        let mut curr = &mut self.head;

        while let Some(node) = curr {
            if node.data == data {
                let next = node.next.take();
                let mut next = node.next.take();
                curr = &mut next;
                return true;
            }
            curr = &mut node.next;
        }
        false
    }

    pub fn to_vec(&self) -> Vec<T> {
        let mut result = Vec::new();
        let mut current = &self.head;
        while let Some(node) = current {
            result.push(node.data.clone());
            current = &node.next;
        }
        result
    }
}

#[derive(Clone, Debug)]
struct StaticNode<T: Clone + PartialEq> {
    data: Option<T>,
    next: Option<usize>,
}

#[cfg(test)]
mod test;
