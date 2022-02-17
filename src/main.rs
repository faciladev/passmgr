use dialoguer::Password;
use rusqlite::{params, Connection, Result};
use sha2::{Digest, Sha256, Sha512};

#[derive(Debug)]
struct Pass {
    id: i32,
    salt: String,
    master: String,
}

fn main() -> Result<()> {
    let menu_item = menu().unwrap();

    //Create master password
    if menu_item.trim() == "1" {
        create_master_pass()
    } else if menu_item.trim() == "2" {
        login()
    } else {
        println!("Wrong input.");
        Ok(())
    }
}

fn menu() -> Result<String> {
    println!("1 - Create master password\n2 - Login");
    let mut menu_item = String::new();
    let _mi = std::io::stdin().read_line(&mut menu_item).unwrap();
    Ok(menu_item)
}

fn create_master_pass() -> Result<()> {
    println!("Answer one security question from the list.");
    println!("1 - In what city were you born?\n2 - What is the name of your favorite pet?\n3 - What high school did you attend?\n4 - What is the name of your first school?\n5 - What was your favorite food as a child?");
    let mut security_answer = String::new();
    let _sa = std::io::stdin().read_line(&mut security_answer).unwrap();
    //Create salt based of security question
    let mut security_hasher = Sha512::new();
    security_hasher.update(security_answer);
    let salt = format!("{:X}", security_hasher.finalize());
    //Create master pass
    let master_pass = Password::new()
        .with_prompt("Enter your master password")
        .interact()
        .unwrap();
    let master_pass_with_salt: String = format!("{}{}", master_pass, salt);
    let mut master_hasher = Sha512::new();
    master_hasher.update(master_pass_with_salt);
    let master_pass_hash: String = format!("{:X}", master_hasher.finalize());

    //Save salt and master pass hash to sqlite
    let conn = Connection::open("pass.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS pass (
                  id              INTEGER PRIMARY KEY,
                  salt            TEXT NOT NULL,
                  master            TEXT NOT NULL
                  )",
        [],
    )?;
    let my_pass = Pass {
        id: 0,
        salt: salt,
        master: master_pass_hash,
    };
    conn.execute(
        "INSERT INTO pass (salt, master) VALUES (?1, ?2)",
        params![my_pass.salt, my_pass.master],
    )?;
    println!("Master password created successfully.");
    login()
}

fn login() -> Result<()> {
    let conn = Connection::open("pass.db")?;
    let mut stmt = conn.prepare("SELECT id, salt, master FROM pass")?;
    let mut logged_in = false;
    let mut logged_in_user = Pass {
        id: 0,
        salt: String::from(""),
        master: String::from(""),
    };
    let master_pass_input = Password::new()
        .with_prompt("Login with master password")
        .interact()
        .unwrap();
    let pass_iter = stmt.query_map([], |row| {
        Ok(Pass {
            id: row.get(0)?,
            salt: row.get(1)?,
            master: row.get(2)?,
        })
    })?;

    for pass in pass_iter {
        let pass_db = pass.unwrap();
        let master_pass_input_with_salt: String = format!("{}{}", master_pass_input, pass_db.salt);
        let mut master_input_hasher = Sha512::new();
        master_input_hasher.update(master_pass_input_with_salt);
        let master_pass_input_hash: String = format!("{:X}", master_input_hasher.finalize());
        if master_pass_input_hash == pass_db.master {
            logged_in = true;
            logged_in_user = pass_db;
        }
    }

    if logged_in == true {
        //Create password for apps
        println!("Enter application name to generate password.");
        let mut app_name = String::new();
        let _sa = std::io::stdin().read_line(&mut app_name).unwrap();
        let mut email_username = String::new();
        println!("Enter email or username associated with password.");
        let _sa = std::io::stdin().read_line(&mut email_username).unwrap();

        let mut app_password_hasher = Sha256::new();
        app_password_hasher.update(format!("{}{}", master_pass_input, email_username));
        let app_pass_input_hash: String = format!("{:X}", app_password_hasher.finalize());
        println!(
            "Your password for {} using {} is: {}",
            app_name, email_username, app_pass_input_hash
        );
    } else {
        println!("Login failed!");
    }
    Ok(())
}
