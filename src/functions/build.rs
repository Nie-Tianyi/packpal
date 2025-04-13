use pulldown_cmark::{Options, html, Parser};
use std::fs;

pub fn build() {
    unimplemented!()
}

fn hydrate_post(md_path: String) {
    let markdown = fs::read_to_string(md_path).unwrap();
    let parser = Parser::new_ext(&*markdown, Options::all());

    let mut content = String::new();
    html::push_html(&mut content, parser);

    let template = fs::read_to_string("./templates/".to_owned() + "posts_template.html").unwrap();
    let res = template
        .replace("<PostHeading/>", &*format!("<h1>{}</h1>", "比特币"))
        .replace("<ContentRoot/>", &*content);

    fs::write("BUILD_FOLDER".to_owned() + "比特币.html", res)?;
}
