#[cfg(test)]
mod test {
    use crate::second::List;

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

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));
        list.peek_mut().map(|value| {
            *value = 42
        });

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }

}

/**
 2 main things to learn here
- when using a mutable reference, ensure only one exists at any given time
- the feature here is to be able to return a mutable reference which makes it possible to modify
the content of the linked list via the iterator
**/
pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>
}

/**
- Main lesson for this is lifelines, and
- how to use as_deref as a way to "unwrap" a variable
- e.g Option<Box<Node>> -----(as_deref)----> Option<&Node>
--- and as_ref(), as a way to create a borrowed view of a value without taking ownership
--- Option<Box<Node>> ---(as_ref())----> Option<&Box<Node>>

as_ref()
    "Let me look at it"

as_mut()
    "Let me modify it"

as_deref()
    "Let me look through the pointer"

as_deref_mut()
    "Let me modify through the pointer"

What this allows us to do -> if you have a mut ref to some big struct/data structure
- you can still hold mut ref to the constituent subfields/parts
**/
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>> // keep a pointer to the “next node to read”.
}

// just a wrapper class that has implemented the iterator trait
pub struct IntoIter<T>(List<T>);

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {

    pub fn new() -> Self {
        List { head: None }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_deref_mut()
        }
    }

    // Borrow the list for some lifetime 'a, and return an iterator
    // whose internal node references are valid for that same lifetime.
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }

    pub fn push(&mut self, elem: T) {
        let temp = self.head.take();
        let node = Box::new(Node {
            elem,
            next: temp
        });
        self.head = Some(node);
    }

    pub fn pop(&mut self) -> Option<T> {
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

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl <'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        // I guess this point of IterMut is that when a variable is a mutable reference,
        // you need to always ensure that only one mutable ref exists at any given time,
        // hence the use of take()
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            // let next_link = node.next.as_ref(); -- as_ref() because node.next returns a Link aka Option<Box<Node>> but we want Option<&Node>. as_ref here give us Option<&Box<Node>>
            // let next_node = next_link.map(|boxed_node| { -- we need a borrowed ref, so that we can do map without worrying about ownership transfer
            //     &**boxed_node
            // });

            // this is short for the above.
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<T> Drop for List<T> {
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

