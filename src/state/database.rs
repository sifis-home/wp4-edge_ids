mod models;
mod schema;

use crate::state::database::models::{NewAlarms, NewConfiguration, NewData, NewWebhook};
use crate::structures::configuration::{NetspotConfig, NetspotConfigMap};
use crate::structures::statistics::{
    AlarmMessage, AlarmMessages, DataMessage, DataMessages, Message,
};

use diesel::prelude::*;
use diesel::sqlite::Sqlite;
use diesel::{Connection, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use crate::structures::webhooks::{Webhook, WebhookItem, WebhookList, Webhooks};
use crate::tasks::RunChecker;
use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};
use tokio::sync::broadcast;

pub enum DatabaseError {
    NotFound,
    Unexpected(String),
}

// TODO: Check if RwLock could be used here
type DbConnection = Arc<Mutex<SqliteConnection>>;

pub struct Database {
    db_connection: DbConnection,
}

impl Database {
    pub fn new(
        database_url: &str,
        messages_rx: broadcast::Receiver<Message>,
        run_checker: RunChecker,
    ) -> Result<Database, String> {
        // Get database connection
        let mut connection = match SqliteConnection::establish(database_url) {
            Ok(connection) => connection,
            Err(err) => return Err(format!("Could not open database: {}", err)),
        };

        // Run migrations
        if let Err(err) = Database::run_migrations(&mut connection) {
            return Err(format!("Could not run migrations: {}", err));
        }

        // Create shared database connection object
        let db_connection = Arc::new(Mutex::new(connection));

        // Start task for writing incoming messages to the database
        tokio::spawn(database_writer(
            db_connection.clone(),
            messages_rx,
            run_checker,
        ));

        // Return complete database
        Ok(Database { db_connection })
    }

    pub fn add_configuration(&self, new_config: &NetspotConfig) -> Result<(), String> {
        match serde_json::to_string(&new_config) {
            Ok(value) => {
                // We have JSON for the new config
                let new_configuration = NewConfiguration { config: &value };
                let mut connection = self.db_connection.lock().unwrap();
                match diesel::insert_into(schema::configurations::dsl::configurations)
                    .values(new_configuration)
                    .execute(&mut *connection)
                {
                    Ok(1) => Ok(()),
                    Err(err) => Err(err.to_string()),
                    Ok(rows) => Err(format!("Unexpected row write count: {}", rows)),
                }
            }
            Err(err) => Err(format!("Could not convert NetspotConfig to JSON: {}", err)),
        }
    }

    pub fn add_webhook(&self, new_webhook: &Webhook) -> Result<(), String> {
        match serde_json::to_string(&new_webhook) {
            Ok(webhook_config) => {
                let new_webhook = NewWebhook {
                    config: &webhook_config,
                };
                let mut connection = self.db_connection.lock().unwrap();
                match diesel::insert_into(schema::webhooks::dsl::webhooks)
                    .values(new_webhook)
                    .execute(&mut *connection)
                {
                    Ok(1) => Ok(()),
                    Err(err) => Err(err.to_string()),
                    Ok(rows) => Err(format!("Unexpected row write count: {}", rows)),
                }
            }
            Err(err) => Err(format!("Could not convert Webhook to JSON: {}", err)),
        }
    }

