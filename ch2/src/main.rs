use rand::Rng;

fn main() {
    let random = rand::thread_rng().gen_range(1..=100);
    println!("{}", random);
}
