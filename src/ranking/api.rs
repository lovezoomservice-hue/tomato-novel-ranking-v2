//! 番茄小说榜单 API 调用。

use std::sync::OnceLock;

use reqwest::blocking::Client;
use reqwest::header::{ACCEPT, ACCEPT_ENCODING, CONNECTION, REFERER, USER_AGENT};
use serde::de::DeserializeOwned;
use serde_json::Value;
use tracing::debug;

use super::models::{Gender, RankingBook, RankingBooksResponse, RankingChannel, RankingInfo};

/// 番茄小说榜单 API 基础 URL。
const RANKING_API_BASE: &str = "https://fanqienovel.com";

static HTTP_CLIENT: OnceLock<Client> = OnceLock::new();

fn http_client() -> &'static Client {
    HTTP_CLIENT.get_or_init(|| {
        Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("build ranking http client")
    })
}

fn get_json<T: DeserializeOwned>(url: &str, params: &[(&str, &str)]) -> anyhow::Result<T> {
    let client = http_client();
    let mut req = client.get(url);
    req = req.header(ACCEPT, "application/json, text/plain, */*");
    req = req.header(
        USER_AGENT,
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36",
    );
    req = req.header(ACCEPT_ENCODING, "gzip, deflate, br");
    req = req.header(CONNECTION, "keep-alive");
    req = req.header(REFERER, format!("{}/", RANKING_API_BASE));
    for (k, v) in params {
        req = req.query(&[(k, v)]);
    }

    debug!(url, ?params, "ranking API request");
    let resp = req.send()?;
    let body: Value = resp.json()?;
    debug!(url, "ranking API response received");
    Ok(serde_json::from_value(body)?)
}

/// 获取榜单分类列表（频道 + 榜单入口）。
pub fn get_ranking_categories() -> anyhow::Result<Vec<RankingChannel>> {
    Ok(build_static_channels())
}

/// 构建静态榜单频道列表（基于番茄小说实际榜单分类）。
fn build_static_channels() -> Vec<RankingChannel> {
    let male_ranks: Vec<RankingInfo> = vec![
        RankingInfo { rank_id: 1, rank_name: "玄幻".into(), cover_imgs: vec![] },
        RankingInfo { rank_id: 2, rank_name: "奇幻".into(), cover_imgs: vec![] },
        RankingInfo { rank_id: 3, rank_name: "都市".into(), cover_imgs: vec![] },
        RankingInfo { rank_id: 4, rank_name: "穿越".into(), cover_imgs: vec![] },
        RankingInfo { rank_id: 5, rank_name: "都市脑洞".into(), cover_imgs: vec![] },
        RankingInfo { rank_id: 6, rank_name: "都市现言".into(), cover_imgs: vec![] },
        RankingInfo { rank_id: 7, rank_name: "科幻".into(), cover_imgs: vec![] },
        RankingInfo { rank_id: 8, rank_name: "游戏".into(), cover_imgs: vec![] },
        RankingInfo { rank_id: 9, rank_name: "悬疑".into(), cover_imgs: vec![] },
        RankingInfo { rank_id: 10, rank_name: "武侠".into(), cover_imgs: vec![] },
    ];

    let female_ranks: Vec<RankingInfo> = vec![
        RankingInfo { rank_id: 21, rank_name: "现代言情".into(), cover_imgs: vec![] },
        RankingInfo { rank_id: 22, rank_name: "总裁".into(), cover_imgs: vec![] },
        RankingInfo { rank_id: 23, rank_name: "穿越言情".into(), cover_imgs: vec![] },
        RankingInfo { rank_id: 24, rank_name: "校园".into(), cover_imgs: vec![] },
        RankingInfo { rank_id: 25, rank_name: "玄幻言情".into(), cover_imgs: vec![] },
        RankingInfo { rank_id: 26, rank_name: "职场".into(), cover_imgs: vec![] },
        RankingInfo { rank_id: 27, rank_name: "古言".into(), cover_imgs: vec![] },
        RankingInfo { rank_id: 28, rank_name: "短篇".into(), cover_imgs: vec![] },
    ];

    let bestseller_ranks: Vec<RankingInfo> = vec![
        RankingInfo { rank_id: 100, rank_name: "畅销总榜".into(), cover_imgs: vec![] },
        RankingInfo { rank_id: 101, rank_name: "飙升榜".into(), cover_imgs: vec![] },
        RankingInfo { rank_id: 102, rank_name: "阅读榜".into(), cover_imgs: vec![] },
    ];

    vec![
        RankingChannel { channel_id: 1, channel_name: "男生".into(), ranks: male_ranks },
        RankingChannel { channel_id: 0, channel_name: "女生".into(), ranks: female_ranks },
        RankingChannel { channel_id: -1, channel_name: "畅销".into(), ranks: bestseller_ranks },
    ]
}

