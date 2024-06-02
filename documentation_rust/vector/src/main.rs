struct Vector {
	x: i32,
	y: i32,
	z: i32,
}

fn main() {
	// Estado inicial tem que esperar que será alterado
	let mut v1: Vector = Vector { x:1, y:2, z:3 };

	println!("V1.x: {}", v1.x);
	println!("V1.y: {}", v1.y);
	println!("V1.z: {}", v1.z);

	// Fazendo um emprestimo de referencia
	inc_x(&mut v1);

	println!("V1.x: {}", v1.x);
	println!("V1.y: {}", v1.y);
	println!("V1.z: {}", v1.z);
}

// Tem que tornar mutavel o parametro a ser recebido
fn inc_x(v: &mut Vector) {
	v.x += 1
}
