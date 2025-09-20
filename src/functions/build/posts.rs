use pulldown_cmark::{html, Options, Parser};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Default)]
pub enum SourceType {
    #[default]
    Markdown,
}

/// hydrate后的HTML文件
#[derive(Clone, Debug)]
pub struct HTMLPost {
    name: String,
    content: String,
}

impl HTMLPost {
    pub fn new(name: String, content: String) -> Self {
        HTMLPost { name, content }
    }

    pub fn write_into_folder(self, path: impl AsRef<Path>) {
        let path = path.as_ref().display();
        let path = format!("{path}{}.html", self.name);
        fs::write(path, self.content).unwrap_or_else(|_| panic!("[错误]该路径父文件夹不存在"))
    }
}

/// 文章元数据
#[derive(Clone, Serialize, Deserialize)]
pub struct PostMetadata {
    #[serde(rename = "post_name")]
    name: String,
    #[serde(rename = "create_at")]
    date: String,
    #[serde(rename = "hashtags")]
    tags: Vec<String>,
}

impl PostMetadata {
    pub fn new(name: String, date: String, tags: Vec<String>) -> Self {
        Self { name, date, tags }
    }

    pub fn from_json(path: impl AsRef<Path>) -> Vec<Self> {
        unimplemented!()
    }

    pub fn write_into(self, path: impl AsRef<Path>) {
        unimplemented!()
    }
}

/// 博客源文件的元数据，包括文件名称、文件类型、相对路径等
#[derive(Clone, Debug)]
pub struct RawPost {
    name: String,                 // 文件名，也是博客的标题
    source_type: SourceType,      // 文件类型，现在只支持 markdown 文件
    path: PathBuf,                // 文件路径（相对路径）
    content: Option<String>,      // 文件内容，懒加载，只有在hydrate的时候才load进内存
    tags: Option<Vec<String>>,    // 文章的tags
    date: Option<String>,         // 文章日期 例如 2024.01.01
    reading_time: Option<String>, // 预计阅读需要的时间 例如 5分钟
}

impl RawPost {
    pub fn new(name: String, source_type: SourceType, path: PathBuf) -> Self {
        RawPost {
            name,
            source_type,
            path,
            content: None,
            tags: None,
            date: None,
            reading_time: None,
        }
    }

    pub fn hydrate(mut self, template: &PostTemplate) -> HTMLPost {
        if self.content.is_none() {
            self.load_content_from_path()
        }

        let binding = self.content.take().unwrap();
        let parser = Parser::new_ext(&binding, Options::all());

        let mut content = String::new();
        html::push_html(&mut content, parser);

        let post_content = template
            .0
            .replace(
                "<PostDate/>",
                &self.date.take().unwrap_or("Undefined Date".to_string()),
            )
            .replace(
                "<ReadingTime/>",
                &self.reading_time.take().unwrap_or("<1分钟".to_string()),
            )
            .replace("<PostTags/>", &self.get_tags())
            .replace(
                "<PostHeading/>",
                &format!("<h1 class=\"post-title\">{}</h1>", self.name),
            )
            .replace("<ContentRoot/>", &content);

        HTMLPost::new(self.name, post_content)
    }

    fn load_content_from_path(&mut self) {
        self.content = Some(
            fs::read_to_string(&self.path).unwrap_or_else(|_| panic!("[错误]找不到博客源文件")),
        );
        self.estimate_reading_time()
    }

    fn estimate_reading_time(&mut self) {
        if let Some(content) = &self.content {
            let mut cc_count = 0;
            for c in content.chars() {
                if is_chinese_character(c) {
                    cc_count += 1;
                }
            }
            let time: usize = cc_count / 250;
            self.reading_time = Some(format!("{time}分钟"))
        }
    }

    pub fn set_tag(&mut self, tags: Vec<String>) {
        self.tags = Some(tags)
    }

    pub fn set_date(&mut self, date: String) {
        self.date = Some(date)
    }

    fn get_tags(&self) -> String {
        if let Some(tags) = &self.tags {
            tags.join("·")
        } else {
            String::from("无标签")
        }
    }
}

#[inline(always)]
fn is_chinese_character(c: char) -> bool {
    if ('\u{4E00}'..='\u{9FFF}').contains(&c) || ('\u{3400}'..='\u{4DBF}').contains(&c) {
        return true;
    }
    false
}

/// 博客模板
pub struct PostTemplate(String);

impl PostTemplate {
    pub fn from_path(path: impl AsRef<Path>) -> Self {
        let template =
            fs::read_to_string(path).unwrap_or_else(|_| panic!("[错误]找不到posts_template.html"));
        PostTemplate(template)
    }
}
