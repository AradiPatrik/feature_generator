use std::{
    fs::{create_dir_all, read_to_string, File},
    path::Path,
};

use handlebars::Handlebars;
use serde::Serialize;

pub fn generate_file(
    parent: &Path,
    handlebars: &Handlebars,
    handlebars_context: &impl Serialize,
    file_name: &str,
    template_content: &str,
) {
    create_dir_all(parent).unwrap_or_else(|_| {
        panic!(
            "{}",
            format!(
                "{:?} <-- this directory should either exists, or we should be able to create it",
                parent,
            )
            .to_string()
        )
    });

    let file = File::create(parent.join(file_name)).unwrap_or_else(|_| {
        panic!(
            "Wasn't able to create file!\nfile: {}\npath: {:?}",
            file_name, parent
        )
    });

    handlebars
        .render_template_to_write(template_content, handlebars_context, file)
        .unwrap_or_else(|error| {
            panic!(
                "Couldn't render template!\nfile: {}\npath: {:?}\n\nDetails\n{:?}",
                file_name, parent, error.desc
            )
        });
}

pub fn append_line_below(file_path: &Path, pattern: &str, line: &str) {
    let file = read_to_string(file_path).unwrap_or_else(|_e| {
        panic!(
            "Error during modifying existing file!\nFile path: {}",
            file_path
                .to_str()
                .expect("Failed to convert path to string, maybe not valid unicode?")
        )
    });
    let mut done = false;
    let lines: Vec<String> = file.lines().fold(Vec::new(), |mut acc, curr_line| {
        let curr_line = curr_line.to_string();
        acc.push(curr_line.clone());
        if curr_line.contains(pattern) && !done {
            acc.push(line.to_string());
            done = true;
        }
        acc
    });

    std::fs::remove_file(file_path).unwrap_or_else(|_| {
        panic!(
            "Error during modifying existing file!\nFile path: {}",
            file_path
                .to_str()
                .expect("Failed to convert path to string, maybe not valid unicode?")
        )
    });
    std::fs::write(file_path, lines.join("\n") + "\n").unwrap_or_else(|_| {
        panic!(
            "Error during modifying existing file!\nFile path: {}",
            file_path
                .to_str()
                .expect("Failed to convert path to string, maybe not valid unicode?")
        )
    });
}

pub fn add_line_to_file(file_path: &Path, line: &str) {
    let file = read_to_string(file_path).unwrap_or_else(|_| {
        panic!(
            "Could not read file {}",
            file_path
                .to_str()
                .expect("Should be able to convert path to string")
        )
    });
    let mut lines: Vec<&str> = file.lines().collect();
    lines.push(line);

    std::fs::remove_file(file_path).unwrap_or_else(|_| {
        panic!(
            "Error during modifying existing file!\nFile path: {}",
            file_path
                .to_str()
                .expect("Failed to convert path to string, maybe not valid unicode?")
        )
    });
    std::fs::write(file_path, lines.join("\n") + "\n").unwrap_or_else(|_| {
        panic!(
            "Error during modifying existing file!\nFile path: {}",
            file_path
                .to_str()
                .expect("Failed to convert path to string, maybe not valid unicode?")
        )
    });
}

pub fn overwrite_file_at_path(file_path: &Path, lines: Vec<String>) {
    let parent_file = file_path.parent().unwrap_or_else(|| {
        panic!("Couldn't find parent path in path: {:?}", file_path,);
    });
    create_dir_all(parent_file).unwrap();
    let _ = std::fs::remove_file(file_path);
    std::fs::write(file_path, lines.join("\n") + "\n").unwrap_or_else(|_| {
        panic!(
            "Error during modifying existing file!\nFile path: {}",
            file_path
                .to_str()
                .expect("Failed to convert path to string, maybe not valid unicode?")
        )
    });
}
