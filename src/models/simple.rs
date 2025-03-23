use crate::{models::Model, tokenizer::TokenizingResult};
use std::collections::HashMap;

type Vocabulary = HashMap<String, u32>;

pub struct Simple {
	vocabulary: Vocabulary,
}

impl Simple {
	pub fn new() -> Self {
		Self { vocabulary: Vocabulary::new() }
	}
}

impl Default for Simple {
	fn default() -> Self {
		Self::new()
	}
}

impl Model for Simple {
	fn train(&mut self, text: &str) {
		println!("Training Simple model with text: {}", text);
	}

	fn save(&self, file_path: &str) {
		println!("Saving Simple model to: {}", file_path);
	}

	fn load(&mut self, file_path: &str) {
		println!("Loading Simple model from: {}", file_path);
	}

	fn tokenize(&self, text: &str) -> TokenizingResult {
		println!("Tokenizing text with Simple model: {}", text);

		let tokens = self.vocabulary.keys().cloned().collect();
		let ids = self.vocabulary.values().cloned().collect();

		TokenizingResult::new(tokens, ids)
	}
}
