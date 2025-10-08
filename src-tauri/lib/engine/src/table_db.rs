use common::data_dir;
use futures_util::StreamExt;
use itertools::Itertools;
use sqlx::{Connection, Executor, Row, SqliteConnection};
use std::fs::File;
use std::sync::LazyLock;

static DB_PATH: LazyLock<String> = LazyLock::new(|| {
    let current_dir = std::env::current_exe().unwrap();
    data_dir!("database").to_string_lossy().into_owned()
});

const SQL_CREATE_TABLE_STATISTICS: &str = "CREATE TABLE IF NOT EXISTS _statistics_ (
            table_name TEXT PRIMARY KEY,
            row_count INTEGER,
            char_count INTEGER
        )";
pub struct Database {}

impl Database {
    /// 获取数据库连接字符串
    pub async fn get_db(db_name: &str) -> crate::Result<String> {
        let path = format!("{}/{}.sqlite", *DB_PATH, db_name);
        let connection_str = format!("sqlite:{}/data.db", path);
        if !std::path::Path::new(&path).exists() {
            std::fs::create_dir_all(&path).expect("Failed to create database directory");
            let db_file = format!("{}/data.db", path);
            File::create(&db_file).expect("Failed to create database file");
            // 初始化统计表
            Self::create_statistics_table_if_not_exists(&connection_str).await?;
        }
        Ok(connection_str)
    }

    /// 删除数据库
    #[allow(unused)]
    pub async fn drop_db(db_name: &str) {
        if db_name.is_empty() {
            return;
        }
        let path = format!("{}/{}.sqlite", *DB_PATH, db_name);
        if std::path::Path::new(&path).exists() {
            std::fs::remove_dir_all(&path).expect("Failed to delete database directory");
        }
    }

    /// 创建一张表
    pub async fn new_table(
        db_name: &str,
        table_name: &str,
        columns: Vec<String>,
    ) -> crate::Result<()> {
        let mut conn = SqliteConnection::connect(&Self::get_db(db_name).await?).await?;
        let sql = &format!(
            "CREATE TABLE IF NOT EXISTS \"{}\" ({})",
            table_name,
            columns
                .iter()
                .filter(|c| c.trim() != "")
                .map(|c| { format!("\"{}\" TEXT", c) })
                .join(",")
        );

        conn.execute(sqlx::query(sql)).await?;
        conn.close().await?;
        Ok(())
    }

    /// 表是否存在
    pub async fn exists_table(db_name: &str, table_name: &str) -> crate::Result<bool> {
        let mut conn = SqliteConnection::connect(&Self::get_db(db_name).await?).await?;
        let sql = &format!(
            "SELECT name FROM sqlite_master WHERE type='table' AND name='{}'",
            table_name
        );
        let exists = sqlx::query(sql).fetch_one(&mut conn).await.is_ok();
        conn.close().await?;

        Ok(exists)
    }

    /// 删除一张表
    pub async fn drop_table(db_name: &str, table_name: &str) -> crate::Result<()> {
        if !Self::exists_table(db_name, table_name).await? {
            return Ok(());
        }
        let mut conn = SqliteConnection::connect(&Self::get_db(db_name).await?).await?;
        conn.execute(sqlx::query(&format!("DROP TABLE \"{}\"", table_name)))
            .await?;
        conn.close().await?;

        Self::delete_statistics(db_name, table_name).await?;

        Ok(())
    }

    /// 添加数据
    pub async fn add_data(
        db_name: &str,
        table_name: &str,
        data: Vec<Vec<String>>,
    ) -> crate::Result<()> {
        let mut conn = SqliteConnection::connect(&Self::get_db(db_name).await?).await?;

        for row in &data {
            let placeholders = (0..row.len()).map(|i| format!("?{}", i + 1)).join(",");
            let sql = format!("INSERT INTO \"{}\" VALUES ({})", table_name, placeholders);

            let mut query = sqlx::query(sql.as_str());
            for value in row {
                query = query.bind(value);
            }
            conn.execute(query).await?;
        }
        conn.close().await?;

        Self::update_statistics(
            db_name,
            table_name,
            data.len() as i64,
            data.iter()
                .flatten()
                .map(|s| s.chars().count() as i64)
                .sum::<i64>(),
        )
        .await?;
        Ok(())
    }

    /// 查询
    pub async fn query(db_name: &str, query: &str) -> crate::Result<Vec<Vec<String>>> {
        let mut conn = SqliteConnection::connect(&Self::get_db(db_name).await?).await?;
        let mut result = Vec::new();
        let mut rows = sqlx::query(query).fetch(&mut conn);

        while let Some(row) = rows.next().await {
            let row = row?;
            let mut row_data = Vec::with_capacity(row.columns().len());

            for (i, _) in row.columns().iter().enumerate() {
                row_data.push(row.try_get::<String, _>(i)?);
            }

            result.push(row_data);
        }
        //conn.close().await?;

        Ok(result)
    }

    /// 创建一张统计表，统计整个数据下的所有表行数，字符数
    pub async fn create_statistics_table_if_not_exists(
        db_connection_str: &str,
    ) -> crate::Result<()> {
        let mut conn = SqliteConnection::connect(db_connection_str).await?;
        conn.execute(sqlx::query(SQL_CREATE_TABLE_STATISTICS))
            .await?;
        conn.close().await?;
        Ok(())
    }

    /// 更新统计表中的某个表的统计数据
    async fn update_statistics(
        db_name: &str,
        table_name: &str,
        row_count: i64,
        char_count: i64,
    ) -> crate::Result<()> {
        let mut conn = SqliteConnection::connect(&Self::get_db(&db_name).await?).await?;
        let sql = "INSERT OR REPLACE INTO _statistics_ (table_name, row_count, char_count) VALUES (?, ?, ?)";
        sqlx::query(sql)
            .bind(table_name.to_string())
            .bind(row_count)
            .bind(char_count)
            .execute(&mut conn)
            .await?;
        conn.close().await?;
        Ok(())
    }

    /// 删除统计表中的某个表的统计数据
    async fn delete_statistics(db_name: &str, tab_name: &str) -> crate::Result<()> {
        let mut conn = SqliteConnection::connect(&Self::get_db(&db_name).await?).await?;
        let sql = "DELETE FROM _statistics_ WHERE table_name = ?";
        sqlx::query(sql)
            .bind(tab_name.to_string())
            .execute(&mut conn)
            .await?;
        conn.close().await?;

        Ok(())
    }

    /// 获取数据库信息
    pub async fn get_db_info(db_name: &str) -> crate::Result<DbInfo> {
        let mut conn = SqliteConnection::connect(&Self::get_db(&db_name).await?).await?;
        let sql = "SELECT SUM(row_count), SUM(char_count) FROM _statistics_";
        let rows = sqlx::query(sql).fetch_one(&mut conn).await?;
        conn.close().await?;
        Ok(DbInfo {
            total_rows: rows.get::<i64, _>(0) as usize,
            total_chars: rows.get::<i64, _>(1) as usize,
        })
    }
}

#[derive(Debug)]
pub struct DbInfo {
    pub total_rows: usize,
    pub total_chars: usize,
}
