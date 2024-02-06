use super::tokenizer::Tokenizer;
use burn::{
    data::dataloader::batcher::Batcher,
    nn::attention::generate_padding_mask,
    tensor::{backend::Backend, Bool, Data, ElementConversion, Int, Tensor},
};
use derive_new::new;
use std::sync::Arc;

pub struct SongClassificationBatcher<B: Backend> {
    tokenizer: Arc<dyn Tokenizer>,
    device: B::Device,
    max_seq_length: usize,
}

impl<B: Backend> SongClassificationBatcher<B> {
    pub fn new(tokenizer: Arc<dyn Tokenizer>, device: B::Device, max_seq_length: usize) -> Self {
        Self {
            tokenizer,
            device,
            max_seq_length,
        }
    }
}

/// Struct for training batch
#[derive(Debug, Clone, new)]
pub struct TextClassificationTrainingBatch<B: Backend> {
    pub tokens: Tensor<B, 2, Int>,    // Tokenized text
    pub labels: Tensor<B, 1, Int>,    // Labels of the text
    pub mask_pad: Tensor<B, 2, Bool>, // Padding mask for the tokenized text
}

/// Struct for inference batch
#[derive(Debug, Clone, new)]
pub struct TextClassificationInferenceBatch<B: Backend> {
    pub tokens: Tensor<B, 2, Int>,    // Tokenized text
    pub mask_pad: Tensor<B, 2, Bool>, // Padding mask for the tokenized text
}
