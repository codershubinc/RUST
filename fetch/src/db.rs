use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};

// Change return type to Result<SqlitePool, ...>
pub async fn sql_db() -> Result<SqlitePool, sqlx::Error> {
    println!("Connecting to database...");
    let pool = SqlitePoolOptions::new()
        .connect("sqlite:files.db?mode=rwc")
        .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS files (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            path TEXT NOT NULL,
            is_dir BOOLEAN NOT NULL
        )",
    )
    .execute(&pool)
    .await?;

    // Return the pool!
    Ok(pool)
}

pub async fn save_file_data(
    pool: &SqlitePool,
    name: &str,
    path: &str,
    is_dir: bool,
) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO files (name, path, is_dir) VALUES (?, ?, ?)")
        .bind(name)
        .bind(path)
        .bind(is_dir)
        .execute(pool)
        .await?;
    Ok(())
}
