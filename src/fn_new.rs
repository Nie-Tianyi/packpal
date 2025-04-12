use crate::POSTS_TEMPLATE;
use std::fs;

/*
 * 初始化项目，创建博客项目的默认文件
 * 创建出来目录结构如下
 * 根目录
 * ├── posts/
 * │   ├── metadata.json 存放文章的元数据
 * │   └── 我的第一篇博客.md
 * ├── templates/
 * │   ├── posts_template.html
 * │   ├── index_template.html
 * │   └── ...后续可能还有其他模板文件
 * ├── build/
 * │   └── 用来存放打包合成后的文件
 * └── README.md
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
