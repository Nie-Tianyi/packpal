use std::fs;
use std::path::Path;

mod posts;

use crate::functions::build::posts::{PostTemplate, RawPost, SourceType};

static DEFAULT_PUBLIC_PATH: &str = "public/";

static DEFAULT_ARTICLES_PATH: &str = "articles/";

/// 扫秒指定文件夹source_dir下的源文件（e.g. markdown文件），生成静态站点文件到dist_dir内
pub fn build(
    source_dir: impl AsRef<Path>,
    template_dir: impl AsRef<Path>,
    dist_dir: impl AsRef<Path>,
) {
    let dist_dir = dist_dir.as_ref().display();
    let template_dir = template_dir.as_ref().display();

    // 扫描所有博客源文件，例如markdown文件
    let sources = scan_source_file(source_dir);
    let post_template = PostTemplate::from_path(format!("{template_dir}posts_template.html"));

    // 在dist_dir下面生成 articles 和 public 文件夹
    let dist_public_dir = format!("{dist_dir}{DEFAULT_PUBLIC_PATH}");
    let dist_articles_dir = format!("{dist_dir}{DEFAULT_ARTICLES_PATH}");
    fs::create_dir_all(&dist_public_dir).expect("[错误]生成public文件夹失败");
    fs::create_dir_all(&dist_articles_dir).expect("[错误]生成articles文件夹失败");
    // 生成博客文件
    for source in sources {
        let post = source.hydrate(&post_template);
        post.write_into_folder(&dist_articles_dir)
    }
}

// 扫描posts文件夹下所有markdown文件，并返回其元数据
fn scan_source_file(dir: impl AsRef<Path>) -> Vec<RawPost> {
    let mut posts = Vec::new();
    for entry in fs::read_dir(&dir)
        .unwrap_or_else(|_| panic!("[错误]读取目录{}失败", dir.as_ref().display()))
        .flatten()
    {
        let name = entry
            .file_name()
            .into_string()
            .expect("[错误]读取文件名时遇到无效UTF-8字符");
        let name = name.split('.').collect::<Vec<&str>>()[0];
        let name = name.to_string();

        let path = entry.path();

        let source_type = match path
            .extension()
            .map(|s| s.to_str().expect("[错误]读取文件名扩展时遇到无效UTF-8字符"))
        {
            Some("md") => SourceType::Markdown,
            _ => continue,
        };

        posts.push(RawPost::new(name, source_type, path))
    }

    posts
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
    fn test_scan_folder() {
        let test_prj_root = new_test_project("test_scan_folder");
        let posts = scan_source_file(format!("{test_prj_root}/posts/"));
        println!("{:?}", posts);
    }

    #[test]
    fn test_build() {
        let test_prj_root = new_test_project("test_build");
        build(
            format!("{test_prj_root}/posts/"),
            format!("{test_prj_root}/templates/"),
            format!("{test_prj_root}/build/"),
        );
    }
}
