use anyhow::Result;
use core::str;
use std::{
    collections::HashMap,
    env::args,
    fs::{self, File},
    path::{Path, PathBuf},
    process::ExitCode,
    time::Instant,
};
use tiny_http::{Header, Method, Request, Response};
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
        self.next_token()
            .map(|ch| ch.iter().collect())
            .map(|ch: String| ch.to_ascii_uppercase())
    }
}

fn search() -> Result<()> {
    let file: TermFreqIndex = serde_json::from_reader(File::open("./assets/index.json")?)?;

    println!("index.json contains {} files", file.len());
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

fn serve(mut request: Request) -> Result<()> {
    println!(
        "INFO: received request! method: {:?}, url: {:?}",
        request.method(),
        request.url()
    );

    match (request.method(), request.url()) {
        (Method::Post, "/api/search") => {
            let mut body = String::new();
            request.as_reader().read_to_string(&mut body)?;

            println!("INFO: searching {body}");

            for term in Lexer::new(&body.chars().collect::<Vec<_>>()) {
                println!("{term}")
            }

            request.respond(Response::from_string(body))?
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
            let dir = args.next().unwrap_or("../docs.gl/gl4".to_string());
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
    println!("Indexed {:?} costs {:?}", dir, index_start.elapsed());

    let dump_file_path = "assets/index.json";

    let save_start = Instant::now();
    serde_json::to_writer(File::create(dump_file_path)?, &tf_index)?;
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

    for token in Lexer::new(&chars) {
        let term = token.trim().to_owned();

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

fn parse_xml_in_dir(dir: &Path) -> Result<TermFreqIndex> {
    let mut term_freq_index = TermFreqIndex::new();
    for file in fs::read_dir(dir)? {
        let filepath = file?.path();

        if filepath.is_dir() {
            let _ = parse_xml_in_dir(&filepath);
        } else if let Some(ext) = filepath.extension() {
            if ext == "xhtml" {
                let content = read_xml(&filepath)?;

                println!("Indexing {:?}...", filepath);
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
