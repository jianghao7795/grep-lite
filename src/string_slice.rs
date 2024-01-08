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
        println!("{}", scoend_world(&mut your_string)) // 实现了
    }

    fn scoend_world(s: &mut String) -> &str {
        *s += " sdfasdf";
        &(*s.as_str())
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
