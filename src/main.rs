use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

use regex::Regex;

fn get_first_char(line: &str) -> char {
    for character in line.chars() {
        if !character.is_whitespace() {
            return character;
        }
    }
    return '\n';
}

fn main() {
    // hardcoded variables... 0_0
    let read_file_path = "content/test.md"; // DEBUG: delete once content_dir works
    // let content_dir: String  = "content/".to_string();
    let template_dir: String = "template/".to_string();
    let template_file: String = "template.html".to_string();
    let template_splitter: String = "<div id=\"content\">".to_string();
    let output_dir: String = "out/".to_string();

    // TODO: copy over all non-html files from ./template to ./out
    // V0 copy all files and rm template.html
    let _ = Command::new("cp")
        .arg("-r")
        .arg(&template_dir)
        .arg(&output_dir)
        .output();
    let _ = Command::new("rm")
        .arg(output_dir.clone() + &template_file)
        .output();

    // grab template file
    let template_file = fs::read_to_string(template_dir + &template_file)
        .expect("Should have been able to read template file");
    // break up template write it back to output file
    let template_file_split: Vec<_> = template_file.split(&template_splitter).collect();
    let header: String = template_file_split[0].to_string() + &template_splitter; // TODO: figure out way to not have to add split back to file
    let footer: String = template_file_split[1].to_string();

    // read file in
    // TODO: recursively read all files in from content_dir
    let contents =
        fs::read_to_string(read_file_path).expect("Should have been able to read the file");

    // WARN: -------------------------------------------------------------------
    // NOTE: everything below should be done recursively for each content file
    // WARN: -------------------------------------------------------------------

    // write file out
    let write_file_path: String = output_dir + &get_file_name(read_file_path.to_string()) + ".html";
    let mut output_file =
        File::create(write_file_path).expect("Should have been able to create {write_file_path}");

    let _ = output_file.write(&header.into_bytes());

    //------------------------------------------------------------------------------
    // TODO: FIX THIS SHIT
    // TODO: create stack to hold current state
    // let mut state_stack: Vec<String> = Vec::new();
    let mut content: String = String::new();
    for line in contents.lines() {
        // get to front of line and determine what type of line it is
        let first_char = get_first_char(line);

        match first_char {
            '#' => content += &heading_processer(line),
            // '>' =>    output_file.write_all(b"Blockquote\n").expect("error writing to file"),
            // '-'|'*' => output_file.write_all(b"Bulleted List\n").expect("error writing to file"),
            // '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => output_file.write_all(b"Ordered List\n").expect("error writing to file"),
            // '`' => output_file.write_all(b"Codeblock\n").expect("error writing to file"),
            // blank line, empty state_stack completely
            // '\n' => continue,
            // assume the line is regular HTML
            // '<' => content += line,
            // paragraph
            _ => content += &paragraph_processer(line),
        }
    }
    //------------------------------------------------------------------------------

    let _ = output_file.write(&content.into_bytes());
    let _ = output_file.write(&footer.into_bytes());
}

fn get_file_name(full_name: String) -> String {
    // remove everything before the file name (everything after last '/')
    let mut temp = full_name
        .split("/")
        .collect::<Vec<_>>()
        .last()
        .expect("COULDNT SPLIT")
        .to_string();
    // remove file extentension (everything before '.')
    temp = temp.split(".").collect::<Vec<_>>()[0].to_string();

    return temp;
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
    // base case of blank line
    if line == "" {
        return line.to_string();
    }

    // use regex to filter links
    // [text](link) // no white space in link FYI
    let links_regex = Regex::new(r"\[(?<text>.*?)\]\((?<link>\S+?)\)").unwrap();
    let formatted_line = links_regex.replace_all(line, "<a href=\"${link}\">${text}</a> ");

    let html = format!("<p>{formatted_line}</p>\n");
    return html;
}
