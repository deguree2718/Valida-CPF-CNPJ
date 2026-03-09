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
	let cpf_certo = String::from("858.505.450-69");
	assert!(is_valido(cpf_certo));
	let cnpj_certo = String::from("74.357.176/0001-31");
	assert!(is_valido(cnpj_certo));
	let cnpj_alfa = String::from("B2.HTH.HYX/0001-96");
	assert!(is_valido(cnpj_alfa));
}
