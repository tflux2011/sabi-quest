// Database schema for Project Nigeria
// Contains all table definitions as specified in PROJECT_CONTEXT.md

use rusqlite::Connection;
use super::DatabaseError;

/// SQL schema for the curriculum database
/// This matches the enhanced schema for Zone-based learning
const SCHEMA_SQL: &str = r#"
-- Core Curriculum Data
-- States represent the 36 Nigerian states + FCT that players can explore
CREATE TABLE IF NOT EXISTS states (
    id TEXT PRIMARY KEY,           -- Short code e.g., "LAG", "ABJ"
    name TEXT NOT NULL,            -- Full name e.g., "Lagos", "FCT"
    region TEXT,                   -- Geographic region e.g., "South West"
    zone TEXT,                     -- Game zone: "mind", "heritage", "innovation"
    unlock_level INTEGER DEFAULT 1, -- Level required to access this state
    landmark_name TEXT,            -- Famous landmark e.g., "National Theatre"
    landmark_image TEXT,           -- Path to landmark image asset
    description TEXT,              -- Welcome text for the state
    fun_fact TEXT,                 -- Interesting fact about the state
    population_rank INTEGER        -- Population ranking (1 = highest)
);

-- Learning Modules (replacing simple lessons)
-- Each module is a themed learning experience for a state
CREATE TABLE IF NOT EXISTS modules (
    id TEXT PRIMARY KEY,
    state_id TEXT NOT NULL,
    subject TEXT NOT NULL,         -- "Mathematics", "Social Studies", "Science", etc.
    title TEXT NOT NULL,           -- In-game module name e.g., "The Balogun Market Challenge"
    description TEXT,              -- Short description of the module
    required_level INTEGER DEFAULT 1,
    total_xp INTEGER DEFAULT 0,    -- Total XP available in this module
    estimated_time INTEGER,        -- Estimated completion time in minutes
    icon TEXT,                     -- Icon identifier for the module
    education_level TEXT DEFAULT 'all', -- Target education level: "primary_lower", "primary_upper", "jss", "sss", "all"
    interest_tags TEXT,            -- JSON array of interest tags: ["history", "culture", "geography", "food", "music", "languages"]
    FOREIGN KEY(state_id) REFERENCES states(id) ON DELETE CASCADE
);

-- Encarta-style rich context for modules
CREATE TABLE IF NOT EXISTS module_context (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    module_id TEXT NOT NULL UNIQUE,
    did_you_know TEXT,             -- Educational fact
    fun_fact TEXT,                 -- Fun engaging fact
    intro_text TEXT,               -- Introduction narrative
    historical_note TEXT,          -- Historical context
    intro_image_url TEXT,          -- Path to intro image
    intro_video_url TEXT,          -- Path to intro video (optional)
    FOREIGN KEY(module_id) REFERENCES modules(id) ON DELETE CASCADE
);

-- Levels within a module (progressive difficulty)
CREATE TABLE IF NOT EXISTS levels (
    id TEXT PRIMARY KEY,
    module_id TEXT NOT NULL,
    title TEXT NOT NULL,
    difficulty TEXT NOT NULL,      -- "easy", "medium", "hard"
    order_index INTEGER DEFAULT 0, -- Order within the module
    xp_reward INTEGER DEFAULT 100,
    unlock_item_id TEXT,           -- Item unlocked on completion
    FOREIGN KEY(module_id) REFERENCES modules(id) ON DELETE CASCADE
);

-- Question Bank for quizzes and challenges
CREATE TABLE IF NOT EXISTS questions (
    id TEXT PRIMARY KEY,
    level_id TEXT NOT NULL,
    question_text TEXT NOT NULL,
    question_type TEXT NOT NULL,   -- "multiple_choice", "input_number", "drag_drop", "fill_blank", "true_false"
    options_json TEXT,             -- JSON array of options for multiple choice
    correct_answer TEXT NOT NULL,  -- Correct option id or value
    xp_reward INTEGER DEFAULT 10,
    explanation TEXT,              -- Explanation shown after answering
    hint TEXT,                     -- Optional hint
    image_url TEXT,                -- Optional image/sprite for the question
    order_index INTEGER DEFAULT 0, -- Order within the level
    FOREIGN KEY(level_id) REFERENCES levels(id) ON DELETE CASCADE
);

