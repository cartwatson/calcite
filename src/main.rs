use std::fs;
use std::io::Write;
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
    // TODO: V1: hardcoded variables... 0_0
    let content_dir: String = "content/".to_string();
    let template_dir: String = "template/".to_string();
    let template_file: String = "template.html".to_string();
    let template_splitter: String = "<div id=\"content\">".to_string();
    let output_dir: String = "out/".to_string();

    // TODO: V1: copy over all non-html files from ./template to ./out
    let _ = Command::new("cp").arg("-r").arg(&template_dir).arg(&output_dir).output();
    let _ = Command::new("rm").arg(output_dir.clone() + &template_file).output();

    // grab template file
    let template_file = fs::read_to_string(template_dir + &template_file).expect("Should have been able to read template file");
    // break up template write it back to output file
    let template_file_split: Vec<_> = template_file.split(&template_splitter).collect();
    let header: String = template_file_split[0].to_string() + &template_splitter; // TODO: figure out way to not have to add split back to file
    let footer: String = template_file_split[1].to_string();

    // recursively read all files in from content_dir
    for file_name in get_files_recursive(content_dir.clone()) {
        let read_file_path = content_dir.clone() + &file_name;
        let write_file_path: String = output_dir.clone() + &remove_extension(read_file_path.to_string()) + ".html";

        // grab .md
        let contents = fs::read_to_string(read_file_path.clone()).expect("Should have been able to read the file");

        // init out file
        let mut output_buffer: fs::File = fs::File::create(write_file_path).expect("Should have been able to create {write_file_path}");
        let _ = output_buffer.write(&header.clone().into_bytes());

        //------------------------------------------------------------------------------
        // FIX: V1: create stack to hold current state
        let mut html_content: String = String::new();
        for line in contents.lines() {
            // get to front of line and determine what type of line it is
            let first_char = get_first_char(line);
            match first_char {
                '#' => html_content += &heading_processer(line),
                // FIX: V1: not single character, needs to be first thing sandwiched in white space (account for ###)
                // '- ' | '* ' => output_file.write_all(b"Bulleted List\n").expect("error writing to file"), // needs trailing space to prevent collision with italic or bold start to paragraph
                // '0. ' | '1. ' | '2. ' | '3. ' | '4. ' | '5. ' | '6. ' | '7. ' | '8. ' | '9. ' => output_file.write_all(b"Ordered List\n").expect("error writing to file"),
                // '```' => output_file.write_all(b"Codeblock\n").expect("error writing to file"),
                // '>' => output_file.write_all(b"Blockquote\n").expect("error writing to file"),
                // blank line, empty state_stack completely
                _ => html_content += &paragraph_processer(line),
                // '\n' => continue,
                // '<' => content += line; // assume the line is regular HTML // doesn't need any processing
            }
        }
        //------------------------------------------------------------------------------

        let _ = output_buffer.write(&html_content.into_bytes());
        let _ = output_buffer.write(&footer.clone().into_bytes());
    }
}

fn remove_extension(full_name: String) -> String {
    // remove file extentension (everything before '.')
    return full_name.split(".").collect::<Vec<_>>()[0].to_string(); // HACK: V1: fix
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
    //
    // [text](link) // no white space in link FYI
    let links_regex = Regex::new(r"\[(?<text>.*?)\]\((?<link>\S+?)\)").unwrap();
    let formatted_line = links_regex.replace_all(line, "<a href=\"${link}\">${text}</a> "); // TODO: V1: add target="_"; internal links redirecter, external pages send to new tab

    // TODO: V1:
    // images
    // bold   // must not start or end with whitespace; _ doesn't work in word only around
    // italic // must not start or end with whitespace; _ doesn't work in word only around
    // inline code // don't mar codeblocks

    let html = format!("<p>{formatted_line}</p>\n");
    return html;
}

fn get_files_recursive(base_dir: String) -> Vec<String> {
    // TODO: V0: recursively read all files in from content_dir
    // input "content/"
    //
    // content/
    // ├── blog
    // │   ├── article1.md
    // │   └── article2.md
    // └── test.md
    //
    // returns [
    //   blog/article1.md
    //   blog/article2.md
    //   test.md
    // ]
    //
    // NOTE: do NOT return "content/" in file name

    let mut entries: Vec<String> = vec![];
    entries.push("content/index.md".to_string()); // DEBUG
    // let temp = fs::read_dir(base_dir);
    // entries.sort();
    return entries;
}
