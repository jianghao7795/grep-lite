pub mod sortor {
    pub fn sortor1() {
        let mut vec = vec![1, 5, 10, 2, 15];
        println!("  排序前： {:?}", vec);

        vec.sort();
        println!("  排序后： {:?}", vec);

        assert_eq!(vec, vec![1, 2, 5, 10, 15]);
    }
}