/// 根据频道 ID 和榜单 ID 获取榜单中的书籍列表。
pub fn get_ranking_books(
    channel_id: i32,
    rank_id: i64,
    page: u32,
    page_size: u32,
) -> anyhow::Result<RankingBooksResponse> {
    if channel_id == -1 {
        return get_bestseller_books(rank_id, page, page_size);
    }

    let gender = if channel_id == 0 {
        Gender::Female
    } else if channel_id == 1 {
        Gender::Male
    } else {
        Gender::All
    };

    let category_id = rank_id as i32;
    let gender_str = gender.as_i32().to_string();
    let cat_str = category_id.to_string();
    let page_count_str = page_size.to_string();
    let page_index = ((page.saturating_sub(1)) * page_size).to_string();
    let sort_str = "0".to_string();
    let neg_one = "-1".to_string();

    let params = vec![
        ("page_count", page_count_str.as_str()),
        ("page_index", page_index.as_str()),
        ("gender", gender_str.as_str()),
        ("category_id", cat_str.as_str()),
        ("sort", sort_str.as_str()),
        ("creation_status", neg_one.as_str()),
        ("word_count", neg_one.as_str()),
        ("book_type", neg_one.as_str()),
    ];

    let url = format!("{}/api/author/library/book_list/v0/", RANKING_API_BASE);
    let data: Value = get_json(&url, &params)?;

    let books = data
        .get("data")
        .and_then(|d| d.get("books"))
        .or_else(|| data.get("books"))
        .and_then(|v| v.as_array())
        .map(|arr| arr.clone())
        .unwrap_or_default();

    let total = books.len() as u64;
    let items: Vec<RankingBook> = books.iter().map(parse_book_from_value).collect();

    Ok(RankingBooksResponse { items, total, page, size: page_size })
}

/// 获取畅销榜单书籍。
fn get_bestseller_books(rank_id: i64, page: u32, page_size: u32) -> anyhow::Result<RankingBooksResponse> {
    let sort_str = match rank_id {
        100 => "0", // 最热
        101 => "0", // 飙升
        102 => "1", // 最新
        _ => "0",
    }
    .to_string();

    let page_count_str = page_size.to_string();
    let page_index = ((page.saturating_sub(1)) * page_size).to_string();
    let neg_one = "-1".to_string();

    let params = vec![
        ("page_count", page_count_str.as_str()),
        ("page_index", page_index.as_str()),
        ("gender", neg_one.as_str()),
        ("category_id", neg_one.as_str()),
        ("sort", sort_str.as_str()),
        ("creation_status", neg_one.as_str()),
        ("word_count", neg_one.as_str()),
        ("book_type", neg_one.as_str()),
    ];

    let url = format!("{}/api/author/library/book_list/v0/", RANKING_API_BASE);
    let data: Value = get_json(&url, &params)?;

    let books = data
        .get("data")
        .and_then(|d| d.get("books"))
        .or_else(|| data.get("books"))
        .and_then(|v| v.as_array())
        .map(|arr| arr.clone())
        .unwrap_or_default();

    let total = books.len() as u64;
    let items: Vec<RankingBook> = books.iter().map(parse_book_from_value).collect();

    Ok(RankingBooksResponse { items, total, page, size: page_size })
}

/// 从 JSON Value 解析 RankingBook。
fn parse_book_from_value(v: &Value) -> RankingBook {
    RankingBook {
        book_id: v.get("book_id").or_else(|| v.get("bookId")).or_else(|| v.get("id"))
            .and_then(|x| x.as_str()).map(|s| s.to_string()).unwrap_or_default(),
        title: v.get("title").or_else(|| v.get("book_name")).or_else(|| v.get("name"))
            .and_then(|x| x.as_str()).map(|s| s.to_string()).unwrap_or_default(),
        author: v.get("author").and_then(|x| x.as_str()).map(|s| s.to_string()).unwrap_or_default(),
        description: v.get("description").or_else(|| v.get("desc")).or_else(|| v.get("Description"))
            .and_then(|x| x.as_str()).map(|s| s.to_string()),
        cover_url: v.get("cover_url").or_else(|| v.get("coverUrl"))
            .or_else(|| v.get("thumb_url")).or_else(|| v.get("thumbUrl"))
            .and_then(|x| x.as_str()).map(|s| s.to_string()),
        word_count: v.get("word_count").or_else(|| v.get("wordCount")).or_else(|| v.get("word"))
            .and_then(|x| x.as_str().map(|s| s.to_string()).or_else(|| x.as_i64().map(|n| n.to_string()))),
        score: v.get("score").or_else(|| v.get("Score")).and_then(|x| x.as_f64()),
        read_count: v.get("read_count").or_else(|| v.get("readCount")).or_else(|| v.get("ReadCount"))
            .and_then(|x| x.as_str().map(|s| s.to_string()).or_else(|| x.as_i64().map(|n| n.to_string()))),
        category: v.get("category").or_else(|| v.get("categoryName"))
            .and_then(|x| x.as_str()).map(|s| s.to_string()),
        finished: v.get("finished").or_else(|| v.get("isFinished")).or_else(|| v.get("chapterStatus"))
            .and_then(|x| {
                if x.is_boolean() {
                    x.as_bool()
                } else if let Some(s) = x.as_str() {
                    match s {
                        "END" | "end" | "1" | "finished" => Some(true),
                        "SERIALIZE" | "serialize" | "0" | "ongoing" => Some(false),
                        _ => None,
                    }
                } else {
                    None
                }
            }),
        chapter_count: v.get("chapter_count").or_else(|| v.get("chapterCount")).or_else(|| v.get("ChaptersCount"))
            .and_then(|x| x.as_u64()).map(|n| n as u32),
    }
}
