use crate::db::{Database, TableInfo};
use common::data_dir;
use itertools::Itertools;
use std::sync::OnceLock;

pub mod db;
mod table_db;

use crate::table_db::DbInfo;
pub use db::AddRecordRequest;
pub use db::SearchRequest;
pub use db::SearchRequestBuilder;
pub use db::SearchResult;

pub type Result<T> = anyhow::Result<T>;
pub struct Engine;

static DB: OnceLock<Database> = OnceLock::new();

impl Engine {
    pub async fn new_table(name: &str) -> Result<()> {
        if let Some(db) = DB.get() {
            db.create_empty_table(name).await?;
        } else {
            panic!("Database not initialized");
        }
        Ok(())
    }

    pub async fn drop_table(name: &str) -> Result<()> {
        if let Some(db) = DB.get() {
            if db.table_names().await?.iter().contains(&name.to_string()) {
                db.drop_table(name).await?;
            }
        } else {
            panic!("Database not initialized");
        }
        Ok(())
    }

    pub async fn add_data(table: &str, data: Vec<AddRecordRequest>) -> Result<()> {
        if let Some(db) = DB.get() {
            db.add_records(table, data).await?;
        } else {
            panic!("Database not initialized");
        }
        Ok(())
    }

    pub async fn delete_data(table: &str, batch_ids: Vec<String>) -> Result<()> {
        if let Some(db) = DB.get() {
            db.delete_records_with_batch_id(table, batch_ids).await?;
        } else {
            panic!("Database not initialized");
        }
        Ok(())
    }

    pub async fn search_data(query: SearchRequest) -> Result<Vec<SearchResult>> {
        if let Some(db) = DB.get() {
            let result = db.search(query).await?;
            Ok(result)
        } else {
            panic!("Database not initialized");
        }
    }

    pub async fn simple_search(
        table: &str,
        vector: Vec<f32>,
        context_size: usize,
        min_score: f32,
        limit: usize,
    ) -> Result<Vec<SearchResult>> {
        if let Some(db) = DB.get() {
            let result = db
                .search(
                    SearchRequestBuilder::default()
                        .table_name(table.to_string())
                        .vector(Some(vector))
                        .context_size(Some(context_size))
                        .min_score(Some(min_score))
                        .limit(Some(limit))
                        .build()?,
                )
                .await?;
            Ok(result)
        } else {
            panic!("Database not initialized");
        }
    }

    pub async fn table_info(table: &str) -> Result<TableInfo> {
        if let Some(db) = DB.get() {
            let result = db.table_info(table).await?;
            Ok(result)
        } else {
            panic!("Database not initialized");
        }
    }
}

pub async fn init() {
    let database_dir = &data_dir!("database").to_string_lossy().into_owned();
    let db = Database::new(database_dir)
        .await
        .expect("Failed to create database");
    DB.get_or_init(|| db);
}

pub struct TableEngine;

impl TableEngine {
    /// 创建表
    /// - db_name: 数据库名称
    /// - name: 表名
    /// - columns: 列名
    pub async fn new_table(db_name: &str, name: &str, columns: Vec<String>) -> Result<()> {
        table_db::Database::new_table(db_name, name, columns).await
    }

    /// 创建表并添加数据
    ///
    /// 注意：columns数量要和每一行的字段数量匹配
    ///
    /// - db_name: 数据库名称
    /// - name: 表名
    /// - columns: 列名
    /// - rows: 行数据
    pub async fn new_table_with_rows(
        db_name: &str,
        tb_name: &str,
        columns: Vec<String>,
        rows: Vec<Vec<String>>,
    ) -> Result<()> {
        Self::new_table(db_name, tb_name, columns).await?;
        Self::add_rows(db_name, tb_name, rows).await?;
        Ok(())
    }

    /// 删除表
    pub async fn drop_table(db_name: &str, table_name: &str) -> Result<()> {
        table_db::Database::drop_table(db_name, table_name).await?;
        Ok(())
    }

    /// 添加数据
    pub async fn add_rows(db_name: &str, table: &str, rows: Vec<Vec<String>>) -> Result<()> {
        table_db::Database::add_data(db_name, table, rows).await?;
        Ok(())
    }

    /// 删除数据
    /// 暂时不需要，先不实现
    #[allow(unused)]
    async fn delete_rows() {
        unimplemented!()
    }

    /// 查询数据
    pub async fn query(db_name: &str, query: &String) -> Result<Vec<Vec<String>>> {
        table_db::Database::query(db_name, query).await
    }

    /// 获取数据库信息
    pub async fn db_info(db_name: &str) -> Result<DbInfo> {
        table_db::Database::get_db_info(db_name).await
    }
}
