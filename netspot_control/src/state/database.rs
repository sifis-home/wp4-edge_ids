mod models;
mod schema;

use crate::configurations::netspot::NetspotConfig;
use diesel::prelude::*;
use diesel::sqlite::Sqlite;
use diesel::{Connection, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenv::dotenv;
use rocket::{debug, warn};
use std::error::Error;
use std::path::PathBuf;
use std::sync::Mutex;
use std::{env, fs};

pub struct Database {
    connection_mutex: Mutex<SqliteConnection>, // TODO: Check if RwLock could be used here
}

impl Database {
    pub fn new() -> Result<Database, String> {
        // Read .env file when available
        if dotenv().is_ok() {
            debug!("Loaded environment variables from .env file");
        }

        // Get database path
        let database_file = match env::var("DB_FILE_PATH") {
            Ok(path) => path,
            Err(_) => return Err("DB_FILE_PATH environment variable must be set".to_string()),
        };

        // Ensure that path to database file exits
        match PathBuf::from(database_file.as_str()).parent() {
            None => return Err("Invalid DB_FILE_PATH environment variable".to_string()),
            Some(path) => {
                if let Err(err) = fs::create_dir_all(path) {
                    return Err(format!(
                        "Could not create database path: {}",
                        err.to_string()
                    ));
                }
            }
        }

        // Get database connection
        let mut connection = match SqliteConnection::establish(&database_file) {
            Ok(connection) => connection,
            Err(err) => return Err(format!("Could not open database: {}", err.to_string())),
        };

        // Run migrations
        if let Err(err) = Database::run_migrations(&mut connection) {
            return Err(format!("Could not run migrations: {}", err.to_string()));
        }

        // Return complete database
        Ok(Database {
            connection_mutex: Mutex::new(connection),
        })
    }

    pub fn get_configuration(&self, with_id: i32) -> Option<NetspotConfig> {
        use self::schema::configurations::dsl::*;
        let mut connection = self.connection_mutex.lock().unwrap();
        match configurations
            .filter(id.eq(with_id))
            .load::<models::Configuration>(&mut *connection)
        {
            Ok(results) => {
                if let Some(result) = results.get(0) {
                    return serde_json::from_str(&result.config).ok();
                }
            }
            Err(err) => warn!("Query failed: {}", err.to_string()),
        }
        None
    }

    fn run_migrations(
        connection: &mut impl MigrationHarness<Sqlite>,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");
        connection.run_pending_migrations(MIGRATIONS)?;
        Ok(())
    }
}
