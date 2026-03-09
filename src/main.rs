use valida_cpf_cnpj::*;

fn main() {
	let cpf = String::from("113.745.449-06");
	let cnpj = String::from("08.829.865/0002-58");

	if is_valido(cpf){
		println!("Deu boa cpf");
	}
	if is_valido(cnpj) {
		println!("Deu boa cnpj");
	}
}
