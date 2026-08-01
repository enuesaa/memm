use anyhow::Result;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait,
    DatabaseConnection,
    EntityTrait,
    Set,
};

use crate::entities::memos;

pub struct MemoRepository;

impl MemoRepository {
    pub async fn find_all(
        db: &DatabaseConnection,
    ) -> Result<Vec<memos::Model>> {
        let memos = memos::Entity::find()
            .all(db)
            .await?;

        Ok(memos)
    }

    pub async fn create(
        db: &DatabaseConnection,
        title: String,
        description: String,
    ) -> Result<memos::Model> {
        let now = Utc::now();

        let memo = memos::ActiveModel {
            title: Set(title),
            description: Set(description),
            created_at: Set(now.into()),
            updated_at: Set(now.into()),
            ..Default::default()
        };

        let inserted = memo.insert(db).await?;

        Ok(inserted)
    }
}