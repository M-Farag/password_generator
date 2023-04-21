use rand::{seq::SliceRandom, thread_rng};
use structopt::StructOpt;

const CHARS_POOL:&str = "abcdefg";

#[derive(Debug, StructOpt)]
#[structopt(name="Password generator CLI app",about="A password generator CLI application")]
struct Arguments {
    #[structopt(long="length", short="l", default_value="8")]
    length:usize
}

impl Arguments {
     fn new() -> Self
    {
         Arguments::from_args()
    }
}

#[derive(Debug)]
pub struct Password {
    as_text:String,
    length:usize
}

impl Password {
    pub fn new() -> Self
    {
        let args = Arguments::new();
        let chars_pool:Vec<char> = CHARS_POOL.chars().collect();
        let as_text:String = (0..args.length).map(
            |_| {
                *chars_pool.choose(&mut thread_rng()).unwrap()
            }
        ).collect();
        
        Self { as_text, length: args.length }

    }
}