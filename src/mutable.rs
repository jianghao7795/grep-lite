// #[allow(dead_code)]
// 作用是实现Clone 和 Copy 使的复制为创建新的struct 不是转移所有权
#[derive(Clone, Copy)]
struct Book {
    author: &'static str,
    title: &'static str,
    year: u32,
}

// 此函数接受一个对 Book 类型的引用
fn borrow_book(book: &Book) {
    println!(
        "I immutably borrowed {} - {} edition",
        book.title, book.year
    );
}

// 此函数接受一个对可变的 Book 类型的引用，它把年份 `year` 改为 2014 年
fn new_edition(book: &mut Book) {
    book.year = book.year + 7;
    println!(
        "{} mutably borrowed {} - {} edition",
        book.author, book.title, book.year
    );
}

pub fn run_book() {
    // 创建一个名为 `immutabook` 的不可变的 Book 实例
    let immutabook = Book {
        // 字符串字面量拥有 `&'static str` 类型
        author: "Douglas Hofstadter",
        title: "Gödel, Escher, Bach",
        year: 1979,
    };

    // let mutbookconst = immutabook;

    // 创建一个 `immutabook` 的可变拷贝，命名为 `mutabook`
    let mut mutabook = immutabook;

    // 不可变地借用一个不可变对象
    borrow_book(&immutabook);

    // 不可变地借用一个可变对象
    borrow_book(&mutabook);

    // 可变地借用一个可变对象
    new_edition(&mut mutabook);

    new_edition(&mut mutabook);
    // 报错！不能可变地借用一个不可变对象
    // new_edition(&mut immutabook);
    // 改正 ^ 注释掉此行
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

// fn change(some_string: &mut String) {
//     some_string.push_str("string");
// }

pub fn run_point() {
    // let mut x = 6;
    // let y = &mut x;
    // *y = *y + 1;
    // println!("{}", y);
    // let mut s = String::from("hello");
    // change(&mut s);
    // s.push_str("55555");
    // println!("{:?}", s);
    let mut point = Point { x: 0, y: 0, z: 0 };

    let borrowed_point = &point;
    let another_borrow = &point;

    // 数据可以通过引用或原始类型来访问
    println!(
        "Point has coordinates: ({}, {}, {})",
        borrowed_point.x, another_borrow.y, point.z
    );

    // // 报错！`point` 不能以可变方式借用，因为当前还有不可变借用。
    // let mutable_borrow = &mut point;
    // // TODO ^ 试一试去掉此行注释
    // println!("{:?}", mutable_borrow);

    // 被借用的值在这里被重新使用
    // println!(
    //     "Point has coordinates: ({}, {}, {})",
    //     borrowed_point.x, another_borrow.y, point.z
    // );
    // 数据可以通过引用或原始类型来访问
    // println!(
    //     "Point has coordinates: ({}, {}, {})",
    //     borrowed_point.x, another_borrow.y, point.z
    // );

    // 报错！`point` 不能以可变方式借用，因为当前还有不可变借用。
    // let mutable_borrow = &mut point;
    // TODO ^ 试一试去掉此行注释

    // 被借用的值在这里被重新使用
    // println!(
    //     "Point has coordinates: ({}, {}, {})",
    //     borrowed_point.x, another_borrow.y, mutable_borrow.z
    // );
    // println!("{}", mutable_borrow.z);

    // 不可变的引用不再用于其余的代码，因此可以使用可变的引用重新借用。
    let mutable_borrow = &mut point;

    // 通过可变引用来修改数据
    mutable_borrow.x = 5;
    mutable_borrow.y = 2;
    mutable_borrow.z = 1;

    // 报错！不能再以不可变方式来借用 `point`，因为它当前已经被可变借用。
    // let y = &point.y;
    // TODO ^ 试一试去掉此行注释

    // 报错！无法打印，因为 `println!` 用到了一个不可变引用。
    // println!("Point Z coordinate is {}", point.z);
    // TODO ^ 试一试去掉此行注释

    // 正常运行！可变引用能够以不可变类型传入 `println!`
    println!(
        "Point has coordinates: ({}, {}, {})",
        mutable_borrow.x, mutable_borrow.y, mutable_borrow.z
    );

    // 可变引用不再用于其余的代码，因此可以重新借用
    let new_borrowed_point = &mut point;
    println!(
        "Point now has coordinates: ({}, {}, {})",
        new_borrowed_point.x, new_borrowed_point.y, new_borrowed_point.z
    );

    // 错误 point 可变量 已被new_borrowed_point 借用
    // println!("point.z is {}", mutable_borrow.z);
}
