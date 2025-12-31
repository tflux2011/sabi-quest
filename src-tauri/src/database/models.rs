// Data models for Project Nigeria
// These structs represent database entities and are used for serialization

use serde::{Deserialize, Serialize};

// ============================================
// ZONE SYSTEM
// ============================================

/// Game zones that group states by learning theme
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum GameZone {
    Mind,       // South West: Logic, Math, Literature
    Heritage,   // North: Agriculture, History, Civics
    Innovation, // South East/South South: Science, Commerce, Industry
}

impl Default for GameZone {
    fn default() -> Self {
        GameZone::Heritage
    }
}

// ============================================
// STATE & LOCATION MODELS
// ============================================

/// Represents a Nigerian state in the game map
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub id: String,
    pub name: String,
    pub region: Option<String>,
    pub zone: Option<String>,
    pub unlock_level: i32,
    pub landmark_name: Option<String>,
    pub landmark_image: Option<String>,
    pub description: Option<String>,
    pub fun_fact: Option<String>,
}

// ============================================
// MODULE SYSTEM (Enhanced Lessons)
// ============================================

/// A learning module within a state (themed educational experience)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub id: String,
    pub state_id: String,
    pub subject: String,
    pub title: String,
    pub description: Option<String>,
    pub required_level: i32,
    pub total_xp: i32,
    pub estimated_time: Option<i32>,
    pub icon: Option<String>,
}

/// Rich "Encarta-style" context for a module
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleContext {
    pub module_id: String,
    pub did_you_know: Option<String>,
    pub fun_fact: Option<String>,
    pub intro_text: Option<String>,
    pub historical_note: Option<String>,
    pub intro_image_url: Option<String>,
    pub intro_video_url: Option<String>,
}

/// A level within a module (progressive difficulty)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Level {
    pub id: String,
    pub module_id: String,
    pub title: String,
    pub difficulty: String,
    pub order_index: i32,
    pub xp_reward: i32,
    pub unlock_item_id: Option<String>,
}

/// Question types supported by the quiz system
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum QuestionType {
    MultipleChoice,
    InputNumber,
    TrueFalse,
    FillBlank,
    DragDrop,
}

/// An option for multiple choice questions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionOption {
    pub id: String,
    pub text: String,
}

/// Represents a quiz question (enhanced version)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    pub id: String,
    pub level_id: String,
    pub question_text: String,
    pub question_type: String,
    pub options: Option<Vec<QuestionOption>>,
    pub correct_answer: String,
    pub xp_reward: i32,
    pub explanation: Option<String>,
    pub hint: Option<String>,
    pub image_url: Option<String>,
    pub order_index: i32,
}

/// Raw question from database before JSON parsing
#[derive(Debug, Clone)]
pub struct QuestionRow {
    pub id: String,
    pub level_id: String,
    pub question_text: String,
    pub question_type: String,
    pub options_json: Option<String>,
    pub correct_answer: String,
    pub xp_reward: i32,
    pub explanation: Option<String>,
    pub hint: Option<String>,
    pub image_url: Option<String>,
    pub order_index: i32,
}

impl QuestionRow {
    /// Converts the raw database row to a Question with parsed options
    pub fn into_question(self) -> Question {
        let options: Option<Vec<QuestionOption>> = self
            .options_json
            .and_then(|json| serde_json::from_str(&json).ok());
        
        Question {
            id: self.id,
            level_id: self.level_id,
            question_text: self.question_text,
            question_type: self.question_type,
            options,
            correct_answer: self.correct_answer,
            xp_reward: self.xp_reward,
            explanation: self.explanation,
            hint: self.hint,
            image_url: self.image_url,
            order_index: self.order_index,
        }
    }
}

// ============================================
// FULL MODULE CONTENT (for API responses)
// ============================================

/// Complete module data with context and levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleWithContent {
    pub module: Module,
    pub context: Option<ModuleContext>,
    pub levels: Vec<LevelWithQuestions>,
}

/// Module with progress for API response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleWithProgress {
    pub module: Module,
    pub context: Option<ModuleContext>,
    pub progress: Option<UserModuleProgress>,
    pub is_unlocked: bool,
}

/// Level with all its questions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelWithQuestions {
    pub level: Level,
    pub questions: Vec<Question>,
}

// ============================================
// LEGACY LESSON SYSTEM (backward compatibility)
// ============================================

/// Represents a lesson within a state (legacy)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lesson {
    pub id: String,
    pub state_id: String,
    pub subject: String,
    pub title: String,
    pub content_md: Option<String>,
    pub difficulty: i32,
    pub estimated_time: Option<i32>,
}

