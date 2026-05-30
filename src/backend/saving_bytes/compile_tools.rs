use crate::backend::linker::link::Linker;
use crate::backend::linker::obj_file::ObjFile;
use crate::backend::saving_bytes::save::compile_instr_to_bytes;
use crate::backend::{
    ast::parser::Parser,
    compiler::{
        byte_code::{Compilable, Compiler},
        instructions::Instructions,
    },
    errors::diagnostics::lexer_error_print::print_lexer_err,
    lexer::{tokenizer::Lexer, tokens::Token},
    saving_bytes::binary_compilation,
};
use crate::clrprintln;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::collections::HashMap;
use std::{
    fs,
    path::{Path, PathBuf},
    process,
    time::{Duration, Instant},
};
use walkdir::WalkDir;

pub static DEPENDECIES_REFS: std::sync::OnceLock<HashMap<String, String>> =
    std::sync::OnceLock::new();

pub fn set_config(map: HashMap<String, String>) {
    DEPENDECIES_REFS.set(map).expect("Config already set");
}

pub fn get_modules() -> &'static HashMap<String, String> {
    DEPENDECIES_REFS.get().expect("Config not initialized")
}
fn is_config_set() -> bool {
    DEPENDECIES_REFS.get().is_some()
}
fn create_spinner(msg: String) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(80));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.green} {msg}")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );
    pb.set_message(msg);
    pb
}

fn debug_print(tokens: &Vec<Token>, ast: Box<dyn Compilable>, instructions: &Vec<Instructions>) {
    for token in tokens {
        println!("{:?}", token);
    }
    println!("{:?}", ast);
    for instruction in instructions {
        println!("{:?}", instruction);
    }
}
///This functions does compilation process of one single file. It creates tokens, build ast, create lookup for imported variables, updates types in type table, creates bytecode and optimizes it.
/// # Returns
/// Singular ObjFile
/// # Example
///```no_run
/// use vertex::backend::saving_bytes::compile_tools::compile_file_to_bytecode;
///
/// let path = "src/main.vtx".to_string();
/// let final_obj = compile_file_to_bytecode(path);
/// //now you can do anything with the ObjFile
/// ```
pub fn compile_file_to_bytecode(
    module_name: String,
    tokens: Vec<Token>,
    lexed_files: &HashMap<String, Vec<Token>>,
) -> ObjFile {
    let file_start = Instant::now();
    let pb = create_spinner(format!("Compiling {}", module_name));

    /*
     * Parser
     */
    let mut main_parser: Parser = Parser::new(tokens);

    let mut parsed_ast = main_parser.parse().unwrap_or_else(|e| {
        pb.finish_and_clear();
        println!("Error at {}:", &module_name);
        println!("\x1b[1;31m{}\x1b[0m", e);
        process::exit(-2)
    });
    /*
     * Lookup
     */
    let mut compiler = Compiler::new();
    compiler.context.lexed_files = lexed_files.clone();

    if let Err(e) = parsed_ast.add_to_lookup(&mut compiler) {
        pb.finish_and_clear();
        clrprintln!("$red|Error at:{}", &module_name);
        clrprintln!("$red|{}", e);
        process::exit(-3);
    }

    /*
     * Type check
     */
    parsed_ast.add_to_type_check(&mut compiler).unwrap();

    /*
     * Bytecode
     */
    if let Err(e) = parsed_ast.compile(&mut compiler) {
        pb.finish_and_clear();
        clrprintln!("$red|Error at $reset|:$cyan|{}", &module_name);
        clrprintln!("$red|{}", e);
        println!("\x1b[1mTry:vertexC error <error code> for fix\x1b[0m");
        process::exit(-3);
    }

    pb.finish_and_clear();
    println!(
        "\x1b[32m✔\x1b[0m {:<50} in {:.4}s",
        format!("Compiled {}", module_name),
        file_start.elapsed().as_secs_f32()
    );

    ObjFile {
        instructions: compiler.out,
        name: module_name,
        imports: compiler.imports.clone(),
    }
}

