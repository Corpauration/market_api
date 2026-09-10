use axum::extract::{ State, };
use sqlx::{ Column, Row, };
use super::{ Provider, };

pub fn run_sql<'executor>(
    executor: impl sqlx::Executor<'executor, Database = sqlx::Postgres>,
    sql: impl sqlx::SqlSafeStr,
)
-> futures::stream::BoxStream<'executor, Result<sqlx::Either<sqlx::postgres::PgQueryResult, sqlx::postgres::PgRow>, sqlx::Error>>
{
    sqlx::raw_sql(sql)
        .fetch_many(executor)
}

pub fn sudo_sql<Sql: AsRef<str>>(sql: Sql) -> Result<impl sqlx::SqlSafeStr, Sql>
where
    sqlx::AssertSqlSafe<Sql>: sqlx::SqlSafeStr,
{
    match cfg!(feature = "allow_sudo_sql")
        && std::env::var("ALLOW_SUDO_SQL").as_deref() == Ok("true")
    {
        true => Ok(sqlx::AssertSqlSafe(sql)),
        false => Err(sql),
    }
}

pub async fn run_sql_into<'executor>(
    executor: impl sqlx::Executor<'executor, Database = sqlx::Postgres>,
    sql: impl sqlx::SqlSafeStr,
    mut consume: impl FnMut(Result<sqlx::Either<sqlx::postgres::PgQueryResult, sqlx::postgres::PgRow>, sqlx::Error>) + Send
)
{
    use futures::{ StreamExt, };

    let mut results = run_sql(executor, sql);

    while let Some(result) = results.next().await {
        consume(result);
    }
}

pub trait PgValueRefToStringResult {
    fn build(value: &sqlx::postgres::PgValueRef<'_>) -> Self;
}
pub fn pg_value_ref_to_string<Result: PgValueRefToStringResult>(value: &sqlx::postgres::PgValueRef<'_>) -> Result {
    Result::build(value)
}
impl PgValueRefToStringResult for Result<String, sqlx::error::BoxDynError> {
    fn build(value: &sqlx::postgres::PgValueRef<'_>) -> Self {
        match value.format() {
            sqlx::postgres::PgValueFormat::Text => {
                value.as_str().map(|value_str| format!("{}", value_str))
            },
            sqlx::postgres::PgValueFormat::Binary => {
                value.as_bytes().map(|bytes| format!("<Bytes: {:?}>", bytes))
            },
        }
    }
}
impl PgValueRefToStringResult for String {
    fn build(value: &sqlx::postgres::PgValueRef<'_>) -> Self {
        <Result<String, sqlx::error::BoxDynError> as PgValueRefToStringResult>::build(value)
            .unwrap_or_else(|error| format!("<Error retrieving value: {}>", error))
    }
}

pub fn pg_row_to_string(row: &sqlx::postgres::PgRow, mut r#yield: impl FnMut(String)) {
    let columns = row.columns();
    for (index, column) in columns.iter().enumerate() {
        let value = row.try_get_raw(index).unwrap_or_else(|_| unreachable!());
        r#yield(format!("  {}: {}\n", column.name(), pg_value_ref_to_string::<String>(&value)));
    }
}

pub async fn run_sql_into_response<'executor>(
    executor: impl sqlx::Executor<'executor, Database = sqlx::Postgres>,
    sql: impl sqlx::SqlSafeStr,
)
-> impl axum::response::IntoResponse
{
    let mut response = String::new();

    run_sql_into(executor, sql, |result| {
        match result {
            Ok(sqlx::Either::Left(query_result)) => {
                response.push_str(&format!("Rows affected: {}\n", query_result.rows_affected()));
            },
            Ok(sqlx::Either::Right(row)) => {
                let mut row_data = String::new();
                pg_row_to_string(&row, |row_string| {
                    row_data.push_str(&format!("  {}\n", row_string));
                });
                response.push_str(&format!("Row data:\n{}\n", row_data));
            },
            Err(error) => {
                response.push_str(&format!("Error executing query: {}\n", error));
            },
        }
    }).await;

    response
}

#[rustfmt::skip]
#[derive(Debug)] #[derive(Clone, Copy)] #[derive(PartialEq, Eq)] #[derive(Hash)]
#[derive(thiserror::Error)] #[error("The current configuration does not allow arbitrary SQL execution.")]
pub struct SudoSqlRejectionError;

impl axum::response::IntoResponse for SudoSqlRejectionError {
    fn into_response(self) -> axum::response::Response {
        axum::http::StatusCode::FORBIDDEN.into_response()
    }
}

pub async fn sudo_run_sql_into_response<'executor, Sql: AsRef<str>>(
    executor: impl sqlx::Executor<'executor, Database = sqlx::Postgres>,
    sql: Sql,
)
-> Result<impl axum::response::IntoResponse, SudoSqlRejectionError>
where
    sqlx::AssertSqlSafe<Sql>: sqlx::SqlSafeStr,
{
    match sudo_sql(sql) {
        Ok(safe_sql) => Ok(run_sql_into_response(executor, safe_sql).await),
        Err(_) => Err(SudoSqlRejectionError),
    }
}

#[rustfmt::skip]
#[derive(Debug)] #[derive(Clone, Copy)] #[derive(PartialEq, Eq)] #[derive(Hash)]
#[derive(serde::Deserialize)]
pub struct SudoRunSqlRequest<Sql> {
    pub sql: Sql,
}

pub async fn handle_sudo_run_sql<'executor, Dependencies, Executor: sqlx::Executor<'executor, Database = sqlx::Postgres>, Sql: AsRef<str>>(
    State(dependencies): State<Dependencies>,
    SudoRunSqlRequest { sql }: SudoRunSqlRequest<Sql>,
)
-> impl axum::response::IntoResponse
where
    Dependencies: Provider<Executor>,
    sqlx::AssertSqlSafe<Sql>: sqlx::SqlSafeStr,
{
    let executor: Executor = dependencies.provide();

    sudo_run_sql_into_response(executor, sql).await
}

pub async fn handle_sudo_run_sql_from_json<'executor, Dependencies, Executor: sqlx::Executor<'executor, Database = sqlx::Postgres>>(
    state: State<Dependencies>,
    axum::Json(request): axum::Json<SudoRunSqlRequest<String>>,
)
-> impl axum::response::IntoResponse
where
    Dependencies: Provider<Executor>,
{
    handle_sudo_run_sql(state, request).await
}
