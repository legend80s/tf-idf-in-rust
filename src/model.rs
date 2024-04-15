use anyhow::Result;
/// all about tf-idf
use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
    io::BufReader,
    path::PathBuf,
};

// use BTreeMap for key sorted alphabetically
pub type TermFreq = BTreeMap<String, usize>;
pub type TermFreqIndex = HashMap<PathBuf, TermFreq>;

pub fn calc_tf(term: &str, doc: &TermFreq) -> f32 {
    let count = *doc.get(term).unwrap_or(&0);
    let total: usize = doc.values().sum();

    count as f32 / total as f32
}

pub fn calc_idf(term: &str, tf_index: &TermFreqIndex) -> f32 {
    let total_docs = tf_index.len();
    let docs_contains_item = tf_index
        .values()
        .filter(|doc| doc.contains_key(term))
        .count()
        // fix negative f32
        .max(1);

    // println!("total_docs: {}", total_docs);
    // println!("docs_contains_item: {}", docs_contains_item);

    let n = total_docs;
    let m = docs_contains_item;

    // fix negative f32
    // if docs_contains_item == 0 {
    //     m = docs_contains_item + 1
    // }

    (n as f32 / m as f32).log10()
}

pub fn read_term_freq_index_from_file() -> Result<TermFreqIndex> {
    let tf_index: TermFreqIndex =
        serde_json::from_reader(BufReader::new(File::open("./assets/index.json")?))?;

    Ok(tf_index)
}
