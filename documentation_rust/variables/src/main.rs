use std::collections::btree_map::Values;

static _GLOBAL_VALUE: i32 = 10; // Tipo estatico

fn main() {
	// Aqui são invocações diferentes
	// Sombreamento
	// Aqui é como substituir a existencia do elemento
	let x = 5;
	println!("The value of x is: {x}");
	let x = 6;
	println!("The value of x is: {x}");

	let y = 10;
	let y = y + 1;
	println!("The value of y is: {y}");

	teste();
	teste1();
	teste2();
	teste3();
	teste4();
	teste5(1,2);

	let value1 = teste6(true);
	let value2 = teste6(false);
	println!("value1: {value1}");
	println!("value2: {value2}");

	teste7();
	teste8(true);
	teste9();
	teste10();
	teste11();
	teste12();
	teste13();
	teste14();
	teste15();
	teste16();
	teste17();
	// Retorno multiplo
	let (v1, v2) = teste18(1,String::from("teste"));
	println!("v1: {}", v1);
	println!("v2: {}", v2);

	teste19();
	teste20();
	teste21();
}

fn teste() {
	// Aqui necessita de mut pq vc altera
	// uma variavel já existente se chamar ela 
	// com um let denovo
	let mut t1 = 10;
	println!("T1: {t1}");
	t1 = 11;
	println!("T1 alterado: {t1}");
}

fn teste1() {
	// Diferenças de nível de escopo
	let t2 = 10;
	{
		let t2 = 12;
		println!("Value: {t2}");
	}

	println!("Value: {t2}");
}

// Tratamento de overflow
fn teste2() {
	let a: u8 = 255;
	let b: u8 = 2;
	
	match a.checked_add(b) {
	    Some(result) => println!("Soma: {}", result),
	    None => println!("Overflow ao somar"),
	}
}

fn teste3() {
	let tup = (1,1,1);
	let (a,_b,_c) = tup;
	println!("TUP: {a}");

	let tup1 = (1,2,3);
	println!("TUP1: {}", tup1.2);
}

fn teste4() {
	let a: [i32; 5] = [1, 2, 3, 4, 5];
	println!("Matriz: {}", a[1]);
}

fn teste5(t1: i32, t2: i32) {
	println!("Value: {t1} {t2}");
}

fn teste6(value: bool) -> i32 {
	if value {
		return 10;
	}

	9
}

fn teste7() {
	let value = 1;

	if value == 1000 {
		println!("t1");
	} else if value == 100 {
		println!("t2");
	} else if value == 10 {
		println!("t3");
	} else {
		println!("t4");
	}
}

fn teste8(condition: bool) {
	let number = if condition { 5 } else { 6 };
	println!("teste8: {number}");
}

// Retorno de loops
fn teste9() {
	let mut counter = 0;

	let result = loop {
	    counter += 1;
	    if counter == 10 {
		break counter * 2;
	    }
	};

	println!("The result is {result}");
}


// Rotulos de loop
// Finaliza a ação de um loop em especifico
fn teste10() {
	println!("---------------------------------");
	let mut count = 0;
	'counting_up: loop {
	    println!("count = {count}");
	    let mut remaining = 10;
    
	    loop {
		println!("remaining = {remaining}");
		if remaining == 9 {
		    break;
		}
		if count == 2 {
		    break 'counting_up;
		}
		remaining -= 1;
	    }
	    println!("---------------------------------");

	    count += 1;
	}
	println!("End count = {count}");
}


fn teste11() {
	let mut number = 3;
    
	while number != 0 {
	    println!("{number}!");
    
	    number -= 1;
	}
    
	println!("LIFTOFF!!!");
}

// percorrendo um array
fn teste12() {
	let a = [10, 20, 30, 40, 50];
	let mut index = 0;
    
	while index < 5 {
	    println!("the value is: {}", a[index]);
    
	    index += 1;
	}
}

// Foreach
fn teste13() {
	let a = [10, 20, 30, 40, 50];
	for value in a {
		println!("{value}");
	}
}

// range
fn teste14() {
	println!("-----------------------");
	// Vai de 1 a 3
	for number in (1..4).rev() {
		println!("{number}!");
	}
	println!("LIFTOFF!!!");
}

//String
fn teste15() {
	let mut s = String::from("hello");

	s.push_str(", world!"); // push_str() appends a literal to a String
    
	println!("{}", s); // This will print `hello, world!`
}

fn teste16() {
	let s1 = String::from("teste");
	// Fazendo uma copia do valor para o s2
	let s2 = s1.clone();
	println!("{s2} {s1}");
}

fn teste17() {
	println!("teste17 static: {}", _GLOBAL_VALUE);
}

fn teste18(value_int: i32, value_str: String) -> (i32, String) {
	(value_int, value_str)
}

// Referencia - Imutaveis por padrão
fn teste19() {
	let value = String::from("teste");
	let len_value = len_value(&value);
	println!("value: {}, len {}", value, len_value);
}

fn len_value(value: &String) -> usize {
	value.len()
}

// Referencias mutaveis
// Aqui foi ajustado para ser possivel alterar o valor
// com base na referencia
fn teste20() {
	let mut value = String::from("teste");
	println!("Value: {}", value);
	complete_value(&mut value);
	println!("Value: {}", value);
}

fn complete_value(value: &mut String) {
	value.push_str(" teste")
}

// Mutabilidade com base em nivel de acesso e fim
// do acesso
fn teste21() {
	let mut s = String::from("hello");

	{
	    let r1 = &mut s;
	    println!("{}", r1);
	} // r1 goes out of scope here, so we can make a new reference with no problems.
    
	let r2 = &mut s;
	println!("{}", r2);
}
