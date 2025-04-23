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

#[derive(Debug)]
pub struct StaticLinkedList<T: Clone + PartialEq> {
    nodes: [StaticNode<T>; MAX_SIZE],
    head: Option<usize>,
    free: Vec<usize>,
}

impl<T: Clone + PartialEq + Default> StaticLinkedList<T> {
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
}

#[cfg(test)]
mod test;
