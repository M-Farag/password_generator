use rand::{thread_rng, seq::SliceRandom};
fn main(){ 

    let length:usize = 10;
    let chars_pool:Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();

    let password:String = (0..length).map(
        |e|{
            *chars_pool.choose(&mut rand::thread_rng()).unwrap()
        }   
     ).collect();

     println!("Password: {}",password);

}