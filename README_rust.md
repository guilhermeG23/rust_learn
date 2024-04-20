Rust normalmente tem mensagens detalhadas dos erros cometidos quando se compila

Compilar em rust:
```
rustc <arquivo>
```

Por padrão a identação é de 4 espaços em branco

Quando chamar uma função com ! na frente, ela é um macro

Rust não tem o garbade collector (Limpeza de memoria), ele tem outro mecanismo chamado de RAII

RAII 
* Resource, aquisicion e inicialization

Quando um recurso é requisitado, o rust tem que saber onde ele é eliminado


Rust alerta sobre problemas no codigo, ele acusa problemas ocorridos, como variaveis não utilizadas, erros de sintaxe e ademais

Ex:
let teste  = 10;
let _teste = 10;

Esse underline força o rust a entender que está variavel não esta sendo utilizada em lugar nenhum e pode continuar a compilar

Rust é estaticamente tipada, ele atribui o tipo com base no dado setado

Exemplo de valor:

let teste0 = 1;
let teste1: i32 = 1;

Dessa forma obriga o valor a ser um inteiro

Toda a variavel no rust é imutavel por padrão (Quase uma constante), para tornar seu valor dinamico, é necessário usar o mut como extensão. 
Essa é uma segurança extra da linguagem para evitar alterações no valores

Vale comentar que caso voce tente alterar o valor antes de qualquer chamada prévia dele, ele retornara um warning falando que era mais facil ter atribuido esse valor como o estado inicial


Shadowing de escopo e limite de operação de escopo
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


Se for usar constantes, vc pode definir fora do escopo do main, porém, 
é necessário definir o tipo do mesmo;


i32 -> Inteiro 32 bites
u32 -> Inteiro positivo de 32 bites

Constante e variavel
Rust trata uma constante diferente de uma let
A let é feito um endereçamento e gerenciamento de memoria para a mesma
afim de esperar que a mesma possa ter váriações
Já a constante, ele faz uma ação de inline, a qual, altera todos os 
pontos de chamada de constante para o valor definido, assim não há o custo de uso
da memoria para constrolar esse valor
Constantes não tem escopo de operação, elas são validas em todos os campos

Tipos de escrita:
* Constantes -> TUDO EM MAISUCULO E COM UNDERLINE DE SEPARAÇÃO DAS PALAVRAS
* variavel -> Tudo em minuscolo e com underline de separação das palavras


Rust trabalha com três regiões de memoria
* Memory alerness -> Necessita saber o local a ser usado na memoria
* Planeje oq vai fazer e qual tipo de regiões de memoria vai se utilizar, afim de otimizar a chamada das ações
* Espaços de memorias
	* Static -> Grava todos os valores fixos do programa na memoria, isso é, elementos staticos e binariosque não se alteram durante a execução do programa
		* O compilador precisa saber o tamanho da execução do programa em tempo de compilação para saber o quanto reservar em ação
		* O tempo de vida de uso da memoria é o tempo que a execução permanecer
	* Stack -> Variaveis normais são atribuidas a este local de memoria
		* Além das variaveis, funções, variaveis local, stacks isoladas de funções e ações dinamicas são atribuidos aqui
		* O stack não tem memoria infinita, ele tem o tamanho máximo de memoria que ele pode instanciar, porém ele limpa elementos conforme a execução afim de não estourar o uso de memoria
		* O tempo de vida é dentro da chamada e finalização do uso do requisitado;
		* O stack é formado durante o tempo de compilação;
	* HEAP -> Memoria instanciada em tempo de execução
		* Memoria dinamica com os valores de entrada de usuários ou retornos imprevisiveis;
			* O limite de memoria é definido pela quantidade de recursos de hardware instanciados
		* Tempo de vida é alem do escopo e pode ser manipulado
		* O valor em HEAP é compartilhado entre threads
		* Ações de heap são limpas após o fim de seu uso geral

Rust não tem nulo, ele consegue voltar vazio