-- Legacy lessons table for backward compatibility
CREATE TABLE IF NOT EXISTS lessons (
    id TEXT PRIMARY KEY,
    state_id TEXT NOT NULL,
    subject TEXT NOT NULL,         -- "Math", "Science", "Social Studies", etc.
    title TEXT NOT NULL,
    content_md TEXT,               -- Markdown content for the lesson
    difficulty INTEGER DEFAULT 1,   -- 1 = Easy, 2 = Medium, 3 = Hard
    estimated_time INTEGER,        -- Estimated completion time in minutes
    FOREIGN KEY(state_id) REFERENCES states(id) ON DELETE CASCADE
);

-- User Save Data - Progress tracking per user per state
CREATE TABLE IF NOT EXISTS user_progress (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    state_id TEXT NOT NULL,
    stars INTEGER DEFAULT 0 CHECK(stars >= 0 AND stars <= 3), -- 0 to 3 stars
    is_completed INTEGER DEFAULT 0,  -- SQLite uses INTEGER for boolean
    best_score INTEGER DEFAULT 0,
    attempts INTEGER DEFAULT 0,
    last_played_at TEXT,            -- ISO 8601 datetime string
    UNIQUE(user_id, state_id),
    FOREIGN KEY(state_id) REFERENCES states(id) ON DELETE CASCADE
);

-- Module progress tracking
CREATE TABLE IF NOT EXISTS user_module_progress (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    module_id TEXT NOT NULL,
    current_level_id TEXT,         -- Current level the user is on
    is_completed INTEGER DEFAULT 0,
    stars INTEGER DEFAULT 0 CHECK(stars >= 0 AND stars <= 3),
    total_xp_earned INTEGER DEFAULT 0,
    best_score INTEGER DEFAULT 0,
    attempts INTEGER DEFAULT 0,
    last_played_at TEXT,
    UNIQUE(user_id, module_id),
    FOREIGN KEY(module_id) REFERENCES modules(id) ON DELETE CASCADE
);

-- Level progress tracking
CREATE TABLE IF NOT EXISTS user_level_progress (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    level_id TEXT NOT NULL,
    is_completed INTEGER DEFAULT 0,
    score INTEGER DEFAULT 0,
    xp_earned INTEGER DEFAULT 0,
    completed_at TEXT,
    UNIQUE(user_id, level_id),
    FOREIGN KEY(level_id) REFERENCES levels(id) ON DELETE CASCADE
);

-- Inventory for collectible items and rewards
CREATE TABLE IF NOT EXISTS inventory (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    item_id TEXT NOT NULL,          -- e.g., "benin_bronze_mask"
    item_type TEXT,                 -- "badge", "outfit", "accessory", "collectible"
    acquired_at TEXT DEFAULT (datetime('now')), -- ISO 8601 datetime
    UNIQUE(user_id, item_id)
);

-- Unlockable items catalog
CREATE TABLE IF NOT EXISTS items (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    item_type TEXT NOT NULL,        -- "badge", "outfit", "accessory", "collectible"
    rarity TEXT DEFAULT 'common',   -- "common", "rare", "epic", "legendary"
    image_url TEXT
);

-- User profiles for avatar customization and XP tracking
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    display_name TEXT NOT NULL,
    avatar_json TEXT,              -- JSON object with avatar customization
    adventurer_type TEXT DEFAULT 'explorer', -- "explorer", "scholar", "warrior", "artist", "storyteller", "chief"
    birth_year INTEGER,            -- User's birth year for age-appropriate content
    education_level TEXT,          -- "primary_lower", "primary_upper", "jss", "sss"
    interests_json TEXT,           -- JSON array of interest IDs: ["history", "culture", "geography", "food", "music", "languages"]
    total_xp INTEGER DEFAULT 0,
    current_level INTEGER DEFAULT 1,
    cowrie_shells INTEGER DEFAULT 0, -- In-game currency
    streak_days INTEGER DEFAULT 0,
    current_zone TEXT DEFAULT 'heritage', -- Current zone: "mind", "heritage", "innovation"
    last_login_at TEXT,
    created_at TEXT DEFAULT (datetime('now'))
);

