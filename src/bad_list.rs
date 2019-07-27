use std::mem;
struct Node {
    elem: i32,
    next: Link,
}
enum Link {
    Empty,
    //First approach was Elem(i32, List)
    //But since List size depends on the elements it contains
    //So when List is recursively mentioned, rust complains
    //Hence use a Box, which allocates a defined memory size
    // in heap and returns the ownership.
    More(Box<Node>),
}

//pub allows this struct to be publicly available
pub struct List {
    head: Link,
}

//Implementation for the struct List
impl List {
    //new is a static method
    pub fn new() -> Self {
        //refer tags/variants of enum using ::
        //Last expression of function in implicitly returned
        List { head: Link::Empty }
    }

    //push is a non static method.
    //Since it has self as an argument.
    pub fn push(&mut self, elem: i32) {
        let node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,

            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        //    self.head.drop();
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
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
