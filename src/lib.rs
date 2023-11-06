// use data_encoding::HEXUPPER;
use error_chain::error_chain;
use ring::digest::{Context, Digest, SHA256};
use std::io::Read;

mod add_two;

error_chain! {
   foreign_links {
       Io(std::io::Error);
       Decode(data_encoding::DecodeError);
   }
}

pub fn sha256_digest<R: Read>(mut reader: R) -> Result<Digest> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok(context.finish())
}

pub fn sheet() {
    println!("hello, world, {}", add_two::add_two::add_two(2));
}
