pub mod projects;

use envie::Envie;
use sea_orm::{Database, DatabaseConnection, DbErr};

pub async fn connect_db() -> Result<DatabaseConnection, DbErr> {
    let env = Envie::load().expect("Failed to load .env file");

    let db_host = env.get("DB_HOST").expect("DB_HOST not found");
    let db_user = env.get("DB_USER").expect("DB_USER not found");
    let db_pass = env.get("DB_PASS").expect("DB_PASS not found");
    let db_table = env.get("DB_TABLE").expect("DB_TABLE not found");

    let conn_string: String = format!("mysql://{}:{}@{}/{}", db_user, db_pass, db_host, db_table);
    let db: DatabaseConnection = Database::connect(conn_string).await?;
    Ok(db)
}

#[derive(Responder)]
#[response(status = 500, content_type = "json")]
pub struct ErrorResponder {
    pub message: String,
}

impl From<DbErr> for ErrorResponder {
    fn from(err: DbErr) -> ErrorResponder {
        ErrorResponder {
            message: err.to_string(),
        }
    }
}

impl From<String> for ErrorResponder {
    fn from(string: String) -> ErrorResponder {
        ErrorResponder { message: string }
    }
}

impl From<&str> for ErrorResponder {
    fn from(str: &str) -> ErrorResponder {
        str.to_owned().into()
    }
}