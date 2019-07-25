use std::mem;
struct Node {
    elem: i32,
    next: Link,
}
enum Link {
    Empty,
    //First approach was Elem(i32, List)
    //But since List size depends on the elemnts it contains
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
        match self.head {
            Link::Empty => {
                //todo
            }
            Link::More => {
                //todo
            }
        }
    }
}
