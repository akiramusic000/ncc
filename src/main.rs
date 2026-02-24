use anyhow::Context;
use mktemp::Temp;
use regex::Regex;
use std::{env::args, fs, path::Path, process::Command};

fn main() -> anyhow::Result<()> {
    let mut temps = Vec::new();

    let args = args()
        .skip(1)
        .map(|arg| {
            let path = Path::new(&arg);
            if !path.exists() {
                return arg;
            }

            let compiled = compile(path);
            let temp = Temp::new_path().release();
            fs::write(&temp, compiled).expect("error!");

            let arg = temp.display().to_string();
            temps.push(temp);
            arg
        })
        .collect::<Vec<_>>();

    let mut command = Command::new("clang")
        .args(["-x", "c"])
        .args(args)
        .spawn()
        .context("failed clang spawn!")?;
    command.wait()?;

    for temp in temps {
        fs::remove_file(temp).context("failed temp rm!")?;
    }

    Ok(())
}

fn compile(path: &Path) -> String {
    let input = fs::read_to_string(path).expect("error!");

    let mut output = Vec::new();
    let mut defer_stack: Vec<Vec<(usize, String)>> = vec![Vec::new()];

    // Regex for UFCS: matches 'identifier.identifier('
    // Group 1: receiver, Group 2: function name
    let ufcs_regex = Regex::new(r"([a-zA-Z_]\w*)\.([a-zA-Z_]\w*)\(").unwrap();

    for (i, line) in input.lines().enumerate() {
        let trimmed = line.trim();

        // 1. Handle Defer
        if trimmed.starts_with("defer ") {
            let statement = trimmed.strip_prefix("defer ").unwrap();
            defer_stack
                .last_mut()
                .unwrap()
                .push((i, statement.to_string()));
            continue; // Don't print the 'defer' line to the C output
        }

        // 2. Handle Scope Entry
        if trimmed.contains('{') {
            defer_stack.push(Vec::new());
        }

        // 3. Handle UFCS (Basic transformation)
        let mut processed_line = line.to_string();
        if ufcs_regex.is_match(&processed_line) {
            processed_line = ufcs_regex
                .replace_all(&processed_line, "$2($1, ")
                .to_string();
            // Clean up trailing comma if no other args existed
            processed_line = processed_line.replace(", )", ")");
        }

        // 4. Handle Scope Exit (Inject defers)
        if trimmed.contains('}')
            && let Some(defers) = defer_stack.pop()
        {
            for (i, d) in defers.iter().rev() {
                output.push(format!("#line {} \"{}\"", i + 1, path.display()));
                output.push(format!("    {}", d));
            }
        }

        output.push(format!("#line {} \"{}\"", i + 1, path.display()));
        output.push(processed_line);
    }

    output.join("\n")
}
