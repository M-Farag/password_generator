use std::error::Error;

use rand::{seq::SliceRandom, thread_rng};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name="Password generator CLI app",about="A password generator CLI application")]
struct Arguments {
    #[structopt(long="length", short="l", default_value="8")]
    length:usize
}

impl Arguments {
     fn new() -> Result<Self,Box<dyn Error>>
    {
         let args = Arguments::from_args();
            if args.length > 35 {
               return  Err("Password length must be at least 35 characters long".into());
            }

            if args.length < 3 {
                return Err("Password length must be at least 3 characters long".into());
            }

            Ok(args)
    }
}

#[derive(Debug)]
pub struct Password {
    as_text:String,
    length:usize
}

impl Password {
    pub fn new() -> Result<Self,Box<dyn Error>>
    {
        let args = Arguments::new()?;
        let chars_pool:Vec<char> = "abcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
        let as_text:String = (0..args.length).map(
            |_| {
                *chars_pool.choose(&mut thread_rng()).unwrap()
            }
        ).collect();
        
        Ok(Self { as_text, length: args.length })

    }
}