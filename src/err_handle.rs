pub mod err_handle {
    use std::{fs::File, io::ErrorKind};

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
    }
}
