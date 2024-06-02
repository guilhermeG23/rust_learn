fn main() {
    println!("Hello, world!");
    teste();
}

fn teste() {
	let nome: String = String::from("teste");
	let idade: i32 = 10;

	println!("{}", nome);
	println!("{}", idade);
	teste1(&idade);
	teste2();
}

//Chamando uma referencia de memoria
fn teste1(idade: &i32) {
	println!("{}", idade);
}

// Ownnership
// Pode emprestar ou transferir a posse da variavel

//Ex: de transferencia de posse
fn teste2() {
	let t1: i32 = 10;
	// Isso vai dar errado pq vai está trasferindo a posse
	// do t1 para o t12 
	//let t12: i32 = t1;
	println!("{}", t1);

	// Uma forma de resolver isso é usando um clone
	// Vc copia o estado atual para o novo estado mantendo
	// o antigo em funcionamento

	let t2: i32 = 11;
	let t21: i32 = t2.clone();
	println!("{}", t2);
	println!("{}", t21);

	// Copia de valores automaticos por estarem na stack
	let t3 = 30;
	let t31 = t3;
	println!("{} {}", t3, t31);
}

// Estados do heap
// clone -> Copia o valor para outro cara
// ownership -> Mover a posso da variavel
// borrowing -> Emprestar a variavel para outro

