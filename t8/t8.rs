/*
* OB -> Ownership & Borrowing
* Borrow checker -> Analise da gestão de memoria
	* Trabalha em tempo de compilador para verificar o codigo
	* Verifica em nível de funcao

Oq ele verifica?
* Null pointer exception -> Apontamento para valor NULL
* segmentation fault -> Referenciar valor null da ruim
* Memory leak -> Muito uso da RAM em momento não necessários
* dangling pointers -> Ponteiros para lugar nenhum ou limpa
* Double free -> Limpeza em pontos já limpos
* Use after free -> Aponta em locais limpos
* Data races -> Valor mutavel que está sendo alterado por mais de 1 ao mesmo tempo
	* Não tem garantia do estado de dados
*/

/*
Tipagem e gestão dos valores
OBRM -> Borrow checker memory

Tipos no RUST
* 
*/
fn main() {
	let t1: i32 = 1; // Tipo copy -> Tipo primitivo escalavel (f64, bool, char)
	let t2: i32 = t1;
	/*
	O Rust copia os valores de a para b
	o b por exemplo não pode se dizer igual que a
	Então eles não se interferem
	*/
	println!("T1: {}", t1);
	println!("T2: {}", t2);
	
	let t3: &i32 = &t1;
	// Referencia direta a variavel
	// Para fazer o apontamento use &
	// Para fazer o uso, use *
	println!("T3: {}", *t3);
	// * é usando somente alguns pontos

	// String é usada em memoria HEAP
	// Static está instanciando um HEAP
	// Respeitar regras de onnership
	let t4 = String::from("T4"); // Tipo não copy

	println!("T4 - {}", t4);
	// O motivo de não ser heap é que demora muito para
	// fazer esses tipos de ações, ele é caro

	// Se ele não pode clonar e não pode fazer referencia
	// Ele faz oq? Ele move a propriedade
	let t5 = t4;

	//Ex: println!("T4 - {}", t4);
	// A variavel vira t5 e a t4 é desativada (removida) após ser transferida
	// Se acionar ela antes da transferencia, tmb não é possível
	// Entao t4 se torna um t5
	println!("T4 - {}", t5);

	// Emprestimo de uma variavel que chama o heap
	// O emprestimo só requisita a chamada ao valor com somente
	// permissao de leitura
	let t6 = &t5;
	println!("T6 - {}", t6);

	// Funcao externa
	tt1("teste1");

	// Converte para o modelo string
	let teste1: String = String::from("teste");
	// Clone faz uma copia do valor e manda para o requisitante
	tt2(teste1.clone());
	tt2(teste1.clone());
	// Referencia do valor
	// Emprestimo
	// Borrowing
	tt3(&teste1);
	tt3(&teste1);

	// Emprestimos de referente mutavel
	// Sò pode alterar o estado uma vez por chamada
	// Necessario terminar um processamento para o proximo
	let mut teste2 = "teste".to_string();
	tt4(&mut teste2);
	tt5(&mut teste2);
	println!("{}", teste2);
}

// Funcoes externas
fn tt1(value: &str) {
	println!("TT1 - {}", value);
}

// Recebe uma string da HEAP
fn tt2(value: String) {
	println!("TT2 - {} -- 1", value);
}

// Recebe uma referencia de string da HEAP
fn tt3(value: &String) {
	println!("TT3 - {} -- 1", value);
}

// Emprestimo mutavel em memoria heap
// Para fazer o emprestimo mutavel por endereço
// E necessário fazer o apontamento em memoria
fn tt4(value: &mut String) {
	*value = value.to_uppercase()
}

fn tt5(value: &mut String) {
	*value = format!("2222_{value}")
}
