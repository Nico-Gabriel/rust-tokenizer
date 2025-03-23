use models::bpe::BPE;
use tokenizer::Tokenizer;

pub mod models;
pub mod tokenizer;

fn main() {
	let mut tokenizer = Tokenizer::new(BPE::new());
	tokenizer.train("Hello, world!");
	tokenizer.save("bpe.json");
	tokenizer.load("bpe.json");
	let result = tokenizer.tokenize("Hello, world!");
	println!("{:?}", result.tokens);
	println!("{:?}", result.ids);
}

#[cfg(test)]
mod tests {

	#[test]
	fn test_simple_tokenizer() {
		todo!("implement simple tokenizer tests");
	}

	#[test]
	fn test_bpe_tokenizer() {
		todo!("implement bpe tokenizer tests");
	}
}
