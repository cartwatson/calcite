use regex::Regex;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::process::Command;

fn main() {
    // TODO: V1: hardcoded variables... 0_0
    // TODO: V1: convert to paths
    let content_dir: String = "content/".to_string();
    let template_dir: String = "template/".to_string();
    let template_file: String = "template.html".to_string();
    let output_dir: String = "out/".to_string(); // TODO: V1: change to `dist/`
    let template_splitter: String = "<div id=\"content\">".to_string();

    // HACK: V1: copy over all non-html files from ./template to ./out
    // TODO: V1: allow multiple template files, in subdir (ie blog has a different template)
    let _ = Command::new("cp").arg("-r").arg(&template_dir).arg(&output_dir).output();
    let _ = Command::new("rm").arg(output_dir.clone() + &template_file).output();

    // grab template file
    let template_file = fs::read_to_string(template_dir + &template_file).expect("Should have been able to read template file\n");
    // break up template write it back to output file
    let template_file_split: Vec<_> = template_file.split(&template_splitter).collect();
    let header: String = template_file_split[0].to_string() + &template_splitter; // TODO: figure out way to not have to add split back to file
    let footer: String = template_file_split[1].to_string();

    for file_path in get_files_recursive(content_dir.clone()) {
        let mut write_file_path = Path::new(&output_dir).join(file_path.strip_prefix(&content_dir).expect("unable to remove `content/` from path"));

        if Path::new(&file_path).extension().expect("Unable to get file extension") != "md" {
            // HACK: V1: figure out better way to copy files
            let _ = Command::new("cp").arg("-r").arg(&file_path).arg(&write_file_path).output();
            continue;
        } else {
            write_file_path = write_file_path.with_extension("html");
        }

        // grab .md
        let contents = fs::read_to_string(&file_path).expect("Should have been able to read the content file\n");

        // init out file
        let mut output_file: fs::File = create_file(write_file_path.as_path());
        let _ = output_file.write(&header.clone().into_bytes()).expect("Unable to write to output file");

        //------------------------------------------------------------------------------
        // FIX: V1: create stack to hold current state
        let mut html_content: String = String::new();
        for line in contents.lines() {
            // base cases
            match line {
                "" => {
                    continue;
                }
                "---" => {
                    html_content += "<hr>\n";
                    continue;
                }
                _ => (), // continue processing
            }

            // get to front of line and determine what type of line it is
            let first_char = get_first_char(line);
            match first_char {
                // heading
                '#' => html_content += &heading_processer(line),

                // images
                '!' => html_content += &image_processer(line),

                // unordered list
                // '-' | '*' => output_file.write_all(b"Bulleted List\n").expect("error writing to file"), // needs trailing space to prevent collision with italic or bold start to paragraph

                // ordered list
                // '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => output_file.write_all(b"Ordered List\n").expect("error writing to file"),

                // code block (need way to distinguish here)
                // '```' => output_file.write_all(b"Codeblock\n").expect("error writing to file"),

                //  blockquote
                // '>' => output_file.write_all(b"Blockquote\n").expect("error writing to file"),

                // assume the line is regular HTML // doesn't need any processing
                '<' => html_content += line,

                // blank line, empty state_stack completely
                _ => html_content += &paragraph_processer(line),
            }
        }
        //------------------------------------------------------------------------------

        let _ = output_file.write(&html_content.into_bytes());
        let _ = output_file.write(&footer.clone().into_bytes());
    }
}

fn get_first_char(line: &str) -> char {
    for character in line.chars() {
        if !character.is_whitespace() {
            return character;
        }
    }
    return '\n';
}

fn image_processer(line: &str) -> String {
    let regexes_and_targets: Vec<(Regex, &str)> = vec![
        // HACK: order matters!
        (
            // images - relative
            Regex::new(r"!\[(?<about>.*?)\]\((?<link>.+?)\)").unwrap(),
            "<img src=\"${link}\" alt=\"${about}\" title=\"${about}\"></img>",
        ),
        (
            // images - web
            Regex::new(r"!\[(?<about>.*?)\]\((?<link>https:\/\/\S+?)\)").unwrap(),
            "<img src=\"${link}\" alt=\"${about}\" title=\"${about}\"></img>",
        ),
    ];

    let mut formatted_line: String = line.to_owned();
    for tuple in regexes_and_targets {
        let regex: Regex = tuple.0;
        let target: &str = tuple.1;

        let temp: String = regex.replace_all(&formatted_line, target).into_owned();
        formatted_line = temp.clone();
    }

    return formatted_line;
}