-- Settings for app configuration (per user)
CREATE TABLE IF NOT EXISTS settings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    setting_key TEXT NOT NULL,
    setting_value TEXT,
    UNIQUE(user_id, setting_key),
    FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Achievements and badges
CREATE TABLE IF NOT EXISTS achievements (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    icon_path TEXT,
    xp_reward INTEGER DEFAULT 0,
    requirement_json TEXT          -- JSON object describing unlock requirements
);

-- User achievements junction table
CREATE TABLE IF NOT EXISTS user_achievements (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    achievement_id TEXT NOT NULL,
    unlocked_at TEXT DEFAULT (datetime('now')),
    UNIQUE(user_id, achievement_id),
    FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY(achievement_id) REFERENCES achievements(id) ON DELETE CASCADE
);

-- =====================================================
-- THE SABI CODEX - Encyclopedia System
-- =====================================================

-- Encyclopedia entries - The knowledge database
CREATE TABLE IF NOT EXISTS encyclopedia_entries (
    id TEXT PRIMARY KEY,
    category TEXT NOT NULL,         -- "folklore", "history", "famous_nigerians", "culture", "geography", "languages"
    title TEXT NOT NULL,
    subtitle TEXT,                  -- Short tagline
    content_md TEXT NOT NULL,       -- Full markdown content
    summary TEXT,                   -- Short preview text
    image_url TEXT,                 -- Header/thumbnail image
    audio_url TEXT,                 -- Audio narration path
    associated_state TEXT,          -- Optional: ties to a specific state
    tier INTEGER DEFAULT 1,         -- 1 = Open, 2 = Unlockable
    unlock_condition TEXT,          -- e.g., "visit_state_oyo", "complete_quiz_xyz"
    xp_reward INTEGER DEFAULT 10,   -- XP earned when first read
    reading_time INTEGER DEFAULT 5, -- Estimated reading time in minutes
    tags TEXT,                      -- JSON array of searchable tags
    created_at TEXT DEFAULT (datetime('now')),
    FOREIGN KEY(associated_state) REFERENCES states(id) ON DELETE SET NULL
);

-- User encyclopedia progress - Track what user has unlocked/read
CREATE TABLE IF NOT EXISTS user_encyclopedia (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    entry_id TEXT NOT NULL,
    is_unlocked INTEGER DEFAULT 0,
    is_read INTEGER DEFAULT 0,
    is_bookmarked INTEGER DEFAULT 0,
    unlocked_at TEXT,
    first_read_at TEXT,
    read_count INTEGER DEFAULT 0,
    UNIQUE(user_id, entry_id),
    FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY(entry_id) REFERENCES encyclopedia_entries(id) ON DELETE CASCADE
);

-- Create indexes for frequently queried columns
CREATE INDEX IF NOT EXISTS idx_modules_state ON modules(state_id);
CREATE INDEX IF NOT EXISTS idx_levels_module ON levels(module_id);
CREATE INDEX IF NOT EXISTS idx_questions_level ON questions(level_id);
CREATE INDEX IF NOT EXISTS idx_lessons_state ON lessons(state_id);
CREATE INDEX IF NOT EXISTS idx_user_progress_user ON user_progress(user_id);
CREATE INDEX IF NOT EXISTS idx_user_module_progress_user ON user_module_progress(user_id);
CREATE INDEX IF NOT EXISTS idx_inventory_user ON inventory(user_id);
CREATE INDEX IF NOT EXISTS idx_encyclopedia_category ON encyclopedia_entries(category);
CREATE INDEX IF NOT EXISTS idx_encyclopedia_state ON encyclopedia_entries(associated_state);
CREATE INDEX IF NOT EXISTS idx_user_encyclopedia_user ON user_encyclopedia(user_id);

-- =====================================================
-- AVATAR & CHARACTER CUSTOMIZATION SYSTEM
-- =====================================================

