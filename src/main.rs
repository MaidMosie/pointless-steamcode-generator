
use rand::Rng;
fn main() {
    //this has like a 0.00000000000000002% chance of being a valid code btw
    println!("");
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            0123456789";
    const PASSWORD_LEN: usize = 5;
    let mut rng = rand::thread_rng();

 let part1: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

 let part2: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();      

 let part3: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    println!("{part1}-{part2}-{part3}");
}
