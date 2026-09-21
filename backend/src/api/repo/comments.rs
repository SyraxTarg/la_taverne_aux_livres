use crate::api::entities::comment;
use sea_orm::*;


pub async fn creer_table_si_inexistante(db: &DatabaseConnection) {
    let builder = db.get_database_backend();
    let schema = sea_orm::Schema::new(builder);

    let stmt = schema.create_table_from_entity(comment::Entity)
        .if_not_exists()
        .to_owned();

    let _ = db.execute(&stmt).await;
    println!("✅ Table 'comments' vérifiée via l'ORM.");
}

pub async fn insert_comment(
    db: &DatabaseConnection,
    new_comment: comment::ActiveModel
) -> Result<comment::Model, DbErr> {
    new_comment.insert(db).await
}

pub async fn get_comments_by_book_id(
    db: &DatabaseConnection,
    book_id: &str,
) -> Result<Vec<comment::Model>, DbErr> {
    comment::Entity::find()
        .filter(comment::Column::BookId.eq(book_id))
        .order_by_desc(comment::Column::CreatedAt) // Les plus récents en premier
        .all(db)
        .await
}

pub async fn get_comment_by_id(
    db: &DatabaseConnection,
    id: i32,
) -> Result<Option<comment::Model>, DbErr> {
    comment::Entity::find_by_id(id).one(db).await
}

pub async fn get_replies_by_parent_id(
    db: &DatabaseConnection,
    parent_id: i32,
) -> Result<Vec<comment::Model>, DbErr> {
    comment::Entity::find()
        .filter(comment::Column::ParentId.eq(parent_id))
        .order_by_asc(comment::Column::CreatedAt)
        .all(db)
        .await
}

pub async fn update_comment(
    db: &DatabaseConnection,
    id: i32,
    new_content: String,
) -> Result<comment::Model, DbErr> {
    let comment_model = comment::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or(DbErr::RecordNotFound(format!("Commentaire {} introuvable", id)))?;

    let mut active: comment::ActiveModel = comment_model.into();
    active.content = Set(new_content);
    active.update(db).await
}

pub async fn delete_comment_and_replies(
    db: &DatabaseConnection,
    id: i32,
) -> Result<u64, DbErr> {
    let txn = db.begin().await?;

    // 1. Vérifier si le commentaire existe
    let exists = comment::Entity::find_by_id(id).one(&txn).await?;
    if exists.is_none() {
        txn.rollback().await?;
        return Err(DbErr::RecordNotFound(format!("Commentaire {} introuvable", id)));
    }

    // 2. Recherche récursive de toutes les réponses et sous-réponses
    let mut ids_to_delete = vec![id];
    let mut index = 0;
    while index < ids_to_delete.len() {
        let current_id = ids_to_delete[index];
        let children = comment::Entity::find()
            .filter(comment::Column::ParentId.eq(current_id))
            .all(&txn)
            .await?;

        for child in children {
            ids_to_delete.push(child.id);
        }
        index += 1;
    }

    // 3. Suppression dans l'ordre inverse : les feuilles (enfants) d'abord, puis le parent racine.
    // Cela garantit l'absence de conflit de clé étrangère quel que soit le paramétrage SQL.
    let mut deleted_count: u64 = 0;
    for comment_id in ids_to_delete.into_iter().rev() {
        let res = comment::Entity::delete_by_id(comment_id).exec(&txn).await?;
        deleted_count += res.rows_affected;
    }

    txn.commit().await?;
    Ok(deleted_count)
}