use valida_cpf_cnpj::is_valido;

#[test]
fn teste_err(){
	let cpf_errado0 = String::from("000.000.000-00");
	assert!(!is_valido(cpf_errado0));
	let cpf_errado1 = String::from("111.111.111-11");
	assert!(!is_valido(cpf_errado1));
	let cnpj_errado0 = String::from("00.000.000/0000-00");
	assert!(!is_valido(cnpj_errado0));
	let cnpj_errado1 = String::from("11.111.111/1111-11");
	assert!(!is_valido(cnpj_errado1));
}

#[test]
fn teste_ok(){
	let cpf_certo = String::from("113.745.449-06");
	assert!(!is_valido(cpf_certo));
	let cnpj_certo = String::from("08.829.865/0001-77");
	assert!(!is_valido(cnpj_certo));
}