fn heading_processer(line: &str) -> String {
    // correlate heading level to html tag
    let mut split_line = line.split(' ').collect::<Vec<_>>();
    let heading_level = split_line[0].chars().count();
    split_line.remove(0); // remove ### from front of line
    let heading = split_line.join(" ");
    let html = format!("<h{heading_level}>{heading}</h{heading_level}>\n");
    return html;
}

fn paragraph_processer(line: &str) -> String {
    // FIX: V1: can't handle the following
    // *just italic****bold italic* just bold**
    let regexes_and_targets: Vec<(Regex, &str)> = vec![
        // HACK: order matters!
        (
            // bold
            Regex::new(r"(\*\*|__|\*_|_\*)(?<word>\S.+\S)(\*\*|__|\*_|_\*)").unwrap(),
            "<strong>${word}</strong>",
        ),
        (
            // italic
            Regex::new(r"(\*|_)(?<word>\S.+\S)(\*|_)").unwrap(),
            "<i>${word}</i>",
        ),
        (
            // inline code
            Regex::new(r"(\`)(?<word>\S.+\S)(\`)").unwrap(),
            "<code>${word}</code>",
        ),
        (
            // https links
            Regex::new(r"\[(?<text>[^\[\]]*?)\]\((?<link>https:\/\/\S+?)\)").unwrap(),
            "<a target=\"_blank\" href=\"${link}\" title=\"${text}\">${text}</a>",
        ),
        (
            // converts local .md to .html
            Regex::new(r".md\)").unwrap(),
            ".html)",
        ),
        (
            // converts local .html to valid links for the dir
            Regex::new(r"\[(?<text>[^\[\)]*?)\]\((?<link>\S+?(.md|.html))\)").unwrap(),
            "<a href=\"${link}\">${text}</a>",
        ),
    ];

    let mut formatted_line: String = line.to_owned();
    for tuple in regexes_and_targets {
        let regex: Regex = tuple.0;
        let target: &str = tuple.1;

        let temp: String = regex.replace_all(&formatted_line, target).into_owned();
        formatted_line = temp.clone();
    }

    return format!("<p>{formatted_line}</p>\n");
}

fn get_files_recursive(base_dir: String) -> Vec<String> {
    // input "content/"
    // eg file structure
    //
    // content/
    // ├── blog
    // │   ├── article1.md
    // │   └── article2.md
    // └── test.md
    //
    // returns
    // [
    //   content/blog/article1.md
    //   content/blog/article2.md
    //   content/test.md
    // ]
    //
    // NOTE: return full local path; include "content/" in file name

    let mut files: Vec<String> = vec![];
    for entry in fs::read_dir(base_dir.clone()).expect("Unable to read provided content directory") {
        // HACK: V2: handle non-UTF8 enconded paths better
        // path.display().to_string() is a hack, taken from [here](https://stackoverflow.com/a/61142928)

        let entry = entry.expect("Unable to convert from ReadDir to DirEntry while obtaining content files");
        let path = entry.path();
        if path.is_dir() {
            let more_files = get_files_recursive(path.display().to_string());
            for file in more_files {
                files.push(file);
            }
        } else {
            files.push(path.display().to_string());
        }
    }

    files.sort();
    return files;
}

fn create_file(file_path: &Path) -> fs::File {
    // strip filename from path
    let dir_path: &Path = file_path.parent().expect("Unable to get parent");
    if !dir_path.try_exists().expect("Unable to verify existance of dir_path during directory creation") {
        let _ = fs::create_dir_all(dir_path).expect("Unable to create dir");
    }
    return File::create(file_path).expect("Should have been able to write output file\n");
}
