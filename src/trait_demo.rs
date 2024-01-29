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

pub mod animal {
    struct Sheep {
        naked: bool,
        name: &'static str,
    }

    trait Animal {
        // 静态方法签名；`Self` 表示实现者类型（implementor type）。
        fn new(name: &'static str) -> Self;

        // 实例方法签名；这些方法将返回一个字符串。
        fn name(&self) -> &'static str;
        fn noise(&self) -> &'static str;

        // trait 可以提供默认的方法定义。
        fn talk(&self) {
            println!("{} says {}", self.name(), self.noise());
        }
    }

    impl Sheep {
        fn is_naked(&self) -> bool {
            self.naked
        }

        fn shear(&mut self) {
            if self.is_naked() {
                // 实现者可以使用它的 trait 方法。
                println!("{} is already naked...", self.name);
                self.naked = false;
            } else {
                println!("{} gets a haircut!", self.name);

                self.naked = true;
            }
        }
    }

    impl Animal for Sheep {
        fn new(name: &'static str) -> Sheep {
            Sheep {
                name: name,
                naked: false,
            }
        }

        fn name(&self) -> &'static str {
            self.name
        }

        fn noise(&self) -> &'static str {
            if self.is_naked() {
                "wuwuwuwuwuwuwu?"
            } else {
                "baaaaah!"
            }
        }

        // 默认 trait 方法可以重载。
        fn talk(&self) {
            // 例如我们可以增加一些安静的沉思。
            println!("{} pauses briefly... {}", self.name, self.noise());
        }
    }

    pub fn demo() {
        let mut dolly: Sheep = Animal::new("Dolly");

        dolly.talk();
        dolly.shear();
        dolly.talk();
        dolly.shear();
    }
}
