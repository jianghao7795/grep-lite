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
