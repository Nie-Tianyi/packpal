use std::fs;

// 博客模板文件
static POSTS_TEMPLATE: &str = include_str!("../templates/posts_template.html");
static METADATA_TEMPLATE: &str = include_str!("../templates/metadata_template.html");
static README: &str = include_str!("../../README.md");

/*
 * 初始化项目，创建博客项目的默认文件
 * 创建出来目录结构如下
 * 根目录
 * ├── posts/                       (Markdown源文件存放的地方)
 * │   ├── metadata.json            (存放文章的元数据，如标题、日期等，还在这里配置博客的基本信息)
 * │   └── 我的第一篇博客.md          (示例博客文章)
 * ├── templates/                   (模板HTML存放地方)
 * │   ├── posts_template.html      (文章模板文件)
 * │   └── index_template.html      (主页模板文件)
 * ├── build/                       (用来存放打包合成后的文件)
 * └── README.md                    (PackPal使用指南)
 */
pub fn new_project(name: String) {
    // 创建项目目录
    let project_dir = format!("{}/", name);
    fs::create_dir_all(&project_dir).expect("创建项目目录失败");
    // 创建posts目录
    let posts_dir = format!("{}/posts", project_dir);
    fs::create_dir_all(&posts_dir).expect("创建posts目录失败");
    // 创建templates目录
    let templates_dir = format!("{}/templates", project_dir);
    fs::create_dir_all(&templates_dir).expect("创建templates目录失败");
    // 创建build目录
    let build_dir = format!("{}/build", project_dir);
    fs::create_dir_all(&build_dir).expect("创建build目录失败");
    // 创建README.md文件
    let readme_path = format!("{}/README.md", project_dir);
    fs::write(&readme_path, README).expect("创建README.md文件失败");
    // 创建posts_template.html文件
    let posts_template_path = format!("{}/posts_template.html", templates_dir);
    fs::write(&posts_template_path, POSTS_TEMPLATE).expect("创建posts_template.html文件失败");
    // 创建index_template.html文件
    let index_template_path = format!("{}/index_template.html", templates_dir);
    fs::write(&index_template_path, "<h1>主页模板</h1>").expect("创建index_template.html文件失败");
    // 创建metadata.json文件
    let metadata_path = format!("{}/metadata.json", posts_dir);
    fs::write(&metadata_path, METADATA_TEMPLATE).expect("创建metadata.json文件失败");
    // 创建我的第一篇博客.md文件
    let post_path = format!("{}/我的第一篇博客.md", posts_dir);
    fs::write(&post_path, "# 我的第一篇博客\n这是我的第一篇博客").expect("创建我的第一篇博客.md文件失败");
    println!("项目初始化完成！");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_project() {
        new_project("test/test_project".to_string());
        assert!(fs::metadata("test_project").is_ok());
        assert!(fs::metadata("test_project/posts").is_ok());
    }
}