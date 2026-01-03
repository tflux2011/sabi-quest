// Tauri commands for database operations
// These commands are exposed to the React frontend via invoke()

use super::{DatabaseError, DatabaseState, models::*, schema, seed_curriculum};
use serde::{Serialize, Deserialize};
use tauri::State as TauriState;
use rand::seq::SliceRandom;
use rand::thread_rng;

/// Initialize the database and create tables
/// This should be called once when the app starts
#[tauri::command]
pub async fn init_database(
    db: TauriState<'_, DatabaseState>,
) -> Result<String, DatabaseError> {
    let conn = db.connection.lock();
    
    // Verify tables exist
    schema::create_tables(&conn)?;
    
    // Check if we need to seed
    let is_seeded = schema::is_database_seeded(&conn)?;
    
    if is_seeded {
        Ok("Database already initialized".to_string())
    } else {
        Ok("Database initialized - ready for seeding".to_string())
    }
}

/// Seed the database with initial data (Lagos and Abuja with comprehensive curriculum)
#[tauri::command]
pub async fn seed_database(
    db: TauriState<'_, DatabaseState>,
) -> Result<String, DatabaseError> {
    let conn = db.connection.lock();
    
    // Check if curriculum is already seeded (use new module table)
    if seed_curriculum::is_curriculum_seeded(&conn)? {
        return Ok("Database already seeded with curriculum".to_string());
    }
    
    // Seed the comprehensive curriculum
    seed_curriculum::seed_curriculum(&conn)?;
    
    log::info!("Database seeded with comprehensive curriculum");
    Ok("Database seeded with Abuja (Social Studies) and Lagos (Math & Logic) modules".to_string())
}

