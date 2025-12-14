mod db;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // init db
    let sql = db::sql_db().await?;

    println!("Welcome to User Store!");
    loop {
        println!("Please enter a command (add, list, remove exit):");
        let mut command: String = String::new();
        std::io::stdin().read_line(&mut command);
        let command = command.trim();
        match command {
            "add" => {
                println!("Enter user name:");
                let mut name = String::new();
                std::io::stdin().read_line(&mut name);
                let name = name.trim();
                db::add_user(&sql, name).await;
                println!("User '{}' added.", name);
            }
            "exit" => {
                println!("Exiting...");
                break;
            }
            "list" => {
                println!("Listing users:");
                db::show_all_users(&sql).await?;
            }
            "remove" => {
                println!("Enter user ID to remove:");
                let mut id_str = String::new();
                std::io::stdin().read_line(&mut id_str);
                let id: i64 = match id_str.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid ID.");
                        continue;
                    }
                };
                db::delete_user(&sql, id).await?;
                println!("User with ID {} removed.", id);
            }
            &_ => {
                println!("Unknown command: {}", command);
            }
        }
    }
    Ok(())
}
