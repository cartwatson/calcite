use std::env;
use std::fs;
use std::fs::File;
// use std::io;
use std::io::prelude::*;

fn get_first_char(line: &str) -> char {
    for character in line.chars() {
        if ! character.is_whitespace() {
            return character;
        }
    }
    return '\n'
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let read_file_path = &args[1]; // DEBUG: delete once content_dir works
    // let content_dir: String  = "content/".to_string();
    let template_dir: String = "template/".to_string();
    let template_splitter: String = "<div id=\"content\">".to_string();
    let output_dir: String   = "out/".to_string();

    // TODO: copy over all non-html files from ./template to ./out

    // read file in
    // TODO: recursively read all files in from content_dir
    let contents = fs::read_to_string(read_file_path).expect("Should have been able to read the file");

    // WARN: -------------------------------------------------------------------
    // NOTE: everything below should be done recursively for each content file
    // WARN: -------------------------------------------------------------------

    // write file out
    let _ = fs::create_dir(&output_dir);
    let write_file_path: String = output_dir + &get_file_name(read_file_path.to_string()) + ".html";
    let mut output_file = File::create(write_file_path).expect("Should have been able to create {write_file_path}");

    // grab template file
    let template_file = fs::read_to_string(template_dir + "template.html").expect("Should have been able to read template file");
    // break up template write it back to output file
    // TODO: figure out way to not have to add split back to file
    let template_file_split: Vec<_> = template_file.split(&template_splitter).collect();
    let header: String = template_file_split[0].to_string() + &template_splitter;
    let footer: String = template_file_split[1].to_string();
    let _ = output_file.write(&header.into_bytes());

//------------------------------------------------------------------------------
// TODO: FIX THIS SHIT
// FIX DEBUG WARN WARNING TODO INFO NOTE ANYTHING: 
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
            _   => content += &paragraph_processer(line),
        }
    }
//------------------------------------------------------------------------------

    let _ = output_file.write(&content.into_bytes());
    let _ = output_file.write(&footer.into_bytes());
}

fn get_file_name(full_name: String) -> String {
    // remove everything before the file name (everything after last '/')
    let mut temp = full_name.split("/").collect::<Vec<_>>().last().expect("COULDNT SPLIT").to_string();
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
    let html = format!("<p>{line}</p>\n");
    return html;
}
