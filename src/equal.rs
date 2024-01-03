pub mod equal_one {
    use rand::{
        distributions::{Distribution, Uniform},
        Rng,
    };

    pub fn equal_one(base: u32) -> bool {
        base == 5
    }

    pub fn rand() {
        let mut rng = rand::thread_rng();
        let n1: u8 = rng.gen();
        let n2: u16 = rng.gen();

        println!("  u8    随机数： {}", n1);
        println!("  u16   随机数： {}", n2);

        println!("  u32   随机数： {}", rng.gen::<u32>());
        println!("  i32   随机数： {}", rng.gen::<i32>());
        println!("  float 随机数： {}", rng.gen::<f64>());
    }

    pub fn rand_num() {
        let mut rng = rand::thread_rng();

        println!("  整数：   {}", rng.gen_range(0..10)); // 半开放范围取随机值，即包括`低位`而不包括`高位`
        println!("  浮点数： {}", rng.gen_range(0.0..10.0));

        let die = Uniform::from(1..90);

        loop {
            let throw = die.sample(&mut rng);
            println!("  丢一次骰子： {}", throw);

            if throw == 6 {
                break;
            }
        }
    }
}

pub mod equal_two {
    use rand::thread_rng;
    use rand_distr::{Distribution, Normal, NormalError};

    pub fn normal() -> Result<(), NormalError> {
        let mut rng = thread_rng();
        let normal = Normal::new(-2.0, 99.0)?;
        let v = normal.sample(&mut rng);
        println!("  正态分布： {}", v);
        Ok(())
    }
}
