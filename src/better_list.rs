use std::mem;

pub struct List<T> {
    head: Link<T>,
}

//type Aliases
type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&mut self) -> Option<&T> {
        //previous code
        //self.head.map(|node|&node.elem)
        //This doesn't run because self.head flows by value
        //inside map and hence we can't return a reference.
        //This is moving out of borrowed content
        //Hence as ref is used which pushes a reference inside map
        self.head.as_ref().map(|node| &node.elem)
    }
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        //as_mut pushes a muutable reference inside map
        self.head.as_mut().map(|node| &mut node.elem)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}
//Tuple structs are alternative form of structs
//useful of trivial wrappers around other types
pub struct IntoIter<T>(List<T>);

//Implement intoiter for List<T>
impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
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
    #[test]
    fn peek() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        list.push(4);
        assert_eq!(list.peek(), Some(&4));
        assert_eq!(list.peek_mut(), Some(&mut 4));
        list.peek_mut().map(|node| *node = 10);
        assert_eq!(list.peek_mut(), Some(&mut 10));
    }
}
