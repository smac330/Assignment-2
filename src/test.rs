use crate::DynamicLinkedList;

#[test]

fn test_dynamic_linked_list_operations() {
    let mut list = DynamicLinkedList::new() as DynamicLinkedList<i32>;

    list.insert(10);
    list.insert(20);
    list.insert(30);
    
    
}