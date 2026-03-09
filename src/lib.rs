use regex::{Captures, Regex};

const TAMANHO_CNPJ_SEM_DV: usize = 12;
const TAMANHO_CPF_SEM_DV: usize = 9;

pub fn is_valido(documento: String) -> bool {
	let regex_formatacao = Regex::new("[./-]").unwrap();
	let replacement = |_caps: &Captures| -> Result<String, &'static str> {
		Ok(String::new())
	};
	let doc_formatado = replace(&regex_formatacao, &documento[..], replacement).unwrap();
	let regex_cnpj = Regex::new("[A-Z\\d]{12}").unwrap();
	let regex_cpf = Regex::new("[0-9]{9}").unwrap();
	if is_digito_repetido(&doc_formatado) {
		return false;
	}
	if regex_cnpj.is_match(&doc_formatado) {
		let mut doc_sem_dv = &doc_formatado[0..TAMANHO_CNPJ_SEM_DV];
		let dv_informado = &doc_formatado[TAMANHO_CNPJ_SEM_DV..];
		let mut dv_calculado = format!("{}", calcula_digito_cnpj(doc_sem_dv));
		let ctrl = format!("{}{}", doc_sem_dv, dv_calculado);
		doc_sem_dv = ctrl.as_str();
		dv_calculado = format!("{}{}", dv_calculado, calcula_digito_cnpj(doc_sem_dv));
		debug_assert!(dv_calculado == dv_informado);
		return dv_calculado == dv_informado;
	} else if regex_cpf.is_match(&doc_formatado){
		let mut doc_sem_dv = &doc_formatado[0..TAMANHO_CPF_SEM_DV];
		let dv_informado = &doc_formatado[TAMANHO_CPF_SEM_DV..];
		let mut dv_calculado = format!("{}", calcula_digito_cpf(doc_sem_dv));
		let ctrl = format!("{}{}", doc_sem_dv, dv_calculado);
		doc_sem_dv = ctrl.as_str();
		dv_calculado = format!("{}{}", dv_calculado, calcula_digito_cpf(doc_sem_dv));
		debug_assert!(dv_calculado == dv_informado);
		return dv_calculado == dv_informado;
	}
	false
}

fn calcula_digito_cpf(cpf: &str) -> u32 {
	let pesos_cpf: Vec<u32> = Vec::from([11, 10, 9, 8, 7, 6, 5, 4, 3, 2]);
	let soma: u32 = cpf.as_bytes()
		.iter()
		.rev()
		.zip(pesos_cpf.iter().rev())
		.map(|(&num_byte, &peso)| {
			(num_byte - b'0') as u32 * peso
		})
		.sum();
	if (soma % 11) < 2 {
		0
	} else {
		11 - (soma % 11)
	}
}

fn calcula_digito_cnpj(cnpj: &str) -> u32 {
	let pesos_cnpj: Vec<u32> = Vec::from([6,5,4,3,2,9,8,7,6,5,4,3,2]);
	let soma: u32 = cnpj.as_bytes()
		.iter()
		.rev()
		.zip(pesos_cnpj.iter().rev())
		.map(|(&num_byte, &peso)| {
			(num_byte - b'0') as u32 * peso
		})
		.sum();
	if (soma % 11) < 2 {
		0
	} else {
		11 - (soma % 11)
	}
}

fn replace<E>(r: &Regex, stack: &str, replace: impl Fn(&Captures) -> Result<String, E>,) -> Result<String, E> {
	let mut new = String::with_capacity(stack.len());
	let mut last_match = 0;
	for caps in r.captures_iter(stack) {
		let m = caps.get(0).unwrap();
		new.push_str(&stack[last_match..m.start()]);
		new.push_str(&replace(&caps)?);
		last_match = m.end();
	}
	new.push_str(&stack[last_match..]);
	Ok(new)
}

fn is_digito_repetido(doc: &str) -> bool {
	let p_char = doc.chars().next().unwrap();
	if p_char.is_ascii_digit() && doc.chars().all(|c| c == p_char) {
		return true;
	}
	false
}
