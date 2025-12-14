use serde::Deserialize;

// 1. Define the structure of a SINGLE File
#[derive(Deserialize, Debug)] 
struct File {
   name: String,  
   is_dir: bool,
   size: u64,
}

// 2. Use the Tokio runtime
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "http://192.168.1.157:8080/files";
    println!("Fetching {}...", url);

    // 3. Await the response
    let res = reqwest::get(url).await?;

    println!("Status: {}", res.status());

    // 4. Parse JSON into a Vector (List) of Files
    let files: Vec<File> = res.json().await?;

    println!("Fetched {} files.", files.len());
    // Use {:#?} for "pretty print" debugging
    if let Some(first) = files.first() {
        println!("First file: {:#?}", first);
    }
println!("All {:#?}",files);
    Ok(())
}
