// pub enum List {
//     Empty,
//     Elem(i32, Box<List>),
// }
// List 实现[ptr] -> (Elem A, ptr) -> (Elem B, *null*)
use std::mem::replace; // 链表中取出元素 匿名偷出元素
pub struct List<T> {
    head: Link<T>,
}
// #[derive(Clone)]
// enum Link {
//     Empty,
//     More(Box<Node>),
// }
type Link<T> = Option<Box<Node<T>>>;
// #[derive(Clone)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        // let new_node = Node {
        //     elem: elem,
        //     next: self.head.clone(),
        // };
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
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
        // match replace(&mut self.head, None) {
        //     None => None,
        //     Some(node) => {
        //         self.head = node.next;
        //         Some(node.elem)
        //     }
        // }
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
    // 获取 链表的头
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }
    // 设置 链表的头
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = replace(&mut self.head, None);
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
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
        // map FnOnce 返回类型为i32 调用map()返回结果Some(i32)
        let result = list.peek_mut().map(|value| {
            *value = 42;
            return 42;
        });
        assert_eq!(result, Some(42));

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }
}
