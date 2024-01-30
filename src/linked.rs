// pub enum List {
//     Empty,
//     Elem(i32, Box<List>),
// }
// List 实现[ptr] -> (Elem A, ptr) -> (Elem B, *null*)
use std::mem::replace; // 链表中取出元素 匿名偷出元素
pub struct List {
    head: Link,
}
// #[derive(Clone)]
enum Link {
    Empty,
    More(Box<Node>),
}

// #[derive(Clone)]
struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        // let new_node = Node {
        //     elem: elem,
        //     next: self.head.clone(),
        // };
        let new_node = Box::new(Node {
            elem,
            next: replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        // let result;
        // match std::mem::replace(&mut self.head, Link::Empty) {
        //     Link::Empty => {
        //         result = None;
        //     }
        //     Link::More(node) => {
        //         result = Some(node.elem);
        //         self.head = node.next;
        //     }
        // };
        // result
        match replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
