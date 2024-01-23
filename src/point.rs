pub fn destructure() {
    // 获得一个 `i32` 类型的引用。`&` 表示取引用。
    // &variable 是自己的地址
    // variable 本身值
    // *variable 取值
    let reference = &4;
    println!("{:p}", reference);

    let val = 7;
    let ref_val = &val;
    println!("{:p}, {:?}", &val, val);
    println!("{:p}, {:p}, {:?}", &ref_val, ref_val, *ref_val);

    let ref val = 3; // 相当于 let val = &3;
    println!("{:p}", val);

    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // 对 vec 的 `iter()` 举出 `&i32`。（通过用 `&x` 匹配）把它解构成 `i32`。
    // 译注：注意 `any` 方法会自动地把 `vec.iter()` 举出的迭代器的元素一个个地
    // 传给闭包。因此闭包接收到的参数是 `&i32` 类型的。
    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
    // 对 vec 的 `into_iter()` 举出 `i32` 类型。无需解构。
    println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // 对数组的 `iter()` 举出 `&i32`。
    println!("2 in array1: {}", array1.iter().any(|&x| x < 0));
    // 对数组的 `into_iter()` 举出 `i32`。
    println!("2 in array2: {}", array2.into_iter().any(|x| x == 2));
}
