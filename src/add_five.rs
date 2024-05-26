pub mod add_five {
    use std::{collections::HashMap, fs::File};

    pub fn add_five(base: u32) -> u32 {
        let mut hash = HashMap::new();
        hash.insert("ab", 88);
        println!("{:?}", hash.get("ab"));
        base + 5
    }

    pub fn add_five_then() {
        println!("test");
        let output_file = File::open("hello1.txt").expect("file can`t open");
        println!("{:?}", output_file);
    }

    pub fn add_five_then_equal_one(base: u32) -> bool {
        crate::equal::equal_one::equal_one(add_five(base))
    }

    pub mod add_six {
        pub fn add_six(base: u32) -> u32 {
            super::add_five(base) + 1
        }

        pub fn delete_six(base: u32) -> u32 {
            self::add_six(base) - 6
        }
    }
}
// super 本文件mod的最上层
// crate 本项目的最上层
