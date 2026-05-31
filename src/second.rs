use std::mem;


#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = crate::first::List::new();

        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}

pub struct List {
    head: Link,
}

type Link = Option<Box<Node>>;

struct Node {
    elem: i32,
    next: Link,
}

impl List {

    pub fn new() -> List {
        List { head: None }
    }

    pub fn push(&mut self, elem: i32) {
        let temp = self.head.take();
        let node = Box::new(Node {
            elem: elem,
            next: temp
        });
        self.head = Some(node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        // let temp = self.head.take();
        // let popped = match temp {
        //     None => None,
        //     Some(node) => {
        //         self.head = node.next;
        //         Some(node.elem)
        //     }
        // };
        // popped
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
}

impl Drop for List {
    // Initial take
    // fn drop(&mut self) {
    //     let mut current = self.head.take();
    //
    //     while current.is_some() {
    //         current = current.unwrap().next.take();
    //     }
    // }

    // Another take with loop because the while syntax is weird
    // fn drop(&mut self) {
    //     let mut current = self.head.take();
    //     loop {
    //         match current {
    //             None => {
    //                 break;
    //             }
    //             Some(mut boxed_node) => {
    //                 current = boxed_node.next.take();
    //             }
    //         }
    //     }
    // }

    fn drop(&mut self) {
        let mut current = self.head.take();

        while let Some(mut boxed_node) = current {
            current = boxed_node.next.take();
        }
    }
}

