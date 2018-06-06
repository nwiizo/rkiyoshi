extern crate rand;
use rand::Rng;

fn main() {
    let mut zndk = 0;
    let mut rng = rand::thread_rng();
    let mut zndk_count = 0;
    loop {
        let cb:bool = rng.gen();
        print!("{} ", if cb {"ZUN!"} else {"DOKO!"});
        zndk_count += 1;
        if cb {
            zndk += 1
        } else if zndk >= 4 {
            break
        } else {
            zndk = 0
        }
    }
    println!("KIYOSHI!!!!!!!!");
    println!("count:{}",zndk_count)
}
