use crate::{models::Model, tokenizer::TokenizingResult};
use std::collections::HashMap;

type Vocabulary = HashMap<String, u32>;

pub struct BPE {
	vocabulary: Vocabulary,
}

impl BPE {
	pub fn new() -> Self {
		Self { vocabulary: Vocabulary::new() }
	}
}

impl Default for BPE {
	fn default() -> Self {
		Self::new()
	}
}

impl Model for BPE {
	fn train(&mut self, text: &str) {
		println!("Training BPE model with text: {}", text);
	}

	fn save(&self, file_path: &str) {
		println!("Saving BPE model to: {}", file_path);
	}

	fn load(&mut self, file_path: &str) {
		println!("Loading BPE model from: {}", file_path);
	}

	fn tokenize(&self, text: &str) -> TokenizingResult {
		println!("Tokenizing text with BPE model: {}", text);

		let tokens = self.vocabulary.keys().cloned().collect();
		let ids = self.vocabulary.values().cloned().collect();

		TokenizingResult::new(tokens, ids)
	}
}
