use regex::Regex;

const TAMANHO_CNPJ_SEM_DV: usize = 12;
const TAMANHO_CPF_SEM_DV: usize = 9;
const PESOS_CPF: [u32; 10] = [11, 10, 9, 8, 7, 6, 5, 4, 3, 2];
const PESOS_CNPJ: [u32; 13] = [6,5,4,3,2,9,8,7,6,5,4,3,2];

pub fn is_valido(documento: String) -> bool {
	let regex_formatacao = Regex::new("[./-]").unwrap();
	let doc_formatado = remover_ocorrencias(&regex_formatacao, &documento[..]);
	let regex_cnpj = Regex::new("[A-Z\\d]{12}").unwrap();
	let regex_cpf = Regex::new("[0-9]{9}").unwrap();
	if is_digito_repetido(&doc_formatado) {
		return false;
	}
	let mut doc_sem_dv: &str;
	let dv_informado: &str;
	let mut dv_calculado: String;
	if regex_cnpj.is_match(&doc_formatado) {
		doc_sem_dv = &doc_formatado[0..TAMANHO_CNPJ_SEM_DV];
		dv_informado = &doc_formatado[TAMANHO_CNPJ_SEM_DV..];
		dv_calculado = format!("{}", calcula_digito(doc_sem_dv));
		let ctrl = format!("{}{}", doc_sem_dv, dv_calculado);
		doc_sem_dv = ctrl.as_str();
		dv_calculado = format!("{}{}", dv_calculado, calcula_digito(doc_sem_dv));
	} else if regex_cpf.is_match(&doc_formatado){
		doc_sem_dv = &doc_formatado[0..TAMANHO_CPF_SEM_DV];
		dv_informado = &doc_formatado[TAMANHO_CPF_SEM_DV..];
		dv_calculado = format!("{}", calcula_digito(doc_sem_dv));
		let ctrl = format!("{}{}", doc_sem_dv, dv_calculado);
		doc_sem_dv = ctrl.as_str();
		dv_calculado = format!("{}{}", dv_calculado, calcula_digito(doc_sem_dv));
	} else {
		return false;
	}
	debug_assert!(dv_calculado == dv_informado);
	dv_calculado == dv_informado
}

fn calcula_digito(doc: &str) -> u32 {
	let soma: u32 = doc.as_bytes()
		.iter()
		.rev()
		.zip(if doc.len() <= 10 {PESOS_CPF.iter().rev()} else {PESOS_CNPJ.iter().rev()})
		.map(|(&byte, &peso)| {
			(byte - b'0') as u32 * peso
		})
		.sum();
	if (soma % 11) < 2 {
		0
	} else {
		11 - (soma % 11)
	}
}

fn remover_ocorrencias(r: &Regex, stack: &str) -> String {
	let mut n = String::with_capacity(stack.len());
	let mut last_match = 0;
	for caps in r.captures_iter(stack) {
		let m = caps.get(0).unwrap();
		n.push_str(&stack[last_match..m.start()]);
		last_match = m.end();
	}
	n.push_str(&stack[last_match..]);
	n
}

fn is_digito_repetido(doc: &str) -> bool {
	let p_char = doc.chars().next().unwrap();
	p_char.is_ascii_digit() && doc.chars().all(|c| c == p_char)
}
