use crate::POSTS_TEMPLATE;
use std::fs;

/*
 * 初始化项目，创建博客项目的默认文件
 * 创建出来目录结构如下
 * 根目录
 * ├── posts/                       (Markdown源文件存放的地方)
 * │   ├── metadata.json            (存放文章的元数据)
 * │   └── 我的第一篇博客.md          (示例博客文章)
 * ├── templates/                   (模板HTML存放地方)
 * │   ├── posts_template.html      (文章模板文件)
 * │   └── index_template.html      (主页模板文件)
 * ├── build/                       (用来存放打包合成后的文件)
 * └── README.md                    (PackPal使用指南)
 */
pub fn new_project(name: String) {

    println!("Creating project {}", name);

    create_new_dir(name.clone(), "build");
    create_new_dir(name.clone(), "posts");
    create_new_dir(name.clone(), "templates");

    println!("Creating default posts template");
    let template_path = name + "/" + "templates/";
    fs::write(template_path + "posts_template", POSTS_TEMPLATE).expect("Fail to create templates");
}

#[inline(always)]
fn create_new_dir(project_name: String, dir_name: &str) {
    let path = project_name + "/" + &dir_name;
    println!("Creating \"{}\" directory at {}", dir_name, path);
    fs::create_dir_all(path).expect("Fail to create directory");
}
