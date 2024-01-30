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

    use std::mem;

    let mut color = String::from("green");

    // 这个闭包打印 `color`。它会立即借用（通过引用，`&`）`color` 并将该借用和
    // 闭包本身存储到 `print` 变量中。`color` 会一直保持被借用状态直到
    // `print` 离开作用域。
    //
    // `println!` 只需传引用就能使用，而这个闭包捕获的也是变量的引用，因此无需
    // 进一步处理就可以使用 `println!`。
    let print = || println!("`color`: {}", color);

    // 使用借用来调用闭包 `color`。
    print();

    // `color` 可再次被不可变借用，因为闭包只持有一个指向 `color` 的不可变引用。
    // let reborrow = &mut color; // error 可变引用同时只能存在一个
    //    let println = || print!("rebroow is {}", )
    fn println(a: &mut String) -> &String {
        a.push_str("test");
        a
    }

    println!("rebroow is {:p}", println(&mut color));
    // print();
    // 在最后使用 `print` 之后，移动或重新借用都是允许的。
    let _color_moved = color;

    let mut count = 0;
    // 这个闭包使 `count` 值增加。要做到这点，它需要得到 `&mut count` 或者
    // `count` 本身，但 `&mut count` 的要求没那么严格，所以我们采取这种方式。
    // 该闭包立即借用 `count`。
    //
    // `inc` 前面需要加上 `mut`，因为闭包里存储着一个 `&mut` 变量。调用闭包时，
    // 该变量的变化就意味着闭包内部发生了变化。因此闭包需要是可变的。
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    // 使用可变借用调用闭包
    inc();

    // 因为之后调用闭包，所以仍然可变借用 `count`
    // 试图重新借用将导致错误
    // let _reborrow = &count;
    // ^ 试一试：将此行注释去掉。
    inc();

    // 闭包不再借用 `&mut count`，因此可以正确地重新借用
    let _count_reborrowed = &mut count;

    // 不可复制类型（non-copy type）。
    let movable = Box::new(3);

    // `mem::drop` 要求 `T` 类型本身，所以闭包将会捕获变量的值。这种情况下，
    // 可复制类型将会复制给闭包，从而原始值不受影响。不可复制类型必须移动
    // （move）到闭包中，因而 `movable` 变量在这里立即移动到了闭包中。
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    // `consume` 消耗了该变量，所以该闭包只能调用一次。
    consume();
    // consume();
    // ^ 试一试：将此行注释去掉。
}

fn apply<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

fn apply_to<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(3)
}

pub fn input() {
    use std::mem;

    let greeting = "hello";
    // 不可复制的类型。
    // `to_owned` 从借用的数据创建有所有权的数据。
    let mut farewell = "goodbye".to_string();

    // 捕获 2 个变量：通过引用捕获 `greeting`，通过值捕获 `farewell`。
    let diary = || {
        // `greeting` 通过引用捕获，故需要闭包是 `Fn`。
        println!("I said {}.", greeting);

        // 下文改变了 `farewell` ，因而要求闭包通过可变引用来捕获它。
        // 现在需要 `FnMut`。
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // 手动调用 drop 又要求闭包通过值获取 `farewell`。
        // 现在需要 `FnOnce`。
        mem::drop(farewell);
    };

    // 以闭包作为参数，调用函数 `apply`。
    apply(diary);

    // 闭包 `double` 满足 `apply_to_3` 的 trait 约束。
    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to(double));
}
