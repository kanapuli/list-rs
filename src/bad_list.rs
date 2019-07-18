//pub allows this enum to be publicly available
pub enum List {
    Empty,
    //First approach was Elem(i32, List)
    //But since List size depends on the elemnts it contains
    //So when List is recursively mentioned, rust complains
    //Hence use a Box, which allocates a defined memory size
    // in heap and returns the ownership.
    Elem(i32, Box<List>),
}
