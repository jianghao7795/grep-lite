use futures::executor::block_on;

struct Song {
    author: String,
    name: String,
}

async fn learn_song() -> Song {
    Song {
        author: "周杰伦".to_string(),
        name: String::from("《菊花台》"),
    }
}

async fn sing_song(song: Song) {
    println!(
        "给大家献上一首{}的{} ~ {}",
        song.author, song.name, "菊花残，满地伤~ ~"
    );
}

async fn dance() {
    println!("唱到情深处，身体不由自主的动了起来~ ~");
}

async fn learn_and_sing() {
    let song = learn_song().await;
    sing_song(song).await;
}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();
    // `join!`可以并发的处理和等待多个`Future`，若`learn_and_sing Future`被阻塞，那`dance Future`可以拿过线程的所有权继续执行。若`dance`也变成阻塞状态，那`learn_and_sing`又可以再次拿回线程所有权，继续执行。
    // 若两个都被阻塞，那么`async main`会变成阻塞状态，然后让出线程所有权，并将其交给`main`函数中的`block_on`执行器
    futures::join!(f2, f1); //会按照参数的顺序执行
}

pub fn run_async() {
    block_on(async_main());

    let mut k = 9;
    let l = k;
    k += l;
    println!("k is {}", k);
    println!("l is {}", l);
    // k += l;

    // k -= l;

    // k |= l;
    let mut borrow = 10;

    let deref = &mut borrow;
    println!("{}", *deref);
    *deref = 99;
    // drop(deref);

    println!("{}", borrow);
    let bb = &borrow;
    println!("{}", bb);
    // println!("{}, {}", *deref, borrow);// 可变引用同时只能存在一个 *deref, borrow不能同时使用

    let mut foo = 6;
    let mut borrowed_foo = &foo;
    println!("{}, {}", borrowed_foo, foo);
    borrowed_foo = &9;

    println!("{}, {}", borrowed_foo, foo);

    foo = 3;
    println!("{}, {}", borrowed_foo, foo);
}
