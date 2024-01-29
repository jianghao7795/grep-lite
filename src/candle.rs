use candle_core::Error;
use candle_core::{Device, Tensor};

pub fn candle_explmt() -> Result<(), Error> {
    let a = Tensor::arange(1f32, 7f32, &Device::Cpu)?;
    let b = Tensor::arange(7f32, 13f32, &Device::Cpu)?;

    println!("{:?}", &a + &b);
    println!("{:?}", &a - &b);
    println!("{:?}", &a * &b);
    println!("{:?}", &a / &b);
    println!("{:?}", &a.sqr());
    println!("{:?}", &a.sqrt());
    println!("{:?}", &a.log());
    println!("{:?}", &a.exp());
    println!("{:?}", &a + 2f64);

    let s_ = (&a * &b)?.sum_all()?;
    println!("{}", s_);

    Ok(())
}
