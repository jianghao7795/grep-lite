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
}
