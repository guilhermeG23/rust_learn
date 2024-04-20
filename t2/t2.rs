fn main() {
	let _teste = "2";
	let teste = "2";
	println!("{}", teste);

	// Esplicitar o valor
	let teste1: i32 = 2;
	println!("{}", teste1);

	// Mutabilidade
	let mut teste2 = 5;
	println!("{}", teste2);

	teste2 = 6;
	println!("{}", teste2);

	// shadowing
	// Usar um mesmo nome apos ser iniciado 
	// com um valor de tipo diferente
	let teste3 = 10;
	println!("{}", teste3);

	let teste3 = "teste";
	println!("{}", teste3);

	// Isolamento de escopo
	// Shadowing de escopo
	let teste4 = 10;
	{
		// Oq faz aqui nao altera o externo por ser de um escopo diferente
		// Vc pode usar o externo, más o contrario nao
		let teste4 = teste4 + 100;
		println!("{}", teste4);
	}
	println!("{}", teste4);

}
