use crate::{DynamicLinkedList, StaticLinkedList};

#[test]
fn test_dynamic_linked_list_operations() {
    let mut list = DynamicLinkedList::new() as DynamicLinkedList<i32>;

    list.insert(10);
    list.insert(20);
    list.insert(30);

}

#[test]
fn test_static_linked_list() {
    let mut list = StaticLinkedList::<i32>::new();

    list.insert(10);
    list.insert(20);
    list.insert(30);


    let mut current = list.head;
    assert!(current.is_some());
    let first_idx = current.unwrap();
    assert_eq!(list.nodes[first_idx].data, Some(10));

    current = list.nodes[first_idx].next;
    assert!(current.is_some());
    let second_idx = current.unwrap();
    assert_eq!(list.nodes[second_idx].data, Some(20));

    current = list.nodes[second_idx].next;
    assert!(current.is_some());
    let third_idx = current.unwrap();
    assert_eq!(list.nodes[third_idx].data, Some(30));

    current = list.nodes[third_idx].next;
    assert!(current.is_none());


    assert!(list.delete_at_index(1));

    current = list.head;
    assert!(current.is_some());
    let first_idx = current.unwrap();
    assert_eq!(list.nodes[first_idx].data, Some(10));

    current = list.nodes[first_idx].next;
    assert!(current.is_some());
    let second_idx = current.unwrap();
    assert_eq!(list.nodes[second_idx].data, Some(30));

    current = list.nodes[second_idx].next;
    assert!(current.is_none());


    assert!(list.update_element_at_index(1, 99));
    assert_eq!(list.nodes[second_idx].data, Some(99));


    assert_eq!(list.get(0), Some(10));
    assert_eq!(list.get(1), Some(99));
    assert_eq!(list.get(2), None);

    assert!(list.find(10));
    assert!(list.find(99));
    assert!(!list.find(20)); 
}