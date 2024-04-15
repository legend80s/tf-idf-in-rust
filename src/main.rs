use anyhow::Result;
use core::str;
use std::{
    collections::{BTreeMap, HashMap},
    env::args,
    fs::{self, File},
    io::{BufReader, BufWriter},
    path::{Path, PathBuf},
    process::ExitCode,
    time::Instant,
};
use tiny_http::{Header, Method, Request, Response};
use xml::{reader::XmlEvent, EventReader};

// use BTreeMap for key sorted alphabetically
type TermFreq = BTreeMap<String, usize>;
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
        while n < self.content.len() && predicate(self.content, n) {
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
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let term = self
            .next_token()
            .map(|ch| ch.iter().collect())
            .map(|ch: String| ch.to_ascii_uppercase().trim().to_string());

        term
    }
}

fn is_word(term: &str) -> bool {
    if term.is_empty() || term.chars().all(|c| c.is_ascii_punctuation()) {
        return false;
    }

    true
}

fn read_term_freq_index_from_file() -> Result<TermFreqIndex> {
    let tf_index: TermFreqIndex = serde_json::from_reader(File::open("./assets/index.json")?)?;

    Ok(tf_index)
}

fn search() -> Result<()> {
    let tf_index: TermFreqIndex = read_term_freq_index_from_file()?;

    println!("index.json contains {} files", tf_index.len());
    Ok(())
}

fn serve_static_file(request: Request, filepath: &str, content_type: &str) -> Result<()> {
    let html = File::open(filepath).expect("file not exists");
    let response = Response::from_file(html).with_header(
        Header::from_bytes("content-type", content_type).expect("should not failed on headers"),
    );
    request.respond(response)?;

    Ok(())
}

fn serve_404(request: Request) -> Result<()> {
    let not_found_html = File::open("./public/404.html").expect("file not exists");
    request.respond(Response::from_file(not_found_html).with_status_code(404))?;

    Ok(())
}

fn calc_tf(term: &str, doc: &TermFreq) -> f32 {
    let count = *doc.get(term).unwrap_or(&0);
    let total: usize = doc.values().sum();

    count as f32 / total as f32
}

