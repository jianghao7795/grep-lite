pub mod string_slice {
    pub fn string_str() {
        println!("string");

        let s = String::from("hello world");
        let hello = &s[0..5]; // string to &str
        let world = &s[6..11];
        let b = "wu bbs".to_string(); // &str to string
        println!("{hello}, b{world}");
        println!("{}", b);

        let s = "holla中国人नमस्ते";
        // let a = &s[0..2];
        // println!("{}", a);
        for c in s.chars() {
            println!("{}", c);
        }
    }

    pub fn slice_example() {
        let my_string = String::from("hello world");

        let word = first_world(&my_string[0..6]);
        println!("{word}");
        let word = first_world(&my_string[..]);
        println!("{word}");

        let word = first_world(&my_string);
        println!("{word}");
        let my_string = "hello world";
        let word = first_world(my_string);
        println!("{word}");
        println!("{my_string}"); // &str

        // let your_string = "nice hello";
        // println!("{}", scoend_world(&your_string.to_string())) 实现不了 fn scoend_world(s: &str) -> &str
        let mut your_string = String::from("nice hello");
        println!("{}", scoend_world(&mut your_string)); // 实现了

        let a = Some(5);
        if let Some(9) = a {
            println!("test5");
        }
    }

    fn scoend_world(s: &mut String) -> &str {
        *s += " sdfasdf";
        // &(s[..])
        &s[..]
    }

    fn first_world(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }
}

pub mod slice {
    use std::mem;

    fn analyze_slice(slice: &[i32]) {
        println!("first element of the slice: {}", slice[0]);
        println!("the slice has {} elements", slice.len());
    }

    pub fn run_slice() {
        // 固定大小的数组（类型标记是多余的）
        let xs: [i32; 5] = [1, 2, 3, 4, 5];

        // 所有元素可以初始化成相同的值
        let ys: [i32; 500] = [0; 500];

        // 索引从 0 开始
        println!("first element of the array: {}", xs[0]);
        println!("second element of the array: {}", xs[1]);

        // `len` 返回数组的大小
        println!("array size: {}", xs.len());

        // 数组是在堆中分配
        println!("array occupies {} bytes", mem::size_of_val(&xs));

        // 数组可以自动地借用成为 slice
        println!("borrow the whole array as a slice");
        analyze_slice(&xs);

        // slice 可以指向数组的一部分
        println!("borrow a section of the array as a slice");
        analyze_slice(&ys[1..4]);

        // 越界的索引会引发 panic（中文意思是：惊恐、恐慌等意）
        // println!("{}", xs[5]); // xs[5] 越界
        println!("{}", ys[5]);
    }
}
