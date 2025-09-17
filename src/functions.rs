#[cfg(test)]
static TEST_ROOT: &str = "test_projects/";

#[cfg(test)]
static BLOG: &str = include_str!("./templates/比特币.md"); // test blog

pub mod build;
pub mod clean;
pub mod deploy;
pub mod new;
