Usar o cargo
```
cargo --version

cargo new teste
cd teste

cargo run .
   Compiling teste v0.1.0 (/home/guilherme/Dev/personal/rust_test/documentation_rust/teste)
    Finished dev [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/teste .`
Hello, world!
```

O cargo gera os seguintes arquivos padrões:
* src/main.rs -> HelloWorld
* Cargo.toml -> Informações do cargo

TOML -> Tom's Obvious, Minimal Language

### Informações do TOML
```
// Informações do pacote
[package]
name = "teste1"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

// Dependencias do pacote
[dependencies]
```

Buildar um executavel
(Tem que estar no diretorio do pacote)
```
cargo build
```

O gerado fica com o mesmo nome do pacote, Ex:
./target/debug/teste1


Cargo.lock
Este arquivo rastreia as versões exatas das dependências do seu projeto


Check confere se o program está compilado ou não, a utilidade desse comando é que ele é bem mais rapido que o build já que não tem toda a parte de construir o executavel
```
cargo check
```

Build --release
```
guilherme@guilherme-B-ON:~/Dev/personal/rust_test/documentation_rust/teste1$ cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
guilherme@guilherme-B-ON:~/Dev/personal/rust_test/documentation_rust/teste1$ cargo build --release
   Compiling teste1 v0.1.0 (/home/guilherme/Dev/personal/rust_test/documentation_rust/teste1)
    Finished release [optimized] target(s) in 0.09s
```

Gera um binário otimizado que será distribuido


LIB RAND
O rust ainda não tem uma lib interna de random, porém, já existem libs extras para essas ações

Quando vc coloca uma lib dentro do cargo.toml e executa um run/build, ele puxa a biblioteca para ser usada

A lib externa é obtida no Crates.io, onde é publicado os codigos e projetos rust

Ele baixa todas as dependencias necessárias da lib


```
cargo update
```
Atualiza as libs com base no numero do versionamento


Várias trechos comentados dentro do cod explicando as ações

Variaveis são imutaveis por padrão, vc tem que setar um mut para ela ser alteravel.
Constantes são eternos imutaveis e podem existir em qualquer nível, más é recomendavel usar elas globalmente afim de evitar redeclarações


Diferença do uso de mut para sobreamento:
* Mut é util para alterar os valores de uma váriavel de forma correeta (Se reutiliza);
* Sobreamento regera a variavel um determinado novo valor, isso é, um novo elemento (Regera);


Tipos de dados
Dois tipos de subconjuntos: Escalar e composto

Escalar -> Inteiros, flutuante, booleanos e string;

Inteiros:
i8,16,32,64,128,size -> Assinado
u8,16,32,64,128,size -> Não assinado

Assinado -> Inteiro positivo, Ex: 0, 127
Não assinado -> Negativo/positivo, Ex: -63, 0, 64

isize e usize -> Referente a arquitetura do PC


Tratamento do overflow
Caso um valor supere o limite teorico, ele vai gerar um problema de panic, sendo um valor true/false, para isso é recomendo que todas as opções do tipo de soma ou entrada de valores possua um tratamento afim de garantir que esse tipo de problema não ocorra


Tipos float
Limitado em f32, f64
f32 -> precisão simples
f64 -> precisão dupla
```
let y: f32 = 3.0; // f32
```

Tipos booleanos
Valor pode ser um true/false
```
let f: bool = false; // with explicit type annotation
```

Tipo char
O char é um valor de 4 bytes do tipo escalar unicode


Tipos compostos
Aqui são arrays, tuplas e demais listas

Tuplas
Listas de tamanho fixo inalteraveis
Pode contem vários tipos de valores diferentes
```
let tup: (i32, f64, u8) = (500, 6.4, 1);
```
Uma tupla vazia é chamada de unit


Matriz
Todos os valores devem ser do mesmo tipo
Vetor de tamanho fixo

P.S: Se vc não tiver certeza sobre usar um vetor ou um array, use o vetor pela flexibilidade dele

Representação de uma matriz:
```
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

Quando vc tenta entrar com um indice não existente, ele vai dar erro e sair da operação, diferente de outras linguagens de baixo nível que até permitem continuar as operações


A estrutura do rust é com base em uso de funções
O modo de escrita é snake_case

Parametros de uma funções:
* Deve haver a declaração de um nome e tipo
* Os parametros são dividos por virgula

```
fn teste(t1: i32, t2: i32) {
	println!("Value: {t1} {t2}");
}
```



Declarações e expressões
* Declarações/instruções são a execução de uma ação e não retornam valores
* Expressões são valores resultantes;
* O uso de ponto e virgula é a diferença da aplicação de um modelo ou do outro;

Exemplo de uma declaração:
```
let y = 6;
```

Exemplo de chamadas de expressões:
* Chamar uma função é uma expressão;
* Chamar uma macro é uma expressão;
* Um novo bloco de escopo criado com chaves é uma expressão;

Ex:
```
    let y = {
        let x = 3;
        x + 1
    };
```

