pub mod data;
//#![allow(dead_code)]

use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
};
use derive_more::Display;
use serde_json::json;
use sqlx::{SqlSafeStr, SqlStr};
use std::fmt::{Display as StdDisplay, Error, Formatter};
use std::sync::Arc;

use super::queries::data::Product;
use crate::routes::AppState;

#[derive(Display)]
pub enum SortDirection {
    #[display("asc")]
    Asc,
    #[display("desc")]
    Desc,
}

#[derive(Display)]
pub enum UsersColumns {
    #[display("id")]
    Id,
    #[display("created_at")]
    CreatedAt,
}

fn query_table(table: &str, sort_by: &str, direction: &str) -> SqlStr {
    let query: String = format!(
        "select * from {} order by {} {};",
        table, sort_by, direction
    );
    sqlx::AssertSqlSafe(query).into_sql_str()
}

pub async fn get_products(
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query = sqlx::query_as::<_, Product>(
        r#"
        SELECT display_name, SUM(quantity) 
        FROM market_products AS a 
        JOIN market_product_display_infos AS b ON a.id=b.market_product_id 
        JOIN market_product_instance_stocks AS c ON a.id=c.market_product_id 
        GROUP BY a.id;
    "#,
    )
    .fetch_all(&data.db_pool)
    .await;
    match query {
        Ok(products) => Ok(Json(
            serde_json::json!({"status":"succes", "data":serde_json::json!({"products":products})}),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status":"error", "message":format!("{:?}", e)})),
        )),
    }
}

pub enum UserDisplayInfosColumns {
    UserId,
    DisplayName,
    UpdatedAt,
}
impl StdDisplay for UserDisplayInfosColumns {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        use UserDisplayInfosColumns::*;
        match self {
            UserId => write!(f, "user_id"),
            DisplayName => write!(f, "display_name"),
            UpdatedAt => write!(f, "updated_at"),
        }
    }
}
