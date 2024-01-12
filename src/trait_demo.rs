pub mod trait_add {
    pub trait Summary {
        fn summarize(&self) -> String;
    }

    pub struct Post {
        pub title: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for Post {
        fn summarize(&self) -> String {
            format!("文章{}, 作者是{}", self.title, self.author)
        }
    }

    pub struct Weibo {
        pub username: String,
        pub content: String,
    }

    impl Summary for Weibo {
        fn summarize(&self) -> String {
            format!("{}发表了微博{}", self.username, self.content)
        }
    }

    pub fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
        a + b
    }

    pub fn demo() {
        let post = Post {
            title: "Rust语言简介".to_string(),
            author: "Sunface".to_string(),
            content: "Rust棒极了!".to_string(),
        };
        let weibo = Weibo {
            username: "sunface".to_string(),
            content: "好像微博没Tweet好用".to_string(),
        };

        println!("{}", post.summarize());
        println!("{}", weibo.summarize());
    }
}
