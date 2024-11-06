mod list;

use crate::list::List;

fn main() {
    let mut list = List::new();
    list.add("hello");
    list.add("testing");
    list.add("world");
    list.print();
}
