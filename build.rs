use std::{collections::HashMap, fs, io::Write};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Val {
    char: Option<String>,
    code: Option<String>,
}

fn load_to_json(filepath: &str,outdir: &str) {
    let content = fs::read_to_string(filepath).unwrap();
    let content_json: HashMap<String, Val> = serde_json::from_str(&content).unwrap();

    let mut scary: HashMap<String, Vec<(String, String)>> = HashMap::new();
    
    for (key, value) in content_json {
        let key_splited: Vec<&str> = key.split('-').collect();

        if key_splited.len() < 2 {
            println!("Skipping invalid key: {}", key);
            continue;
        }

        let module = key_splited[0];
        let me = key_splited[1];

        if let Some(code) = value.code.clone() {
            scary
                .entry(module.to_string())
                .or_insert_with(Vec::new)
                .push((me.to_string(), code));
        } else {
            println!("Missing code or char in value: {:?}", value);
        }
    }
    let mut file = fs::File::create(format!("{outdir}/icons.rs")).unwrap();
    writeln!(file, "// This File is Gen Auto").expect("NIGGA");
    for (key,value) in &scary{
        writeln!(file,"pub mod {key} {{").expect("NIGGA");
        for(name,code) in value {
                
            writeln!(file,"\tpub const RS{}: &str = \"\\u{{{}}}\";",name.to_uppercase(),code).unwrap();
        }
        writeln!(file,"}}\n").expect("NIGGA");
        
    }


}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() <= 1 {
        println!("Provide OUT_DIR\nExample:\nnerdicons-gen-rs OUT_DIR");
        return;
    }
    load_to_json("./glyphnames.json", args[1].clone().as_str());

}
