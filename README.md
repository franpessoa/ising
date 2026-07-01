# Modelo de Ising
## Como rodar
É necessário ter a linguagem de programação Rust instalada.
Para isso, é recomendada a ferramenta [rustup](https://rustup.rs/).

No diretório raíz, rode
```bash
cargo run --bin ising --release
```

## Configurando
Por enquanto, as configurações estão "hard-coded" no código, no arquivo [ising/src/main.rs](ising/src/main.rs), dentro da função `main()`.

Para alterar o tamanho do grid, use as variáveis inteiras `x` e `y`, no começo da função.
```rust
fn main() {
    let x = 100;
    let y = 100;

...
```

Para alterar os parâmetros H, J e a temperatura inicial, modifique o campo `config` da variável `sys`.
```rust
let mut sys = Ising {
        lambda: PBCGrid::new_from_vec(
            x,
            y,
            (0..x * y)
                .into_iter()
                .map(|_| Spin::new(rng.random()))
                .collect::<Vec<Spin>>(),
        ),
        config: Config {
            h: 0.0, // PARÂMETRO H/FORÇA DO CAMPO EXTERNO
            j: 1.0, // PARÂMETRO J/FORÇA DAS INTERAÇÕES ENTRE VIZINHOS
            t: 6.0, // TEMPERATURA INICIAL
        },
    };
```

Para alterar o decréscimo de temperatura, o número de iterações e a temperatura mínima, modifique o loop principal:
```rust
for en in 0..1500000 { // NÚMERO DE ITERAÇÔES
    if en % 500 == 0 && sys.config.t > 1.0 { // 500 = NÚMERO DE PASSOS POR DECRÉSCIMO, 1.0 = TEMPERATURA MÍNIMA
        sys.config.t -= 0.005 // 0.005 = DECRÉSCIMO POR LOOP
    }
    sys = metropolis_pass(sys, rand::rng());
    res.push(sys.clone());
}
```
