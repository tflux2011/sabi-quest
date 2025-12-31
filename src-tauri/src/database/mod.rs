// Database module for Project Nigeria
// Handles SQLite connection, schema initialization, and data operations

mod schema;
mod models;
mod commands;
mod seed_curriculum;

// Re-export commands for use in lib.rs
pub use commands::*;

use rusqlite::Connection;
use parking_lot::Mutex;
use std::path::PathBuf;
use thiserror::Error;

/// Database-related errors with proper error handling
#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Failed to initialize database: {0}")]
    InitializationError(String),
    
    #[error("Database query failed: {0}")]
    QueryError(String),
    
    #[error("Failed to get app data directory")]
    AppDataDirError,
    
    #[error("SQLite error: {0}")]
    SqliteError(#[from] rusqlite::Error),
    
    #[error("Serialization error: {0}")]
    SerializationError(String),
}

// Implement conversion to string for Tauri command error handling
impl serde::Serialize for DatabaseError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

/// Thread-safe database connection wrapper
/// Uses parking_lot::Mutex for better performance than std::sync::Mutex
pub struct DatabaseState {
    pub connection: Mutex<Connection>,
    pub db_path: PathBuf,
}

impl DatabaseState {
    /// Creates a new database state with the given connection and path
    pub fn new(connection: Connection, db_path: PathBuf) -> Self {
        Self {
            connection: Mutex::new(connection),
            db_path,
        }
    }
}

/// Gets the database file path in the app's local data directory
/// This ensures the database is stored in a secure, user-specific location
fn resolve_database_path(app_handle: &tauri::AppHandle) -> Result<PathBuf, DatabaseError> {
    use tauri::Manager;
    
    let app_data_dir = app_handle
        .path()
        .app_local_data_dir()
        .map_err(|_| DatabaseError::AppDataDirError)?;
    
    // Ensure the directory exists
    std::fs::create_dir_all(&app_data_dir)
        .map_err(|e| DatabaseError::InitializationError(e.to_string()))?;
    
    Ok(app_data_dir.join("curriculum.db"))
}

/// Initializes the database connection and creates tables if they don't exist
pub fn initialize_database(app_handle: &tauri::AppHandle) -> Result<DatabaseState, DatabaseError> {
    let db_path = resolve_database_path(app_handle)?;
    
    log::info!("Initializing database at: {:?}", db_path);
    
    // Open or create the database file
    let connection = Connection::open(&db_path)?;
    
    // Enable foreign key constraints (disabled by default in SQLite)
    connection.execute_batch("PRAGMA foreign_keys = ON;")?;
    
    // Set journal mode to WAL for better concurrent read performance
    connection.execute_batch("PRAGMA journal_mode = WAL;")?;
    
    // Run the schema initialization
    schema::create_tables(&connection)?;
    
    log::info!("Database initialized successfully");
    
    Ok(DatabaseState::new(connection, db_path))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_database_error_serialization() {
        let error = DatabaseError::InitializationError("test error".to_string());
        let serialized = serde_json::to_string(&error).unwrap();
        assert!(serialized.contains("test error"));
    }
}