/// Legacy question with integer ID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegacyQuestion {
    pub id: i64,
    pub lesson_id: String,
    pub question_text: String,
    pub question_type: String,
    pub options: Vec<String>,
    pub correct_answer: String,
    pub xp_reward: i32,
    pub explanation: Option<String>,
}

// ============================================
// USER & PROGRESS MODELS
// ============================================

/// User progress for a specific state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProgress {
    pub user_id: i64,
    pub state_id: String,
    pub stars: i32,
    pub is_completed: bool,
    pub best_score: i32,
    pub attempts: i32,
    pub last_played_at: Option<String>,
}

/// User progress for a module
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserModuleProgress {
    pub user_id: i64,
    pub module_id: String,
    pub current_level_id: Option<String>,
    pub is_completed: bool,
    pub stars: i32,
    pub total_xp_earned: i32,
    pub best_score: i32,
    pub attempts: i32,
    pub last_played_at: Option<String>,
}

/// User progress for a level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserLevelProgress {
    pub user_id: i64,
    pub level_id: String,
    pub is_completed: bool,
    pub stars: i32,
    pub best_score: i32,
    pub attempts: i32,
    pub xp_earned: i32,
    pub last_played_at: Option<String>,
}

/// Inventory item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryItem {
    pub id: i64,
    pub user_id: i64,
    pub item_id: String,
    pub item_type: Option<String>,
    pub acquired_at: String,
}

/// Education level enum for Nigerian school system
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EducationLevel {
    PrimaryLower,  // Primary 1-3 (Ages 6-8)
    PrimaryUpper,  // Primary 4-6 (Ages 9-11)
    Jss,           // Junior Secondary School 1-3 (Ages 12-14)
    Sss,           // Senior Secondary School 1-3 (Ages 15-17)
}

impl EducationLevel {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "primary_lower" => Some(EducationLevel::PrimaryLower),
            "primary_upper" => Some(EducationLevel::PrimaryUpper),
            "jss" => Some(EducationLevel::Jss),
            "sss" => Some(EducationLevel::Sss),
            _ => None,
        }
    }
    
    pub fn as_str(&self) -> &'static str {
        match self {
            EducationLevel::PrimaryLower => "primary_lower",
            EducationLevel::PrimaryUpper => "primary_upper",
            EducationLevel::Jss => "jss",
            EducationLevel::Sss => "sss",
        }
    }
    
    /// Get the display name for this education level
    pub fn display_name(&self) -> &'static str {
        match self {
            EducationLevel::PrimaryLower => "Primary 1-3",
            EducationLevel::PrimaryUpper => "Primary 4-6",
            EducationLevel::Jss => "JSS 1-3",
            EducationLevel::Sss => "SS 1-3",
        }
    }
}

/// User profile with avatar customization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub display_name: String,
    pub avatar: AvatarConfig,
    pub birth_year: Option<i32>,
    pub education_level: Option<String>,
    pub total_xp: i64,
    pub current_level: i32,
    pub cowrie_shells: i64,
    pub streak_days: i32,
    pub last_login_at: Option<String>,
    pub created_at: String,
}

/// Avatar customization options as defined in PROJECT_CONTEXT.md
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AvatarConfig {
    pub skin: String,      // e.g., "tone_1" through "tone_5"
    pub head: String,      // Hair/head style
    pub top: String,       // Outfit top
    pub accessory: Option<String>, // Optional accessory
}

impl AvatarConfig {
    /// Creates a default avatar configuration
    pub fn default_avatar() -> Self {
        Self {
            skin: "tone_3".to_string(),
            head: "style_1".to_string(),
            top: "shirt_default".to_string(),
            accessory: None,
        }
    }
}

/// Achievement definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Achievement {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub icon_path: Option<String>,
    pub xp_reward: i32,
}

/// User's unlocked achievement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAchievement {
    pub achievement: Achievement,
    pub unlocked_at: String,
}

/// State with progress information for the map view
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateWithProgress {
    pub state: GameState,
    pub progress: Option<UserProgress>,
    pub is_unlocked: bool,
    pub lessons_count: i32,
    pub modules_count: i32,
}

/// Lesson content with questions for quiz mode (legacy)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LessonContent {
    pub lesson: Lesson,
    pub questions: Vec<LegacyQuestion>,
    pub state_name: String,
}

/// Quiz result after completing a lesson
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizResult {
    pub correct_answers: i32,
    pub total_questions: i32,
    pub xp_earned: i64,
    pub stars_earned: i32,
    pub is_new_best: bool,
    pub items_unlocked: Vec<String>,
}

/// Result after completing a level (new module system)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelResult {
    pub passed: bool,
    pub correct_answers: i32,
    pub total_questions: i32,
    pub stars_earned: i32,
    pub xp_earned: i64,
    pub is_new_best: bool,
    pub item_unlocked: Option<String>,
}

