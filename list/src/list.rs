#[derive(Debug)]
struct Node {
    name: String,
    next: Option<Box<Node>>,
}

impl Node {
    //pub fn new(value: &str) -> Self {
    //    Self {
    //        name: String::from(value),
    //        next: None,
    //    }
    //}
}

#[derive(Debug)]
pub struct List {
    head: Option<Box<Node>>,
}

impl List {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn add(&mut self, value: &str) {
        let new_node = Box::new(Node {
            name: String::from(value),
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn print(&self) {
        let mut current: &Node;
        match self.head.as_ref() {
            None => {
                println!("None");
                return;
            }
            Some(head) => current = head,
        }
        while let Some(next) = current.next.as_ref() {
            print!("{} -> ", current.name);
            current = next;
        }
        println!("{}", current.name);
    }
}
