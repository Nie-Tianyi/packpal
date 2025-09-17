use pulldown_cmark::{Options, Parser, html};
use std::fs;
use std::path::Path;

pub fn build() {
    unimplemented!()
}

fn hydrate_post(
    md_path: impl AsRef<Path>,
    template_path: impl AsRef<Path>,
    output_path: impl AsRef<Path>,
) {
    let markdown = fs::read_to_string(md_path).unwrap();
    let parser = Parser::new_ext(&*markdown, Options::all());

    let mut content = String::new();
    html::push_html(&mut content, parser);

    let template = fs::read_to_string(template_path).unwrap();
    let res = template
        .replace(
            "<PostHeading/>",
            &*format!("<h1 class=\"post-title\">{}</h1>", "比特币"),
        )
        .replace("<ContentRoot/>", &*content);

    fs::write(output_path, res).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::functions::new::new_project;
    use crate::functions::{BLOG, TEST_ROOT};

    fn new_test_project(project_name: &str) -> String {
        let test_project_root = format!("{TEST_ROOT}{project_name}");
        new_project(&test_project_root);
        fs::write(format!("{}/posts/比特币.md", &test_project_root), BLOG)
            .expect("尝试创建测试博客失败");
        test_project_root
    }

    #[test]
    fn test_hydration() {
        let test_project_root = new_test_project("test_hydration");
        hydrate_post(
            format!("{test_project_root}/posts/比特币.md"),
            format!("{test_project_root}/templates/posts_template.html"),
            format!("{test_project_root}/build/比特币.html"),
        )
    }
}
