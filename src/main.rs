fn main() {
    println!("Prime 17 {}", is_prime(17));
    println!("Prime 1 {}", is_prime(1));
}

fn is_prime(n: u64) -> bool {
    let limite = (n as f64).sqrt() as u64;

    if n <= 1 {
        return false;
    }

    for divisor  in 2..=limite {
        if n % divisor == 0 {
            return false;
        }
    }

    true
}
