PROJECT NIGERIA: EDUCATIONAL RPG SPECIFICATION

1. Project Overview
   Name: Project Nigeria (Code Name) Type: Offline-First Desktop Educational RPG Stack: Tauri (Rust + v2), React (TypeScript), SQLite, Phaser.js (Map), Framer Motion (UI). Vision: An "Encarta-level" interactive learning experience where Nigerian students travel a stylized map of Nigeria, unlocking curriculum-aligned challenges in every state. Target Audience: Primary/Secondary school students (K-12). Key Constraint: Must work 100% offline after initial download. Low hardware requirements.

2. Technical Architecture
   2.1 File Structure Strategy
   Frontend (/src): React + TypeScript.

/src/game: Phaser.js game logic (The Map, The Avatar).

/src/ui: React overlays (HUD, Quiz Modals, Menus).

/src/stores: Zustand stores for UserProgress, Inventory, Settings.

Backend (/src-tauri): Rust.

Handles heavy lifting: Database queries, File System access, Asset loading.

Database: curriculum.db (SQLite) stored in AppLocalData.

2.2 Database Schema (SQLite)
Use this schema to generate SQL migrations.

SQL

-- Core Curriculum Data
CREATE TABLE states (
id TEXT PRIMARY KEY, -- e.g., "LAG", "ABJ"
name TEXT NOT NULL, -- e.g., "Lagos", "FCT"
region TEXT, -- e.g., "South West"
unlock_level INTEGER DEFAULT 1
);

CREATE TABLE lessons (
id TEXT PRIMARY KEY,
state_id TEXT,
subject TEXT, -- "Math", "Science", "Social Studies"
title TEXT,
content_md TEXT, -- Markdown content for the lesson
FOREIGN KEY(state_id) REFERENCES states(id)
);

-- Question Bank
CREATE TABLE questions (
id INTEGER PRIMARY KEY AUTOINCREMENT,
lesson_id TEXT,
question_text TEXT,
question_type TEXT, -- "multiple_choice", "drag_drop"
options_json TEXT, -- JSON array of options
correct_answer TEXT,
xp_reward INTEGER,
FOREIGN KEY(lesson_id) REFERENCES lessons(id)
);

-- User Save Data
CREATE TABLE user_progress (
user_id INTEGER,
state_id TEXT,
stars INTEGER DEFAULT 0, -- 0 to 3
is_completed BOOLEAN DEFAULT 0,
UNIQUE(user_id, state_id)
);

CREATE TABLE inventory (
id INTEGER PRIMARY KEY AUTOINCREMENT,
user_id INTEGER,
item_id TEXT, -- e.g., "benin_bronze_mask"
acquired_at DATETIME DEFAULT CURRENT_TIMESTAMP
); 3. Game Design & Mechanics
3.1 The "Overworld" (Map System)
Visual Style: Stylized, colorful 2D vector map (like Civilization V strategic view or Super Mario World).

Navigation: Click-to-move or WASD.

State Locking:

Player starts in Abuja (Tutorial Zone).

Completing Abuja unlocks neighboring states (Niger, Kaduna, Nasarawa, Kogi).

Landmarks: Each state must display 1 famous landmark sprite (e.g., National Theatre for Lagos).

3.2 The Avatar System
Base: Neutral child figure.

Customization:

Skin Tone: 5 shades.

Outfit: Unlockable traditional attires (e.g., Yoruba Agbada, Hausa Babanriga, Igbo Isiagu).

Data Structure:

JSON

{
"avatar": {
"skin": "tone_3",
"head": "style_2",
"top": "shirt_lagos_fc",
"accessory": "glasses_nerd"
}
}
3.3 The "Encarta" Learning Loop
Discovery: Player arrives at a State.

Briefing: A rich-media modal opens.

Show: High-res photo/video of the state.

Text: "Welcome to Kano! Famous for the Groundnut Pyramids and Dye Pits."

The Challenge: "To earn the Kano Badge, you must pass the Mathematics of Commerce quiz."

Reward: On success -> XP, State Badge, and potentially a cultural item.

4. UI/UX Guidelines (The "Juice")
   Font: Nunito or Quicksand (Rounded, friendly, legible).

Color Palette:

