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
}

mod test; 