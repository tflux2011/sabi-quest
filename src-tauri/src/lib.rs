// Project Nigeria - Tauri Backend
// Educational RPG for Nigerian students

mod database;

use database::initialize_database;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .setup(|app| {
            // Initialize logging in debug mode
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            // Initialize the SQLite database
            let db_state = initialize_database(app.handle())
                .expect("Failed to initialize database");
            
            log::info!("Database path: {:?}", db_state.db_path);
            
            // Register database state for use in commands
            app.manage(db_state);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Database commands
            database::init_database,
            database::seed_database,
            database::get_all_states,
            database::get_lesson_content,
            database::get_user,
            database::get_adventurer_bonuses,
            database::calculate_xp_with_bonus,
            database::update_progress,
            database::get_database_path,
            database::update_user_profile,
            // New module-based curriculum commands
            database::get_modules_for_state,
            database::get_module_content,
            database::update_level_progress,
            database::reset_user_progress,
            database::get_recommended_modules,
            // The Sabi Codex - Encyclopedia commands
            database::get_all_encyclopedia_entries,
            database::get_encyclopedia_by_category,
            database::get_encyclopedia_by_state,
            database::get_encyclopedia_entry,
            database::mark_encyclopedia_read,
            database::toggle_encyclopedia_bookmark,
            database::unlock_encyclopedia_entry,
            database::get_codex_stats,
            database::search_encyclopedia,
            database::get_bookmarked_entries,
            // Avatar & Character commands
            database::get_user_avatar,
            database::update_user_avatar,
            database::get_avatar_items,
            database::unlock_avatar_item,
            database::has_created_character,
            // Quest system commands
            database::get_quests,
            database::start_quest,
            database::claim_quest_rewards,
            database::get_cultural_guide,
            // Artifact & Collectibles commands
            database::get_artifacts,
            database::get_collection_stats,
            database::toggle_artifact_favorite,
            database::mark_artifact_viewed,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

