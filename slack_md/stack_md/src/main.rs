use std::fs;
use std::fs::File;
use std::io::Write;
use std::io::prelude::*;

extern crate serde_json;
use serde_json::{Value, Error};

static DATA_DIR: &'static str = "../data/";
static OUT_FILE: &'static str = "ret.md";

fn list_json(dir:&str) -> Vec<String> {
    let mut json_list: Vec<String> = Vec::new();
    let paths = fs::read_dir(dir).unwrap();
    for path in paths {
        let mut p = path.unwrap().path();
        let f = p.to_string_lossy().to_string();
        json_list.push(f);
    }

    json_list
}

fn write_md(fp:&mut std::fs::File, json:String) -> Result<(), Error> {
    let mut f = File::open(json).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");

    let values: Value = serde_json::from_str(&contents)?;

    match values {
        Value::Array(l) => {
            for v in l {
                let text = format!("### {}\n", v["text"]).replace("\"", "");
                fp.write(text.as_bytes());
            }
        },
        _ => ()
    }
//    for v in values {
//        println!("{:?}", v);
//    }
    Ok(())
}

fn main() {
    let mut fp = fs::File::create(OUT_FILE).unwrap();
    let jsons = list_json(DATA_DIR);

    for json in jsons {
        write_md(&mut fp, json);

    }

}

