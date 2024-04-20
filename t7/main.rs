// Funcoes tem que ser escritas em snake_case
// Ex: fn test_one()

// Toda a funcao que tem retornos, tem que apontar o tipo do retorno
fn t5(v1: i32, v2: i32) -> i32 {
	return v1 + v2;
}

// Nao precisa necessariamente setar o return, o ultimo valor
// apresentado pela funcao, ira retornar ao requisitante
// A ultima expressão normalmente sera o retorno da funcao
fn t6(v1: i32, v2: i32) -> i32 {
	11;
	v1 + v2
}

/*
Ao colocar ; numa sequencia, vc invalida ela como retorno
Se tudo tiver ; no final da linha, a ação do block vai retornar um
unittype (vazio)
*/


// {} -> Kerley breaktes
fn main() {
	t1();
	t2("teste2");
	t3(1);
	t4();

	// Bloco de expressões
	// Unit type -> usado quando não tem valor de retorno especifico
	let value1: () = {
		t1();
		t2("teste");
	};

	println!("{:?}", value1);

	// Agora com funcoes que tem um retorno

	let value2: i32 = t5(1, 2);

	println!("{:?}", value2);

	// O ultimo retorno das ações do bloco é oq sera o valor do escopo superior
	// O ultimo valor sem o ; sera o que vai para o escopo superior
	let value3 = {
		22;
		23
	};
	println!("{:?}", value3);

	println!("----------\nFuncao T6");
	println!("{:?}", t6(1,211));

	println!("----------\nFuncao T10");
	println!("{}", t10(true));
	println!("{}", t10(false));

	println!("----------\nFuncao T11");
	t11("1 2 4 4 5 6");
}

// Funcao de extra
fn t1() {
	println!("teste");	
}

// Parametros
// É obrigado a setar o tipo da entrada a ser usada
fn t2(value: &str) {
	println!("{}", value);
}

// Parametro tem que seguir o snake_case
fn t3(value: i32) {
	println!("{}", value);
}

// Em rust, não tem parametros opcionais, se a funcao tem, toda a vez que tem chamar a funcao com o parametro
// Como paleativo, vc pode criar um obj que vai ter os valores default para a funcao
// parametros em rust são tratados com base em posicao e não pelas chaves que eles compoem, então não pode apontar a chave valor do argumento
// * Pater-mett

// Macros permitem chamar ações sem parametros

// Funcoes em rust tem o valor extra de gerenciamento de memoria devido
// Ao que vai acionar, a compilação em tempo de boot e ademais


// ---------------------------
/*
statements -> Key worlds que fazem algo más não produz retorno;
expression -> Key worlds que fazem algo e que produzem retorno;
*/

static T1: i32 = 5 * 5;
fn t4() {
	println!("{}", T1);
}

// Return tem mais valor para tratamento
// Onde vc precisa que a funcao escape uma ação em vez de
// Retornar o valor default
fn t10(value: bool) -> i32 {
	// Early return -> Escape de condição
	if value {
		return 99;
	}
	88
}

// Funcao de composicao
fn clear_array(values: &str) -> i32 {
	values.parse().unwrap()
}

fn t11(values: &str) {
	let array_values: Vec<i32> = values
	    .trim()
	    .split(" ")
	    .map(clear_array)
	    .collect();
	
	println!("{:?}", array_values);

	// para funcao curtas de 1 linha, melhor
	// fazer uma funcao anonima dentro do map
	// Unwrap vai usar o contexto do tipo do
	// valor do let para saber que é para virar um i32

	// O i32 que teve no v2 é pq ele nao
	// consegue trabalhar com o tipo inicia dos dados
	// por ele ter sido convertido, formas de resolver isso
	// é especificando o tipo v2
	let array_values1: Vec<i32> = values
	    .trim()
	    .split(" ")
	    .map(|v1| v1.parse().unwrap())
	    .map(|v2: i32| v2 * 2)
	    .collect();
    
	println!("{:?}", array_values1);

	// O falando o tipo de retorno da ação anterior
	let array_values1: Vec<i32> = values
	    .trim()
	    .split(" ")
	    .map(|v1| v1.parse::<i32>().unwrap())
	    .map(|v2| v2 * 2)
	    .collect();

	println!("{:?}", array_values1);
}
