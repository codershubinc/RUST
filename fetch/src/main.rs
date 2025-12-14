mod db;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize DB ONCE at the start
    let pool = db::sql_db().await?;

    let url = "http://localhost:8080/api/files/list";
    println!("Fetching {}...", url);

    let res = reqwest::get(url).await?;
    println!("Status: {}", res.status());

    let files: Value = res.json().await?;

    if let Some(file_arr) = files.as_array() {
        println!("Total {} files found", file_arr.len());

        for file in file_arr {
            let name = file["name"].as_str().unwrap_or("unknown");
            let path = file["path"].as_str().unwrap_or("unknown");
            let is_dir = file["is_dir"].as_bool().unwrap_or(false);

            println!("Saving: {}", name);

            db::save_file_data(&pool, name, path, is_dir).await?;
        }
        println!("All data saved to database.");
    }
    Ok(())
}