//NOTE:This is just entry point for the compilation process, and it
//shouldn't be used any further in the compilation process
pub fn build_prj(dir: String, out: String, debug: bool, _vm_path: Option<PathBuf>) {
    ensure_target_dir();
    let total_start = Instant::now();

    let src_path = Path::new(&dir)
        .canonicalize()
        .unwrap_or_else(|_| std::path::PathBuf::from(&dir));

    println!(
        "\n\x1b[1;32mBuilding\x1b[0m {} -> out/{}\n",
        src_path.display(),
        out
    );

    /*
     * Lexing phase
     */
    let main_vtx_files = get_vertex_files_recursive(&dir);
    let mut files_to_lex = Vec::new();

    // Add main project files
    for file in &main_vtx_files {
        files_to_lex.push((file.clone(), dir.clone(), None));
    }

    // Add dependency files
    if is_config_set() {
        for (name, path) in get_modules() {
            let dep_src = format!("{}/src", path);
            for file in get_vertex_files_recursive(&dep_src) {
                files_to_lex.push((file, dep_src.clone(), Some(name.clone())));
            }
        }
    }

    let tokens_map: HashMap<String, Vec<Token>> = files_to_lex
        .par_iter()
        .map(|(file, base_dir, prefix)| {
            let content =
                fs::read_to_string(file).unwrap_or_else(|_| panic!("Cannot find module {}", file));
            let main_lexer: Lexer = Lexer::new(content);

            let tokens: Vec<Token> = match main_lexer.tokenize() {
                Err(e) => {
                    clrprintln!("$red|Error at {}:", file);
                    print_lexer_err(e, fs::read_to_string(file).unwrap());
                    process::exit(-1);
                }
                Ok(tokens) => tokens,
            };

            // Normalize key
            let rel_path = if file.starts_with(base_dir) {
                file.strip_prefix(base_dir)
                    .unwrap()
                    .trim_start_matches('/')
                    .to_string()
            } else {
                file.clone()
            };

            let key = match prefix {
                Some(p) => format!("{}/{}", p, rel_path),
                None => rel_path,
            };

            (key, tokens)
        })
        .collect();

    /*
     * Compile phase
     */
    let mut objs: Vec<ObjFile> = Vec::new();

    for file in main_vtx_files {
        let key = if file.starts_with(&dir) {
            file.strip_prefix(&dir)
                .unwrap()
                .trim_start_matches('/')
                .to_string()
        } else {
            file.clone()
        };
        let tokens = tokens_map
            .get(&key)
            .unwrap_or_else(|| panic!("Token map missing key: {}", key))
            .clone();
        objs.push(compile_file_to_bytecode(key, tokens, &tokens_map));
    }

    /*
     * Linking
     */

    let pb_linking = create_spinner("Linking".to_string());
    let link_start = Instant::now();
    let mut final_file = Linker::link(&mut objs); // Link all Obj files
    final_file = Compiler::optimize(final_file); // Optimize the final bytecode emmited by the Linker

    if debug {
        pb_linking.suspend(|| {
            println!("\n--- BYTECODE ---");
            for (i, instr) in final_file.iter().enumerate() {
                println!("{}->{:?}", i, instr);
            }
            println!("----------------\n");
        });
    }

    pb_linking.finish_and_clear();
    println!(
        "\x1b[32m✔\x1b[0m {:<50} in {:.4}s",
        "Linked",
        link_start.elapsed().as_secs_f32()
    );

    /*
     * Write output
     */
    let pb_writing = create_spinner("Writing output".to_string());
    let write_start = Instant::now();

    let out_path = format!("out/{}", out);

    compile_instr_to_bytes(out_path, &mut final_file).expect("Cannot load binary file");

    pb_writing.finish_and_clear();
    println!(
        "\x1b[32m✔\x1b[0m {:<50} in {:.4}s",
        "Finished writing",
        write_start.elapsed().as_secs_f32()
    );

    binary_compilation::compile_to_binary(&out);

    /*
     * TOTAL TIME
     */
    println!(
        "\n\x1b[1;32mBuild finished\x1b[0m in {:.3}s",
        total_start.elapsed().as_secs_f32()
    );
}

fn ensure_target_dir() {
    let target = std::env::current_dir().unwrap().join("out/bin");

    if !&target.exists() {
        fs::create_dir_all(target).expect("Could not create the binary output directory");
    }
}

fn get_vertex_files_recursive(dir: &str) -> Vec<String> {
    let mut files = Vec::new();
    for entry in WalkDir::new(dir) {
        let entry = entry.expect("Cannot read entry");
        if entry.file_type().is_file()
            && let Some(ext) = entry.path().extension()
            && ext == "vtx"
        {
            files.push(entry.path().to_string_lossy().to_string());
        }
    }

    files
}
