// Biblioteca que permite inputs
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
	// Um loop infinito
	loop {
		println!("Guess the number!");

		println!("Please input your guess.");

		// Obtendo um valor secreto
		let secret_number = rand::thread_rng().gen_range(1..=100);
		println!("{secret_number}"); 

		// Invocando uma instancia de string vazia para uso
		let mut guess = String::new();

		// Os valores inputados podem ser tratados como, OK e ERR
		// Se o valor for incompativel, o ERR será usado e 
		// Levara ao uso do expect
		// Em caso de OK, vai ser usada a entrada inputado
		io::stdin()
		.read_line(&mut guess)
		.expect("Failed to read line");

		// Aqui esta reatribuindo o valor
		// para um inteiro após a obtencao
		// dele como uma string

		// trim limpa espaços e quebra de linhas antes e após
		// os valores, o parse é um suporte de string
		// que ajuda a converter valores
		//let guess: u32 = guess.trim().parse().expect("Please type a number!");
		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num, 
			Err(_) => continue,
		};

		println!("You guessed: {guess}");

		// Pode usar as duas formas de obter valores
		// Para serem apresentados
		let x:i32 = 32;
		println!("{} --- {guess} --- {secret_number}", x); 

		// Aqui lista o 3 tipos de resultados possiveis de
		// Uma comparação de valores, menor, igual ou maior
		match guess.cmp(&secret_number) {
		    Ordering::Less => println!("Too small!"),
		    Ordering::Greater => println!("Too big!"),
		    Ordering::Equal => {
			// Multiplas acoes dentro de uma match
			println!("You win!");
			break;
		    }
		}
	}
}

/*
fn main() {
	println!("Guess the number!");

	println!("Please input your guess.");

	// Invocando uma instancia de string vazia para uso
	let mut guess = String::new();

	// Invocando diretamente a lib sem a necessidade do use
	std::io::stdin()
	    .read_line(&mut guess)
	    .expect("Failed to read line");

	println!("You guessed: {guess}");
}
*/
