use rand::{thread_rng, seq::SliceRandom};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name="Password generator",about="My password generator")]
struct Arguments {
    #[structopt(long="length", short="l", default_value="8")]
    length:usize
}
fn main(){ 

    let arguments = Arguments::from_args();

    println!("My arguments: {}",arguments.length);

    let length:usize = 10;
    let chars_pool:Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();

    let password:String = (0..length).map(
        |e|{
            *chars_pool.choose(&mut rand::thread_rng()).unwrap()
        }   
     ).collect();

     println!("Password: {}",password);

}