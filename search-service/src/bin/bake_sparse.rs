use fastembed::{SparseInitOptions, SparseModel, SparseTextEmbedding};

fn main() {
    let cache = std::env::args().nth(1).unwrap_or_else(|| "/app/models".to_string());
    std::fs::create_dir_all(&cache).expect("create cache dir");
    let mut opts = SparseInitOptions::default();
    opts.model_name = SparseModel::BGEM3;
    opts.cache_dir = cache.clone().into();
    opts.show_download_progress = true;
    let _model = SparseTextEmbedding::try_new(opts).expect("download model");
    println!("baked sparse model into {}", cache);
}
