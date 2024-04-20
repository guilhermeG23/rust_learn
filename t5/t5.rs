// Biblioteca externa
use std::io; // Essa é proprietaria do rust

fn main() {

	// Char simples
	let t1 = 't';
	println!("{t1}");

	// Cadeia de caracteres -> Em memoria, um caracter afrente do outro
	/*
	'' -> Caracter
	"" -> String literal
	*/

	// Tipo str
	let t2 = "teste";
	println!("{t2}");

	// String slice
	// String reference
	// Na pratica, o trecho pega a posição de memoria do primeiro caracter e quantas casas a frente ele é usado para dai apresentar o string
	// Esse trecho fica alocado na memoria static, então é só limpa após o fim do programa, mas a variavel é desalocada após o fim do escopo
	let t3: &str = "teste1";
	println!("{t3}");


	// Strings dinamicas precisam usar a memoria heap
	// Heap string
	let mut t4 = String::new();
	// Adicionada caracter por caracter
	t4.push('t');
	t4.push('4');
	// Adiciona uma cadeia de caracter
	t4.push_str(" t4");
	t4.push_str(" t4");
	println!("{t4}");

	// Gerar a string já com o conteudo
	let t5 = String::from("t5");
	println!("{t5}");

	// Entrada de um iteravel
	//let t6 = ['t', '6'];
	//let t7 = String::from_iter(t6);
	//println!("{t7}");

	// Usando o into para fazer a conversão do valor para o tipo da variavel
	// Tem que especificar o tipo do valor para aplicar a ação de into
	let t8: String = "t8".into();
	println!("{t8}");

	// Entrada de valores
	let mut t9 = String::new();
	println!("Entrada: ");

	io::stdin()
	    .read_line(&mut t9)
	    .expect("Deu ruim ler o valor");

	println!("Saida: {t9}");
}
