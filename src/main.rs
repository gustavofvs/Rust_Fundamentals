fn main() {
    println!("--- Teste do Algoritmo Prime ---");
    println!("17 é primo? {}", is_prime(17));
    println!("91 é primo? {}", is_prime(91));
    println!("2 é primo? {}", is_prime(2));
}

fn is_prime(n: u64) -> bool {
    // 1. Otimização: Define o limite de checagem.
    let limite = (n as f64).sqrt() as u64;

    // 2. Caso Base: 1 e números menores não são primos.
    if n <= 1 {
        return false;
    }

    // 3. Loop de Verificação: Checa de 2 até o limite.
    for divisor in 2..=limite {
        // Se o resto da divisão for zero, o número não é primo.
        if n % divisor == 0 {
            return false;
        }
    }

    // 4. Se o loop terminou sem encontrar divisores, o número é primo.
    true
}