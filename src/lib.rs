const MAX_SIZE: usize = 100;

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
    /// Description: creates a new, empty linked list.
    /// Returns: a new instance of DynamicLinkedList.
    pub fn new() -> Self {
        Self { head: None }
    }

    /// Description: appends a new element to the end of the list.
    /// Parameters: data 
    pub fn insert(&mut self, data: T) {
        let new_node = Box::new(Node { data, next: None });
        let mut curr = &mut self.head;

        while let Some(node) = curr {
            curr = &mut node.next;
        }

        *curr = Some(new_node);
    }

    /// Description: inserts an element at the specified index.
    /// Parameters: index, data
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


    /// Description: deletes the first node that matches the given data.
    /// Parameters: data
    /// Returns: true if the element was found and deleted, otherwise false.
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

#[derive(Debug)]
pub struct StaticLinkedList<T: Clone + PartialEq> {
    nodes: [StaticNode<T>; MAX_SIZE],
    head: Option<usize>,
    free: Vec<usize>,
}

impl<T: Clone + PartialEq + Default> StaticLinkedList<T> {
    /// Description: creates a new empty static linked list.
    /// Returns: a new instance of StaticLinkedList.
    pub fn new() -> Self {
        let mut free = Vec::with_capacity(MAX_SIZE);
        for i in (0..MAX_SIZE).rev() {
            free.push(i);
        }

        Self {
            nodes: array_init::array_init(|_| StaticNode {
                data: None,
                next: None,
            }),
            head: None,
            free,
        }
    }

    /// Description: appends a new element to the end of the static list.
    /// Parameters: data
    pub fn insert(&mut self, data: T) {
        if let Some(new_idx) = self.free.pop() {
            self.nodes[new_idx] = StaticNode {
                data: Some(data),
                next: None,
            };

            match self.head {
                None => {
                    self.head = Some(new_idx);
                }
                Some(mut idx) => {
                    while let Some(next_idx) = self.nodes[idx].next {
                        idx = next_idx;
                    }
                    self.nodes[idx].next = Some(new_idx);
                }
            }
        }
    }

    /// Description: inserts an element at the given index.
    /// Parameters: index, data
    pub fn insert_at_index(&mut self, index: usize, data: T) {
        if self.free.is_empty() {
            return;
        }

        let new_idx = self.free.pop().unwrap();
        self.nodes[new_idx] = StaticNode {
            data: Some(data),
            next: None,
        };

        if index == 0 {
            self.nodes[new_idx].next = self.head;
            self.head = Some(new_idx);
            return;
        }

        let mut curr = self.head;
        for _ in 0..index - 1 {
            match curr {
                Some(i) => curr = self.nodes[i].next,
                None => {
                    self.free.push(new_idx);
                    return;
                }
            }
        }

        if let Some(i) = curr {
            self.nodes[new_idx].next = self.nodes[i].next;
            self.nodes[i].next = Some(new_idx);
        } else {
            self.free.push(new_idx);
        }

    }

    pub fn delete_element(&mut self, data: T) -> bool {
        let mut curr = &mut self.head;

        while let Some(idx) = *curr {
            if self.nodes[idx].data.as_ref() == Some(&data) {
                curr = &mut self.nodes[idx].next;
                self.nodes[idx] = StaticNode {
                    data: None,
                    next: None,
                };
                self.free.push(idx);
                return true;
            }
            curr = &mut self.nodes[idx].next;
        }
        false
    }

    pub fn delete_at_index(&mut self, index: usize) -> bool {
        if index == 0 {
            if let Some(i) = self.head {
                let next = self.nodes[i].next;
                self.nodes[i].data = None;
                self.nodes[i].next = None;
                self.free.push(i);
                self.head = next;
                return true;
            } else {
                return false;
            }
        }

        let mut current = self.head;
        for _ in 0..index - 1 {
            match current {
                Some(i) => current = self.nodes[i].next,
                None => return false,
            }
        }

        let prev_idx = match current {
            Some(i) => i,
            None => return false,
        };

        let target_idx = match self.nodes[prev_idx].next {
            Some(i) => i,
            None => return false, 
        };

        let next = self.nodes[target_idx].next;

        self.nodes[target_idx].data = None;
        self.nodes[target_idx].next = None;
        self.nodes[prev_idx].next = next;
        self.free.push(target_idx);
        true
    }

    pub fn update_element(&mut self, old_data: T, new_data: T) -> bool {
        let mut curr = self.head;

        while let Some(idx) = curr {
            if let Some(ref val) = self.nodes[idx].data {
                if *val == old_data {
                    self.nodes[idx].data = Some(new_data);
                    return true;
                }
            }
            curr = self.nodes[idx].next;
        }
        false
    }

    pub fn update_element_at_index(&mut self, index: usize, data: T) -> bool {
        let mut curr = self.head;

        for _ in 0..index {
            match curr {
                Some(i) => curr = self.nodes[i].next,
                None => return false,
            }
        }

        if let Some(i) = curr {
            self.nodes[i].data = Some(data);
            return true;
        }

        false
    }

    pub fn find(&self, data: T) -> bool {
        let mut curr = self.head;

        while let Some(idx) = curr {
            if self.nodes[idx].data.as_ref() == Some(&data) {
                return true;
            }
            curr = self.nodes[idx].next;
        }

        false
    }

    pub fn get(&self, index: usize) -> Option<T> {
        let mut curr = self.head;

        for _ in 0..index {
            match curr {
                Some(i) => curr = self.nodes[i].next,
                None => return None,
            }
        }

        match curr {
            Some(i) => self.nodes[i].data.clone(),
            None => None,
        }
    }

}

#[cfg(test)]
mod test;