fn calc_idf(term: &str, tf_index: &TermFreqIndex) -> f32 {
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

fn serve(mut request: Request) -> Result<()> {
    println!(
        "INFO: method: {:?}, url: {:?}",
        request.method(),
        request.url()
    );

    match (request.method(), request.url()) {
        (Method::Post, "/api/search") => {
            let mut body = String::new();
            request.as_reader().read_to_string(&mut body)?;

            println!("INFO: searching {body}");

            let mut result: Vec<(&PathBuf, f32)> = Vec::new();
            let tf_index = read_term_freq_index_from_file()?;

            for (path, doc) in &tf_index {
                let mut rank = 0f32;
                for term in Lexer::new(&body.chars().collect::<Vec<_>>()) {
                    if is_word(&term) {
                        // println!("{term}");
                        rank += calc_tf(&term, &doc) * calc_idf(&term, &tf_index);
                    }
                }

                result.push((path, rank));
            }

            result.sort_by(|(_, rank1), (_, rank2)| {
                rank2
                    .partial_cmp(rank1)
                    .expect(&format!("{rank1} and {rank2} are not comparable"))
            });

            let mut vec = Vec::new();

            for (doc, rank) in result.iter().take(10) {
                vec.push(format!("{:?} => {}", doc, rank));
                println!("{:?} => {}", doc, rank);
            }

            request.respond(Response::from_string(vec.join("\n")))?
        }

        (Method::Get, "/" | "/index.html" | "/index") => {
            serve_static_file(request, "./public/index.html", "text/html; charset=UTF-8")?
        }
        (Method::Get, "/index.js") => serve_static_file(
            request,
            "./public/index.js",
            "text/javascript; charset=UTF-8",
        )?,
        _ => serve_404(request)?,
    }

    Ok(())
}

/// cargo run index <dir>
/// cargo run search <dir>
/// cargo run serve
fn entry() -> Result<()> {
    let mut args = args();

    let program = args.next().expect("the program not gonna empty");

    let sub_command = args
        .next()
        .ok_or_else(|| {
            usage(&program);
            eprintln!("ERROR: no subCommand is provided");
        })
        .unwrap();

    match sub_command.as_str() {
        "index" => {
            let dir = args.next().unwrap_or("../docs.gl/".to_string());
            index(&dir)?
        }
        "search" => search()?,
        "serve" => {
            let server = tiny_http::Server::http("0.0.0.0:8080").unwrap();

            println!("INFO: listening at http://{}", server.server_addr());

            for request in server.incoming_requests() {
                serve(request)?
            }
        }
        _ => {
            usage(&program);
            eprintln!("ERROR: unknown subcommand: {sub_command}");
        }
    }

    Ok(())
}

fn usage(program: &str) {
    eprintln!("Usage {program} <SUBCOMMAND> [OPTIONS]");
    eprintln!("Subcommands:");
    eprintln!("    index <folder>       Indexing files under folder and save to index.json");
    eprintln!("    search <index-file>   Check how many documents are indexed");
    eprintln!("    serve                Start HTTP server with web interface");
}

fn main() -> ExitCode {
    match entry() {
        core::result::Result::Ok(_) => ExitCode::SUCCESS,
        Err(_) => ExitCode::FAILURE,
    }
}

fn index(dir: &str) -> Result<()> {
    let dir = Path::new(dir);

    let index_start = Instant::now();
    let tf_index = parse_xml_in_dir(&dir)?;
    println!("\n---------------------------------------\n");
    println!(
        "Indexed folder {:?} of {} files costs {:?}",
        dir,
        tf_index.len(),
        index_start.elapsed()
    );

    let dump_file_path = "assets/index.json";

    let save_start = Instant::now();
    serde_json::to_writer(BufWriter::new(File::create(dump_file_path)?), &tf_index)?;
    // serde_json::to_writer_pretty(File::create(dump_file_path)?, &tf_index)?;

    println!(
        "Saving to {dump_file_path:?} costs {:?}",
        save_start.elapsed()
    );

    // for (path, tf) in tf_index {
    //     println!("{:?} has {} terms", path, tf.len());
    // }

    // println!("{content}", content = read_xml(dir)?);

    Ok(())
}

fn index_document(content: &str) -> TermFreq {
    let chars = content.trim().chars().collect::<Vec<_>>();
    let mut tf = TermFreq::new();

    for term in Lexer::new(&chars) {
        if is_word(&term) {
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

fn walk_file<P>(dir: &Path, predicate: P) -> Result<Vec<PathBuf>>
where
    P: Fn(&PathBuf) -> bool + std::clone::Clone,
{
    let mut files = Vec::new();

    for file in fs::read_dir(dir)? {
        let filepath = file?.path();

        if filepath.is_dir() {
            let mut sub = walk_file(&filepath, predicate.clone())?;
            files.append(&mut sub);
        } else if predicate(&filepath) {
            files.push(filepath);
        }
    }

    return Ok(files);
}

fn parse_xml_in_dir(dir: &Path) -> Result<TermFreqIndex> {
    let mut term_freq_index = TermFreqIndex::new();

    let files = walk_file(dir, |fp| {
        if let Some(ext) = fp.extension() {
            return ext == "xhtml";
        }

        false
    })?;

    for filepath in files {
        let content = read_xml(&filepath)?;

        println!("Indexing {:?}...", filepath);
        let tf = index_document(&content);
        let key = filepath;

        term_freq_index.insert(key, tf);
    }

    Ok(term_freq_index)
}

fn read_xml(filepath: &PathBuf) -> Result<String> {
    let parser = EventReader::new(BufReader::new(File::open(filepath)?));
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
