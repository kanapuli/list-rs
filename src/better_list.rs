use std::mem;

pub struct List<T> {
    head: Link<T>,
}

//type Aliases
type Link<T> = Option<Box<Node<T>>>;

struct Node {
    elem: T,
    next: Link<T>,
}

impl List {
    pub fn new() -> List {
        List { head: None }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }
    pub fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();

        list.push(1);
        list.push(2);
        list.push(3);
        list.push(4);
        list.push(5);

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        //If uncommented, this should make test fail
        //assert_eq!(list.pop(), Some(1));
    }
}
