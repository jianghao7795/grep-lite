pub mod iterator {
    use std::collections::HashMap;

    pub fn iterat() {
        let arr = [1, 2, 3];
        // 对数组序列进行迭代 arr.into_iter() 将数组转化为迭代器
        for v in arr.into_iter() {
            println!("{}", v);
        }

        //直接对数值序列进行迭代
        for v in 1..10.into() {
            println!("{}", v);
        }

        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        for val in v1_iter {
            println!("{}", val);
        }

        let arr = [1, 2, 3];
        let mut arr_iter = arr.into_iter();
        loop {
            match arr_iter.next() {
                Some(vb) => println!("{}", vb),
                None => {
                    println!("None");
                    break;
                }
            }
        }
        // assert_eq!(arr_iter.next(), Some(1));
        // assert_eq!(arr_iter.next(), Some(2));
        // assert_eq!(arr_iter.next(), Some(3));
        // assert_eq!(arr_iter.next(), None);

        let values = vec![11, 12, 13];
        {
            match IntoIterator::into_iter(values) {
                mut iter => loop {
                    match iter.next() {
                        Some(x) => {
                            println!("{}", x);
                        }
                        None => break,
                    }
                },
            };
        };

        let values = vec![1, 2, 3];
        //最终你完全可以写出这样的奇怪代码
        for v in values.into_iter().into_iter().into_iter().into_iter() {
            println!("{}", v)
        }

        {
            let mut values = vec![1, 2, 3];
            // let mut values = vec![1, 2, 3];
            // 对 values 中的元素进行可变借用
            let mut values_iter_mut = values.iter_mut();
            if let Some(v) = values_iter_mut.next() {
                *v = 999;
            }

            println!("{:#?}", values);
        }

        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        let total: i32 = v1_iter.clone().sum();
        assert_eq!(total, 6);

        // v1_iter 是借用了 v1，因此 v1 可以照常使用
        println!("{:?}", v1);

        // 以下代码会报错，因为 `sum` 拿到了迭代器 `v1_iter` 的所有权
        println!("{:?}", v1_iter);

        {
            let v1 = vec![56, 98, 99];
            // 这里的 map 方法是一个迭代者适配器，它是惰性的，不产生任何行为，因此我们还需要一个消费者适配器进行收尾：指明返回类型
            let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
            println!("{:?}", v2);
        }
        {
            let names = ["sunface", "sunfei"];
            let ages = [90, 40];
            let forks: HashMap<_, _> = names.into_iter().zip(ages).collect();
            // let b: Option<i32> = Some(1);

            println!("{:?}", forks);
        }

        {
            struct Counter {
                count: u32,
            }

            impl Counter {
                fn new() -> Counter {
                    Counter { count: 0 }
                }
            }

            impl Iterator for Counter {
                type Item = u32;
                fn next(&mut self) -> Option<Self::Item> {
                    if self.count < 5 {
                        self.count += 1;
                        Some(self.count)
                    } else {
                        None
                    }
                }
            }

            let counter = Counter::new();
            // assert_eq!(counter.next(), Some(1), "{}", 1);
            // assert_eq!(counter.next(), Some(2), "{}", 2);
            // assert_eq!(counter.next(), Some(3), "{}", 3);
            // assert_eq!(counter.next(), Some(4), "{}", 4);
            // assert_eq!(counter.next(), Some(5), "{}", 5);
            // assert_eq!(counter.next(), None, "None");

            let sum: u32 = counter
                .zip(Counter::new())
                .map(|(a, b)| a * b)
                .filter(|x| x % 3 == 0)
                .sum();

            println!("{}", sum);

            let v = vec![1u64, 2, 3, 4, 5, 6];
            let val = v
                .iter()
                .enumerate()
                // 每两个元素剔除一个
                // [1, 3, 5]
                .filter(|&(idx, _)| idx % 2 == 0)
                .map(|(_, val)| val)
                // 累加 1+3+5 = 9
                .fold(0u64, |sum, acm| sum + acm);

            println!("{}", val);
        }
    }
}
