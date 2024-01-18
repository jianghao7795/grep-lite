// use std::mem;

pub fn run_closure() {
    // 函数
    fn func(i: &mut i32) {
        *i = *i + 1;
    }

    // 调用函数和闭包。
    // func(&mut i);
    // func(&mut i);
    let mut i = 8;
    //只需传引用就能使用，而这个闭包捕获的也是变量的引用
    let mut closure_inferred = || {
        i = i + 1; // i 是可变量的引用 &mut i
        println!("{i}");
    };

    closure_inferred();
    // 没有执行closure_inferred i 变量的借用消除 后面可以使用
    let mut closure_annotated = || {
        println!("{i}");
        i += 2;
    };
    closure_annotated();
    // let i = &mut i;
    // closure_inferred();
    func(&mut i); // 重新定义i 的借用
    println!("success");
    // println!("closure_annotated: {}", closure_annotated());
    // println!("closure_inferred: {}", closure_inferred());
    println!("i is {}", i);

    // 闭包不再借用 `&mut count`，因此可以正确地重新借用
    // let count_reborrowed = &mut i;
    func(&mut i);
    i = i + 1;
    println!("{i}");
    // closure_inferred();
    // closure_annotated();
    // 没有参数的闭包，返回一个 `i32` 类型。
    // 返回类型是自动推导的。
    let one = || 1;
    println!("closure returning one: {}", one());
}