/// Get all states with their progress for the current user
#[tauri::command]
pub async fn get_all_states(
    db: TauriState<'_, DatabaseState>,
    user_id: i64,
) -> Result<Vec<StateWithProgress>, DatabaseError> {
    let conn = db.connection.lock();
    
    let mut stmt = conn.prepare(
        r#"
        SELECT 
            s.id, s.name, s.region, s.zone, s.unlock_level, s.landmark_name, s.landmark_image, s.description, s.fun_fact,
            up.stars, up.is_completed, up.best_score, up.attempts, up.last_played_at,
            (SELECT COUNT(*) FROM lessons WHERE state_id = s.id) as lessons_count,
            (SELECT COUNT(*) FROM modules WHERE state_id = s.id) as modules_count,
            (SELECT MAX(current_level) FROM users WHERE id = ?1) >= s.unlock_level as is_unlocked
        FROM states s
        LEFT JOIN user_progress up ON s.id = up.state_id AND up.user_id = ?1
        ORDER BY s.unlock_level, s.name
        "#,
    ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    let states = stmt.query_map([user_id], |row| {
        let game_state = GameState {
            id: row.get(0)?,
            name: row.get(1)?,
            region: row.get(2)?,
            zone: row.get(3)?,
            unlock_level: row.get(4)?,
            landmark_name: row.get(5)?,
            landmark_image: row.get(6)?,
            description: row.get(7)?,
            fun_fact: row.get(8)?,
        };
        
        let progress = if row.get::<_, Option<i32>>(9)?.is_some() {
            Some(UserProgress {
                user_id,
                state_id: game_state.id.clone(),
                stars: row.get(9)?,
                is_completed: row.get::<_, i32>(10)? != 0,
                best_score: row.get(11)?,
                attempts: row.get(12)?,
                last_played_at: row.get(13)?,
            })
        } else {
            None
        };
        
        Ok(StateWithProgress {
            state: game_state,
            progress,
            is_unlocked: row.get::<_, i32>(16)? != 0,
            lessons_count: row.get(14)?,
            modules_count: row.get(15)?,
        })
    }).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    states.collect::<Result<Vec<_>, _>>()
        .map_err(|e| DatabaseError::QueryError(e.to_string()))
}

/// Get lesson content with questions for a specific state (LEGACY - for backward compatibility)
/// Note: This now returns empty questions array - use get_module_content for the new curriculum
#[tauri::command]
pub async fn get_lesson_content(
    db: TauriState<'_, DatabaseState>,
    state_id: String,
) -> Result<Vec<LessonContent>, DatabaseError> {
    let conn = db.connection.lock();
    
    // Get the state name
    let state_name: String = conn.query_row(
        "SELECT name FROM states WHERE id = ?1",
        [&state_id],
        |row| row.get(0),
    ).map_err(|e| DatabaseError::QueryError(format!("State not found: {}", e)))?;
    
    // Get all lessons for this state (legacy)
    let mut lesson_stmt = conn.prepare(
        "SELECT id, state_id, subject, title, content_md, difficulty, estimated_time 
         FROM lessons WHERE state_id = ?1"
    ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    let lessons: Vec<Lesson> = lesson_stmt.query_map([&state_id], |row| {
        Ok(Lesson {
            id: row.get(0)?,
            state_id: row.get(1)?,
            subject: row.get(2)?,
            title: row.get(3)?,
            content_md: row.get(4)?,
            difficulty: row.get(5)?,
            estimated_time: row.get(6)?,
        })
    })
    .map_err(|e| DatabaseError::QueryError(e.to_string()))?
    .collect::<Result<Vec<_>, _>>()
    .map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    // Return lessons with empty questions (use get_module_content for curriculum)
    let result: Vec<LessonContent> = lessons.into_iter().map(|lesson| {
        LessonContent {
            lesson,
            questions: vec![],
            state_name: state_name.clone(),
        }
    }).collect();
    
    Ok(result)
}

/// Get user profile by ID
#[tauri::command]
pub async fn get_user(
    db: TauriState<'_, DatabaseState>,
    user_id: i64,
) -> Result<User, DatabaseError> {
    let conn = db.connection.lock();
    
    conn.query_row(
        "SELECT id, display_name, avatar_json, birth_year, education_level, total_xp, current_level, cowrie_shells, streak_days, last_login_at, created_at 
         FROM users WHERE id = ?1",
        [user_id],
        |row| {
            let avatar_json: String = row.get(2)?;
            let avatar: AvatarConfig = serde_json::from_str(&avatar_json)
                .unwrap_or_else(|_| AvatarConfig::default_avatar());
            
            Ok(User {
                id: row.get(0)?,
                display_name: row.get(1)?,
                avatar,
                birth_year: row.get(3)?,
                education_level: row.get(4)?,
                total_xp: row.get(5)?,
                current_level: row.get(6)?,
                cowrie_shells: row.get(7)?,
                streak_days: row.get(8)?,
                last_login_at: row.get(9)?,
                created_at: row.get(10)?,
            })
        },
    ).map_err(|e| DatabaseError::QueryError(format!("User not found: {}", e)))
}

/// Update user profile with onboarding information
#[tauri::command]
pub async fn update_user_profile(
    db: TauriState<'_, DatabaseState>,
    user_id: i64,
    display_name: String,
    birth_year: Option<i32>,
    education_level: Option<String>,
) -> Result<User, DatabaseError> {
    // Validate education level if provided
    if let Some(ref level) = education_level {
        let valid_levels = ["primary_lower", "primary_upper", "jss", "sss"];
        if !valid_levels.contains(&level.as_str()) {
            return Err(DatabaseError::QueryError(format!("Invalid education level: {}", level)));
        }
    }
    
    // Validate birth year if provided (reasonable range: 2000-2020)
    if let Some(year) = birth_year {
        if year < 2000 || year > 2020 {
            return Err(DatabaseError::QueryError(format!("Invalid birth year: {}", year)));
        }
    }
    
    // Scope the connection lock
    {
        let conn = db.connection.lock();
        conn.execute(
            "UPDATE users SET display_name = ?1, birth_year = ?2, education_level = ?3 WHERE id = ?4",
            rusqlite::params![display_name, birth_year, education_level, user_id],
        ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    }
    
    // Return updated user
    get_user(db, user_id).await
}

/// Update user progress after completing a quiz
#[tauri::command]
pub async fn update_progress(
    db: TauriState<'_, DatabaseState>,
    user_id: i64,
    state_id: String,
    score: i32,
    total_questions: i32,
) -> Result<QuizResult, DatabaseError> {
    let conn = db.connection.lock();
    
    // Calculate stars based on score percentage
    let percentage = (score as f64 / total_questions as f64) * 100.0;
    let stars = if percentage >= 90.0 { 3 } 
                else if percentage >= 70.0 { 2 } 
                else if percentage >= 50.0 { 1 } 
                else { 0 };
    
    // Calculate XP earned (base XP per correct answer)
    let xp_earned = (score * 10) as i64;
    
    // Check current best score
    let current_best: Option<i32> = conn.query_row(
        "SELECT best_score FROM user_progress WHERE user_id = ?1 AND state_id = ?2",
        [&user_id.to_string(), &state_id],
        |row| row.get(0),
    ).ok();
    
    let is_new_best = current_best.map(|best| score > best).unwrap_or(true);
    
    // Update or insert progress
    conn.execute(
        r#"
        INSERT INTO user_progress (user_id, state_id, stars, is_completed, best_score, attempts, last_played_at)
        VALUES (?1, ?2, ?3, ?4, ?5, 1, datetime('now'))
        ON CONFLICT(user_id, state_id) DO UPDATE SET
            stars = MAX(stars, ?3),
            is_completed = CASE WHEN ?3 > 0 THEN 1 ELSE is_completed END,
            best_score = MAX(best_score, ?5),
            attempts = attempts + 1,
            last_played_at = datetime('now')
        "#,
        rusqlite::params![user_id, state_id, stars, if stars > 0 { 1 } else { 0 }, score],
    ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    // Update user XP
    conn.execute(
        "UPDATE users SET total_xp = total_xp + ?1 WHERE id = ?2",
        [xp_earned, user_id],
    ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    // Check for level up (every 100 XP = 1 level)
    conn.execute(
        "UPDATE users SET current_level = (total_xp / 100) + 1 WHERE id = ?1",
        [user_id],
    ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    Ok(QuizResult {
        correct_answers: score,
        total_questions,
        xp_earned,
        stars_earned: stars,
        is_new_best,
        items_unlocked: vec![], // TODO: Implement item unlocking logic
    })
}

/// Get the database file path (for debugging purposes)
#[tauri::command]
pub async fn get_database_path(
    db: TauriState<'_, DatabaseState>,
) -> Result<String, DatabaseError> {
    Ok(db.db_path.to_string_lossy().to_string())
}

// ============================================================
// NEW MODULE-BASED CURRICULUM COMMANDS
// ============================================================

/// Get all modules for a specific state with user progress
#[tauri::command]
pub async fn get_modules_for_state(
    db: TauriState<'_, DatabaseState>,
    state_id: String,
    user_id: i64,
) -> Result<Vec<ModuleWithProgress>, DatabaseError> {
    log::info!("get_modules_for_state called with state_id: {}, user_id: {}", state_id, user_id);
    let conn = db.connection.lock();
    
    let mut stmt = conn.prepare(
        r#"
        SELECT 
            m.id, m.state_id, m.subject, m.title, m.description, m.required_level, m.total_xp, m.estimated_time, m.icon,
            mc.did_you_know, mc.fun_fact, mc.intro_text, mc.historical_note, mc.intro_image_url, mc.intro_video_url,
            ump.current_level_id, ump.is_completed, ump.stars, ump.total_xp_earned, ump.best_score, ump.attempts, ump.last_played_at,
            (SELECT current_level FROM users WHERE id = ?2) >= m.required_level as is_unlocked
        FROM modules m
        LEFT JOIN module_context mc ON m.id = mc.module_id
        LEFT JOIN user_module_progress ump ON m.id = ump.module_id AND ump.user_id = ?2
        WHERE m.state_id = ?1
        ORDER BY m.required_level, m.title
        "#
    ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    let modules = stmt.query_map([&state_id, &user_id.to_string()], |row| {
        let module = Module {
            id: row.get(0)?,
            state_id: row.get(1)?,
            subject: row.get(2)?,
            title: row.get(3)?,
            description: row.get(4)?,
            required_level: row.get(5)?,
            total_xp: row.get(6)?,
            estimated_time: row.get(7)?,
            icon: row.get(8)?,
        };
        
        let context = if row.get::<_, Option<String>>(9)?.is_some() {
            Some(ModuleContext {
                module_id: module.id.clone(),
                did_you_know: row.get(9)?,
                fun_fact: row.get(10)?,
                intro_text: row.get(11)?,
                historical_note: row.get(12)?,
                intro_image_url: row.get(13)?,
                intro_video_url: row.get(14)?,
            })
        } else {
            None
        };
        
        let progress = if row.get::<_, Option<i32>>(16)?.is_some() {
            Some(UserModuleProgress {
                user_id,
                module_id: module.id.clone(),
                current_level_id: row.get(15)?,
                is_completed: row.get::<_, i32>(16)? != 0,
                stars: row.get(17)?,
                total_xp_earned: row.get(18)?,
                best_score: row.get(19)?,
                attempts: row.get(20)?,
                last_played_at: row.get(21)?,
            })
        } else {
            None
        };
        
        Ok(ModuleWithProgress {
            module,
            context,
            progress,
            is_unlocked: row.get::<_, i32>(22)? != 0,
        })
    }).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    let result: Vec<ModuleWithProgress> = modules.collect::<Result<Vec<_>, _>>()
        .map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    log::info!("get_modules_for_state returning {} modules", result.len());
    Ok(result)
}

/// Get full module content with levels and questions
#[tauri::command]
pub async fn get_module_content(
    db: TauriState<'_, DatabaseState>,
    module_id: String,
) -> Result<ModuleWithContent, DatabaseError> {
    let conn = db.connection.lock();
    
    // Get module
    let module: Module = conn.query_row(
        "SELECT id, state_id, subject, title, description, required_level, total_xp, estimated_time, icon FROM modules WHERE id = ?1",
        [&module_id],
        |row| Ok(Module {
            id: row.get(0)?,
            state_id: row.get(1)?,
            subject: row.get(2)?,
            title: row.get(3)?,
            description: row.get(4)?,
            required_level: row.get(5)?,
            total_xp: row.get(6)?,
            estimated_time: row.get(7)?,
            icon: row.get(8)?,
        })
    ).map_err(|e| DatabaseError::QueryError(format!("Module not found: {}", e)))?;
    
    // Get context
    let context: Option<ModuleContext> = conn.query_row(
        "SELECT module_id, did_you_know, fun_fact, intro_text, historical_note, intro_image_url, intro_video_url FROM module_context WHERE module_id = ?1",
        [&module_id],
        |row| Ok(ModuleContext {
            module_id: row.get(0)?,
            did_you_know: row.get(1)?,
            fun_fact: row.get(2)?,
            intro_text: row.get(3)?,
            historical_note: row.get(4)?,
            intro_image_url: row.get(5)?,
            intro_video_url: row.get(6)?,
        })
    ).ok();
    
    // Get levels with questions
    let mut level_stmt = conn.prepare(
        "SELECT id, module_id, title, difficulty, order_index, xp_reward, unlock_item_id FROM levels WHERE module_id = ?1 ORDER BY order_index"
    ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    let levels: Vec<Level> = level_stmt.query_map([&module_id], |row| {
        Ok(Level {
            id: row.get(0)?,
            module_id: row.get(1)?,
            title: row.get(2)?,
            difficulty: row.get(3)?,
            order_index: row.get(4)?,
            xp_reward: row.get(5)?,
            unlock_item_id: row.get(6)?,
        })
    }).map_err(|e| DatabaseError::QueryError(e.to_string()))?
    .collect::<Result<Vec<_>, _>>()
    .map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    // Get questions for each level
    let mut levels_with_questions = Vec::new();
    let mut rng = thread_rng();
    
    for level in levels {
        let mut question_stmt = conn.prepare(
            "SELECT id, level_id, question_text, question_type, options_json, correct_answer, xp_reward, explanation, hint, image_url, order_index 
             FROM questions WHERE level_id = ?1 ORDER BY order_index"
        ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
        
        let mut questions: Vec<Question> = question_stmt.query_map([&level.id], |row| {
            let options_json: Option<String> = row.get(4)?;
            let options: Option<Vec<QuestionOption>> = options_json.and_then(|json| {
                serde_json::from_str(&json).ok()
            });
            
            Ok(Question {
                id: row.get(0)?,
                level_id: row.get(1)?,
                question_text: row.get(2)?,
                question_type: row.get(3)?,
                options,
                correct_answer: row.get(5)?,
                xp_reward: row.get(6)?,
                explanation: row.get(7)?,
                hint: row.get(8)?,
                image_url: row.get(9)?,
                order_index: row.get(10)?,
            })
        }).map_err(|e| DatabaseError::QueryError(e.to_string()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| DatabaseError::QueryError(e.to_string()))?;
        
        // Randomize question order for each session
        questions.shuffle(&mut rng);
        
        // Also randomize the order of multiple choice options (but preserve correct_answer mapping)
        for question in &mut questions {
            if let Some(ref mut options) = question.options {
                options.shuffle(&mut rng);
            }
        }
        
        levels_with_questions.push(LevelWithQuestions { level, questions });
    }
    
    Ok(ModuleWithContent {
        module,
        context,
        levels: levels_with_questions,
    })
}

/// Update user progress after completing a level
#[tauri::command]
pub async fn update_level_progress(
    db: TauriState<'_, DatabaseState>,
    user_id: i64,
    level_id: String,
    correct_answers: i32,
    total_questions: i32,
) -> Result<LevelResult, DatabaseError> {
    let conn = db.connection.lock();
    
    // Get level info including state_id from module
    let (module_id, xp_reward, unlock_item_id, state_id): (String, i64, Option<String>, String) = conn.query_row(
        "SELECT l.module_id, l.xp_reward, l.unlock_item_id, m.state_id 
         FROM levels l 
         JOIN modules m ON l.module_id = m.id 
         WHERE l.id = ?1",
        [&level_id],
        |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
    ).map_err(|e| DatabaseError::QueryError(format!("Level not found: {}", e)))?;
    
    // Calculate score and stars
    let percentage = (correct_answers as f64 / total_questions as f64) * 100.0;
    let stars = if percentage >= 90.0 { 3 } 
                else if percentage >= 70.0 { 2 } 
                else if percentage >= 50.0 { 1 } 
                else { 0 };
    
    let passed = stars >= 1;
    let xp_earned = if passed { 
        ((xp_reward as f64) * (percentage / 100.0)) as i64 
    } else { 
        (correct_answers * 5) as i64 // Small XP even for failing
    };
    
    // Check current best score
    let current_best: Option<i32> = conn.query_row(
        "SELECT score FROM user_level_progress WHERE user_id = ?1 AND level_id = ?2",
        [&user_id.to_string(), &level_id],
        |row| row.get(0)
    ).ok();
    
    let is_new_best = current_best.map(|best| correct_answers > best).unwrap_or(true);
    
    // Update level progress (using actual schema columns: is_completed, score, xp_earned, completed_at)
    conn.execute(
        r#"
        INSERT INTO user_level_progress (user_id, level_id, is_completed, score, xp_earned, completed_at)
        VALUES (?1, ?2, ?3, ?4, ?5, datetime('now'))
        ON CONFLICT(user_id, level_id) DO UPDATE SET
            is_completed = CASE WHEN ?3 THEN 1 ELSE is_completed END,
            score = MAX(score, ?4),
            xp_earned = MAX(xp_earned, ?5),
            completed_at = CASE WHEN ?3 THEN datetime('now') ELSE completed_at END
        "#,
        rusqlite::params![user_id, level_id, passed, correct_answers, xp_earned]
    ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    // Update module progress
    let levels_completed: i32 = conn.query_row(
        "SELECT COUNT(*) FROM user_level_progress ulp 
         JOIN levels l ON ulp.level_id = l.id 
         WHERE ulp.user_id = ?1 AND l.module_id = ?2 AND ulp.is_completed = 1",
        [&user_id.to_string(), &module_id],
        |row| row.get(0)
    ).unwrap_or(0);
    
    let total_levels: i32 = conn.query_row(
        "SELECT COUNT(*) FROM levels WHERE module_id = ?1",
        [&module_id],
        |row| row.get(0)
    ).unwrap_or(0);
    
    let module_completed = levels_completed >= total_levels;
    
    // Update module progress (using actual schema: current_level_id, is_completed, stars, total_xp_earned, best_score, attempts, last_played_at)
    conn.execute(
        r#"
        INSERT INTO user_module_progress (user_id, module_id, current_level_id, is_completed, stars, total_xp_earned, best_score, attempts, last_played_at)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 1, datetime('now'))
        ON CONFLICT(user_id, module_id) DO UPDATE SET
            current_level_id = ?3,
            is_completed = CASE WHEN ?4 THEN 1 ELSE is_completed END,
            stars = MAX(stars, ?5),
            total_xp_earned = total_xp_earned + CASE WHEN ?8 THEN ?6 ELSE 0 END,
            best_score = MAX(best_score, ?7),
            attempts = attempts + 1,
            last_played_at = datetime('now')
        "#,
        rusqlite::params![user_id, module_id, level_id, module_completed, stars, xp_earned, correct_answers, is_new_best]
    ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    // Update user_progress for the state (this is what shows on state cards)
    // Use the stars from THIS level completion (already calculated above)
    // This ensures the state shows the most recent performance
    conn.execute(
        r#"
        INSERT INTO user_progress (user_id, state_id, stars, is_completed, best_score, attempts, last_played_at)
        VALUES (?1, ?2, ?3, ?4, ?5, 1, datetime('now'))
        ON CONFLICT(user_id, state_id) DO UPDATE SET
            stars = MAX(stars, ?3),
            is_completed = CASE WHEN ?4 THEN 1 ELSE is_completed END,
            best_score = MAX(best_score, ?5),
            attempts = attempts + 1,
            last_played_at = datetime('now')
        "#,
        rusqlite::params![user_id, state_id, stars, module_completed, correct_answers]
    ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    // Update user XP (only if new best score)
    if is_new_best {
        conn.execute(
            "UPDATE users SET total_xp = total_xp + ?1 WHERE id = ?2",
            [xp_earned, user_id]
        ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
        
        // Check for level up
        conn.execute(
            "UPDATE users SET current_level = (total_xp / 100) + 1 WHERE id = ?1",
            [user_id]
        ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    }
    
    // Handle item unlock
    let item_unlocked = if passed && unlock_item_id.is_some() {
        unlock_item_id
    } else {
        None
    };
    
    Ok(LevelResult {
        passed,
        correct_answers,
        total_questions,
        stars_earned: stars,
        xp_earned,
        is_new_best,
        item_unlocked,
    })
}

/// Reset user progress (for testing)
#[tauri::command]
pub async fn reset_user_progress(
    db: TauriState<'_, DatabaseState>,
    user_id: i64,
) -> Result<String, DatabaseError> {
    let conn = db.connection.lock();
    
    conn.execute_batch(&format!(
        r#"
        DELETE FROM user_progress WHERE user_id = {user_id};
        DELETE FROM user_module_progress WHERE user_id = {user_id};
        DELETE FROM user_level_progress WHERE user_id = {user_id};
        UPDATE users SET total_xp = 0, current_level = 1, cowrie_shells = 100 WHERE id = {user_id};
        "#
    )).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    Ok("User progress reset successfully".to_string())
}

// ============================================
// THE SABI CODEX - Encyclopedia Commands
// ============================================

/// Get all encyclopedia entries with user progress
/// Returns entries with their unlock status and reading progress
#[tauri::command]
pub async fn get_all_encyclopedia_entries(
    db: TauriState<'_, DatabaseState>,
    user_id: i64,
) -> Result<Vec<EncyclopediaEntryWithProgress>, DatabaseError> {
    let conn = db.connection.lock();
    
    let mut stmt = conn.prepare(
        r#"
        SELECT 
            e.id, e.category, e.title, e.subtitle, e.content_md, e.summary,
            e.image_url, e.audio_url, e.associated_state, e.tier,
            e.unlock_condition, e.xp_reward, e.reading_time, e.tags,
            ue.is_unlocked, ue.is_read, ue.is_bookmarked,
            ue.unlocked_at, ue.first_read_at, ue.read_count
        FROM encyclopedia_entries e
        LEFT JOIN user_encyclopedia ue ON e.id = ue.entry_id AND ue.user_id = ?1
        ORDER BY e.category, e.title
        "#
    ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    let entries = stmt.query_map([user_id], |row| {
        let entry = EncyclopediaEntry {
            id: row.get(0)?,
            category: row.get(1)?,
            title: row.get(2)?,
            subtitle: row.get(3)?,
            content_md: row.get(4)?,
            summary: row.get(5)?,
            image_url: row.get(6)?,
            audio_url: row.get(7)?,
            associated_state: row.get(8)?,
            tier: row.get(9)?,
            unlock_condition: row.get(10)?,
            xp_reward: row.get(11)?,
            reading_time: row.get(12)?,
            tags: row.get(13)?,
        };
        
        let is_unlocked: Option<bool> = row.get(14).ok();
        let progress = if is_unlocked.is_some() {
            Some(UserEncyclopediaProgress {
                entry_id: entry.id.clone(),
                is_unlocked: row.get(14).unwrap_or(false),
                is_read: row.get(15).unwrap_or(false),
                is_bookmarked: row.get(16).unwrap_or(false),
                unlocked_at: row.get(17).ok(),
                first_read_at: row.get(18).ok(),
                read_count: row.get(19).unwrap_or(0),
            })
        } else {
            None
        };
        
        // Tier 1 entries are always accessible, Tier 2 requires unlock
        let is_accessible = entry.tier == 1 || progress.as_ref().map_or(false, |p| p.is_unlocked);
        
        Ok(EncyclopediaEntryWithProgress {
            entry,
            progress,
            is_accessible,
        })
    }).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    entries.collect::<Result<Vec<_>, _>>()
        .map_err(|e| DatabaseError::QueryError(e.to_string()))
}

/// Get encyclopedia entries by category
#[tauri::command]
pub async fn get_encyclopedia_by_category(
    db: TauriState<'_, DatabaseState>,
    user_id: i64,
    category: String,
) -> Result<Vec<EncyclopediaEntryWithProgress>, DatabaseError> {
    let conn = db.connection.lock();
    
    let mut stmt = conn.prepare(
        r#"
        SELECT 
            e.id, e.category, e.title, e.subtitle, e.content_md, e.summary,
            e.image_url, e.audio_url, e.associated_state, e.tier,
            e.unlock_condition, e.xp_reward, e.reading_time, e.tags,
            ue.is_unlocked, ue.is_read, ue.is_bookmarked,
            ue.unlocked_at, ue.first_read_at, ue.read_count
        FROM encyclopedia_entries e
        LEFT JOIN user_encyclopedia ue ON e.id = ue.entry_id AND ue.user_id = ?1
        WHERE e.category = ?2
        ORDER BY e.title
        "#
    ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    let entries = stmt.query_map(rusqlite::params![user_id, category], |row| {
        let entry = EncyclopediaEntry {
            id: row.get(0)?,
            category: row.get(1)?,
            title: row.get(2)?,
            subtitle: row.get(3)?,
            content_md: row.get(4)?,
            summary: row.get(5)?,
            image_url: row.get(6)?,
            audio_url: row.get(7)?,
            associated_state: row.get(8)?,
            tier: row.get(9)?,
            unlock_condition: row.get(10)?,
            xp_reward: row.get(11)?,
            reading_time: row.get(12)?,
            tags: row.get(13)?,
        };
        
        let is_unlocked: Option<bool> = row.get(14).ok();
        let progress = if is_unlocked.is_some() {
            Some(UserEncyclopediaProgress {
                entry_id: entry.id.clone(),
                is_unlocked: row.get(14).unwrap_or(false),
                is_read: row.get(15).unwrap_or(false),
                is_bookmarked: row.get(16).unwrap_or(false),
                unlocked_at: row.get(17).ok(),
                first_read_at: row.get(18).ok(),
                read_count: row.get(19).unwrap_or(0),
            })
        } else {
            None
        };
        
        let is_accessible = entry.tier == 1 || progress.as_ref().map_or(false, |p| p.is_unlocked);
        
        Ok(EncyclopediaEntryWithProgress {
            entry,
            progress,
            is_accessible,
        })
    }).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    entries.collect::<Result<Vec<_>, _>>()
        .map_err(|e| DatabaseError::QueryError(e.to_string()))
}

/// Get encyclopedia entries for a specific state (Local Context)
#[tauri::command]
pub async fn get_encyclopedia_by_state(
    db: TauriState<'_, DatabaseState>,
    user_id: i64,
    state_id: String,
) -> Result<Vec<EncyclopediaEntryWithProgress>, DatabaseError> {
    let conn = db.connection.lock();
    
    let mut stmt = conn.prepare(
        r#"
        SELECT 
            e.id, e.category, e.title, e.subtitle, e.content_md, e.summary,
            e.image_url, e.audio_url, e.associated_state, e.tier,
            e.unlock_condition, e.xp_reward, e.reading_time, e.tags,
            ue.is_unlocked, ue.is_read, ue.is_bookmarked,
            ue.unlocked_at, ue.first_read_at, ue.read_count
        FROM encyclopedia_entries e
        LEFT JOIN user_encyclopedia ue ON e.id = ue.entry_id AND ue.user_id = ?1
        WHERE e.associated_state = ?2
        ORDER BY e.category, e.title
        "#
    ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    let entries = stmt.query_map(rusqlite::params![user_id, state_id], |row| {
        let entry = EncyclopediaEntry {
            id: row.get(0)?,
            category: row.get(1)?,
            title: row.get(2)?,
            subtitle: row.get(3)?,
            content_md: row.get(4)?,
            summary: row.get(5)?,
            image_url: row.get(6)?,
            audio_url: row.get(7)?,
            associated_state: row.get(8)?,
            tier: row.get(9)?,
            unlock_condition: row.get(10)?,
            xp_reward: row.get(11)?,
            reading_time: row.get(12)?,
            tags: row.get(13)?,
        };
        
        let is_unlocked: Option<bool> = row.get(14).ok();
        let progress = if is_unlocked.is_some() {
            Some(UserEncyclopediaProgress {
                entry_id: entry.id.clone(),
                is_unlocked: row.get(14).unwrap_or(false),
                is_read: row.get(15).unwrap_or(false),
                is_bookmarked: row.get(16).unwrap_or(false),
                unlocked_at: row.get(17).ok(),
                first_read_at: row.get(18).ok(),
                read_count: row.get(19).unwrap_or(0),
            })
        } else {
            None
        };
        
        let is_accessible = entry.tier == 1 || progress.as_ref().map_or(false, |p| p.is_unlocked);
        
        Ok(EncyclopediaEntryWithProgress {
            entry,
            progress,
            is_accessible,
        })
    }).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    entries.collect::<Result<Vec<_>, _>>()
        .map_err(|e| DatabaseError::QueryError(e.to_string()))
}

/// Get a single encyclopedia entry by ID with full content
#[tauri::command]
pub async fn get_encyclopedia_entry(
    db: TauriState<'_, DatabaseState>,
    user_id: i64,
    entry_id: String,
) -> Result<EncyclopediaEntryWithProgress, DatabaseError> {
    let conn = db.connection.lock();
    
    let result = conn.query_row(
        r#"
        SELECT 
            e.id, e.category, e.title, e.subtitle, e.content_md, e.summary,
            e.image_url, e.audio_url, e.associated_state, e.tier,
            e.unlock_condition, e.xp_reward, e.reading_time, e.tags,
            ue.is_unlocked, ue.is_read, ue.is_bookmarked,
            ue.unlocked_at, ue.first_read_at, ue.read_count
        FROM encyclopedia_entries e
        LEFT JOIN user_encyclopedia ue ON e.id = ue.entry_id AND ue.user_id = ?1
        WHERE e.id = ?2
        "#,
        rusqlite::params![user_id, entry_id],
        |row| {
            let entry = EncyclopediaEntry {
                id: row.get(0)?,
                category: row.get(1)?,
                title: row.get(2)?,
                subtitle: row.get(3)?,
                content_md: row.get(4)?,
                summary: row.get(5)?,
                image_url: row.get(6)?,
                audio_url: row.get(7)?,
                associated_state: row.get(8)?,
                tier: row.get(9)?,
                unlock_condition: row.get(10)?,
                xp_reward: row.get(11)?,
                reading_time: row.get(12)?,
                tags: row.get(13)?,
            };
            
            let is_unlocked: Option<bool> = row.get(14).ok();
            let progress = if is_unlocked.is_some() {
                Some(UserEncyclopediaProgress {
                    entry_id: entry.id.clone(),
                    is_unlocked: row.get(14).unwrap_or(false),
                    is_read: row.get(15).unwrap_or(false),
                    is_bookmarked: row.get(16).unwrap_or(false),
                    unlocked_at: row.get(17).ok(),
                    first_read_at: row.get(18).ok(),
                    read_count: row.get(19).unwrap_or(0),
                })
            } else {
                None
            };
            
            let is_accessible = entry.tier == 1 || progress.as_ref().map_or(false, |p| p.is_unlocked);
            
            Ok(EncyclopediaEntryWithProgress {
                entry,
                progress,
                is_accessible,
            })
        }
    ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    Ok(result)
}

/// Mark an encyclopedia entry as read and award XP
#[tauri::command]
pub async fn mark_encyclopedia_read(
    db: TauriState<'_, DatabaseState>,
    user_id: i64,
    entry_id: String,
) -> Result<MarkReadResult, DatabaseError> {
    let conn = db.connection.lock();
    
    // Check if entry exists and get XP reward
    let (tier, xp_reward): (i32, i32) = conn.query_row(
        "SELECT tier, xp_reward FROM encyclopedia_entries WHERE id = ?1",
        [&entry_id],
        |row| Ok((row.get(0)?, row.get(1)?))
    ).map_err(|_| DatabaseError::QueryError("Entry not found".to_string()))?;
    
    // Check if already read
    let already_read: bool = conn.query_row(
        "SELECT is_read FROM user_encyclopedia WHERE user_id = ?1 AND entry_id = ?2",
        rusqlite::params![user_id, entry_id],
        |row| row.get(0)
    ).unwrap_or(false);
    
    // Insert or update user_encyclopedia record
    conn.execute(
        r#"
        INSERT INTO user_encyclopedia (user_id, entry_id, is_unlocked, is_read, is_bookmarked, first_read_at, read_count)
        VALUES (?1, ?2, 1, 1, 0, datetime('now'), 1)
        ON CONFLICT(user_id, entry_id) DO UPDATE SET
            is_read = 1,
            first_read_at = COALESCE(first_read_at, datetime('now')),
            read_count = read_count + 1
        "#,
        rusqlite::params![user_id, entry_id]
    ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    // Award XP if first time reading
    let xp_awarded = if !already_read {
        conn.execute(
            "UPDATE users SET total_xp = total_xp + ?1 WHERE id = ?2",
            rusqlite::params![xp_reward, user_id]
        ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
        xp_reward
    } else {
        0
    };
    
    Ok(MarkReadResult {
        success: true,
        xp_awarded,
        was_first_read: !already_read,
    })
}

/// Result of marking an entry as read
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkReadResult {
    pub success: bool,
    pub xp_awarded: i32,
    pub was_first_read: bool,
}

/// Toggle bookmark status for an encyclopedia entry
#[tauri::command]
pub async fn toggle_encyclopedia_bookmark(
    db: TauriState<'_, DatabaseState>,
    user_id: i64,
    entry_id: String,
) -> Result<bool, DatabaseError> {
    let conn = db.connection.lock();
    
    // Insert or toggle bookmark
    conn.execute(
        r#"
        INSERT INTO user_encyclopedia (user_id, entry_id, is_unlocked, is_read, is_bookmarked)
        VALUES (?1, ?2, 0, 0, 1)
        ON CONFLICT(user_id, entry_id) DO UPDATE SET
            is_bookmarked = NOT is_bookmarked
        "#,
        rusqlite::params![user_id, entry_id]
    ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    // Return new bookmark state
    let is_bookmarked: bool = conn.query_row(
        "SELECT is_bookmarked FROM user_encyclopedia WHERE user_id = ?1 AND entry_id = ?2",
        rusqlite::params![user_id, entry_id],
        |row| row.get(0)
    ).unwrap_or(false);
    
    Ok(is_bookmarked)
}

/// Unlock an encyclopedia entry (manual unlock or via reward)
#[tauri::command]
pub async fn unlock_encyclopedia_entry(
    db: TauriState<'_, DatabaseState>,
    user_id: i64,
    entry_id: String,
) -> Result<UnlockResult, DatabaseError> {
    let conn = db.connection.lock();
    
    // Check if entry exists
    let tier: i32 = conn.query_row(
        "SELECT tier FROM encyclopedia_entries WHERE id = ?1",
        [&entry_id],
        |row| row.get(0)
    ).map_err(|_| DatabaseError::QueryError("Entry not found".to_string()))?;
    
    // Tier 1 entries are always accessible
    if tier == 1 {
        return Ok(UnlockResult {
            success: true,
            already_unlocked: true,
            message: "This entry is always available".to_string(),
        });
    }
    
    // Check if already unlocked
    let already_unlocked: bool = conn.query_row(
        "SELECT is_unlocked FROM user_encyclopedia WHERE user_id = ?1 AND entry_id = ?2",
        rusqlite::params![user_id, entry_id],
        |row| row.get(0)
    ).unwrap_or(false);
    
    if already_unlocked {
        return Ok(UnlockResult {
            success: true,
            already_unlocked: true,
            message: "Entry was already unlocked".to_string(),
        });
    }
    
    // Unlock the entry
    conn.execute(
        r#"
        INSERT INTO user_encyclopedia (user_id, entry_id, is_unlocked, is_read, is_bookmarked, unlocked_at)
        VALUES (?1, ?2, 1, 0, 0, datetime('now'))
        ON CONFLICT(user_id, entry_id) DO UPDATE SET
            is_unlocked = 1,
            unlocked_at = COALESCE(unlocked_at, datetime('now'))
        "#,
        rusqlite::params![user_id, entry_id]
    ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    Ok(UnlockResult {
        success: true,
        already_unlocked: false,
        message: "Entry unlocked successfully!".to_string(),
    })
}

/// Result of unlocking an entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlockResult {
    pub success: bool,
    pub already_unlocked: bool,
    pub message: String,
}

/// Get Codex statistics for the user
#[tauri::command]
pub async fn get_codex_stats(
    db: TauriState<'_, DatabaseState>,
    user_id: i64,
) -> Result<CodexStats, DatabaseError> {
    let conn = db.connection.lock();
    
    // Get total entries
    let total_entries: i32 = conn.query_row(
        "SELECT COUNT(*) FROM encyclopedia_entries",
        [],
        |row| row.get(0)
    ).unwrap_or(0);
    
    // Get unlocked entries (tier 1 + user unlocked tier 2)
    let unlocked_entries: i32 = conn.query_row(
        r#"
        SELECT COUNT(*) FROM encyclopedia_entries e
        LEFT JOIN user_encyclopedia ue ON e.id = ue.entry_id AND ue.user_id = ?1
        WHERE e.tier = 1 OR ue.is_unlocked = 1
        "#,
        [user_id],
        |row| row.get(0)
    ).unwrap_or(0);
    
    // Get read entries
    let read_entries: i32 = conn.query_row(
        "SELECT COUNT(*) FROM user_encyclopedia WHERE user_id = ?1 AND is_read = 1",
        [user_id],
        |row| row.get(0)
    ).unwrap_or(0);
    
    // Get bookmarked entries
    let bookmarked_entries: i32 = conn.query_row(
        "SELECT COUNT(*) FROM user_encyclopedia WHERE user_id = ?1 AND is_bookmarked = 1",
        [user_id],
        |row| row.get(0)
    ).unwrap_or(0);
    
    // Get counts by category
    let mut stmt = conn.prepare(
        r#"
        SELECT 
            e.category,
            COUNT(*) as total,
            SUM(CASE WHEN e.tier = 1 OR ue.is_unlocked = 1 THEN 1 ELSE 0 END) as unlocked,
            SUM(CASE WHEN ue.is_read = 1 THEN 1 ELSE 0 END) as read
        FROM encyclopedia_entries e
        LEFT JOIN user_encyclopedia ue ON e.id = ue.entry_id AND ue.user_id = ?1
        GROUP BY e.category
        ORDER BY e.category
        "#
    ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    let entries_by_category = stmt.query_map([user_id], |row| {
        Ok(CategoryCount {
            category: row.get(0)?,
            total: row.get(1)?,
            unlocked: row.get(2)?,
            read: row.get(3)?,
        })
    }).map_err(|e| DatabaseError::QueryError(e.to_string()))?
    .collect::<Result<Vec<_>, _>>()
    .map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    Ok(CodexStats {
        total_entries,
        unlocked_entries,
        read_entries,
        bookmarked_entries,
        entries_by_category,
    })
}

/// Search encyclopedia entries by title or content
#[tauri::command]
pub async fn search_encyclopedia(
    db: TauriState<'_, DatabaseState>,
    user_id: i64,
    query: String,
) -> Result<Vec<EncyclopediaEntryWithProgress>, DatabaseError> {
    let conn = db.connection.lock();
    let search_pattern = format!("%{}%", query);
    
    let mut stmt = conn.prepare(
        r#"
        SELECT 
            e.id, e.category, e.title, e.subtitle, e.content_md, e.summary,
            e.image_url, e.audio_url, e.associated_state, e.tier,
            e.unlock_condition, e.xp_reward, e.reading_time, e.tags,
            ue.is_unlocked, ue.is_read, ue.is_bookmarked,
            ue.unlocked_at, ue.first_read_at, ue.read_count
        FROM encyclopedia_entries e
        LEFT JOIN user_encyclopedia ue ON e.id = ue.entry_id AND ue.user_id = ?1
        WHERE e.title LIKE ?2 OR e.summary LIKE ?2 OR e.tags LIKE ?2
        ORDER BY e.category, e.title
        LIMIT 50
        "#
    ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    let entries = stmt.query_map(rusqlite::params![user_id, search_pattern], |row| {
        let entry = EncyclopediaEntry {
            id: row.get(0)?,
            category: row.get(1)?,
            title: row.get(2)?,
            subtitle: row.get(3)?,
            content_md: row.get(4)?,
            summary: row.get(5)?,
            image_url: row.get(6)?,
            audio_url: row.get(7)?,
            associated_state: row.get(8)?,
            tier: row.get(9)?,
            unlock_condition: row.get(10)?,
            xp_reward: row.get(11)?,
            reading_time: row.get(12)?,
            tags: row.get(13)?,
        };
        
        let is_unlocked: Option<bool> = row.get(14).ok();
        let progress = if is_unlocked.is_some() {
            Some(UserEncyclopediaProgress {
                entry_id: entry.id.clone(),
                is_unlocked: row.get(14).unwrap_or(false),
                is_read: row.get(15).unwrap_or(false),
                is_bookmarked: row.get(16).unwrap_or(false),
                unlocked_at: row.get(17).ok(),
                first_read_at: row.get(18).ok(),
                read_count: row.get(19).unwrap_or(0),
            })
        } else {
            None
        };
        
        let is_accessible = entry.tier == 1 || progress.as_ref().map_or(false, |p| p.is_unlocked);
        
        Ok(EncyclopediaEntryWithProgress {
            entry,
            progress,
            is_accessible,
        })
    }).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    entries.collect::<Result<Vec<_>, _>>()
        .map_err(|e| DatabaseError::QueryError(e.to_string()))
}

/// Get bookmarked encyclopedia entries
#[tauri::command]
pub async fn get_bookmarked_entries(
    db: TauriState<'_, DatabaseState>,
    user_id: i64,
) -> Result<Vec<EncyclopediaEntryWithProgress>, DatabaseError> {
    let conn = db.connection.lock();
    
    let mut stmt = conn.prepare(
        r#"
        SELECT 
            e.id, e.category, e.title, e.subtitle, e.content_md, e.summary,
            e.image_url, e.audio_url, e.associated_state, e.tier,
            e.unlock_condition, e.xp_reward, e.reading_time, e.tags,
            ue.is_unlocked, ue.is_read, ue.is_bookmarked,
            ue.unlocked_at, ue.first_read_at, ue.read_count
        FROM encyclopedia_entries e
        JOIN user_encyclopedia ue ON e.id = ue.entry_id AND ue.user_id = ?1
        WHERE ue.is_bookmarked = 1
        ORDER BY e.category, e.title
        "#
    ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    let entries = stmt.query_map([user_id], |row| {
        let entry = EncyclopediaEntry {
            id: row.get(0)?,
            category: row.get(1)?,
            title: row.get(2)?,
            subtitle: row.get(3)?,
            content_md: row.get(4)?,
            summary: row.get(5)?,
            image_url: row.get(6)?,
            audio_url: row.get(7)?,
            associated_state: row.get(8)?,
            tier: row.get(9)?,
            unlock_condition: row.get(10)?,
            xp_reward: row.get(11)?,
            reading_time: row.get(12)?,
            tags: row.get(13)?,
        };
        
        let progress = Some(UserEncyclopediaProgress {
            entry_id: entry.id.clone(),
            is_unlocked: row.get(14).unwrap_or(false),
            is_read: row.get(15).unwrap_or(false),
            is_bookmarked: row.get(16).unwrap_or(false),
            unlocked_at: row.get(17).ok(),
            first_read_at: row.get(18).ok(),
            read_count: row.get(19).unwrap_or(0),
        });
        
        let is_accessible = entry.tier == 1 || progress.as_ref().map_or(false, |p| p.is_unlocked);
        
        Ok(EncyclopediaEntryWithProgress {
            entry,
            progress,
            is_accessible,
        })
    }).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    entries.collect::<Result<Vec<_>, _>>()
        .map_err(|e| DatabaseError::QueryError(e.to_string()))
}

// =====================================================
// AVATAR & CHARACTER CUSTOMIZATION COMMANDS
// =====================================================

/// Get or create user's avatar configuration
#[tauri::command]
pub async fn get_user_avatar(
    db: TauriState<'_, DatabaseState>,
    user_id: i64,
) -> Result<UserAvatar, DatabaseError> {
    let conn = db.connection.lock();
    
    let result = conn.query_row(
        r#"
        SELECT user_id, skin_tone, hairstyle, outfit, accessory, background, character_name
        FROM user_avatar WHERE user_id = ?1
        "#,
        [user_id],
        |row| {
            Ok(UserAvatar {
                user_id: row.get(0)?,
                skin_tone: row.get(1)?,
                hairstyle: row.get(2)?,
                outfit: row.get(3)?,
                accessory: row.get(4)?,
                background: row.get(5)?,
                character_name: row.get(6)?,
            })
        },
    );
    
    match result {
        Ok(avatar) => Ok(avatar),
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            // Create default avatar
            conn.execute(
                r#"
                INSERT INTO user_avatar (user_id, skin_tone, hairstyle, outfit, background)
                VALUES (?1, 'skin_3', 'hair_1', 'outfit_school', 'bg_default')
                "#,
                [user_id],
            ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
            
            Ok(UserAvatar::default())
        }
        Err(e) => Err(DatabaseError::QueryError(e.to_string())),
    }
}

/// Update user's avatar configuration
#[tauri::command]
pub async fn update_user_avatar(
    db: TauriState<'_, DatabaseState>,
    user_id: i64,
    skin_tone: Option<String>,
    hairstyle: Option<String>,
    outfit: Option<String>,
    accessory: Option<String>,
    background: Option<String>,
    character_name: Option<String>,
) -> Result<UserAvatar, DatabaseError> {
    let conn = db.connection.lock();
    
    // Ensure avatar exists
    conn.execute(
        r#"
        INSERT OR IGNORE INTO user_avatar (user_id) VALUES (?1)
        "#,
        [user_id],
    ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    // Build dynamic update query
    let mut updates = Vec::new();
    let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();
    
    if let Some(ref v) = skin_tone { updates.push("skin_tone = ?"); params.push(Box::new(v.clone())); }
    if let Some(ref v) = hairstyle { updates.push("hairstyle = ?"); params.push(Box::new(v.clone())); }
    if let Some(ref v) = outfit { updates.push("outfit = ?"); params.push(Box::new(v.clone())); }
    if accessory.is_some() { updates.push("accessory = ?"); params.push(Box::new(accessory.clone())); }
    if let Some(ref v) = background { updates.push("background = ?"); params.push(Box::new(v.clone())); }
    if character_name.is_some() { updates.push("character_name = ?"); params.push(Box::new(character_name.clone())); }
    
    if !updates.is_empty() {
        updates.push("updated_at = datetime('now')");
        let sql = format!(
            "UPDATE user_avatar SET {} WHERE user_id = ?",
            updates.join(", ")
        );
        params.push(Box::new(user_id));
        
        let params_ref: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();
        conn.execute(&sql, params_ref.as_slice())
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    }
    
    // Return updated avatar
    get_user_avatar_sync(&conn, user_id)
}

fn get_user_avatar_sync(conn: &rusqlite::Connection, user_id: i64) -> Result<UserAvatar, DatabaseError> {
    conn.query_row(
        r#"
        SELECT user_id, skin_tone, hairstyle, outfit, accessory, background, character_name
        FROM user_avatar WHERE user_id = ?1
        "#,
        [user_id],
        |row| {
            Ok(UserAvatar {
                user_id: row.get(0)?,
                skin_tone: row.get(1)?,
                hairstyle: row.get(2)?,
                outfit: row.get(3)?,
                accessory: row.get(4)?,
                background: row.get(5)?,
                character_name: row.get(6)?,
            })
        },
    ).map_err(|e| DatabaseError::QueryError(e.to_string()))
}

/// Get all avatar items by category
#[tauri::command]
pub async fn get_avatar_items(
    db: TauriState<'_, DatabaseState>,
    user_id: i64,
    category: Option<String>,
) -> Result<Vec<AvatarItemWithStatus>, DatabaseError> {
    let conn = db.connection.lock();
    
    // Get user's cowrie shells for affordability check
    let cowries: i64 = conn.query_row(
        "SELECT cowrie_shells FROM users WHERE id = ?1",
        [user_id],
        |row| row.get(0),
    ).unwrap_or(0);
    
    let sql = match category {
        Some(_) => r#"
            SELECT ai.id, ai.category, ai.name, ai.description, ai.image_key,
                   ai.rarity, ai.unlock_cost, ai.unlock_condition, ai.is_premium, ai.sort_order,
                   CASE WHEN uai.item_id IS NOT NULL THEN 1 ELSE 0 END as is_unlocked,
                   COALESCE(uai.is_equipped, 0) as is_equipped
            FROM avatar_items ai
            LEFT JOIN user_avatar_items uai ON ai.id = uai.item_id AND uai.user_id = ?1
            WHERE ai.category = ?2
            ORDER BY ai.sort_order, ai.name
        "#,
        None => r#"
            SELECT ai.id, ai.category, ai.name, ai.description, ai.image_key,
                   ai.rarity, ai.unlock_cost, ai.unlock_condition, ai.is_premium, ai.sort_order,
                   CASE WHEN uai.item_id IS NOT NULL THEN 1 ELSE 0 END as is_unlocked,
                   COALESCE(uai.is_equipped, 0) as is_equipped
            FROM avatar_items ai
            LEFT JOIN user_avatar_items uai ON ai.id = uai.item_id AND uai.user_id = ?1
            ORDER BY ai.category, ai.sort_order, ai.name
        "#,
    };
    
    let mut stmt = conn.prepare(sql).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    let items: Vec<AvatarItemWithStatus> = if let Some(ref cat) = category {
        stmt.query_map(rusqlite::params![user_id, cat], |row| {
            map_avatar_item_row(row, cowries)
        }).map_err(|e| DatabaseError::QueryError(e.to_string()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| DatabaseError::QueryError(e.to_string()))?
    } else {
        stmt.query_map([user_id], |row| {
            map_avatar_item_row(row, cowries)
        }).map_err(|e| DatabaseError::QueryError(e.to_string()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| DatabaseError::QueryError(e.to_string()))?
    };
    
    Ok(items)
}

fn map_avatar_item_row(row: &rusqlite::Row, cowries: i64) -> rusqlite::Result<AvatarItemWithStatus> {
    let unlock_cost: i32 = row.get(6)?;
    let is_unlocked: bool = row.get::<_, i32>(10)? != 0;
    
    Ok(AvatarItemWithStatus {
        item: AvatarItem {
            id: row.get(0)?,
            category: row.get(1)?,
            name: row.get(2)?,
            description: row.get(3)?,
            image_key: row.get(4)?,
            rarity: row.get(5)?,
            unlock_cost,
            unlock_condition: row.get(7)?,
            is_premium: row.get::<_, i32>(8)? != 0,
            sort_order: row.get(9)?,
        },
        is_unlocked,
        is_equipped: row.get::<_, i32>(11)? != 0,
        can_afford: cowries >= unlock_cost as i64,
    })
}

/// Unlock an avatar item (purchase with cowries or meet condition)
#[tauri::command]
pub async fn unlock_avatar_item(
    db: TauriState<'_, DatabaseState>,
    user_id: i64,
    item_id: String,
) -> Result<AvatarItemWithStatus, DatabaseError> {
    let conn = db.connection.lock();
    
    // Get item details
    let (unlock_cost, _rarity): (i32, String) = conn.query_row(
        "SELECT unlock_cost, rarity FROM avatar_items WHERE id = ?1",
        [&item_id],
        |row| Ok((row.get(0)?, row.get(1)?)),
    ).map_err(|e| DatabaseError::QueryError(format!("Item not found: {}", e)))?;
    
    // Check if already unlocked
    let already_unlocked: bool = conn.query_row(
        "SELECT COUNT(*) > 0 FROM user_avatar_items WHERE user_id = ?1 AND item_id = ?2",
        rusqlite::params![user_id, &item_id],
        |row| row.get(0),
    ).unwrap_or(false);
    
    if already_unlocked {
        return Err(DatabaseError::QueryError("Item already unlocked".to_string()));
    }
    
    // Check if user can afford (starter items are free)
    if unlock_cost > 0 {
        let cowries: i64 = conn.query_row(
            "SELECT cowrie_shells FROM users WHERE id = ?1",
            [user_id],
            |row| row.get(0),
        ).unwrap_or(0);
        
        if cowries < unlock_cost as i64 {
            return Err(DatabaseError::QueryError("Not enough cowrie shells".to_string()));
        }
        
        // Deduct cowries
        conn.execute(
            "UPDATE users SET cowrie_shells = cowrie_shells - ?1 WHERE id = ?2",
            rusqlite::params![unlock_cost, user_id],
        ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    }
    
    // Unlock the item
    conn.execute(
        "INSERT INTO user_avatar_items (user_id, item_id) VALUES (?1, ?2)",
        rusqlite::params![user_id, &item_id],
    ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    // Return updated item status
    let cowries: i64 = conn.query_row(
        "SELECT cowrie_shells FROM users WHERE id = ?1",
        [user_id],
        |row| row.get(0),
    ).unwrap_or(0);
    
    conn.query_row(
        r#"
        SELECT ai.id, ai.category, ai.name, ai.description, ai.image_key,
               ai.rarity, ai.unlock_cost, ai.unlock_condition, ai.is_premium, ai.sort_order
        FROM avatar_items ai WHERE ai.id = ?1
        "#,
        [&item_id],
        |row| {
            let unlock_cost: i32 = row.get(6)?;
            Ok(AvatarItemWithStatus {
                item: AvatarItem {
                    id: row.get(0)?,
                    category: row.get(1)?,
                    name: row.get(2)?,
                    description: row.get(3)?,
                    image_key: row.get(4)?,
                    rarity: row.get(5)?,
                    unlock_cost,
                    unlock_condition: row.get(7)?,
                    is_premium: row.get::<_, i32>(8)? != 0,
                    sort_order: row.get(9)?,
                },
                is_unlocked: true,
                is_equipped: false,
                can_afford: cowries >= unlock_cost as i64,
            })
        },
    ).map_err(|e| DatabaseError::QueryError(e.to_string()))
}

/// Check if user has completed character creation
#[tauri::command]
pub async fn has_created_character(
    db: TauriState<'_, DatabaseState>,
    user_id: i64,
) -> Result<bool, DatabaseError> {
    let conn = db.connection.lock();
    
    let has_name: bool = conn.query_row(
        "SELECT character_name IS NOT NULL FROM user_avatar WHERE user_id = ?1",
        [user_id],
        |row| row.get(0),
    ).unwrap_or(false);
    
    Ok(has_name)
}

// =====================================================
// QUEST SYSTEM COMMANDS
// =====================================================

/// Get all quests for a user with progress
#[tauri::command]
pub async fn get_quests(
    db: TauriState<'_, DatabaseState>,
    user_id: i64,
    quest_type: Option<String>,
    state_id: Option<String>,
) -> Result<Vec<QuestWithProgress>, DatabaseError> {
    let conn = db.connection.lock();
    
    // Get user level
    let user_level: i32 = conn.query_row(
        "SELECT current_level FROM users WHERE id = ?1",
        [user_id],
        |row| row.get(0),
    ).unwrap_or(1);
    
    let sql = r#"
        SELECT q.id, q.title, q.description, q.quest_type, q.category,
               q.state_id, q.guide_id, q.required_level, q.prerequisite_quest_id,
               q.requirements_json, q.xp_reward, q.cowrie_reward, q.artifact_reward_id,
               q.item_rewards_json, q.intro_dialogue, q.progress_dialogue, q.completion_dialogue,
               q.icon, q.sort_order, q.is_repeatable, q.cooldown_hours,
               uq.status, uq.progress_json, uq.started_at, uq.completed_at, uq.claimed_at, uq.completion_count,
               cg.id as guide_id, cg.name as guide_name, cg.title as guide_title,
               cg.avatar_image, cg.greeting, cg.catchphrase
        FROM quests q
        LEFT JOIN user_quests uq ON q.id = uq.quest_id AND uq.user_id = ?1
        LEFT JOIN cultural_guides cg ON q.guide_id = cg.id
        WHERE (?2 IS NULL OR q.quest_type = ?2)
          AND (?3 IS NULL OR q.state_id = ?3)
        ORDER BY q.quest_type, q.sort_order, q.title
    "#;
    
    let mut stmt = conn.prepare(sql).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    let quests = stmt.query_map(rusqlite::params![user_id, quest_type, state_id], |row| {
        let quest = Quest {
            id: row.get(0)?,
            title: row.get(1)?,
            description: row.get(2)?,
            quest_type: row.get(3)?,
            category: row.get(4)?,
            state_id: row.get(5)?,
            guide_id: row.get(6)?,
            required_level: row.get(7)?,
            prerequisite_quest_id: row.get(8)?,
            requirements_json: row.get(9)?,
            xp_reward: row.get(10)?,
            cowrie_reward: row.get(11)?,
            artifact_reward_id: row.get(12)?,
            item_rewards_json: row.get(13)?,
            intro_dialogue: row.get(14)?,
            progress_dialogue: row.get(15)?,
            completion_dialogue: row.get(16)?,
            icon: row.get(17)?,
            sort_order: row.get(18)?,
            is_repeatable: row.get::<_, i32>(19)? != 0,
            cooldown_hours: row.get(20)?,
        };
        
        let user_progress = if row.get::<_, Option<String>>(21)?.is_some() {
            Some(UserQuest {
                quest_id: quest.id.clone(),
                status: row.get(21)?,
                progress_json: row.get(22)?,
                started_at: row.get(23)?,
                completed_at: row.get(24)?,
                claimed_at: row.get(25)?,
                completion_count: row.get(26)?,
            })
        } else {
            None
        };
        
        let guide = if row.get::<_, Option<String>>(27)?.is_some() {
            Some(CulturalGuide {
                id: row.get(27)?,
                name: row.get(28)?,
                title: row.get(29)?,
                description: None,
                personality: None,
                avatar_image: row.get(30)?,
                state_id: None,
                region: None,
                greeting: row.get(31)?,
                catchphrase: row.get(32)?,
                voice_style: None,
            })
        } else {
            None
        };
        
        // Parse requirements
        let requirements: Vec<QuestRequirement> = quest.requirements_json
            .as_ref()
            .and_then(|json| serde_json::from_str(json).ok())
            .unwrap_or_default();
        
        // Determine availability
        let is_available = user_level >= quest.required_level;
        
        // Calculate progress
        let progress_percent = if requirements.is_empty() {
            0
        } else {
            let total: i32 = requirements.iter().map(|r| r.count).sum();
            let current: i32 = requirements.iter().filter_map(|r| r.current).sum();
            if total > 0 { (current * 100 / total) as i32 } else { 0 }
        };
        
        Ok(QuestWithProgress {
            quest,
            guide,
            user_progress,
            requirements,
            is_available,
            progress_percent,
        })
    }).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    quests.collect::<Result<Vec<_>, _>>()
        .map_err(|e| DatabaseError::QueryError(e.to_string()))
}

/// Start a quest
#[tauri::command]
pub async fn start_quest(
    db: TauriState<'_, DatabaseState>,
    user_id: i64,
    quest_id: String,
) -> Result<UserQuest, DatabaseError> {
    let conn = db.connection.lock();
    
    // Check quest exists and get requirements
    let requirements_json: Option<String> = conn.query_row(
        "SELECT requirements_json FROM quests WHERE id = ?1",
        [&quest_id],
        |row| row.get(0),
    ).map_err(|e| DatabaseError::QueryError(format!("Quest not found: {}", e)))?;
    
    // Initialize progress JSON with current counts at 0
    let initial_progress = requirements_json.as_ref().map(|json| {
        let mut reqs: Vec<QuestRequirement> = serde_json::from_str(json).unwrap_or_default();
        for req in &mut reqs {
            req.current = Some(0);
        }
        serde_json::to_string(&reqs).unwrap_or_default()
    });
    
    // Insert or update quest progress
    conn.execute(
        r#"
        INSERT INTO user_quests (user_id, quest_id, status, progress_json, started_at)
        VALUES (?1, ?2, 'active', ?3, datetime('now'))
        ON CONFLICT(user_id, quest_id) DO UPDATE SET
            status = 'active',
            progress_json = COALESCE(excluded.progress_json, user_quests.progress_json),
            started_at = COALESCE(user_quests.started_at, datetime('now'))
        "#,
        rusqlite::params![user_id, &quest_id, initial_progress],
    ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    // Return updated quest
    conn.query_row(
        r#"
        SELECT quest_id, status, progress_json, started_at, completed_at, claimed_at, completion_count
        FROM user_quests WHERE user_id = ?1 AND quest_id = ?2
        "#,
        rusqlite::params![user_id, &quest_id],
        |row| {
            Ok(UserQuest {
                quest_id: row.get(0)?,
                status: row.get(1)?,
                progress_json: row.get(2)?,
                started_at: row.get(3)?,
                completed_at: row.get(4)?,
                claimed_at: row.get(5)?,
                completion_count: row.get(6)?,
            })
        },
    ).map_err(|e| DatabaseError::QueryError(e.to_string()))
}

/// Claim quest rewards
#[tauri::command]
pub async fn claim_quest_rewards(
    db: TauriState<'_, DatabaseState>,
    user_id: i64,
    quest_id: String,
) -> Result<QuestRewardResult, DatabaseError> {
    let conn = db.connection.lock();
    
    // Get quest and verify it's completed but not claimed
    let (xp_reward, cowrie_reward, artifact_id, status): (i32, i32, Option<String>, String) = conn.query_row(
        r#"
        SELECT q.xp_reward, q.cowrie_reward, q.artifact_reward_id, uq.status
        FROM quests q
        JOIN user_quests uq ON q.id = uq.quest_id
        WHERE q.id = ?1 AND uq.user_id = ?2
        "#,
        rusqlite::params![&quest_id, user_id],
        |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
    ).map_err(|e| DatabaseError::QueryError(format!("Quest not found: {}", e)))?;
    
    if status != "completed" {
        return Err(DatabaseError::QueryError("Quest not completed yet".to_string()));
    }
    
    // Award XP and cowries
    conn.execute(
        "UPDATE users SET total_xp = total_xp + ?1, cowrie_shells = cowrie_shells + ?2 WHERE id = ?3",
        rusqlite::params![xp_reward, cowrie_reward, user_id],
    ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    // Award artifact if any
    let artifact_name = if let Some(ref art_id) = artifact_id {
        conn.execute(
            r#"
            INSERT OR IGNORE INTO user_artifacts (user_id, artifact_id, obtain_method)
            VALUES (?1, ?2, 'quest')
            "#,
            rusqlite::params![user_id, art_id],
        ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
        
        conn.query_row(
            "SELECT name FROM artifacts WHERE id = ?1",
            [art_id],
            |row| row.get(0),
        ).ok()
    } else {
        None
    };
    
    // Mark as claimed
    conn.execute(
        r#"
        UPDATE user_quests SET status = 'claimed', claimed_at = datetime('now'),
            completion_count = completion_count + 1
        WHERE user_id = ?1 AND quest_id = ?2
        "#,
        rusqlite::params![user_id, &quest_id],
    ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    Ok(QuestRewardResult {
        xp_earned: xp_reward,
        cowries_earned: cowrie_reward,
        artifact_unlocked: artifact_name,
    })
}

/// Quest reward result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestRewardResult {
    pub xp_earned: i32,
    pub cowries_earned: i32,
    pub artifact_unlocked: Option<String>,
}

/// Get cultural guide for a state
#[tauri::command]
pub async fn get_cultural_guide(
    db: TauriState<'_, DatabaseState>,
    state_id: String,
) -> Result<Option<CulturalGuide>, DatabaseError> {
    let conn = db.connection.lock();
    
    let result = conn.query_row(
        r#"
        SELECT id, name, title, description, personality, avatar_image,
               state_id, region, greeting, catchphrase, voice_style
        FROM cultural_guides WHERE state_id = ?1
        "#,
        [&state_id],
        |row| {
            Ok(CulturalGuide {
                id: row.get(0)?,
                name: row.get(1)?,
                title: row.get(2)?,
                description: row.get(3)?,
                personality: row.get(4)?,
                avatar_image: row.get(5)?,
                state_id: row.get(6)?,
                region: row.get(7)?,
                greeting: row.get(8)?,
                catchphrase: row.get(9)?,
                voice_style: row.get(10)?,
            })
        },
    );
    
    match result {
        Ok(guide) => Ok(Some(guide)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(DatabaseError::QueryError(e.to_string())),
    }
}

// =====================================================
// ARTIFACT & COLLECTIBLES COMMANDS
// =====================================================

/// Get all artifacts with collection status
#[tauri::command]
pub async fn get_artifacts(
    db: TauriState<'_, DatabaseState>,
    user_id: i64,
    category: Option<String>,
    state_id: Option<String>,
    collected_only: Option<bool>,
) -> Result<Vec<ArtifactWithStatus>, DatabaseError> {
    let conn = db.connection.lock();
    
    let sql = r#"
        SELECT a.id, a.name, a.description, a.long_description, a.category,
               a.state_id, a.region, a.image_url, a.thumbnail_url, a.model_3d_url,
               a.color_primary, a.color_secondary, a.rarity, a.historical_period,
               a.cultural_significance, a.unlock_type, a.unlock_source_id, a.cowrie_cost, a.sort_order,
               ua.obtained_at, ua.obtain_method, ua.is_favorite, ua.is_new, ua.display_slot,
               s.name as state_name
        FROM artifacts a
        LEFT JOIN user_artifacts ua ON a.id = ua.artifact_id AND ua.user_id = ?1
        LEFT JOIN states s ON a.state_id = s.id
        WHERE (?2 IS NULL OR a.category = ?2)
          AND (?3 IS NULL OR a.state_id = ?3)
          AND (?4 = 0 OR ua.artifact_id IS NOT NULL)
        ORDER BY a.rarity DESC, a.sort_order, a.name
    "#;
    
    let collected_filter = collected_only.unwrap_or(false) as i32;
    
    let mut stmt = conn.prepare(sql).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    let artifacts = stmt.query_map(
        rusqlite::params![user_id, category, state_id, collected_filter],
        |row| {
            let artifact = Artifact {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                long_description: row.get(3)?,
                category: row.get(4)?,
                state_id: row.get(5)?,
                region: row.get(6)?,
                image_url: row.get(7)?,
                thumbnail_url: row.get(8)?,
                model_3d_url: row.get(9)?,
                color_primary: row.get(10)?,
                color_secondary: row.get(11)?,
                rarity: row.get(12)?,
                historical_period: row.get(13)?,
                cultural_significance: row.get(14)?,
                unlock_type: row.get(15)?,
                unlock_source_id: row.get(16)?,
                cowrie_cost: row.get(17)?,
                sort_order: row.get(18)?,
            };
            
            let user_data = if row.get::<_, Option<String>>(19)?.is_some() {
                Some(UserArtifact {
                    artifact_id: artifact.id.clone(),
                    obtained_at: row.get(19)?,
                    obtain_method: row.get(20)?,
                    is_favorite: row.get::<_, i32>(21)? != 0,
                    is_new: row.get::<_, i32>(22)? != 0,
                    display_slot: row.get(23)?,
                })
            } else {
                None
            };
            
            let is_collected = user_data.is_some();
            let state_name: Option<String> = row.get(24)?;
            
            Ok(ArtifactWithStatus {
                artifact,
                is_collected,
                user_data,
                can_unlock: false, // Will be computed based on requirements
                state_name,
            })
        },
    ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    artifacts.collect::<Result<Vec<_>, _>>()
        .map_err(|e| DatabaseError::QueryError(e.to_string()))
}

/// Get collection statistics
#[tauri::command]
pub async fn get_collection_stats(
    db: TauriState<'_, DatabaseState>,
    user_id: i64,
) -> Result<CollectionStats, DatabaseError> {
    let conn = db.connection.lock();
    
    let total_artifacts: i32 = conn.query_row(
        "SELECT COUNT(*) FROM artifacts",
        [],
        |row| row.get(0),
    ).unwrap_or(0);
    
    let collected_count: i32 = conn.query_row(
        "SELECT COUNT(*) FROM user_artifacts WHERE user_id = ?1",
        [user_id],
        |row| row.get(0),
    ).unwrap_or(0);
    
    let favorites_count: i32 = conn.query_row(
        "SELECT COUNT(*) FROM user_artifacts WHERE user_id = ?1 AND is_favorite = 1",
        [user_id],
        |row| row.get(0),
    ).unwrap_or(0);
    
    // Get by rarity
    let mut rarity_stmt = conn.prepare(
        r#"
        SELECT a.rarity, COUNT(*) as total,
               SUM(CASE WHEN ua.artifact_id IS NOT NULL THEN 1 ELSE 0 END) as collected
        FROM artifacts a
        LEFT JOIN user_artifacts ua ON a.id = ua.artifact_id AND ua.user_id = ?1
        GROUP BY a.rarity
        "#
    ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    let by_rarity: Vec<RarityCount> = rarity_stmt.query_map([user_id], |row| {
        Ok(RarityCount {
            rarity: row.get(0)?,
            total: row.get(1)?,
            collected: row.get(2)?,
        })
    }).map_err(|e| DatabaseError::QueryError(e.to_string()))?
    .filter_map(|r| r.ok())
    .collect();
    
    // Get by category
    let mut cat_stmt = conn.prepare(
        r#"
        SELECT a.category, COUNT(*) as total,
               SUM(CASE WHEN ua.artifact_id IS NOT NULL THEN 1 ELSE 0 END) as collected
        FROM artifacts a
        LEFT JOIN user_artifacts ua ON a.id = ua.artifact_id AND ua.user_id = ?1
        GROUP BY a.category
        "#
    ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    let by_category: Vec<CategoryArtifactCount> = cat_stmt.query_map([user_id], |row| {
        Ok(CategoryArtifactCount {
            category: row.get(0)?,
            total: row.get(1)?,
            collected: row.get(2)?,
        })
    }).map_err(|e| DatabaseError::QueryError(e.to_string()))?
    .filter_map(|r| r.ok())
    .collect();
    
    // Get by state
    let mut state_stmt = conn.prepare(
        r#"
        SELECT a.state_id, s.name, COUNT(*) as total,
               SUM(CASE WHEN ua.artifact_id IS NOT NULL THEN 1 ELSE 0 END) as collected
        FROM artifacts a
        LEFT JOIN user_artifacts ua ON a.id = ua.artifact_id AND ua.user_id = ?1
        LEFT JOIN states s ON a.state_id = s.id
        WHERE a.state_id IS NOT NULL
        GROUP BY a.state_id
        "#
    ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    let by_state: Vec<StateArtifactCount> = state_stmt.query_map([user_id], |row| {
        Ok(StateArtifactCount {
            state_id: row.get(0)?,
            state_name: row.get::<_, Option<String>>(1)?.unwrap_or_default(),
            total: row.get(2)?,
            collected: row.get(3)?,
        })
    }).map_err(|e| DatabaseError::QueryError(e.to_string()))?
    .filter_map(|r| r.ok())
    .collect();
    
    // Get newest artifact
    let newest_artifact = conn.query_row(
        r#"
        SELECT a.id, a.name, a.description, a.long_description, a.category,
               a.state_id, a.region, a.image_url, a.thumbnail_url, a.model_3d_url,
               a.color_primary, a.color_secondary, a.rarity, a.historical_period,
               a.cultural_significance, a.unlock_type, a.unlock_source_id, a.cowrie_cost, a.sort_order
        FROM artifacts a
        JOIN user_artifacts ua ON a.id = ua.artifact_id
        WHERE ua.user_id = ?1
        ORDER BY ua.obtained_at DESC
        LIMIT 1
        "#,
        [user_id],
        |row| {
            Ok(Artifact {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                long_description: row.get(3)?,
                category: row.get(4)?,
                state_id: row.get(5)?,
                region: row.get(6)?,
                image_url: row.get(7)?,
                thumbnail_url: row.get(8)?,
                model_3d_url: row.get(9)?,
                color_primary: row.get(10)?,
                color_secondary: row.get(11)?,
                rarity: row.get(12)?,
                historical_period: row.get(13)?,
                cultural_significance: row.get(14)?,
                unlock_type: row.get(15)?,
                unlock_source_id: row.get(16)?,
                cowrie_cost: row.get(17)?,
                sort_order: row.get(18)?,
            })
        },
    ).ok();
    
    Ok(CollectionStats {
        total_artifacts,
        collected_count,
        by_rarity,
        by_category,
        by_state,
        favorites_count,
        newest_artifact,
    })
}

/// Toggle artifact favorite status
#[tauri::command]
pub async fn toggle_artifact_favorite(
    db: TauriState<'_, DatabaseState>,
    user_id: i64,
    artifact_id: String,
) -> Result<bool, DatabaseError> {
    let conn = db.connection.lock();
    
    conn.execute(
        r#"
        UPDATE user_artifacts SET is_favorite = NOT is_favorite
        WHERE user_id = ?1 AND artifact_id = ?2
        "#,
        rusqlite::params![user_id, &artifact_id],
    ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    let is_favorite: bool = conn.query_row(
        "SELECT is_favorite FROM user_artifacts WHERE user_id = ?1 AND artifact_id = ?2",
        rusqlite::params![user_id, &artifact_id],
        |row| row.get(0),
    ).unwrap_or(false);
    
    Ok(is_favorite)
}

/// Mark artifact as viewed (no longer new)
#[tauri::command]
pub async fn mark_artifact_viewed(
    db: TauriState<'_, DatabaseState>,
    user_id: i64,
    artifact_id: String,
) -> Result<(), DatabaseError> {
    let conn = db.connection.lock();
    
    conn.execute(
        "UPDATE user_artifacts SET is_new = 0 WHERE user_id = ?1 AND artifact_id = ?2",
        rusqlite::params![user_id, &artifact_id],
    ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    Ok(())
}