Um ponto importante é que o x + 1 não tem o ponto e virgula no fim por motivos que ele é o retorno do bloco, quando uma expressão não tem o ;, significa que ela se torna um return automaticamente


Funções com retorno
* Funções que desejamos que tenham retornos, devem ter declarados os tipos do retornos
* Caso a função tenha multiplos retornos, use o return para todas as ações 'extras' e deixe a ação padrão sem o ponto e virgula

```
fn teste6(value: bool) -> i32 {
	if value {
		return 10;
	}

	9
}
```


Rust é uma linguagem que fornece informações de erros detalhadas afim de acelerar o desenvolvimento


Rust não converter valores para verificar condições, isso deve ser tratado pelo desenvolvedor


Condições
```
	if value == 1000 {
		println!("t1");
	} else if value == 100 {
		println!("t2");
	} else if value == 10 {
		println!("t3");
	} else {
		println!("t4");
	}
```

Quando ele entra numa if else , o primeiro que der certo, ele já continua para a proxima ação e ignora os que ainda não foram executados



If inline
```
    let number = if condition { 5 } else { 6 };
```


Pilha e Heap
* Pilha, estilo FIFO, todos os dados recebidos são predefinidos e fixos na estrutura;
	* São mais rapidos de operar devido a seu método de organização, porém, são mais custosos de modificar devido a estrutura fixa do FIFO
* Heap, são valores dinamicos em tempo de alteração;
	* São mais demorados para processar devido ao dinamismo e alocação de espaço em memoria


Regras de propriedade
Primeiro, vamos dar uma olhada nas regras de propriedade:

* Cada valor em Rust tem um proprietário .
* Só pode haver um proprietário por vez.
* Quando o proprietário sai do escopo, o valor será eliminado


Uso de strings
* Strings são imutaveis, então não são adequadas a todos os cenários;


Memoria e alocação:
* Memoria é solicitada para a criação de algo
* È devolvida após o termino de uso;
Por esse motivo não há um garbade collect para limpar

```
let s1 = String::from("teste");
let s2 = s1;
```

S2 está fazendo uma referencia a S1, o valor que é um head que fica referenciado para os dois, onde ambos só mantem a as informações de:
* Endereço inicial
* tamanho
* capacidade

QUando haver a limpeza, somente o s2 será limpo pq causa que ele tem o s1 como seu valor, essa copia superficial é chamada de move no Rust e só tem o obj de tonar mais rápido as operações, agora se vc precisa transferir somente o  valor, necessita usar um .clone para duplicar o valor para dentro de outra variavel


Espaços de memoria
Static -> Valores de tamanho fixo, strings literais, programa binario
	* Tempo de vida é a existencia do programa;

Esse modelo de declaração é mais rapido para acessar
```
static _GLOBAL_VALUE: i32 = 10; // Tipo estatico
```

Stack -> Pilha de execuções, variaveis
	* Tamanho dinamico pq depende dq existir para formar a pilha;
	* Porém a stack tem tamanho fixo final com base no tamanho total da pilha;
	* Variaveis, argumentos, funções (stack frame), threads são armazenados na stack;
	* Tempo de vida limitado ao escopo;

Heap -> Valores dinamico em tempo de compilação/execução
	* Setar manualmente o lifetime;
	* Valores em head são compartilhados em threads;
	* Limpeza via RAII;


No rust não há objetos nulos



Uso de referencias
* Referencias são uso do ponteiro de um elemento valido
* Só possui o valor de leitura, não pode alterado sem as necessidades requeridas

Emprestimos mutaveis são validos até o fim do uso daquele que pegou emprestado, se tentar usar no meio do tempo, vai falhar


Gerando uma documentação basica de forma automatica
```
cargo doc
```

Testa o codigo
```
cargo test
```


DOA (Arquitetura orientada a desastres ou VDM (Vai dar merda))
* Observabilidade, esccalabilidade, failover, solid, tdd, cdc, cicd e readme.md;
* Observabilidade -> Ver os detalhes do processo;
	* Opentelemetry é compativel com a linguagem
* Escabilidade -> Aumentar/diminuir capacidade com base na necessidade ou quando quiser;
* failover -> Assincro sempre que possível
	* Voltar em determinado ponto em caso de necessidade
* SOLID -> Padrões de desenvolvimento
	* Principios de design
* TDD -> Garantir todos os retornos possíveis e que não haja imprivisibilidade;
	* Set trechos para o cargo test operar e garantir as regras de negocio
* CDC -> Consumer-driven contracts
	* Garantir a integridade da ferramenta
	* Melhor alterar para aceitar um novo valor que mudar e quebrar onde vc não sabe;
* CICD -> Esteira de entregas e teste de carga
	* Implementar um cenário secundario e mover uma parte do tragefo para aquele lado para testar
	* Method canary

Segundo um pessoal, o Rust demora muito para compilar (Tem que pensar em boas formas de diminuir esse tempo)
