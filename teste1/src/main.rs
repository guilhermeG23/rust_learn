mod utils;

// Importar a ação do utils para ser executada dentro do shell rust
use utils::terminal::{limpar_terminal, esperar_enter};

fn main() {
	println!("Hello, world!");
	limpar_terminal();
	println!("Hello, world!");
	esperar_enter();
	println!("Hello, world!");
}
