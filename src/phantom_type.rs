use std::marker::PhantomData;
use std::ops::Add;

#[derive(PartialEq)]
struct PhantomTuple<T, U>(T, PhantomData<U>);

#[derive(PartialEq)]
struct PhantomStruct<T, U> {
    first: T,
    phantom: PhantomData<U>,
}

#[derive(Debug, Clone, Copy)]
enum Inch {}

#[derive(Debug, Clone, Copy)]
enum Mm {}
#[derive(Debug, Clone, Copy)]
struct Length<Unit>(f64, PhantomData<Unit>);

/// `Add` trait 定义了 `+` 运算符的行为。
impl<Unit> Add for Length<Unit> {
    type Output = Length<Unit>;

    // add() 返回一个含有和的新的 `Length` 结构体。
    fn add(self, rhs: Length<Unit>) -> Length<Unit> {
        // `+` 调用了针对 `f64` 类型的 `Add` 实现。
        Length(self.0 + rhs.0, PhantomData)
    }
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

    // 指定 `one_foot` 拥有虚类型参数 `Inch`。
    let one_foot: Length<Inch> = Length(125.0, PhantomData);
    // `one_meter` 拥有虚类型参数 `Mm`。
    let one_meter: Length<Mm> = Length(1000.0, PhantomData);

    // `+` 调用了我们对 `Length<Unit>` 实现的 `add()` 方法。
    //
    // 由于 `Length` 了实现了 `Copy`，`add()` 不会消耗 `one_foot`
    // 和 `one_meter`，而是复制它们作为 `self` 和 `rhs`。
    let two_feet = one_foot + one_foot;
    let two_meters = one_meter + one_meter;

    // 加法正常执行。
    println!("one foot + one_foot = {:?} in", two_feet.0);
    println!("one meter + one_meter = {:?} mm", two_meters.0);
}
