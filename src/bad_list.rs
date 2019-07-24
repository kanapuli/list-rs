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
    pub fn new() -> Self {
        List { head: Link::Empty }
    }
}