    pub fn delete_configuration(&self, with_id: i32) -> Result<(), DatabaseError> {
        let mut connection = self.db_connection.lock().unwrap();
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

    pub fn delete_webhook(&self, with_id: i32) -> Result<(), DatabaseError> {
        let mut connection = self.db_connection.lock().unwrap();
        match diesel::delete(
            schema::webhooks::dsl::webhooks.filter(schema::webhooks::id.eq(with_id)),
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

    pub fn get_alarms(
        &self,
        time: Option<i64>,
        last: Option<i32>,
    ) -> Result<AlarmMessages, DatabaseError> {
        let mut query = schema::alarms::dsl::alarms
            .select(schema::alarms::message)
            .into_boxed();
        if let Some(time) = time {
            query = query.filter(schema::alarms::time.gt(time));
        }
        if let Some(last) = last {
            query = query.order(schema::alarms::time.desc()).limit(last.into())
        }
        let mut connection = self.db_connection.lock().unwrap();
        match query.load::<String>(&mut *connection) {
            Ok(rows) => {
                let mut results = AlarmMessages::new();
                for row in rows {
                    if let Ok(message) = serde_json::from_str::<AlarmMessage>(&row) {
                        results.push(message);
                    }
                }
                if last.is_some() {
                    results.reverse();
                }
                Ok(results)
            }
            Err(err) => Err(DatabaseError::Unexpected(err.to_string())),
        }
    }

    pub fn get_data(
        &self,
        time: Option<i64>,
        last: Option<i32>,
    ) -> Result<DataMessages, DatabaseError> {
        let mut query = schema::data::dsl::data
            .select(schema::data::message)
            .into_boxed();
        if let Some(time) = time {
            query = query.filter(schema::data::time.gt(time));
        }
        if let Some(last) = last {
            query = query.order(schema::data::time.desc()).limit(last.into())
        }
        let mut connection = self.db_connection.lock().unwrap();
        match query.load::<String>(&mut *connection) {
            Ok(rows) => {
                let mut results = DataMessages::new();
                for row in rows {
                    if let Ok(message) = serde_json::from_str::<DataMessage>(&row) {
                        results.push(message);
                    }
                }
                if last.is_some() {
                    results.reverse();
                }
                Ok(results)
            }
            Err(err) => Err(DatabaseError::Unexpected(err.to_string())),
        }
    }

    pub fn get_configuration(&self, with_id: i32) -> Option<NetspotConfig> {
        let mut connection = self.db_connection.lock().unwrap();
        match schema::configurations::dsl::configurations
            .filter(schema::configurations::id.eq(with_id))
            .load::<models::Configuration>(&mut *connection)
        {
            Ok(results) => {
                if let Some(result) = results.get(0) {
                    return serde_json::from_str(&result.config).ok();
                }
            }
            Err(err) => eprintln!("Query failed: {}", err),
        }
        None
    }

    pub fn get_configurations(&self) -> Result<NetspotConfigMap, String> {
        let mut connection = self.db_connection.lock().unwrap();
        match schema::configurations::dsl::configurations
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
                                result.id, err
                            ));
                        }
                    }
                }
                Ok(netspot_configurations)
            }
            Err(err) => Err(format!("Query failed: {}", err)),
        }
    }

    pub fn get_webhook(&self, with_id: i32) -> Option<Webhook> {
        let mut connection = self.db_connection.lock().unwrap();
        match schema::webhooks::dsl::webhooks
            .filter(schema::webhooks::id.eq(with_id))
            .load::<models::Configuration>(&mut *connection)
        {
            Ok(results) => {
                if let Some(result) = results.get(0) {
                    return serde_json::from_str(&result.config).ok();
                }
            }
            Err(err) => eprintln!("Query failed: {}", err),
        }
        None
    }

    pub fn get_webhooks(&self) -> Result<Webhooks, String> {
        let mut connection = self.db_connection.lock().unwrap();
        match schema::webhooks::dsl::webhooks.load::<models::Configuration>(&mut *connection) {
            Ok(results) => {
                let mut webhooks = Webhooks::new();
                for result in results {
                    match serde_json::from_str::<Webhook>(&result.config) {
                        Ok(webhook) => {
                            webhooks.insert(result.id, webhook);
                        }
                        Err(err) => {
                            return Err(format!(
                                "Parsing configuration {} failed: {}",
                                result.id, err
                            ));
                        }
                    }
                }
                Ok(webhooks)
            }
            Err(err) => Err(format!("Query failed: {}", err)),
        }
    }

    pub fn list_webhooks(&self) -> Result<WebhookList, String> {
        let mut connection = self.db_connection.lock().unwrap();
        match schema::webhooks::dsl::webhooks.load::<models::Configuration>(&mut *connection) {
            Ok(results) => {
                let mut webhooks = WebhookList::new();
                for result in results {
                    match serde_json::from_str::<Webhook>(&result.config) {
                        Ok(hook) => {
                            webhooks.push(WebhookItem {
                                id: result.id,
                                name: hook.name,
                            });
                        }
                        Err(err) => {
                            return Err(format!(
                                "Parsing configuration {} failed: {}",
                                result.id, err
                            ));
                        }
                    }
                }
                Ok(webhooks)
            }
            Err(err) => Err(format!("Query failed: {}", err)),
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
                let mut connection = self.db_connection.lock().unwrap();
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
                err
            ))),
        }
    }

    pub fn set_webhook(&self, with_id: i32, new_config: &Webhook) -> Result<(), DatabaseError> {
        match serde_json::to_string(&new_config) {
            Ok(config_json) => {
                let new_config = NewWebhook {
                    config: &config_json,
                };
                let mut connection = self.db_connection.lock().unwrap();
                match diesel::update(schema::webhooks::dsl::webhooks)
                    .filter(schema::webhooks::id.eq(with_id))
                    .set(new_config)
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
                err
            ))),
        }
    }

    fn run_migrations(
        connection: &mut impl MigrationHarness<Sqlite>,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");
        connection.run_pending_migrations(MIGRATIONS)?;
        Ok(())
    }
}

