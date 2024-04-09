use anyhow::Result;
use core::str;
use std::{
    collections::HashMap,
    fs::{self, File},
    path::{Path, PathBuf},
};
use xml::{reader::XmlEvent, EventReader};

type TermFreq = HashMap<String, usize>;
type TermFreqIndex = HashMap<PathBuf, TermFreq>;

struct Lexer<'a> {
    content: &'a [char],
}

impl<'a> Lexer<'a> {
    fn new(content: &'a [char]) -> Self {
        Self { content }
    }

    fn chop(&mut self, n: usize) -> Option<&'a [char]> {
        let token = Some(&self.content[0..n]);
        self.content = &self.content[n..];

        return token;
    }

    fn chop_while<P>(&mut self, mut predicate: P) -> Option<&'a [char]>
    where
        P: FnMut(&[char], usize) -> bool,
    {
        let mut n = 0;
        while self.content.len() != 0 && predicate(self.content, n) {
            n += 1;
        }

        return self.chop(n);
    }

    fn next_token(&mut self) -> Option<&'a [char]> {
        if self.content.len() == 0 {
            return None;
        }

        // 123
        // 1.23
        if self.content[0].is_numeric() {
            return self.chop_while(|content, idx| {
                let c = content[idx];
                c.is_numeric() || c == '.' && content[idx + 1].is_numeric()
            });
        }

        // hello
        // gl4
        // GL_INVALID_VALUE
        if self.content[0].is_alphabetic() {
            return self.chop_while(|content, idx| {
                let c = content[idx];

                c.is_alphanumeric() || c == '_'
            });
        }

        // else then return a single character such as "√"
        return self.chop(1);
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = &'a [char];

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

fn main() -> Result<()> {
    let dir = Path::new("../docs.gl/gl4");

    let doc_tf = read_xml_in_dir(&dir)?;
    for (path, tf) in doc_tf {
        println!("\n{:?}", path);
        println!("{:?}", tf.len());
    }

    // println!("{content}", content = read_xml(dir)?);

    Ok(())
}

fn index_document(content: &str) -> TermFreq {
    let chars = content.trim().chars().collect::<Vec<_>>();
    let mut tf = TermFreq::new();

    for token in Lexer::new(&chars) {
        let term = token
            .iter()
            // .map(|c| c.to_ascii_uppercase())
            .collect::<String>()
            .trim()
            .to_owned();

        if term.is_empty() || term.chars().all(|c| c.is_ascii_punctuation()) {
            // ignore empty and punctuations
        } else {
            let count = tf.entry(term).or_insert(0);
            *count += 1
        }
    }

    tf

    // let mut tf = tf.into_iter().collect::<Vec<_>>();
    // tf.sort_by_key(|(_key, val)| *val);
    // tf.reverse();

    // for (key, val) in tf.into_iter().take(10) {
    //     println!("{} => {}", key, val)
    // }
}

fn read_xml_in_dir(dir: &Path) -> Result<TermFreqIndex> {
    let mut term_freq_index = TermFreqIndex::new();
    for file in fs::read_dir(dir)? {
        let filepath = file?.path();

        if filepath.is_dir() {
            let _ = read_xml_in_dir(&filepath);
        } else if let Some(ext) = filepath.extension() {
            if ext == "xhtml" {
                let content = read_xml(&filepath)?;

                println!("indexing {:?}", filepath);
                let tf = index_document(&content);
                let key = filepath;

                term_freq_index.insert(key, tf);
            }
        }
    }

    Ok(term_freq_index)
}

fn read_xml(filepath: &PathBuf) -> Result<String> {
    let parser = EventReader::new(File::open(filepath)?);
    let mut content = String::new();

    for event in parser {
        if let Ok(XmlEvent::Characters(text)) = event {
            content.push_str(&text);
            // insert blank as separator avoid "Name2" when parsing xml as below:
            // <strong>Function / Feature Name</strong>
            // <strong>2.0</strong>
            content.push(' ');
        }
    }

    Ok(content)
}
