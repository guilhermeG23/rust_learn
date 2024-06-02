//Usando lib de terceiros
use rpassword:: prompt_password;

pub fn esperar_enter() {
	prompt_password("Pressione ENTER para continuar...").unwrap();
}

pub fn limpar_terminal() {
	print!("{esc}c", esc = 27 as char);
}

pub fn exibir_menu(titulo: &str, itens: &[&str], sair: bool) -> u32 {
	limpar_terminal()

	let completo: String = String::from("Masterclass Rust ::")+ titulo;
	println!("{}", completo);
	println!("{}", String::from("=").repeat(completo.len()));

	return 10;
}
