mod models;
mod schema;

use crate::state::database::models::NewConfiguration;
use crate::structures::configuration::{NetspotConfig, NetspotConfigMap};
use diesel::prelude::*;
use diesel::sqlite::Sqlite;
use diesel::{Connection, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenv::dotenv;
use rocket::{debug, warn};
use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;
use std::sync::Mutex;
use std::{env, fs};

pub enum DatabaseError {
    NotFound,
    Unexpected(String),
}

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
            Err(_) => {
                return Err("DB_FILE_PATH environment variable must be set".to_string());
            }
        };

        // Ensure that path to database file exits
        match PathBuf::from(database_file.as_str()).parent() {
            None => {
                return Err("Invalid DB_FILE_PATH environment variable".to_string());
            }
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

    pub fn add_configuration(&self, new_config: &NetspotConfig) -> Result<(), String> {
        match serde_json::to_string(&new_config) {
            Ok(value) => {
                // We have JSON for the new config
                let new_configuration = NewConfiguration { config: &value };
                let mut connection = self.connection_mutex.lock().unwrap();
                match diesel::insert_into(schema::configurations::dsl::configurations)
                    .values(new_configuration)
                    .execute(&mut *connection)
                {
                    Ok(1) => Ok(()),
                    Err(err) => Err(err.to_string()),
                    Ok(rows) => Err(format!("Unexpected row write count: {}", rows)),
                }
            }
            Err(err) => Err(format!(
                "Could not convert NetspotConfig to JSON: {}",
                err.to_string()
            )),
        }
    }

    pub fn get_configuration(&self, with_id: i32) -> Option<NetspotConfig> {
        let mut connection = self.connection_mutex.lock().unwrap();
        match schema::configurations::dsl::configurations
            .filter(schema::configurations::id.eq(with_id))
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

    pub fn delete_configuration(&self, with_id: i32) -> Result<(), DatabaseError> {
        let mut connection = self.connection_mutex.lock().unwrap();
        match diesel::delete(
            schema::configurations::dsl::configurations
                .filter(schema::configurations::id.eq(with_id)),
        )
        .execute(&mut *connection)
        {
            Ok(0) => Err(DatabaseError::NotFound),
            Ok(1) => Ok(()),
            Err(err) => Err(DatabaseError::Unexpected(err.to_string())),
            Ok(rows) => Err(DatabaseError::Unexpected(format!(
                "Unexpected row delete count: {}",
                rows
            ))),
        }
    }

    pub fn set_configuration(
        &self,
        with_id: i32,
        new_config: &NetspotConfig,
    ) -> Result<(), DatabaseError> {
        match serde_json::to_string(&new_config) {
            Ok(value) => {
                // We have JSON for the new config
                let new_configuration = NewConfiguration { config: &value };
                let mut connection = self.connection_mutex.lock().unwrap();
                match diesel::update(schema::configurations::dsl::configurations)
                    .filter(schema::configurations::id.eq(with_id))
                    .set(new_configuration)
                    .execute(&mut *connection)
                {
                    Ok(0) => Err(DatabaseError::NotFound),
                    Ok(1) => Ok(()),
                    Err(err) => Err(DatabaseError::Unexpected(err.to_string())),
                    Ok(rows) => Err(DatabaseError::Unexpected(format!(
                        "Unexpected row update count: {}",
                        rows
                    ))),
                }
            }
            Err(err) => Err(DatabaseError::Unexpected(format!(
                "Could not convert NetspotConfig to JSON: {}",
                err.to_string()
            ))),
        }
    }

    pub fn get_configurations(&self) -> Result<NetspotConfigMap, String> {
        let mut connection = self.connection_mutex.lock().unwrap();
        return match schema::configurations::dsl::configurations
            .load::<models::Configuration>(&mut *connection)
        {
            Ok(results) => {
                let mut netspot_configurations = HashMap::new();
                for result in results {
                    match serde_json::from_str::<NetspotConfig>(&result.config) {
                        Ok(v) => {
                            netspot_configurations.insert(result.id, v);
                        }
                        Err(err) => {
                            return Err(format!(
                                "Parsing configuration {} failed: {}",
                                result.id,
                                err.to_string()
                            ));
                        }
                    }
                }
                Ok(netspot_configurations)
            }
            Err(err) => Err(format!("Query failed: {}", err.to_string())),
        };
    }

    fn run_migrations(
        connection: &mut impl MigrationHarness<Sqlite>,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");
        connection.run_pending_migrations(MIGRATIONS)?;
        Ok(())
    }
}
