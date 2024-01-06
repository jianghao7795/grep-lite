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
}
