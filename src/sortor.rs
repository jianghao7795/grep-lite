pub mod sortor {
    pub fn sortor1() {
        let mut vec = vec![1, 5, 10, 2, 15];
        println!("  排序前： {:?}", vec);

        vec.sort();
        println!("  排序后： {:?}", vec);

        assert_eq!(vec, vec![1, 2, 5, 10, 15]);
    }
    pub fn sortor2() {
        let mut vec = vec![1.1, 5.3, 10.8, 2.7, 15.6];
        println!("  排序前： {:?}", vec);

        vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
        println!("  排序后： {:?}", vec);

        assert_eq!(vec, vec![1.1, 2.7, 5.3, 10.8, 15.6]);
    }
}

pub mod random {
    use rand::distributions::{Distribution, Standard};
    use rand::{thread_rng, Rng};
    use rand_distr::Alphanumeric;
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Distribution<Point> for Standard {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
            let (rand_x, rand_y) = rng.gen();
            Point {
                x: rand_x,
                y: rand_y,
            }
        }
    }
    pub fn point() {
        let mut rng = rand::thread_rng();
        let rand_tuple = rng.gen::<(i32, bool, f64)>();
        let rand_point: Point = rng.gen();

        println!("  随机值元组：   {:?}", rand_tuple);
        println!("  随机值结构体： {:?}", rand_point);
        println!("  随机值： {}, {}", rand_point.x, rand_point.y);

        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(60)
            .map(char::from) // 需要u8转化为char ([U8]).collect()无法转化成String 或 &str
            .collect();
        println!("  随机密码： {}", rand_string);
        // let rand_string: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect(); 错误
        // let mut demo: Vec<u8>;
        // let demo_str = demo.map(char::from).collect();
        // let demo: &[u8] = b"sdfsdfsf";
        // let demo_str: String = demo.map(|x| (x as char)).collect();
        // let demo_char = demo.map(|x| (x as char));
        // let demo_str: String = (&demo_char).collect();
    }
}
