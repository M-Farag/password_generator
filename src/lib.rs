use std::error::Error;

use rand::{seq::SliceRandom, thread_rng};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name="Password generator CLI app",about="A password generator CLI application")]
struct Arguments {
    #[structopt(long="length", short="l", default_value="8")]
    length:usize,

    #[structopt(long="uppercase", short="u")]
    use_uppercase:bool,

    #[structopt(long="numbers", short="n")]
    use_numbers:bool,

    #[structopt(long="special", short="s")]
    use_special:bool,
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
        let mut chars_pool:Vec<char> = "abcdefghijklmnopqrstuvwxy".chars().collect();
        
        if args.use_numbers {
            chars_pool.extend("0123456789".chars());
        }

        if args.use_uppercase {
            chars_pool.extend("ABCDEFGHIJKLMNOPQRSTUVWXY".chars());
        }

        if args.use_special {
            chars_pool.extend("!@#$%^&*()_+-=".chars());
        }

        let as_text:String = (0..args.length).map(
            |_| {
                *chars_pool.choose(&mut thread_rng()).unwrap()
            }
        ).collect();
        
        Ok(Self { as_text, length: args.length })

    }
}