-- Avatar items catalog (outfits, hairstyles, accessories)
CREATE TABLE IF NOT EXISTS avatar_items (
    id TEXT PRIMARY KEY,
    category TEXT NOT NULL,           -- "skin_tone", "hairstyle", "outfit", "accessory", "background"
    name TEXT NOT NULL,
    description TEXT,
    image_key TEXT,                   -- CSS/SVG identifier for rendering
    rarity TEXT DEFAULT 'common',     -- "starter", "common", "rare", "epic", "legendary"
    unlock_cost INTEGER DEFAULT 0,    -- Cost in cowrie shells (0 = free/starter)
    unlock_condition TEXT,            -- JSON: {"type": "level", "value": 5} or {"type": "quest", "value": "quest_id"}
    is_premium INTEGER DEFAULT 0,     -- Premium items (future IAP)
    sort_order INTEGER DEFAULT 0
);

-- User's unlocked avatar items
CREATE TABLE IF NOT EXISTS user_avatar_items (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    item_id TEXT NOT NULL,
    unlocked_at TEXT DEFAULT (datetime('now')),
    is_equipped INTEGER DEFAULT 0,    -- Currently wearing this item
    UNIQUE(user_id, item_id),
    FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY(item_id) REFERENCES avatar_items(id) ON DELETE CASCADE
);

