//! 番茄小说榜单模块。
//!
//! 提供榜单分类列表和榜单内小说列表的获取能力，
//! 用于 Web UI 的榜单批量下载入口。

mod api;
pub mod models;

pub use api::*;
