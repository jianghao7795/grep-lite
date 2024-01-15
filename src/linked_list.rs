pub mod linked {
    use List::*;

    enum List {
        Cons(u32, Box<List>),
        Nil,
    }

    impl List {
        fn new() -> List {
            Nil
        }

        fn prepend(self, elem: u32) -> List {
            Cons(elem, Box::new(self))
        }

        fn len(&self) -> u32 {
            match self {
                Cons(_, tail) => 1 + tail.len(),
                Nil => 0,
            }
        }
        // 将列表以字符串（堆分配的）的形式返回
        fn stringify(&self) -> String {
            match *self {
                Cons(head, ref tail) => {
                    // `format!` 和 `print!` 类似，但返回的是一个堆分配的字符串，而不是
                    // 打印结果到控制台上
                    format!("{}, {}", head, tail.stringify())
                }
                Nil => {
                    format!("Nil")
                }
            }
        }
    }

    pub fn run_linked() {
        let mut list = List::new();

        list = list.prepend(1);
        list = list.prepend(2);
        list = list.prepend(3);

        println!("linked list has length: {}", list.len());
        println!("{}", list.stringify());
    }
}
