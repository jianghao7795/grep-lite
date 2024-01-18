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
                Cons(_, ref tail) => 1 + tail.len(),
                Nil => 0,
            }
        }
        // 将列表以字符串（堆分配的）的形式返回
        fn stringify(&self) -> String {
            match self {
                // 赋值语句中左边的 `ref` 关键字等价于右边的 `&` 符号
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
        list = list.prepend(6);

        println!("linked list has length: {}", list.len());
        println!("{}", list.stringify());
    }

    pub fn run_ref() {
        // 解引用使用 *
        // 解构使用 &，ref， 和 ref mut
        let reference = &4;
        match reference {
            &val => println!("Got a value via destructuring: {:p}", &val),
        }
        match *reference {
            val => println!("Got a value via destructuring: {:p}", &val),
        }

        let ref is_a_reference = 4;
        println!("{:p}", is_a_reference);

        // 获得一个 `i32` 类型的引用。`&` 表示获取一个引用。
        let reference = &4;

        match reference {
            // 如果 `reference` 是对 `&val` 进行模式匹配，则会产生如下比较行为：
            // `&i32`
            // `&val`
            // ^ 我们看到，如果匹配的 `&` 都去掉了，那么就是 `i32` 赋给 `val`。
            &val => println!("Got a value via destructuring: {}", val),
        }

        // 为了避免 `&` 的使用，需要在匹配前解引用*。
        match *reference {
            val => println!("Got a value via dereferencing: {:?}", val),
        }

        // 如果没有一个引用头部（以 & 开头）会是怎样？ `reference` 是一个 `&`，
        // 因为右边已经是一个引用。
        // 下面这个不是引用，因为右边不是。
        let _not_a_reference = 3;

        // Rust 对这种情况提供了 `ref`。它更改了赋值行为，使得可以对具体值
        // 创建引用。这将得到一个引用。
        let ref _is_a_reference = 3;

        // 相应地，定义两个非引用的值，通过 `ref` 和 `mut` 可以取得引用。
        let value = 5;
        let mut mut_value = 6;
        println!("Got a reference to a value: {:?}", value);
        // 使用 `ref` 关键字来创建引用。
        match value {
            ref r => println!("Got a reference to a value: {:p}", r), // ref 可获得地址
        }

        // 类似地使用 `ref mut`。
        match mut_value {
            ref mut m => {
                // 获得一个引用。在增加内容之前，要先得到解引用（Gotta
                // dereference it before we can add anything to it）。
                *m += 10;
                println!("We added 10. `mut_value`: {:?}", m);
            }
        }
    }
}
