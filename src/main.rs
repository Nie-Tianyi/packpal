use pulldown_cmark::{Options, Parser, html};
use std::error::Error;
use std::fs;

static TEMPLATE_FOLDER: &str = "./templates/";
static BUILD_FOLDER: &str = "./build/";
static POSTS_FOLDER: &str = "./posts/";

fn main() -> Result<(), Box<dyn Error>> {
    let markdown = fs::read_to_string(POSTS_FOLDER.to_owned() + "比特币.md")?;
    let parser = Parser::new_ext(&*markdown, Options::all());

    let mut content = String::new();
    html::push_html(&mut content, parser);

    let template = fs::read_to_string(TEMPLATE_FOLDER.to_owned() + "template.html")?;
    let res = template
        .replace("<PostHeading/>", &*format!("<h1>{}</h1>", "比特币"))
        .replace("<ContentRoot/>", &*content);

    fs::write(BUILD_FOLDER.to_owned() + "比特币.html", res)?;

    Ok(())
}
