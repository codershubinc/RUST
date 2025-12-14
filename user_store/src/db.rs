use sqlx::Row;
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};

// Change return type to Result<SqlitePool, ...>
pub async fn sql_db() -> Result<SqlitePool, sqlx::Error> {
    println!("Connecting to database...");
    let pool = SqlitePoolOptions::new()
        .connect("sqlite:files.db?mode=rwc")
        .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS  users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL
        );",
    )
    .execute(&pool)
    .await?;

    // Return the pool!
    Ok(pool)
}

pub async fn clear_user_table(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM files").execute(pool).await?;
    Ok(())
}

pub async fn add_user(pool: &SqlitePool, name: &str) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO users (name) VALUES (?)")
        .bind(name)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn show_all_users(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    let rows = sqlx::query("SELECT id, name FROM users")
        .fetch_all(pool)
        .await?;

    for row in rows {
        println!(
            "ID: {}, Name: {}",
            row.get::<i64, _>("id"),
            row.get::<String, _>("name")
        );
    }
    Ok(())
}

pub async fn delete_user(pool: &SqlitePool, user_id: i64) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(())
}
