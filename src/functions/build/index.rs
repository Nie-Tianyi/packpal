use crate::functions::build::Avatar;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct IndexInfo {
    pub site_name: String,
    pub motto: String,
    pub github: String,
    pub email: String,
    pub date: String,
    pub(crate) avatar: Avatar,
    pub(crate) posts: Vec<PostInfo>,
}

impl IndexInfo {
    pub fn new(
        site_name: String,
        motto: String,
        github: String,
        email: String,
        date: String,
        avatar: Avatar,
        posts: Vec<PostInfo>,
    ) -> Self {
        Self {
            site_name,
            motto,
            github,
            email,
            date,
            avatar,
            posts,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PostInfo {
    pub title: String,
    pub date: String,
    pub url: String,
    pub excerpt: String,
}

#[derive(Debug, Clone)]
pub struct HTMLIndexPage(String);

impl HTMLIndexPage {
    pub fn write_into_file(&self, path: PathBuf) {
        fs::write(&path, self.0.as_bytes()).expect("[错误]写入索引页面文件失败，无效路径");
    }
}

#[derive(Debug, Clone)]
pub struct IndexTemplate(String);

impl IndexTemplate {
    pub fn imports(path: impl AsRef<Path>) -> Self {
        let content = fs::read_to_string(path).expect("[错误]读取索引模板文件失败，无效路径");
        Self(content)
    }

    pub fn render(&self, index: IndexInfo) -> HTMLIndexPage {
        let mut content = self.0.clone();
        content = content
            .replace("{{site_name}}", &index.site_name)
            .replace("{{motto}}", &index.motto)
            .replace("{{avatar}}", &index.avatar.url)
            .replace("{{github}}", &index.github)
            .replace("{{date}}", &index.date);

        let post_card_template = r###"
            <article class="blog-card">
                <div class="post-date">
                    <i class="fa fa-calendar-o date-icon"></i>
                    <time datetime="{{date}}">{{date}}</time>
                </div>
                <h3 class="post-title">
                    <a href="{{url}}">{{title}}</a>
                </h3>
                <p class="post-excerpt">
                    {{excerpt}}
                </p>
                <a href="{{url}}" class="read-more">
                    阅读全文
                    <i class="fa fa-long-arrow-right"></i>
                </a>
            </article>
        "###;

        let post_card_template_style_2 = r###"
            <li>
                <a href="{{url}}">
                    <i class="fa fa-angle-right list-bullet"></i>
                    <span class="list-date">{{date}}</span>
                    <span>{{title}}</span>
                </a>
            </li>
        "###;

        let mut post_cards = String::new();
        let mut post_cards_style_2 = String::new();

        for (i, post) in index.posts.iter().enumerate() {
            if i < 3 {
                let post_card = post_card_template
                    .replace("{{title}}", &post.title)
                    .replace("{{date}}", &post.date)
                    .replace("{{url}}", &post.url)
                    .replace("{{excerpt}}", &post.excerpt);
                post_cards.push_str(&post_card);
            } else {
                let post_card = post_card_template_style_2
                    .replace("{{title}}", &post.title)
                    .replace("{{date}}", &post.date)
                    .replace("{{url}}", &post.url);
                post_cards_style_2.push_str(&post_card);
            }
        }

        content = content.replace("{{post_cards}}", &post_cards);
        content = content.replace("{{post_cards_style_2}}", &post_cards_style_2);

        HTMLIndexPage(content)
    }
}
