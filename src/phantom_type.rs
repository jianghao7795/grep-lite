use std::marker::PhantomData;

#[derive(PartialEq)]
struct PhantomTuple<T, U>(T, PhantomData<U>);

#[derive(PartialEq)]
struct PhantomStruct<T, U> {
    first: T,
    phantom: PhantomData<U>,
}

pub fn run_phantom() {
    // 这里的 `f32` 和 `f64` 是隐藏参数。
    // 被指定为 `<char, f32>` 的 `PhantomTuple` 类型。
    let _tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
    // 被指定为 `<char, f64>` `PhantomTuple` 类型。
    let _tuple2: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);

    // 被指定为 `<char, f32>` 的类型。
    let _struct1: PhantomStruct<char, f64> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };
    // 被指定为 `<char, f64>` 的类型。
    let _struct2: PhantomStruct<char, f64> = PhantomStruct {
        first: 'B',
        phantom: PhantomData,
    };

    // 编译期错误！类型不匹配，所以这些值不能够比较：
    println!("_tuple1 == _tuple2 yields: {}", _tuple1 == _tuple2);

    // 编译期错误！类型不匹配，所以这些值不能够比较：
    println!("_struct1 == _struct2 yields: {}", _struct1 == _struct2);
    //
}
