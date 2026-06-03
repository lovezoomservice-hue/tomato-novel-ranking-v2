use serde::{Deserialize, Serialize};

/// 榜单频道类型。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Gender {
    /// 全部
    All = -1,
    /// 女性
    Female = 0,
    /// 男性
    Male = 1,
}

impl From<i32> for Gender {
    fn from(v: i32) -> Self {
        match v {
            0 => Gender::Female,
            1 => Gender::Male,
            _ => Gender::All,
        }
    }
}

impl Gender {
    pub fn as_i32(self) -> i32 {
        match self {
            Gender::All => -1,
            Gender::Female => 0,
            Gender::Male => 1,
        }
    }
}

/// 榜单排序方式。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RankingSort {
    /// 最热
    Hot = 0,
    /// 最新
    Newest = 1,
    /// 字数最多
    WordCount = 2,
}

impl From<i32> for RankingSort {
    fn from(v: i32) -> Self {
        match v {
            1 => RankingSort::Newest,
            2 => RankingSort::WordCount,
            _ => RankingSort::Hot,
        }
    }
}

impl RankingSort {
    pub fn as_i32(self) -> i32 {
        self as i32
    }
}

/// 榜单分类（频道）。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankingChannel {
    pub channel_id: i32,
    pub channel_name: String,
    pub ranks: Vec<RankingInfo>,
}

/// 榜单信息。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankingInfo {
    pub rank_id: i64,
    pub rank_name: String,
    pub cover_imgs: Vec<String>,
}

/// 榜单中的书籍条目（用于榜单列表展示）。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankingBook {
    /// 书籍 ID（book_id）
    pub book_id: String,
    /// 书名
    pub title: String,
    /// 作者
    pub author: String,
    /// 简介
    #[serde(default)]
    pub description: Option<String>,
    /// 封面 URL
    #[serde(default)]
    pub cover_url: Option<String>,
    /// 总字数（文本）
    #[serde(default)]
    pub word_count: Option<String>,
    /// 评分
    #[serde(default)]
    pub score: Option<f64>,
    /// 阅读数
    #[serde(default)]
    pub read_count: Option<String>,
    /// 分类
    #[serde(default)]
    pub category: Option<String>,
    /// 完结状态
    #[serde(default)]
    pub finished: Option<bool>,
    /// 章节数
    #[serde(default)]
    pub chapter_count: Option<u32>,
}

/// 榜单分类列表响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankingCategoriesResponse {
    pub channels: Vec<RankingChannel>,
}

/// 榜单书籍列表响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankingBooksResponse {
    pub items: Vec<RankingBook>,
    pub total: u64,
    pub page: u32,
    pub size: u32,
}
