use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub fn signed_demo() {
    // 创建一个消息通道, 返回一个元组：(发送者，接收者)
    let (tx, rx) = mpsc::channel();
    // for i in 0..5 {
    //     // 创建线程，并发送消息

    // }

    thread::spawn(move || {
        // 发送一个数字1, send方法返回Result<T,E>，通过unwrap进行快速错误处理
        tx.send(3).unwrap();

        // 下面代码将报错，因为编译器自动推导出通道传递的值是i32类型，那么Option<i32>类型将产生不匹配错误
        // tx.send(Some(1)).unwrap()
    });
    println!("receive {}", rx.recv().unwrap());

    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move || {
        tx.send(String::from("hi from raw tx")).unwrap();
    });

    thread::spawn(move || {
        tx1.send(String::from("hi from cloned tx")).unwrap();
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

pub fn signed_double() {
    let (send, recv) = mpsc::channel();

    let num_threads = 3;
    for i in 0..num_threads {
        let thread_send = send.clone();

        thread::spawn(move || {
            thread_send.send(i).unwrap();
            println!("thread {:?} finished", i);
        });
    }
    drop(send); // 通道关闭的两个条件：发送者全部drop或接收者被drop，要结束for循环显然是要求发送者全部drop，但是由于send自身没有被drop，会导致该循环永远无法结束，最终主线程会一直阻塞。
    for x in recv {
        println!("Got: {}", x);
    }
    println!("finished iterating");
}
pub fn mutex() {
    // let m = Mutex::new(5);

    // let mut num = m.lock().unwrap();
    // *num = 7;

    // let mut num1 = m.lock().unwrap();
    // *num1 = 9;
    // // num.unlock();

    // println!("m = {:?}", m);
}

pub fn thread_arc() {
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
