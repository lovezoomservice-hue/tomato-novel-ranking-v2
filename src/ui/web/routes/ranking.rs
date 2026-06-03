use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;
use serde_json::json;

use crate::ranking;
use crate::ui::web::state::AppState;

/// 获取榜单分类列表（频道 + 榜单入口）。
pub(crate) async fn api_ranking_categories(
    State(_state): State<AppState>,
) -> Result<Json, (StatusCode, Json)> {
    let channels = tokio::task::spawn_blocking(ranking::get_ranking_categories)
        .await
        .map_err(|_| api_error(StatusCode::INTERNAL_SERVER_ERROR, "榜单分类任务执行失败"))?
        .map_err(|err| api_error(StatusCode::BAD_GATEWAY, format!("获取榜单分类失败: {err}")))?;

    Ok(Json(json!({ "channels": channels })))
}

#[derive(Debug, Deserialize)]
pub(crate) struct BooksQuery {
    /// 频道 ID：0=女生，1=男生，-1=畅销
    pub(crate) channel_id: i32,
    /// 榜单 ID（对应分类 ID）
    pub(crate) rank_id: i64,
    /// 页码（从 1 开始）
    pub(crate) page: Option<u32>,
    /// 每页数量
    pub(crate) size: Option<u32>,
}

/// 获取榜单内的书籍列表。
pub(crate) async fn api_ranking_books(
    State(_state): State<AppState>,
    Query(q): Query<BooksQuery>,
) -> Result<Json, (StatusCode, Json)> {
    let page = q.page.unwrap_or(1).max(1);
    let size = q.size.unwrap_or(50).min(100).max(1);

    let resp = tokio::task::spawn_blocking(move || {
        ranking::get_ranking_books(q.channel_id, q.rank_id, page, size)
    })
    .await
    .map_err(|_| api_error(StatusCode::INTERNAL_SERVER_ERROR, "榜单书籍任务执行失败"))?
    .map_err(|err| api_error(StatusCode::BAD_GATEWAY, format!("获取榜单书籍失败: {err}")))?;

    Ok(Json(json!({
        "items": resp.items,
        "total": resp.total,
        "page": resp.page,
        "size": resp.size,
    })))
}

fn api_error(status: StatusCode, message: impl Into<String>) -> (StatusCode, Json) {
    (status, Json(json!({ "error": message.into() })))
}

