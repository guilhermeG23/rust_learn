Cargo -> Ferramenta administrativa do Rust
* Criado para gerenciar projetos, dependecias e ademais;

```
cargo new proj
```

Cria um binary projetct com nome de proj

Gera um estrutura do rust:
* cargo.toml -> Informações da estrutura


Para se usar o cargo, é necessario ter um main dentro do src para haver o funcionamento da ferramenta

Quando se tem essa estrutura o projeto fica como um rust workspace
* Para compilar o projeto, entre no diretorio e execute:

```
cargo build -> Gera o projeto
cargo run -> Gera o projeto e executa ele
cargo build --release -> Gera o projeto com otimnização (Diminuir o tamanho total do projeto)
```

Cargo pode ser customizado com extensões, Ex:
* ```cargo fmt``` -> Formata o code automaticamente

Ferramenta cargo-watch é para monitorar alteraçãos no estado do codigo:
```
cargo install cargo-watch
```

Executar a ação do watch
```
cargo watch -x run
```

Ele fica monitorando e recompilando em tempo de save para amostrar em terminal as alterações
