use crate::tokenizer::TokenizingResult;

pub mod bpe;
pub mod simple;

pub trait Model {
	fn train(&mut self, text: &str);
	fn save(&self, file_path: &str);
	fn load(&mut self, file_path: &str);
	fn tokenize(&self, text: &str) -> TokenizingResult;
}
