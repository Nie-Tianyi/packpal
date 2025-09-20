#[cfg(test)]
static TEST_ROOT: &str = "test_projects/"; // 所有生成的测试项目都在这个文件夹下面

#[cfg(test)]
static BLOG: &str = include_str!("./templates/比特币.md"); // 用于测试的markdown博客文件

pub mod build;
pub mod clean;
pub mod deploy;
pub mod new;
