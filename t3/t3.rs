// Tipos em rust
/*
Escalares -> 
	* Represeta um unico valor contido dentro de uma escala conhecida
	* Permite a comparação direta entre valores
	* Tipos:
		* Inteiros, flutuantes, booleano e char (string -> UTF-8/unicode)
Compostos -> 
	* Servem para agregar multiplos valores
	* tipos:
		* tupla -> (1,2,3,4,'a')
		* matriz -> [1,2,3,4,5]
			* Coleção de tipos iguais
*/

/*

inteiros
Trabalha com signed e unsigned
* singned -> Trabalha com sinais (+-)
* unsigned -> Não trabalha com sinais

8 	| i8	| u8 
16 	| i16	| u16 
32 	| i32	| u32 
64 	| i64	| u64 
128 	| i128	| u128 
arch 	| isize | usize 

arch -> trabalha com o tamanho da arquitetura

Por default ele seta o i32 como int caso detecte um inteiro
Se setar com um valor maior que o suportado pelo range, ele da erro

*/

fn main() {
	// Tipos de erros de limite
	//let _t1: u8 = 256;
	//let _t2: u8 = -256;
	//let _t3: u8 = 10 * 100;

	// Pode usar o underline para separar os numeros e facilitar o entendimento
	let _t1: i32 = 199_199_199;

	// É possível passar valores de multiplas formas como:
	let _t2 = 0xff;
	let _t3 = 0o12;
	let _t4 = 0b111;
	let _t5 = b'a';

	//Float
	// Por padrão é f64
	// f64 tem dupla precisão
	let _t6 = 60.00;
	let _t7: f32 = 40.00;
	let _t8: f64 = 40.00;

	//bool
	let _t9 = true;
	let _t10 = false;
	let _t11: bool = true;

	//char
	// 4 bites por char
	// Pode ser qualquer simbolo da tabela unicode com 4 bites
	// tem que usar aspas simples
	let _t12 = 'a';
	let _t12: char = 'a';

	//tupla
	// tem tamanho fixo
	// Pode setar mais de um tipo na tupla
	// Uma vez criada, ela não pode ser aumentada ou diminuida
	// porem pode ser alteravel com o mut
	// mas o mut é limitado ao tipo inicial da posicao da tupla, isso é, inteiro nao pode receber nada alem de inteiro
	// porem, voce pode mudar a tupla toda contanto que respeite a quantidade de elementos iniciais
	let _t13 = (1, 2, 3);
	let mut _t13: (i32, i32, i32) = (1,2,3);
	println!("{:?}", _t13);

	// Pegar um valor de posicao
	println!("{:?}", _t13.0);

	// Alterar valor da tupla
	_t13.0 = 10;
	println!("{:?}", _t13.0);

	// Desestruturação de tuplas
	// Alimenta novos elementos com base numa tuplas existente
	let (_t14, _t15, _t16) = _t13;
	println!("{}", _t14);
	println!("{}", _t15);
	println!("{}", _t16);

	// Alterando array
	// Nao pode pq alterou a quantidade de elementos iniciais -> let mut _t13: (char, char, char, char) = ("1","2","3", "t4");
	let mut _t13: (char, char, char) = ('1','2','3');
	println!("{:?}", _t13.0);

	// Alterando todos os valores da tupla
	_t13 = ('q', 'w', 'e');
	println!("{:?}", _t13.0);


	// Array
	// Array só pode ter um tipo de dado
	// Se quiser especificar o array, coloca o tipo de dado e a repetição de vezes dele dentro do array
	let mut _t14: [i32;4] = [1,2,3,4];
	println!("{:?}", _t14);
	println!("{:?}", _t14[3]);

	// Alterando o valor com base na posicao
	_t14[0] = 10;
	println!("{:?}", _t14);

	// Alterando todos os elementos
	_t14 = [11,12,13,14];
	println!("{:?}", _t14);

	// Ele confere o tamanho do array em tempo de compilação, isso garante que não passe erros

	// Imprimir so uma parte dos elementos -> Slice
	// Traz o 1 e 2
	println!("{:?}", &_t14[1..3]);
	// Traz do 1 para frente
	println!("{:?}", &_t14[1..]);
	// Traz do 2 para tras
	println!("{:?}", &_t14[..2]);


}
