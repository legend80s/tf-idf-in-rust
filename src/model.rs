use anyhow::Result;
use serde::{Deserialize, Serialize};
/// all about tf-idf
use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
    io::BufReader,
    path::PathBuf,
};

// use BTreeMap for key sorted alphabetically
pub type TermFreq = BTreeMap<String, usize>;
pub type TermFreqPerDoc = HashMap<PathBuf, TermFreq>;
pub type DocFreq = HashMap<String, usize>;
#[derive(Default, Serialize, Deserialize)]
pub struct Model {
    pub tfpd: TermFreqPerDoc, // { 'doc_path' => { term: term_count_in_the_doc } }
    pub df: DocFreq,          // { term: num_of_doc_count_contains_the_term }
}
impl Model {
    pub fn new() -> Model {
        Default::default()
    }
}

pub fn calc_tf(term: &str, doc: &TermFreq) -> f32 {
    let count = *doc.get(term).unwrap_or(&0);
    let total: usize = doc.values().sum();

    count as f32 / total as f32
}

pub fn calc_idf(term: &str, model: &Model) -> f32 {
    let docs_contains_item = model.df.get(term).unwrap_or(&1);

    // println!("total_docs: {}", total_docs);
    // println!("docs_contains_item: {}", docs_contains_item);

    let n = model.tfpd.len();
    let m = *docs_contains_item;

    // fix negative f32
    // if docs_contains_item == 0 {
    //     m = docs_contains_item + 1
    // }

    (n as f32 / m as f32).log10()
}

pub fn read_term_freq_index_from_file() -> Result<Model> {
    let model: Model = serde_json::from_reader(BufReader::new(File::open("./assets/index.json")?))?;

    Ok(model)
}