Primary: Nigeria Green (#008751) & White.

Accents: Gold (Rewards), Terra Cotta (Map terrain), Sky Blue (UI backgrounds).

Feedback:

Correct Answer: "Ding" sound + Green flash + Confetti particle effect.

Wrong Answer: Gentle "Thud" sound + Shake animation (do not be discouraging).

Transitions: All modals must enter with a spring animation (using Framer Motion).

5. Implementation Roadmap (Prompts for AI)
   Use these prompt stages to guide the coding assistant.

Phase 1: Setup & Database
"Create a Tauri v2 app with React and TypeScript. Configure SQLite integration. Create a Rust command init_database that runs the SQL schema provided in the Context file. Create a seed script to populate the 'Lagos' and 'Abuja' states with dummy math questions."

Phase 2: The Map (Phaser + React)
"Install Phaser.js. Create a React component GameMap.tsx that loads a Phaser game instance. Inside Phaser, draw a simple interactive map where clicking a 'node' (representing a state) emits an event to React to open a modal. Use the color #008751 for the land."

Phase 3: The Learning Modal
"Create a LessonModal component using Framer Motion. It should take stateId as a prop. When opened, it should invoke a Tauri command get_lesson_content(stateId) to fetch data from SQLite. Display the title and a 'Start Quiz' button."

Phase 4: Gamification
"Implement the User Progress logic. Create a Zustand store useGameStore. When a quiz is passed, update the local SQLite database to set is_completed = true for that state and add +50 XP to the user's score."

6. Asset Requirements (Placeholder Strategy)
   Images: Use placeholders ( https://placehold.co/600x400) initially.

Map: Use a simplified SVG path of Nigeria for the prototype.

Sound: Search for "UI Sound Pack Free" (Kenney.nl assets are recommended).

1. The Visual Theme: "Afro-Pop Clay"
   We will use a style called Soft 3D / Claymorphism. This style uses inflated shapes, rounded corners, and soft shadows to make buttons look like candy or toys. It is incredibly popular in modern apps because it looks friendly and touchable.Vibe: Nintendo Switch UI meets "Into the Spider-Verse" colors.Shapes: Everything is rounded. No sharp corners. (Border-radius: 16px to 24px).Texture: Subtle noise textures to avoid looking like cheap plastic.

2. The Color Palette
   Instead of the standard harsh Green/White flag colors, we will use a vibrant, adjusted palette inspired by Nigerian geography.Color NameHex CodeUsageLagos Lush#00C896Primary Brand Color. Used for main buttons, success states, and the Map landmass.Zaria Gold#FFD166Secondary. Used for Stars, XP bars, Coin rewards, and highlighting key items.Niger Indigo#4D5382Text & Headings. Softer than pure black. easier on young eyes.Jos Clay#EF476FAccents. Used for "Close" buttons, locked levels, or urgent notifications.Cloud Cream#F7F9FCBackground. A very light cool grey/blue. Never use pure white (#FFFFFF) for backgrounds.

3. Typography
   We need fonts that are legible for children but have personality.Headings: Fredoka One or Baloo 2.Why: It’s chubby, rounded, and looks fun.Body Text: Nunito or Quicksand.Why: Highly readable geometric sans-serif with rounded edges.

4. UI Layout Concepts
   A. The Main Map (The "Home" Screen)Instead of a flat map, imagine an isometric view (tilted 3D).The Water: Animated subtle waves (using simple CSS shaders).The States: Floating "islands" or puzzle pieces.The Player: A 3D-looking avatar standing on their current location.The HUD (Heads Up Display):Top Left: Avatar Head + Name + Level Circle.Top Right: Currency (Cowrie Shells) + Streak Flame.Bottom Right: A big, floating "Backpack" button (Inventory).

B. The Challenge Mode (The "Quiz")Don't make it look like a test paper. Make it look like a game show.Card Stack: The question appears on a white card in the center.Answers: Large, pill-shaped buttons.Progress: A bar at the top that fills up with green liquid as they answer correctly.The Mascot: A helper character (e.g., a friendly Lizard or Eagle) peeking over the corner of the card, reacting to answers (Smiling if right, confused if wrong).

5. The "Juice" (Animations & Polish)"Juice" is what game designers call the non-essential visual effects that make a game feel satisfying.Button Press: Buttons should physically "squish" down when clicked (CSS transform: scale(0.95)).Confetti: When a level is finished, blast confetti from the corners. Use Nigerian flag colors for the confetti.Number Rolling: If a user earns 50 points, don't just show "50". Roll the number up: 1... 12... 34... 50!Parallax: When the mouse moves over the map, the background clouds should move slower than the map, creating depth.