-- User's current avatar configuration (what they're wearing)
CREATE TABLE IF NOT EXISTS user_avatar (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL UNIQUE,
    skin_tone TEXT DEFAULT 'skin_3',
    hairstyle TEXT DEFAULT 'hair_1',
    outfit TEXT DEFAULT 'outfit_school',
    accessory TEXT,
    background TEXT DEFAULT 'bg_default',
    character_name TEXT,              -- User's chosen character name
    updated_at TEXT DEFAULT (datetime('now')),
    FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- =====================================================
-- CULTURAL GUIDES & NPC SYSTEM
-- =====================================================

-- Cultural guide characters (NPCs for each state/region)
CREATE TABLE IF NOT EXISTS cultural_guides (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,               -- e.g., "Mama Calabar", "Alhaji Kano"
    title TEXT,                       -- e.g., "Guardian of Cross River Heritage"
    description TEXT,
    personality TEXT,                 -- Brief personality description
    avatar_image TEXT,                -- Path to guide's portrait
    state_id TEXT,                    -- Primary state (can be NULL for regional guides)
    region TEXT,                      -- Region they represent
    greeting TEXT,                    -- Initial greeting message
    catchphrase TEXT,                 -- Signature saying
    voice_style TEXT,                 -- For future TTS: "warm", "wise", "energetic"
    FOREIGN KEY(state_id) REFERENCES states(id) ON DELETE SET NULL
);

-- =====================================================
-- QUEST & STORY SYSTEM
-- =====================================================

-- Quest definitions
CREATE TABLE IF NOT EXISTS quests (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    quest_type TEXT NOT NULL,         -- "main", "side", "daily", "weekly", "achievement"
    category TEXT,                    -- "exploration", "learning", "collection", "mastery"
    state_id TEXT,                    -- Associated state (NULL for global quests)
    guide_id TEXT,                    -- Cultural guide who gives this quest
    
    -- Requirements
    required_level INTEGER DEFAULT 1,
    prerequisite_quest_id TEXT,       -- Must complete this quest first
    requirements_json TEXT,           -- JSON: [{"type": "complete_module", "target": "mod_id", "count": 1}]
    
    -- Rewards
    xp_reward INTEGER DEFAULT 0,
    cowrie_reward INTEGER DEFAULT 0,
    artifact_reward_id TEXT,          -- Artifact unlocked on completion
    item_rewards_json TEXT,           -- JSON array of item IDs
    
    -- Story/Dialogue
    intro_dialogue TEXT,              -- Guide's intro when quest starts
    progress_dialogue TEXT,           -- Guide's check-in message
    completion_dialogue TEXT,         -- Guide's celebration message
    
    -- Display
    icon TEXT,
    sort_order INTEGER DEFAULT 0,
    is_repeatable INTEGER DEFAULT 0,
    cooldown_hours INTEGER,           -- For daily/weekly quests
    
    FOREIGN KEY(state_id) REFERENCES states(id) ON DELETE SET NULL,
    FOREIGN KEY(guide_id) REFERENCES cultural_guides(id) ON DELETE SET NULL,
    FOREIGN KEY(prerequisite_quest_id) REFERENCES quests(id) ON DELETE SET NULL
);

-- User quest progress
CREATE TABLE IF NOT EXISTS user_quests (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    quest_id TEXT NOT NULL,
    status TEXT DEFAULT 'available',  -- "locked", "available", "active", "completed", "claimed"
    progress_json TEXT,               -- JSON tracking requirement progress
    started_at TEXT,
    completed_at TEXT,
    claimed_at TEXT,                  -- When rewards were claimed
    completion_count INTEGER DEFAULT 0, -- For repeatable quests
    UNIQUE(user_id, quest_id),
    FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY(quest_id) REFERENCES quests(id) ON DELETE CASCADE
);

-- =====================================================
-- ARTIFACTS & COLLECTIBLES SYSTEM
-- =====================================================

-- Cultural artifacts that can be collected
CREATE TABLE IF NOT EXISTS artifacts (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    long_description TEXT,            -- Detailed historical/cultural info
    category TEXT NOT NULL,           -- "mask", "textile", "instrument", "sculpture", "jewelry", "weapon", "pottery", "document"
    state_id TEXT,                    -- State of origin
    region TEXT,                      -- Region of origin
    
    -- Display
    image_url TEXT,
    thumbnail_url TEXT,
    model_3d_url TEXT,                -- Future: 3D model for museum view
    color_primary TEXT,               -- For placeholder rendering
    color_secondary TEXT,
    
    -- Rarity & Value
    rarity TEXT DEFAULT 'common',     -- "common", "uncommon", "rare", "epic", "legendary"
    historical_period TEXT,           -- e.g., "Pre-colonial", "19th Century"
    cultural_significance TEXT,       -- Brief significance note
    
    -- Unlock conditions
    unlock_type TEXT NOT NULL,        -- "quest", "module", "achievement", "purchase", "event"
    unlock_source_id TEXT,            -- ID of quest/module/achievement that unlocks this
    cowrie_cost INTEGER DEFAULT 0,    -- If purchasable
    
    sort_order INTEGER DEFAULT 0,
    FOREIGN KEY(state_id) REFERENCES states(id) ON DELETE SET NULL
);

-- User's collected artifacts
CREATE TABLE IF NOT EXISTS user_artifacts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    artifact_id TEXT NOT NULL,
    obtained_at TEXT DEFAULT (datetime('now')),
    obtain_method TEXT,               -- "quest", "module", "purchase", "gift"
    is_favorite INTEGER DEFAULT 0,    -- User marked as favorite
    is_new INTEGER DEFAULT 1,         -- Hasn't been viewed yet
    display_slot INTEGER,             -- Position in museum display (NULL = not displayed)
    UNIQUE(user_id, artifact_id),
    FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY(artifact_id) REFERENCES artifacts(id) ON DELETE CASCADE
);

-- =====================================================
-- STORY & DIALOGUE SYSTEM
-- =====================================================

-- Story chapters (main storyline progression)
CREATE TABLE IF NOT EXISTS story_chapters (
    id TEXT PRIMARY KEY,
    chapter_number INTEGER NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    required_states_json TEXT,        -- JSON array of state IDs that must be visited
    required_quests_json TEXT,        -- JSON array of quest IDs that must be completed
    intro_cutscene TEXT,              -- Text/JSON for intro sequence
    outro_cutscene TEXT,              -- Text/JSON for outro sequence
    sort_order INTEGER DEFAULT 0
);

-- User story progress
CREATE TABLE IF NOT EXISTS user_story_progress (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    chapter_id TEXT NOT NULL,
    status TEXT DEFAULT 'locked',     -- "locked", "available", "in_progress", "completed"
    started_at TEXT,
    completed_at TEXT,
    UNIQUE(user_id, chapter_id),
    FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY(chapter_id) REFERENCES story_chapters(id) ON DELETE CASCADE
);

