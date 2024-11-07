mod list;

use crate::list::List;

fn main() {
    let mut list = match List::read() {
        Ok(result) => result,
        Err(_) => List::new(),
    };
    list.add("hello");
    list.add("testing");
    list.add("world");
    list.print();
    list.write();
}