/// App settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub sound_enabled: bool,
    pub music_volume: f32,
    pub sfx_volume: f32,
    pub notifications_enabled: bool,
    pub language: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            sound_enabled: true,
            music_volume: 0.7,
            sfx_volume: 0.8,
            notifications_enabled: true,
            language: "en".to_string(),
        }
    }
}

// ============================================
// THE SABI CODEX - Encyclopedia System
// ============================================

/// Encyclopedia entry category
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EncyclopediaCategory {
    Folklore,
    History,
    FamousNigerians,
    Culture,
    Geography,
    Languages,
}

/// An encyclopedia entry in The Sabi Codex
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncyclopediaEntry {
    pub id: String,
    pub category: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub content_md: String,
    pub summary: Option<String>,
    pub image_url: Option<String>,
    pub audio_url: Option<String>,
    pub associated_state: Option<String>,
    pub tier: i32,                    // 1 = Open, 2 = Unlockable
    pub unlock_condition: Option<String>,
    pub xp_reward: i32,
    pub reading_time: Option<i32>,
    pub tags: Option<String>,         // JSON array
}

/// User's progress on an encyclopedia entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserEncyclopediaProgress {
    pub entry_id: String,
    pub is_unlocked: bool,
    pub is_read: bool,
    pub is_bookmarked: bool,
    pub unlocked_at: Option<String>,
    pub first_read_at: Option<String>,
    pub read_count: i32,
}

/// Encyclopedia entry with user progress combined
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncyclopediaEntryWithProgress {
    pub entry: EncyclopediaEntry,
    pub progress: Option<UserEncyclopediaProgress>,
    pub is_accessible: bool,  // Computed: tier 1 or (tier 2 and unlocked)
}

/// Statistics for the Sabi Codex
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodexStats {
    pub total_entries: i32,
    pub unlocked_entries: i32,
    pub read_entries: i32,
    pub bookmarked_entries: i32,
    pub entries_by_category: Vec<CategoryCount>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryCount {
    pub category: String,
    pub total: i32,
    pub unlocked: i32,
    pub read: i32,
}

// ============================================
// AVATAR & CHARACTER CUSTOMIZATION SYSTEM
// ============================================

/// Avatar item categories
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AvatarItemCategory {
    SkinTone,
    Hairstyle,
    Outfit,
    Accessory,
    Background,
}

/// An avatar customization item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvatarItem {
    pub id: String,
    pub category: String,
    pub name: String,
    pub description: Option<String>,
    pub image_key: Option<String>,
    pub rarity: String,
    pub unlock_cost: i32,
    pub unlock_condition: Option<String>,
    pub is_premium: bool,
    pub sort_order: i32,
}

/// User's unlocked avatar item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAvatarItem {
    pub item_id: String,
    pub unlocked_at: String,
    pub is_equipped: bool,
}

/// User's current avatar configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAvatar {
    pub user_id: i64,
    pub skin_tone: String,
    pub hairstyle: String,
    pub outfit: String,
    pub accessory: Option<String>,
    pub background: String,
    pub character_name: Option<String>,
}

impl Default for UserAvatar {
    fn default() -> Self {
        Self {
            user_id: 1,
            skin_tone: "skin_3".to_string(),
            hairstyle: "hair_1".to_string(),
            outfit: "outfit_school".to_string(),
            accessory: None,
            background: "bg_default".to_string(),
            character_name: None,
        }
    }
}

/// Avatar item with unlock status for UI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvatarItemWithStatus {
    pub item: AvatarItem,
    pub is_unlocked: bool,
    pub is_equipped: bool,
    pub can_afford: bool,
}

// ============================================
// CULTURAL GUIDES & NPC SYSTEM
// ============================================

/// A cultural guide NPC
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CulturalGuide {
    pub id: String,
    pub name: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub personality: Option<String>,
    pub avatar_image: Option<String>,
    pub state_id: Option<String>,
    pub region: Option<String>,
    pub greeting: Option<String>,
    pub catchphrase: Option<String>,
    pub voice_style: Option<String>,
}

// ============================================
// QUEST & STORY SYSTEM
// ============================================

/// Quest types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum QuestType {
    Main,
    Side,
    Daily,
    Weekly,
    Achievement,
}

/// Quest status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum QuestStatus {
    Locked,
    Available,
    Active,
    Completed,
    Claimed,
}