-- =====================================================
-- ADDITIONAL INDEXES
-- =====================================================

CREATE INDEX IF NOT EXISTS idx_avatar_items_category ON avatar_items(category);
CREATE INDEX IF NOT EXISTS idx_user_avatar_items_user ON user_avatar_items(user_id);
CREATE INDEX IF NOT EXISTS idx_quests_state ON quests(state_id);
CREATE INDEX IF NOT EXISTS idx_quests_type ON quests(quest_type);
CREATE INDEX IF NOT EXISTS idx_user_quests_user ON user_quests(user_id);
CREATE INDEX IF NOT EXISTS idx_user_quests_status ON user_quests(status);
CREATE INDEX IF NOT EXISTS idx_artifacts_state ON artifacts(state_id);
CREATE INDEX IF NOT EXISTS idx_artifacts_rarity ON artifacts(rarity);
CREATE INDEX IF NOT EXISTS idx_user_artifacts_user ON user_artifacts(user_id);
CREATE INDEX IF NOT EXISTS idx_cultural_guides_state ON cultural_guides(state_id);
"#;

/// Creates all database tables if they don't exist
pub fn create_tables(conn: &Connection) -> Result<(), DatabaseError> {
    conn.execute_batch(SCHEMA_SQL)
        .map_err(|e| DatabaseError::InitializationError(format!("Failed to create tables: {}", e)))?;
    
    // Run migrations for existing databases
    run_migrations(conn)?;
    
    log::info!("Database schema created/verified successfully");
    Ok(())
}

/// Runs database migrations to update existing databases with new columns
fn run_migrations(conn: &Connection) -> Result<(), DatabaseError> {
    // Migration: Add adventurer_type column to users table if it doesn't exist
    let has_adventurer_type: bool = conn
        .query_row(
            "SELECT COUNT(*) > 0 FROM pragma_table_info('users') WHERE name = 'adventurer_type'",
            [],
            |row| row.get(0),
        )
        .unwrap_or(false);
    
    if !has_adventurer_type {
        log::info!("Running migration: Adding adventurer_type column to users table");
        conn.execute(
            "ALTER TABLE users ADD COLUMN adventurer_type TEXT DEFAULT 'explorer'",
            [],
        )
        .map_err(|e| DatabaseError::InitializationError(format!("Migration failed: {}", e)))?;
    }
    
    Ok(())
}

/// Checks if the database has been seeded with initial data
pub fn is_database_seeded(conn: &Connection) -> Result<bool, DatabaseError> {
    let count: i32 = conn
        .query_row("SELECT COUNT(*) FROM states", [], |row| row.get(0))
        .map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    Ok(count > 0)
}

/// Resets the database by dropping all tables and recreating them
/// WARNING: This will delete all user data! Use only for development/testing
#[allow(dead_code)]
pub fn reset_database(conn: &Connection) -> Result<(), DatabaseError> {
    log::warn!("Resetting database - all data will be lost!");
    
    conn.execute_batch(
        r#"
        DROP TABLE IF EXISTS user_achievements;
        DROP TABLE IF EXISTS achievements;
        DROP TABLE IF EXISTS settings;
        DROP TABLE IF EXISTS users;
        DROP TABLE IF EXISTS inventory;
        DROP TABLE IF EXISTS user_progress;
        DROP TABLE IF EXISTS questions;
        DROP TABLE IF EXISTS lessons;
        DROP TABLE IF EXISTS states;
        "#,
    )
    .map_err(|e| DatabaseError::InitializationError(e.to_string()))?;
    
    create_tables(conn)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;
    
    #[test]
    fn test_create_tables() {
        let conn = Connection::open_in_memory().unwrap();
        assert!(create_tables(&conn).is_ok());
    }
    
    #[test]
    fn test_is_database_seeded_empty() {
        let conn = Connection::open_in_memory().unwrap();
        create_tables(&conn).unwrap();
        assert!(!is_database_seeded(&conn).unwrap());
    }
}
