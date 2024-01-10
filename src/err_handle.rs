pub mod err_handle {
    use std::{
        fs::{File, OpenOptions},
        io::{ErrorKind, Read, Write},
        net::IpAddr,
    };

    use std::process;

    pub fn err_result() {
        {
            // match 错误处理
            let f = File::open("hello.txt");

            let f = match f {
                Ok(file) => file,
                Err(error) => {
                    panic!("file not open: {:?}", error)
                }
            };
            println!("{:?}", f);
            // let s = f.write(b"test");

            // let s = match s {
            //     Ok(num) => num,
            //     Err(error) => {
            //         panic!("fail to write to file: {:?}", error)
            //     }
            // };

            // println!("{s}");
        }
        {
            // match 错误处理 并对错误修复
            let f = File::open("hello.txt");

            let f = match f {
                Ok(file) => file,
                Err(error) => match error.kind() {
                    ErrorKind::NotFound => match File::create("hello.txt") {
                        Ok(fc) => fc,
                        Err(e) => panic!("Problem creating the file: {:?}", e),
                    },
                    other_error => panic!("Problem opening the file: {:?}", other_error),
                },
            };
            println!("{:?}", f);
            // let s = f.write(b"test");

            // let s = match s {
            //     Ok(num) => num,
            //     Err(error) => {
            //         panic!("fail to write to file: {:?}", error)
            //     }
            // };
            // println!("{s}");
        }
        // ? 直接返回 Result<T>
        let path = "hello1.txt";
        let mut output = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .append(true)
            .open(path)
            .unwrap_or_else(|e| {
                println!("打开文件错误: {e}");
                process::exit(1);
            });
        println!("{:?}", output);
        // let mut output = OpenOptions::new() // 可以追加写入
        //     .write(true)
        //     .create(true)
        //     .append(true)
        //     .open(path)?;

        let write_number = output
            .write(b"We will generate a digest of this text, me.\n")
            .unwrap();
        println!("{write_number}");
        // let write_number = output.write(b"sssss\n").unwrap();
        // println!("{write_number}");

        {
            let mut username = String::new();
            File::open("hello.txt")
                .unwrap_or_else(|e| {
                    println!("打开文件错误: {e}");
                    process::exit(1);
                })
                .read_to_string(&mut username)
                .expect("fail read to file");
            println!("{username}");
        }
        {
            let home: IpAddr = "127.0.0.1"
                .parse()
                .expect("Hardcoded IP address should be valid");
            println!("{:?}", home);
        }
    }
}