/// A quest definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quest {
    pub id: String,
    pub title: String,
    pub description: String,
    pub quest_type: String,
    pub category: Option<String>,
    pub state_id: Option<String>,
    pub guide_id: Option<String>,
    pub required_level: i32,
    pub prerequisite_quest_id: Option<String>,
    pub requirements_json: Option<String>,
    pub xp_reward: i32,
    pub cowrie_reward: i32,
    pub artifact_reward_id: Option<String>,
    pub item_rewards_json: Option<String>,
    pub intro_dialogue: Option<String>,
    pub progress_dialogue: Option<String>,
    pub completion_dialogue: Option<String>,
    pub icon: Option<String>,
    pub sort_order: i32,
    pub is_repeatable: bool,
    pub cooldown_hours: Option<i32>,
}

/// Quest requirement structure (parsed from JSON)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestRequirement {
    pub requirement_type: String,    // "complete_module", "visit_state", "collect_artifact", "score_quiz", "read_codex"
    pub target: String,              // Target ID
    pub count: i32,                  // Required count
    pub current: Option<i32>,        // Current progress (filled by app)
}

/// User's quest progress
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserQuest {
    pub quest_id: String,
    pub status: String,
    pub progress_json: Option<String>,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
    pub claimed_at: Option<String>,
    pub completion_count: i32,
}

/// Quest with full details for UI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestWithProgress {
    pub quest: Quest,
    pub guide: Option<CulturalGuide>,
    pub user_progress: Option<UserQuest>,
    pub requirements: Vec<QuestRequirement>,
    pub is_available: bool,
    pub progress_percent: i32,
}

// ============================================
// ARTIFACTS & COLLECTIBLES SYSTEM
// ============================================

/// Artifact rarity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

/// A cultural artifact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifact {
    pub id: String,
    pub name: String,
    pub description: String,
    pub long_description: Option<String>,
    pub category: String,             // "mask", "textile", "instrument", etc.
    pub state_id: Option<String>,
    pub region: Option<String>,
    pub image_url: Option<String>,
    pub thumbnail_url: Option<String>,
    pub model_3d_url: Option<String>,
    pub color_primary: Option<String>,
    pub color_secondary: Option<String>,
    pub rarity: String,
    pub historical_period: Option<String>,
    pub cultural_significance: Option<String>,
    pub unlock_type: String,          // "quest", "module", "achievement", "purchase"
    pub unlock_source_id: Option<String>,
    pub cowrie_cost: i32,
    pub sort_order: i32,
}

/// User's collected artifact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserArtifact {
    pub artifact_id: String,
    pub obtained_at: String,
    pub obtain_method: Option<String>,
    pub is_favorite: bool,
    pub is_new: bool,
    pub display_slot: Option<i32>,
}

/// Artifact with collection status for UI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactWithStatus {
    pub artifact: Artifact,
    pub is_collected: bool,
    pub user_data: Option<UserArtifact>,
    pub can_unlock: bool,             // Has met requirements
    pub state_name: Option<String>,   // Resolved state name
}

/// Museum/collection statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionStats {
    pub total_artifacts: i32,
    pub collected_count: i32,
    pub by_rarity: Vec<RarityCount>,
    pub by_category: Vec<CategoryArtifactCount>,
    pub by_state: Vec<StateArtifactCount>,
    pub favorites_count: i32,
    pub newest_artifact: Option<Artifact>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RarityCount {
    pub rarity: String,
    pub total: i32,
    pub collected: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryArtifactCount {
    pub category: String,
    pub total: i32,
    pub collected: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateArtifactCount {
    pub state_id: String,
    pub state_name: String,
    pub total: i32,
    pub collected: i32,
}

// ============================================
// STORY & DIALOGUE SYSTEM
// ============================================

/// A story chapter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryChapter {
    pub id: String,
    pub chapter_number: i32,
    pub title: String,
    pub description: Option<String>,
    pub required_states_json: Option<String>,
    pub required_quests_json: Option<String>,
    pub intro_cutscene: Option<String>,
    pub outro_cutscene: Option<String>,
    pub sort_order: i32,
}

/// User's story progress
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStoryProgress {
    pub chapter_id: String,
    pub status: String,               // "locked", "available", "in_progress", "completed"
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
}

/// Chapter with progress for UI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChapterWithProgress {
    pub chapter: StoryChapter,
    pub progress: Option<UserStoryProgress>,
    pub is_available: bool,
}

/// Dialogue entry for conversations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueEntry {
    pub speaker: String,              // Guide name or "NARRATOR"
    pub text: String,
    pub emotion: Option<String>,      // "happy", "serious", "excited", etc.
    pub avatar_override: Option<String>,
}

/// Full dialogue sequence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueSequence {
    pub entries: Vec<DialogueEntry>,
    pub background: Option<String>,
    pub music: Option<String>,
}
