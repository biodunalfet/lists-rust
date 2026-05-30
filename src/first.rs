use std::mem;

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

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

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List {
            head: Link::Empty
        }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty) // take ownership of self.head, but replace it with Empty, so it's still valid
        });
        self.head = Link::More(new_node); // new head is new elem, but the next is what the previous head which we 'stole'
    }

    pub fn pop(&mut self) -> Option<i32> {
        // bro, I want to do a few things with what you're point to. Hold this empty first
        // I'll return it once I'm done.
        let temp = mem::replace(&mut self.head, Link::Empty);
        match temp {
            Link::Empty => None,
            Link::More(node) => {
                // read the value of the head (perhaps copy the elem into some temp variable)
                // then set the head to the node's.next
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);

        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}
