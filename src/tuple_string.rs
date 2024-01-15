pub mod tuple {
    fn reverse(pair: (i32, bool)) -> (bool, i32) {
        let (integer, boolean) = pair;
        (boolean, integer)
    }

    #[derive(Debug)]
    struct Matrix(f32, f32, f32, f32);

    pub fn run_reverse() {
        // 包含各种不同类型的元组
        let long_tuple = (
            1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
        );

        // 通过元组的索引来访问具体的值
        println!("long tuple first value: {}", long_tuple.0);
        println!("long tuple second value: {}", long_tuple.1);

        // 元组也可以充当元组的元素
        let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

        // 元组可以打印
        println!("tuple of tuples: {:?}", tuple_of_tuples);

        let pair = (1, true);
        println!("pair is {:?}", pair);

        println!("the reversed pair is {:?}", reverse(pair));

        // 创建单元素元组需要一个额外的逗号，这是为了和括号包含的普通数据作区分。
        println!("one element tuple: {:?}", (5u32,));
        println!("just an integer: {:?}", (5u32));

        // 解构元组，将值赋给创建的绑定变量
        let tuple = (1, "hello", 4.5, true);

        let (a, b, c, d) = tuple;
        println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

        let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
        println!("{:?}", matrix)
    }
}
