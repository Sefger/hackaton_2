use sha2::{Digest, Sha256};

/// Deterministic fake embedding: sha256(text) expanded to `dim` floats in [-1, 1], then
/// L2-normalized so cosine distance is stable.
pub fn hash_embedding(text: &str, dim: usize) -> Vec<f32> {
    let mut out = Vec::with_capacity(dim);
    let mut hasher = Sha256::new();
    hasher.update(text.as_bytes());
    let mut digest = hasher.finalize().to_vec();
    while out.len() < dim {
        if digest.is_empty() {
            let mut h = Sha256::new();
            h.update(out.len().to_le_bytes());
            h.update(text.as_bytes());
            digest = h.finalize().to_vec();
        }
        let b = digest.remove(0);
        out.push((b as f32) / 127.5 - 1.0);
    }
    let norm: f32 = out.iter().map(|x| x * x).sum::<f32>().sqrt().max(1e-6);
    out.iter_mut().for_each(|x| *x /= norm);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic_same_text_same_embedding() {
        let a = hash_embedding("hello", 16);
        let b = hash_embedding("hello", 16);
        assert_eq!(a, b);
    }

    #[test]
    fn different_text_different_embedding() {
        let a = hash_embedding("hello", 16);
        let b = hash_embedding("world", 16);
        assert_ne!(a, b);
    }

    #[test]
    fn length_matches_dim() {
        assert_eq!(hash_embedding("x", 1024).len(), 1024);
    }
}
