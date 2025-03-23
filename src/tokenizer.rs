use crate::models::Model;

pub struct TokenizingResult {
	pub tokens: Vec<String>,
	pub ids: Vec<u32>,
}

impl TokenizingResult {
	pub fn new(tokens: Vec<String>, ids: Vec<u32>) -> Self {
		Self { tokens, ids }
	}
}

pub struct Tokenizer<M: Model> {
	pub model: M,
}

impl<M: Model> Tokenizer<M> {
	pub fn new(model: M) -> Self {
		Self { model }
	}

	pub fn train(&mut self, text: &str) {
		self.model.train(text);
	}

	pub fn save(&self, file_path: &str) {
		self.model.save(file_path);
	}

	pub fn load(&mut self, file_path: &str) {
		self.model.load(file_path);
	}

	pub fn tokenize(&self, text: &str) -> TokenizingResult {
		self.model.tokenize(text)
	}
}
