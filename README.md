# 🦀 Rust Fundamentals: Algoritmo Otimizado de Verificação de Primalidade

## 📝 Descrição do Projeto

Este projeto (`primo_rapido`) marca a conclusão do primeiro desafio de lógica e sintaxe em Rust. O objetivo foi criar uma função eficiente e matematicamente otimizada (`is_prime`) para verificar se um número é primo.

O processo de construção deste código serviu para fixar os conceitos básicos de variáveis, tipos otimizados (`u64`) e fluxo de controle.

---

## 💡 Conceitos Essenciais Fixados

A principal conquista deste algoritmo foi o domínio da sintaxe e da eficiência em Rust:

### 1. Otimização Algorítmica (Performance)
* **Raiz Quadrada:** O algoritmo só checa divisores até a raiz quadrada do número (`limite = (n as f64).sqrt() as u64`), garantindo que a função seja logaritmicamente mais rápida para números grandes.

### 2. Fluxo de Controle Idiomático
* **Early Exit (Retorno Antecipado):** Uso do `if n <= 1 { return false; }` para rejeitar o caso base imediatamente, garantindo que o programa não gaste tempo no loop desnecessariamente.
* **Retorno Final Implícito:** A linha `true` no final da função sem ponto e vírgula é o padrão Rust para retornar o valor final, indicando que todas as checagens falharam (e o número é primo).

### 3. Rigidez de Tipos
* Uso de `u64` (Unsigned 64-bit Integer) para garantir que apenas números positivos (IDs, contagens) sejam aceitos.

---

## 💻 Código Final (`src/main.rs`)

O algoritmo que implementa a verificação otimizada:

```rust
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