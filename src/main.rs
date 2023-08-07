use rand::Rng;

fn main() {
    // let mut rng = rand::thread_rng();
    // let n1: u8 = rng.gen();
    // let n2: u32 = rng.gen();
    // println!("{}-{}", n1, n2);
    // println!("{}", rng.gen::<u32>());
    // println!("{}", rng.gen::<f32>());

    let mut rng = rand::thread_rng();
    println!("{}", rng.gen_range(0..10));
}
