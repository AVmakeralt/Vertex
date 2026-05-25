use std::fs;
use std::path::Path;
use std::process::{self, Command};
use std::time::{Duration, Instant};

use indicatif::{ProgressBar, ProgressStyle};

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

/// You need to have zig toolchain installed to compile this. We are creating
/// temp_launcher and than compiling it with:
/// `zig build-exe tmp_launcher_path.zig runtime_path -lc -lunwind
/// -Doptimize=ReleaseSmall femit-bin=out/bin/out`
pub fn compile_to_binary(out: &str) {
    let pb = create_spinner("Compiling with zig".to_string());
    let compiler_timer = Instant::now();
    let bytecode_path = format!("out/{}", out);
    let temp_launcher = format!(
        r#"
const std = @import("std");
extern fn vm_entry(ptr: [*]const u8, len: usize) void;
var program = @embedFile("{bytecode_path}");
pub fn main() !void {{
    vm_entry(program.ptr, program.len);
}}
"#,
        bytecode_path = bytecode_path
    );
    let tmp_launcher_path = "tmp_launcher.zig";
    fs::write(tmp_launcher_path, temp_launcher).unwrap();
    let runtime_path = find_libvm_runtime(Path::new("."))
        .expect("Build error: Cannot find static libvm_runtime.a library. Ensure it is in the same directory as vertexC or set VERTEX_RUNTIME_PATH.");
    let output = Command::new("zig")
        .args([
            "build-exe",
            "tmp_launcher.zig",
            &runtime_path,
            "-lc",
            "-lunwind",
            "-Doptimize=ReleaseSmall",
            &format!("-femit-bin=out/bin/{}", out),
        ])
        .output()
        .expect("Failed to run zig");

    if !output.status.success() {
        pb.finish_and_clear();
        eprintln!("\x1b[1;31mZig compilation failed:\x1b[0m");
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        process::exit(-1);
    }
    fs::remove_file(tmp_launcher_path).unwrap();

    pb.finish_and_clear();
    println!(
        "\x1b[32m✔\x1b[0m {:<50} in {:.4}s",
        "Finished compiling with zig",
        compiler_timer.elapsed().as_secs_f32()
    );
}

fn find_libvm_runtime(_start: &Path) -> Option<String> {
    // 1. Check environment variable
    if let Ok(path) = std::env::var("VERTEX_RUNTIME_PATH")
        && Path::new(&path).is_file()
    {
        return Some(path);
    }

    // 2. Check near the executable
    if let Ok(mut exe_path) = std::env::current_exe() {
        exe_path.pop(); // Remove executable name
        let p = exe_path.join("libvm_runtime.a");
        if p.is_file() {
            return Some(p.to_string_lossy().to_string());
        }
    }

    // 3. Check CWD
    if let Ok(cwd) = std::env::current_dir() {
        let p = cwd.join("libvm_runtime.a");
        if p.is_file() {
            return Some(p.to_string_lossy().to_string());
        }
    }

    None // Fallback if nothing is found
}
