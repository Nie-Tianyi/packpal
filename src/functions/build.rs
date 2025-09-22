use crate::functions::build::index::{IndexInfo, IndexTemplate};
use chrono::Datelike;
use std::fs;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

mod index;
mod posts;

use crate::functions::build::posts::{PostMetadataList, PostTemplate, RawPost, SourceType};

#[derive(Clone, Debug)]
pub struct Avatar {
    data: Vec<u8>, // 头像的二进制数据
    url: String,   // 相对于网站根目录的url
}

impl Avatar {
    pub fn imports(path: PathBuf) -> Self {
        let data = fs::read(&path).expect("[错误]读取头像文件失败，无效路径");
        Self {
            data,
            url: format!("public/{}", path.file_name().unwrap().to_string_lossy()),
        }
    }

    pub fn write_into_file(&self, path: PathBuf) {
        fs::write(&path, &self.data).expect("[错误]写入头像文件失败，无效路径");
    }
}

#[derive(Clone)]
pub struct SiteFactory {
    blog_name: String,
    avatar: Avatar,
    email: String,
    github: String,
    motto: String,
    posts: Vec<RawPost>,
    metadata: PostMetadataList,
    post_template: PostTemplate,
    index_template: IndexTemplate,
}

impl SiteFactory {
    pub fn new(
        blog_name: String,
        avatar: Avatar,
        email: String,
        github: String,
        motto: String,
        posts: Vec<RawPost>,
        metadata: PostMetadataList,
        post_template: PostTemplate,
        index_template: IndexTemplate,
    ) -> Self {
        SiteFactory {
            blog_name,
            avatar,
            email,
            github,
            motto,
            posts,
            metadata,
            post_template,
            index_template,
        }
    }

    pub fn build(self, out_dir: impl AsRef<Path>) {
        let out_dir = out_dir.as_ref().display();
        // 在dist_dir下面生成 articles 和 public 文件夹
        let dist_public_dir = format!("{out_dir}public/");
        let dist_articles_dir = format!("{out_dir}articles/");
        fs::create_dir_all(&dist_public_dir).expect("[错误]构建时创建文件夹失败");
        fs::create_dir_all(&dist_articles_dir).expect("[错误]构建时创建文件夹失败");

        let metadata_list = PostMetadataList::from_json("posts.json");
        let mut post_info_list = Vec::new();
        for mut post in self.posts {
            if let Some(metadata) = self.metadata.get(&post.name) {
                post.set_date(metadata.date.clone());
                post.set_tag(metadata.tags.clone());
            }
            let (post_info, target) = post.render(&self.post_template);
            target.write_into_folder(&dist_articles_dir);
            post_info_list.push(post_info);
        }

        self.avatar
            .write_into_file(format!("{}{}", out_dir, self.avatar.url).into());

        let this_year = chrono::Local::now().year();
        let index_info = IndexInfo {
            site_name: self.blog_name.clone(),
            motto: self.motto.clone(),
            avatar: self.avatar.clone(),
            email: self.email.clone(),
            github: self.github.clone(),
            posts: post_info_list,
            date: this_year.to_string(),
        };
        let index = self.index_template.render(index_info);
        index.write_into_file(format!("{out_dir}index.html").into());
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct SiteConfig {
    blog_name: String,
    #[serde(rename = "avatar")]
    avatar_path: String,
    email: String,
    github: String,
    motto: String,
}

impl SiteConfig {
    fn from_json(path: PathBuf) -> SiteConfig {
        let config = fs::read_to_string(path).expect("[错误]读取配置文件失败，无效路径");
        serde_json::from_str(&config).expect("[错误]解析配置文件失败，无效JSON格式")
    }
}

/// 扫秒指定文件夹source_dir下的源文件（e.g. markdown文件），生成静态站点文件到dist_dir内
pub fn build(
    source_dir: impl AsRef<Path>,
    template_dir: impl AsRef<Path>,
    dist_dir: impl AsRef<Path>,
) {
    let raw_posts = scan_source_file(&source_dir);
    let metadata = PostMetadataList::from_json(format!("{}/metadata.json", source_dir.as_ref().display()));
    let config = SiteConfig::from_json("config.json".into());
    
    let factory = SiteFactory::new(
        config.blog_name,
        Avatar::imports(config.avatar_path.into()),
        config.email,
        config.github,
        config.motto,
        raw_posts,
        metadata,
        PostTemplate::imports(format!("{}/post.html", template_dir.as_ref().display())),
        IndexTemplate::imports(format!("{}/index.html", template_dir.as_ref().display())),
    );
    factory.build(dist_dir);
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