async fn database_writer(
    db_connection: DbConnection,
    mut message_rx: broadcast::Receiver<Message>,
    mut run_checker: RunChecker,
) {
    println!("Database writer started.");
    let mut cleanup_interval = tokio::time::interval(Duration::from_secs(60));
    while run_checker.keep_running() {
        tokio::select! {
            Ok(message) = message_rx.recv() => write_message(message, &db_connection),
            _ = cleanup_interval.tick() => cleanup_messages(&db_connection),
            _ = run_checker.shutdown_recv() => {},
        }
    }
    println!("Database writer stopped.");
}

fn write_message(message: Message, db_connection: &DbConnection) {
    if let Ok(json) = message.to_json() {
        match message {
            Message::Alarm(message) => {
                write_alarms(db_connection, message.time, &json);
            }
            Message::Data(message) => {
                write_data(db_connection, message.time, &json);
            }
        }
    }
}

fn cleanup_messages(db_connection: &DbConnection) {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(duration) => {
            // We remove all results that are older than one hour
            let older_than = (duration - Duration::from_secs(60 * 60)).as_nanos() as i64;
            println!("Cleaning messages older than {older_than}.");
            let mut connection = db_connection.lock().unwrap();
            match diesel::delete(schema::alarms::dsl::alarms)
                .filter(schema::alarms::time.lt(older_than))
                .execute(&mut *connection)
            {
                Ok(rows) => println!("{rows} alarms message(s) removed."),
                Err(err) => eprintln!("cleanup_messages error: {}", err),
            };
            match diesel::delete(schema::data::dsl::data)
                .filter(schema::data::time.lt(older_than))
                .execute(&mut *connection)
            {
                Ok(rows) => println!("{rows} data message(s) removed."),
                Err(err) => eprintln!("cleanup_messages error: {}", err),
            };
        }
        Err(err) => {
            eprintln!("Unexpected error: {}", err);
        }
    }
}

fn write_alarms(db_connection: &DbConnection, time: i64, message: &str) {
    let new_alarms = NewAlarms { time, message };
    let mut connection = db_connection.lock().unwrap();
    match diesel::insert_into(schema::alarms::dsl::alarms)
        .values(new_alarms)
        .execute(&mut *connection)
    {
        Ok(_) => (),
        Err(err) => eprintln!("write_alarms error: {}", err),
    };
}

fn write_data(db_connection: &DbConnection, time: i64, message: &str) {
    let new_data = NewData { time, message };
    let mut connection = db_connection.lock().unwrap();
    match diesel::insert_into(schema::data::dsl::data)
        .values(new_data)
        .execute(&mut *connection)
    {
        Ok(_) => (),
        Err(err) => eprintln!("write_data error: {}", err),
    };
}
