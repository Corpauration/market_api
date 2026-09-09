#![allow(dead_code)]

use std::fmt::{Display, Formatter, Error};
use derive_more::Display;
use sqlx::{SqlSafeStr, SqlStr};

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

 

fn query_table(table: &str, sort_by: &str, direction: &str)->SqlStr{
    let query:String = format!("select * from {} order by {} {};", table, sort_by, direction);
    sqlx::AssertSqlSafe(query).into_sql_str()
}

pub enum UserDisplayInfosColumns {
    UserId,
    DisplayName,
    UpdatedAt,
}
impl Display for UserDisplayInfosColumns{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        use UserDisplayInfosColumns::*;
        match self{
            UserId => write!(f, "user_id"),
            DisplayName => write!(f, "display_name"),
            UpdatedAt => write!(f, "updated_at"),
        }
    }
}

