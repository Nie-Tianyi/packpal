use std::fs;
use pulldown_cmark::{html, Options};

pub fn build() {
    unimplemented!()
}


// fn hydrate_post(md_path: String) {
//     let markdown = fs::read_to_string(POSTS_FOLDER.to_owned() + "比特币.md")?;
//     let parser = MarkdownParser::new_ext(&*markdown, Options::all());
//
//     let mut content = String::new();
//     html::push_html(&mut content, parser);
//
//     let template = fs::read_to_string(TEMPLATE_FOLDER.to_owned() + "posts_template.html")?;
//     let res = template
//         .replace("<PostHeading/>", &*format!("<h1>{}</h1>", "比特币"))
//         .replace("<ContentRoot/>", &*content);
//
//     fs::write(BUILD_FOLDER.to_owned() + "比特币.html", res)?;
// }