// Curriculum seed data for Project Nigeria
// Contains real Nigerian educational content for secondary school students

use rusqlite::Connection;
use super::DatabaseError;

/// Seeds the database with comprehensive curriculum for Abuja and Lagos
pub fn seed_curriculum(conn: &Connection) -> Result<(), DatabaseError> {
    // Seed states FIRST (required by user_progress foreign key)
    seed_states(conn)?;
    
    // Create default user (after states exist)
    seed_default_user(conn)?;
    
    // Seed items that can be unlocked
    seed_items(conn)?;
    
    // Seed Abuja modules (Heritage Zone)
    seed_abuja_modules(conn)?;
    
    // Seed Lagos modules (Mind Zone)
    seed_lagos_modules(conn)?;
    
    // Seed The Sabi Codex encyclopedia entries
    seed_encyclopedia(conn)?;
    
    // Seed RPG features
    seed_avatar_items(conn)?;
    seed_cultural_guides(conn)?;
    seed_artifacts(conn)?;
    seed_quests(conn)?;
    
    log::info!("Curriculum seeded successfully with Abuja and Lagos modules");
    Ok(())
}

fn seed_states(conn: &Connection) -> Result<(), DatabaseError> {
    conn.execute_batch(r#"
        -- Abuja (FCT) - Heritage Zone - Tutorial/Starting Zone
        INSERT OR REPLACE INTO states (id, name, region, zone, unlock_level, landmark_name, landmark_image, description, fun_fact)
        VALUES ('ABJ', 'Abuja', 'North Central', 'heritage', 1, 'Aso Rock', '/assets/images/abuja-landmark.png',
                'Welcome to Abuja, the Federal Capital Territory! This purpose-built city became Nigeria''s capital in 1991, replacing Lagos. Here, you''ll learn about government, citizenship, and civic duties.',
                'Abuja was designed by Japanese architect Kenzo Tange and is one of the few purpose-built capital cities in Africa!');
        
        -- Lagos - Mind Zone - First major challenge
        INSERT OR REPLACE INTO states (id, name, region, zone, unlock_level, landmark_name, landmark_image, description, fun_fact)
        VALUES ('LAG', 'Lagos', 'South West', 'mind', 2, 'National Theatre', '/assets/images/lagos-landmark.png',
                'Welcome to Lagos! The economic powerhouse of Nigeria and one of Africa''s largest cities. Master mathematics, logic, and the art of commerce in West Africa''s busiest markets.',
                'Lagos is the smallest state in Nigeria by land area but has the highest population - over 20 million people!');
        
        -- Neighboring states (locked initially)
        INSERT OR REPLACE INTO states (id, name, region, zone, unlock_level, landmark_name, landmark_image, description, fun_fact)
        VALUES 
            ('NIG', 'Niger', 'North Central', 'heritage', 3, 'Gurara Falls', '/assets/images/niger-landmark.png',
             'Niger State - The largest state in Nigeria by land area, home to beautiful waterfalls and wildlife.',
             'Niger State is named after the River Niger and contains Kainji Dam, Nigeria''s first hydroelectric power station!'),
            ('KAD', 'Kaduna', 'North West', 'heritage', 3, 'Kajuru Castle', '/assets/images/kaduna-landmark.png',
             'Kaduna State - A historic center of learning and industry in Northern Nigeria.',
             'Kaduna was once the capital of Northern Nigeria and hosts one of the largest textile industries in Africa!'),
            ('KAN', 'Kano', 'North West', 'heritage', 4, 'Kano City Walls', '/assets/images/kano-landmark.png',
             'Kano State - One of Nigeria''s oldest cities with over 1000 years of history, famous for indigo dye pits and groundnut pyramids.',
             'Kano''s Kurmi Market is one of the oldest and largest in West Africa, dating back over 500 years!'),
            ('EDO', 'Edo', 'South South', 'spirit', 5, 'Benin City Walls', '/assets/images/edo-landmark.png',
             'Edo State - Home of the Great Benin Empire, famous for its bronze artworks and rich royal heritage.',
             'The Benin Moat (Iya) was the largest man-made earthwork in the world before its destruction!'),
            ('OGU', 'Ogun', 'South West', 'mind', 4, 'Olumo Rock', '/assets/images/ogun-landmark.png',
             'Ogun State - The Gateway State, birthplace of Nobel Laureate Wole Soyinka.',
             'Ogun State has the highest concentration of industries in Nigeria!'),
            ('OYO', 'Oyo', 'South West', 'mind', 4, 'Mapo Hall', '/assets/images/oyo-landmark.png',
             'Oyo State - Home to the ancient Oyo Empire and the historic city of Ibadan.',
             'Ibadan in Oyo State was once the largest city in Africa by geographical area!'),
            ('IMO', 'Imo', 'South East', 'spirit', 5, 'Oguta Lake', '/assets/images/imo-landmark.png',
             'Imo State - The Eastern Heartland, known for its rich culture and educational excellence.',
             'Owerri, the capital of Imo State, is one of the fastest growing cities in Nigeria!'),
            ('RIV', 'Rivers', 'South South', 'spirit', 5, 'Port Harcourt Pleasure Park', '/assets/images/rivers-landmark.png',
             'Rivers State - The Treasure Base of the Nation, home to Nigeria''s oil wealth and the vibrant city of Port Harcourt.',
             'Port Harcourt was named after Lewis Harcourt, a British Colonial Secretary, and is called the Garden City of Nigeria!'),
            ('CRS', 'Cross River', 'South South', 'spirit', 6, 'Obudu Mountain Resort', '/assets/images/crossriver-landmark.png',
             'Cross River State - The People''s Paradise, home to Africa''s oldest rainforest and the famous Calabar Carnival.',
             'Cross River contains the last remaining virgin tropical rainforest in Nigeria and hosts Africa''s biggest street party!');
    "#).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    Ok(())
}

fn seed_items(conn: &Connection) -> Result<(), DatabaseError> {
    conn.execute_batch(r#"
        -- Unlockable items for modules
        INSERT OR REPLACE INTO items (id, name, description, item_type, rarity, image_url)
        VALUES 
            ('badge_citizen', 'Citizen Badge', 'Awarded for understanding civic responsibilities', 'badge', 'common', 'assets/items/badge_citizen.png'),
            ('badge_legislator', 'Young Legislator Badge', 'Awarded for mastering government processes', 'badge', 'rare', 'assets/items/badge_legislator.png'),
            ('badge_voter', 'Informed Voter Badge', 'Awarded for understanding democracy', 'badge', 'common', 'assets/items/badge_voter.png'),
            ('badge_trader', 'Market Champion Badge', 'Awarded for mastering market mathematics', 'badge', 'common', 'assets/items/badge_trader.png'),
            ('badge_accountant', 'Young Accountant Badge', 'Awarded for financial calculation skills', 'badge', 'rare', 'assets/items/badge_accountant.png'),
            ('badge_coder', 'Logic Master Badge', 'Awarded for solving logic puzzles', 'badge', 'rare', 'assets/items/badge_coder.png'),
            ('outfit_agbada', 'Yoruba Agbada', 'Traditional flowing robe worn at ceremonies', 'outfit', 'epic', 'assets/items/outfit_agbada.png'),
            ('outfit_kaftan', 'Northern Kaftan', 'Elegant traditional Northern Nigerian attire', 'outfit', 'epic', 'assets/items/outfit_kaftan.png'),
            ('accessory_cap_red', 'Red Chief Cap', 'Traditional red cap worn by leaders', 'accessory', 'rare', 'assets/items/cap_red.png'),
            ('accessory_gele', 'Gele Headwrap', 'Beautiful traditional headwrap', 'accessory', 'rare', 'assets/items/gele.png');
    "#).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    Ok(())
}

fn seed_abuja_modules(conn: &Connection) -> Result<(), DatabaseError> {
    // Module 1: The People's Court (Social Studies/Civic Education)
    conn.execute_batch(r#"
        -- =====================================================
        -- ABUJA MODULE 1: THE PEOPLE'S COURT (Social Studies)
        -- =====================================================
        INSERT OR REPLACE INTO modules (id, state_id, subject, title, description, required_level, total_xp, estimated_time, icon)
        VALUES ('abj_civics_001', 'ABJ', 'Social Studies', 'The People''s Court', 
                'Learn how Nigeria''s government works! Pass bills through the National Assembly and understand your rights as a citizen.',
                1, 500, 20, 'gavel');
        
        -- Module Context (Encarta-style)
        INSERT OR REPLACE INTO module_context (module_id, did_you_know, fun_fact, intro_text, historical_note)
        VALUES ('abj_civics_001',
                'The National Assembly Complex in Abuja is one of the largest legislative buildings in Africa, covering over 850,000 square meters!',
                'Nigeria''s democracy has three "arms" like a three-legged stool: Executive (President), Legislative (National Assembly), and Judiciary (Courts). If one leg breaks, the stool falls!',
                'Welcome, future lawmaker! In this module, you will learn how laws are made in Nigeria, understand the three branches of government, and discover your rights and responsibilities as a Nigerian citizen.',
                'Nigeria returned to democratic rule on May 29, 1999, after years of military rule. This date is now celebrated as Democracy Day!');
        
        -- Level 1: Introduction to Government
        INSERT OR REPLACE INTO levels (id, module_id, title, difficulty, order_index, xp_reward, unlock_item_id)
        VALUES ('abj_civics_001_lvl1', 'abj_civics_001', 'The Three Arms of Government', 'easy', 1, 100, 'badge_citizen');
        
        -- Level 1 Questions
        INSERT OR REPLACE INTO questions (id, level_id, question_text, question_type, options_json, correct_answer, xp_reward, explanation, hint, order_index)
        VALUES 
            ('abj_c1_q1', 'abj_civics_001_lvl1', 
             'Which of the following is NOT one of the three arms of government in Nigeria?',
             'multiple_choice',
             '[{"id":"a","text":"Executive"},{"id":"b","text":"Legislative"},{"id":"c","text":"Military"},{"id":"d","text":"Judiciary"}]',
             'c', 15,
             'The three arms of government are: Executive (headed by the President), Legislative (National Assembly), and Judiciary (Courts). The military is not an arm of government.',
             'Think about who makes laws, who enforces them, and who interprets them.',
             1),
            
            ('abj_c1_q2', 'abj_civics_001_lvl1',
             'The President of Nigeria is the head of which arm of government?',
             'multiple_choice',
             '[{"id":"a","text":"Legislative"},{"id":"b","text":"Executive"},{"id":"c","text":"Judiciary"},{"id":"d","text":"Senate"}]',
             'b', 15,
             'The President heads the Executive arm. The Executive implements and enforces laws passed by the Legislature.',
             'The President "executes" or carries out the laws.',
             2),
            
            ('abj_c1_q3', 'abj_civics_001_lvl1',
             'The National Assembly is made up of two chambers. What are they called?',
             'multiple_choice',
             '[{"id":"a","text":"House of Commons and House of Lords"},{"id":"b","text":"Senate and House of Representatives"},{"id":"c","text":"Upper House and Lower House"},{"id":"d","text":"Congress and Parliament"}]',
             'b', 15,
             'Nigeria''s National Assembly consists of the Senate (109 members) and the House of Representatives (360 members).',
             'One chamber sounds like a group of senior advisors, the other represents the people.',
             3),
            
            ('abj_c1_q4', 'abj_civics_001_lvl1',
             'True or False: The Supreme Court is the highest court in Nigeria.',
             'true_false',
             '[{"id":"true","text":"True"},{"id":"false","text":"False"}]',
             'true', 10,
             'The Supreme Court is indeed the highest court in Nigeria. Its decisions are final and binding on all other courts.',
             NULL,
             4),
            
            ('abj_c1_q5', 'abj_civics_001_lvl1',
             'How many senators represent each state in Nigeria?',
             'multiple_choice',
             '[{"id":"a","text":"1"},{"id":"b","text":"2"},{"id":"c","text":"3"},{"id":"d","text":"4"}]',
             'c', 15,
             'Each of Nigeria''s 36 states has 3 senators, plus 1 from the FCT, making a total of 109 senators.',
             '36 states × 3 = 108, plus 1 from FCT = 109',
             5);
        
        -- Level 2: How Laws Are Made
        INSERT OR REPLACE INTO levels (id, module_id, title, difficulty, order_index, xp_reward, unlock_item_id)
        VALUES ('abj_civics_001_lvl2', 'abj_civics_001', 'How a Bill Becomes Law', 'medium', 2, 150, 'badge_legislator');
        
        -- Level 2 Questions
        INSERT OR REPLACE INTO questions (id, level_id, question_text, question_type, options_json, correct_answer, xp_reward, explanation, hint, order_index)
        VALUES 
            ('abj_c2_q1', 'abj_civics_001_lvl2',
             'What is a "Bill" in the context of lawmaking?',
             'multiple_choice',
             '[{"id":"a","text":"A type of money"},{"id":"b","text":"A proposed law that hasn''t been passed yet"},{"id":"c","text":"A court document"},{"id":"d","text":"A presidential speech"}]',
             'b', 20,
             'A Bill is a draft or proposed law. It only becomes an Act (law) after being passed by both chambers of the National Assembly and signed by the President.',
             'Think of it as a "potential" law.',
             1),
            
            ('abj_c2_q2', 'abj_civics_001_lvl2',
             'After both the Senate and House of Representatives pass a bill, who must sign it for it to become law?',
             'multiple_choice',
             '[{"id":"a","text":"The Chief Justice"},{"id":"b","text":"The Vice President"},{"id":"c","text":"The President"},{"id":"d","text":"The Senate President"}]',
             'c', 20,
             'The President must sign a bill for it to become law. This is called "Presidential Assent." The President can also refuse to sign (veto) the bill.',
             'The head of the Executive branch gives final approval.',
             2),
            
            ('abj_c2_q3', 'abj_civics_001_lvl2',
             'What happens if the President refuses to sign a bill (vetoes it)?',
             'multiple_choice',
             '[{"id":"a","text":"The bill is automatically destroyed"},{"id":"b","text":"The National Assembly can override the veto with 2/3 majority"},{"id":"c","text":"A new President must be elected"},{"id":"d","text":"The bill becomes law anyway after 30 days"}]',
             'b', 25,
             'If the President vetoes a bill, the National Assembly can override the veto if two-thirds of members in both chambers vote in favor. This is a check on presidential power.',
             'The Legislature has a way to override the Executive.',
             3),
            
            ('abj_c2_q4', 'abj_civics_001_lvl2',
             'The first reading of a bill involves:',
             'multiple_choice',
             '[{"id":"a","text":"Detailed debate on the bill"},{"id":"b","text":"Just introducing the title of the bill"},{"id":"c","text":"Voting on the bill"},{"id":"d","text":"Presidential approval"}]',
             'b', 20,
             'The First Reading is just an introduction where the title of the bill is read. The detailed debate happens during the Second Reading.',
             'It''s the first step - not much happens yet.',
             4),
            
            ('abj_c2_q5', 'abj_civics_001_lvl2',
             'Which reading of a bill involves detailed debate and discussion?',
             'multiple_choice',
             '[{"id":"a","text":"First Reading"},{"id":"b","text":"Second Reading"},{"id":"c","text":"Third Reading"},{"id":"d","text":"Final Reading"}]',
             'b', 20,
             'The Second Reading is when the principles and merits of the bill are debated. Members discuss whether the bill should proceed.',
             'The middle reading is where the main discussion happens.',
             5);
        
        -- Level 3: Citizens' Rights and Responsibilities
        INSERT OR REPLACE INTO levels (id, module_id, title, difficulty, order_index, xp_reward, unlock_item_id)
        VALUES ('abj_civics_001_lvl3', 'abj_civics_001', 'Rights and Responsibilities', 'medium', 3, 150, 'badge_voter');
        
        -- Level 3 Questions  
        INSERT OR REPLACE INTO questions (id, level_id, question_text, question_type, options_json, correct_answer, xp_reward, explanation, hint, order_index)
        VALUES 
            ('abj_c3_q1', 'abj_civics_001_lvl3',
             'At what age can a Nigerian citizen vote in elections?',
             'multiple_choice',
             '[{"id":"a","text":"16 years"},{"id":"b","text":"18 years"},{"id":"c","text":"21 years"},{"id":"d","text":"25 years"}]',
             'b', 20,
             'In Nigeria, citizens can vote once they turn 18 years old. This is the legal voting age.',
             'It''s the same age you become a legal adult.',
             1),
            
            ('abj_c3_q2', 'abj_civics_001_lvl3',
             'Which of these is a CIVIC RESPONSIBILITY (duty) rather than a right?',
             'multiple_choice',
             '[{"id":"a","text":"Right to vote"},{"id":"b","text":"Right to education"},{"id":"c","text":"Paying taxes"},{"id":"d","text":"Freedom of speech"}]',
             'c', 20,
             'Paying taxes is a responsibility - something citizens are expected to do for the country. Rights are things the country must provide or protect for citizens.',
             'A responsibility is something you MUST do, not something you receive.',
             2),
            
            ('abj_c3_q3', 'abj_civics_001_lvl3',
             'The Nigerian Constitution guarantees freedom of religion. What does this mean?',
             'multiple_choice',
             '[{"id":"a","text":"Everyone must be religious"},{"id":"b","text":"Only Christianity and Islam are allowed"},{"id":"c","text":"People can practice any religion or none at all"},{"id":"d","text":"Religion is banned"}]',
             'c', 20,
             'Freedom of religion means every Nigerian can choose to practice any religion, or no religion at all. Nigeria is a secular state.',
             'Freedom means the ability to choose.',
             3),
            
            ('abj_c3_q4', 'abj_civics_001_lvl3',
             'True or False: The Nigerian National Anthem must be respected by all citizens.',
             'true_false',
             '[{"id":"true","text":"True"},{"id":"false","text":"False"}]',
             'true', 15,
             'Yes! Respecting national symbols like the flag, anthem, and coat of arms is a civic responsibility of all Nigerians.',
             NULL,
             4),
            
            ('abj_c3_q5', 'abj_civics_001_lvl3',
             'What body is responsible for conducting elections in Nigeria?',
             'multiple_choice',
             '[{"id":"a","text":"EFCC"},{"id":"b","text":"INEC"},{"id":"c","text":"NDLEA"},{"id":"d","text":"CBN"}]',
             'b', 25,
             'INEC (Independent National Electoral Commission) is responsible for organizing and conducting elections in Nigeria.',
             'It has "Electoral" in its name.',
             5);
    "#).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    Ok(())
}

fn seed_lagos_modules(conn: &Connection) -> Result<(), DatabaseError> {
    // Module 1: The Balogun Market Challenge (Mathematics)
    conn.execute_batch(r#"
        -- =====================================================
        -- LAGOS MODULE 1: THE BALOGUN MARKET CHALLENGE (Mathematics)
        -- =====================================================
        INSERT OR REPLACE INTO modules (id, state_id, subject, title, description, required_level, total_xp, estimated_time, icon)
        VALUES ('lag_math_001', 'LAG', 'Mathematics', 'The Balogun Market Challenge', 
                'Master the art of buying and selling in West Africa''s biggest market! Learn arithmetic, percentages, and financial calculations.',
                2, 600, 25, 'calculator');
        
        -- Module Context (Encarta-style)
        INSERT OR REPLACE INTO module_context (module_id, did_you_know, fun_fact, intro_text, historical_note)
        VALUES ('lag_math_001',
                'Balogun Market on Lagos Island is so large that it has no single address! It sprawls across multiple streets and is famous for fabrics like Ankara and Lace.',
                'Lagos traders are known for their sharp mental arithmetic. Many can calculate complex totals faster than a calculator!',
                'Welcome to Balogun Market, young trader! Here, you''ll learn the mathematics of commerce - from simple addition to calculating profit, loss, and percentages. The market waits for no one, so sharpen your mind!',
                'Lagos has been a trading hub for centuries. Even before the British arrived, Yoruba merchants traded goods across West Africa using cowrie shells as currency.');
        
        -- Level 1: Haggling 101 - Basic Arithmetic
        INSERT OR REPLACE INTO levels (id, module_id, title, difficulty, order_index, xp_reward, unlock_item_id)
        VALUES ('lag_math_001_lvl1', 'lag_math_001', 'Haggling 101: Basic Arithmetic', 'easy', 1, 100, 'badge_trader');
        
        -- Level 1 Questions
        INSERT OR REPLACE INTO questions (id, level_id, question_text, question_type, options_json, correct_answer, xp_reward, explanation, hint, order_index)
        VALUES 
            ('lag_m1_q1', 'lag_math_001_lvl1',
             'Mama Nkechi sells 5 yards of Ankara fabric in the morning and 3 yards in the afternoon. How many yards did she sell in total?',
             'multiple_choice',
             '[{"id":"a","text":"7 yards"},{"id":"b","text":"8 yards"},{"id":"c","text":"15 yards"},{"id":"d","text":"2 yards"}]',
             'b', 15,
             '5 + 3 = 8 yards. When we combine quantities, we add them together.',
             'Add the morning sales to the afternoon sales.',
             1),
            
            ('lag_m1_q2', 'lag_math_001_lvl1',
             'You have ₦500. You take a Danfo bus that costs ₦200. How much change do you have left?',
             'multiple_choice',
             '[{"id":"a","text":"₦200"},{"id":"b","text":"₦300"},{"id":"c","text":"₦700"},{"id":"d","text":"₦250"}]',
             'b', 15,
             '₦500 - ₦200 = ₦300. When we spend money, we subtract the cost from what we have.',
             'Take away the bus fare from your total money.',
             2),
            
            ('lag_m1_q3', 'lag_math_001_lvl1',
             'A trader has 24 oranges. If she sells 9 oranges, how many does she have left?',
             'multiple_choice',
             '[{"id":"a","text":"33 oranges"},{"id":"b","text":"15 oranges"},{"id":"c","text":"13 oranges"},{"id":"d","text":"16 oranges"}]',
             'b', 15,
             '24 - 9 = 15 oranges. Subtraction tells us what remains after we take some away.',
             'Start with 24 and take away 9.',
             3),
            
            ('lag_m1_q4', 'lag_math_001_lvl1',
             'Balogun Market opens at 8:00 AM and closes at 6:00 PM. How many hours is the market open?',
             'multiple_choice',
             '[{"id":"a","text":"8 hours"},{"id":"b","text":"10 hours"},{"id":"c","text":"12 hours"},{"id":"d","text":"14 hours"}]',
             'b', 15,
             'From 8 AM to 6 PM is 10 hours. Count: 8→9→10→11→12→1→2→3→4→5→6 = 10 hours.',
             'Count the hours from morning to evening.',
             4),
            
            ('lag_m1_q5', 'lag_math_001_lvl1',
             'Emeka bought suya for ₦150, a drink for ₦100, and bread for ₦250. What is his total spending?',
             'multiple_choice',
             '[{"id":"a","text":"₦400"},{"id":"b","text":"₦450"},{"id":"c","text":"₦500"},{"id":"d","text":"₦550"}]',
             'c', 20,
             '₦150 + ₦100 + ₦250 = ₦500. Add all expenses to find the total.',
             'Add all three amounts together.',
             5);
        
        -- Level 2: The Wholesaler - Multiplication
        INSERT OR REPLACE INTO levels (id, module_id, title, difficulty, order_index, xp_reward, unlock_item_id)
        VALUES ('lag_math_001_lvl2', 'lag_math_001', 'The Wholesaler: Multiplication', 'medium', 2, 150, NULL);
        
        -- Level 2 Questions
        INSERT OR REPLACE INTO questions (id, level_id, question_text, question_type, options_json, correct_answer, xp_reward, explanation, hint, order_index)
        VALUES 
            ('lag_m2_q1', 'lag_math_001_lvl2',
             'A carton of Indomie contains 40 packs. How many packs are in 3 cartons?',
             'multiple_choice',
             '[{"id":"a","text":"43 packs"},{"id":"b","text":"120 packs"},{"id":"c","text":"100 packs"},{"id":"d","text":"80 packs"}]',
             'b', 20,
             '40 × 3 = 120 packs. Multiplication is repeated addition: 40 + 40 + 40 = 120.',
             '40 three times equals...',
             1),
            
            ('lag_m2_q2', 'lag_math_001_lvl2',
             'A trader buys 12 crates of eggs. Each crate contains 30 eggs. What is the total number of eggs?',
             'multiple_choice',
             '[{"id":"a","text":"42 eggs"},{"id":"b","text":"300 eggs"},{"id":"c","text":"360 eggs"},{"id":"d","text":"420 eggs"}]',
             'c', 20,
             '12 × 30 = 360 eggs. Multiply the number of crates by eggs per crate.',
             'Number of crates times eggs per crate.',
             2),
            
            ('lag_m2_q3', 'lag_math_001_lvl2',
             'A Danfo bus carries 14 passengers. If 5 buses are full, how many passengers are there in total?',
             'multiple_choice',
             '[{"id":"a","text":"19 passengers"},{"id":"b","text":"56 passengers"},{"id":"c","text":"70 passengers"},{"id":"d","text":"65 passengers"}]',
             'c', 20,
             '14 × 5 = 70 passengers. Five buses with 14 passengers each.',
             'Multiply passengers per bus by number of buses.',
             3),
            
            ('lag_m2_q4', 'lag_math_001_lvl2',
             'If one yard of lace costs ₦2,500, how much will 8 yards cost?',
             'multiple_choice',
             '[{"id":"a","text":"₦10,000"},{"id":"b","text":"₦15,000"},{"id":"c","text":"₦20,000"},{"id":"d","text":"₦25,000"}]',
             'c', 25,
             '₦2,500 × 8 = ₦20,000. Multiply the price per yard by the number of yards.',
             '₦2,500 eight times.',
             4),
            
            ('lag_m2_q5', 'lag_math_001_lvl2',
             'A shop sells 25 bags of rice per day. How many bags will they sell in 2 weeks (14 days)?',
             'multiple_choice',
             '[{"id":"a","text":"250 bags"},{"id":"b","text":"300 bags"},{"id":"c","text":"350 bags"},{"id":"d","text":"400 bags"}]',
             'c', 25,
             '25 × 14 = 350 bags. Daily sales multiplied by number of days.',
             '25 bags times 14 days.',
             5);
        
        -- Level 3: Profit and Loss
        INSERT OR REPLACE INTO levels (id, module_id, title, difficulty, order_index, xp_reward, unlock_item_id)
        VALUES ('lag_math_001_lvl3', 'lag_math_001', 'The Business Mind: Profit & Loss', 'hard', 3, 200, 'badge_accountant');
        
        -- Level 3 Questions
        INSERT OR REPLACE INTO questions (id, level_id, question_text, question_type, options_json, correct_answer, xp_reward, explanation, hint, order_index)
        VALUES 
            ('lag_m3_q1', 'lag_math_001_lvl3',
             'A trader buys goods for ₦5,000 (cost price) and sells them for ₦6,500 (selling price). What is the profit?',
             'multiple_choice',
             '[{"id":"a","text":"₦1,000"},{"id":"b","text":"₦1,500"},{"id":"c","text":"₦2,000"},{"id":"d","text":"₦11,500"}]',
             'b', 25,
             'Profit = Selling Price - Cost Price = ₦6,500 - ₦5,000 = ₦1,500. Profit is what you gain after selling.',
             'Subtract what you paid from what you received.',
             1),
            
            ('lag_m3_q2', 'lag_math_001_lvl3',
             'Chidi bought a phone for ₦45,000 but had to sell it for ₦40,000 because it was damaged. What is his loss?',
             'multiple_choice',
             '[{"id":"a","text":"₦5,000"},{"id":"b","text":"₦10,000"},{"id":"c","text":"₦85,000"},{"id":"d","text":"₦4,500"}]',
             'a', 25,
             'Loss = Cost Price - Selling Price = ₦45,000 - ₦40,000 = ₦5,000. Loss occurs when you sell for less than you bought.',
             'Loss is the difference when selling price is lower.',
             2),
            
            ('lag_m3_q3', 'lag_math_001_lvl3',
             'A market woman buys 100 oranges for ₦2,000 and sells each orange for ₦30. What is her total profit?',
             'multiple_choice',
             '[{"id":"a","text":"₦500"},{"id":"b","text":"₦1,000"},{"id":"c","text":"₦3,000"},{"id":"d","text":"₦5,000"}]',
             'b', 30,
             'Total selling price = 100 × ₦30 = ₦3,000. Profit = ₦3,000 - ₦2,000 = ₦1,000.',
             'First find total sales, then subtract cost.',
             3),
            
            ('lag_m3_q4', 'lag_math_001_lvl3',
             'If a trader makes a profit of ₦200 on each bag sold and sells 15 bags, what is the total profit?',
             'multiple_choice',
             '[{"id":"a","text":"₦2,000"},{"id":"b","text":"₦2,500"},{"id":"c","text":"₦3,000"},{"id":"d","text":"₦3,500"}]',
             'c', 25,
             'Total Profit = Profit per item × Number of items = ₦200 × 15 = ₦3,000.',
             'Multiply profit per bag by number of bags.',
             4),
            
            ('lag_m3_q5', 'lag_math_001_lvl3',
             'A shopkeeper bought 50 shirts at ₦1,000 each and sold them all at ₦1,200 each. What is the total profit?',
             'multiple_choice',
             '[{"id":"a","text":"₦5,000"},{"id":"b","text":"₦10,000"},{"id":"c","text":"₦15,000"},{"id":"d","text":"₦60,000"}]',
             'b', 35,
             'Cost = 50 × ₦1,000 = ₦50,000. Sales = 50 × ₦1,200 = ₦60,000. Profit = ₦60,000 - ₦50,000 = ₦10,000.',
             'Calculate total cost and total sales, then find the difference.',
             5);
        
        -- =====================================================
        -- LAGOS MODULE 2: SILICON VALLEY OF NIGERIA (Logic/Coding)
        -- =====================================================
        INSERT OR REPLACE INTO modules (id, state_id, subject, title, description, required_level, total_xp, estimated_time, icon)
        VALUES ('lag_logic_001', 'LAG', 'Logic & Coding', 'Yaba Tech: Logic Puzzles', 
                'Welcome to Nigeria''s tech hub! Train your brain with logic puzzles and algorithmic thinking.',
                2, 450, 20, 'cpu');
        
        -- Module Context
        INSERT OR REPLACE INTO module_context (module_id, did_you_know, fun_fact, intro_text, historical_note)
        VALUES ('lag_logic_001',
                'Yaba in Lagos is called "Yabacon Valley" - Nigeria''s answer to Silicon Valley! It hosts hundreds of tech startups and innovation hubs.',
                'Nigerian software developers work at top companies like Google, Microsoft, and Meta. The tech industry in Lagos is growing faster than almost anywhere in Africa!',
                'Welcome to Yaba, young coder! Here in Nigeria''s tech capital, we''ll train your logical thinking skills. No coding experience needed - just your brain and problem-solving abilities!',
                'Nigeria''s first computer was installed at the University of Ibadan in 1963. Today, Nigeria has over 100 million internet users!');
        
        -- Level 1: Pattern Recognition
        INSERT OR REPLACE INTO levels (id, module_id, title, difficulty, order_index, xp_reward, unlock_item_id)
        VALUES ('lag_logic_001_lvl1', 'lag_logic_001', 'Pattern Detective', 'easy', 1, 100, NULL);
        
        -- Level 1 Questions
        INSERT OR REPLACE INTO questions (id, level_id, question_text, question_type, options_json, correct_answer, xp_reward, explanation, hint, order_index)
        VALUES 
            ('lag_l1_q1', 'lag_logic_001_lvl1',
             'What number comes next in the pattern? 2, 4, 6, 8, ?',
             'multiple_choice',
             '[{"id":"a","text":"9"},{"id":"b","text":"10"},{"id":"c","text":"12"},{"id":"d","text":"16"}]',
             'b', 15,
             'The pattern adds 2 each time: 2+2=4, 4+2=6, 6+2=8, 8+2=10.',
             'Look at how much is added between each number.',
             1),
            
            ('lag_l1_q2', 'lag_logic_001_lvl1',
             'Find the odd one out: Apple, Orange, Carrot, Banana',
             'multiple_choice',
             '[{"id":"a","text":"Apple"},{"id":"b","text":"Orange"},{"id":"c","text":"Carrot"},{"id":"d","text":"Banana"}]',
             'c', 15,
             'Carrot is a vegetable. The others are all fruits.',
             'Think about which category each belongs to.',
             2),
            
            ('lag_l1_q3', 'lag_logic_001_lvl1',
             'What letter comes next? A, C, E, G, ?',
             'multiple_choice',
             '[{"id":"a","text":"H"},{"id":"b","text":"I"},{"id":"c","text":"J"},{"id":"d","text":"K"}]',
             'b', 15,
             'The pattern skips one letter each time: A(b)C(d)E(f)G(h)I.',
             'Count how many letters are skipped.',
             3),
            
            ('lag_l1_q4', 'lag_logic_001_lvl1',
             'If all Roses are Flowers, and all Flowers need Water, then:',
             'multiple_choice',
             '[{"id":"a","text":"All Water is Roses"},{"id":"b","text":"All Roses need Water"},{"id":"c","text":"All Water is Flowers"},{"id":"d","text":"Roses don''t need Water"}]',
             'b', 20,
             'This is logical deduction. Since Roses are Flowers, and Flowers need Water, Roses must need Water.',
             'Follow the chain of logic from Roses to Flowers to Water.',
             4),
            
            ('lag_l1_q5', 'lag_logic_001_lvl1',
             'What number should replace the question mark? 1, 4, 9, 16, ?',
             'multiple_choice',
             '[{"id":"a","text":"20"},{"id":"b","text":"25"},{"id":"c","text":"32"},{"id":"d","text":"36"}]',
             'b', 20,
             'These are square numbers: 1²=1, 2²=4, 3²=9, 4²=16, 5²=25.',
             'Think about multiplication with the same number twice.',
             5);
        
        -- Level 2: Algorithmic Thinking
        INSERT OR REPLACE INTO levels (id, module_id, title, difficulty, order_index, xp_reward, unlock_item_id)
        VALUES ('lag_logic_001_lvl2', 'lag_logic_001', 'Step-by-Step Solutions', 'medium', 2, 150, 'badge_coder');
        
        -- Level 2 Questions
        INSERT OR REPLACE INTO questions (id, level_id, question_text, question_type, options_json, correct_answer, xp_reward, explanation, hint, order_index)
        VALUES 
            ('lag_l2_q1', 'lag_logic_001_lvl2',
             'A robot can only move: Forward (F), Left (L), Right (R). To go from Start to the Door (straight ahead, turn right, then straight), what commands are needed?',
             'multiple_choice',
             '[{"id":"a","text":"F, R, F"},{"id":"b","text":"F, L, F"},{"id":"c","text":"R, F, F"},{"id":"d","text":"F, F, R"}]',
             'a', 25,
             'First go Forward to reach the turn, then turn Right, then go Forward to the door. F, R, F.',
             'Think step by step through the path.',
             1),
            
            ('lag_l2_q2', 'lag_logic_001_lvl2',
             'If START = 1, then after these steps: ADD 5, MULTIPLY by 2, SUBTRACT 3, what is the result?',
             'multiple_choice',
             '[{"id":"a","text":"8"},{"id":"b","text":"9"},{"id":"c","text":"10"},{"id":"d","text":"11"}]',
             'b', 25,
             'Step by step: 1 + 5 = 6, then 6 × 2 = 12, then 12 - 3 = 9.',
             'Follow each instruction in order.',
             2),
            
            ('lag_l2_q3', 'lag_logic_001_lvl2',
             'A program has a bug! It should print: 1, 2, 3, 4, 5 but instead prints: 1, 2, 4, 5. What step is broken?',
             'multiple_choice',
             '[{"id":"a","text":"Step 1"},{"id":"b","text":"Step 2"},{"id":"c","text":"Step 3"},{"id":"d","text":"Step 5"}]',
             'c', 25,
             'The number 3 is missing, which means Step 3 (printing "3") is the broken step.',
             'Look at what output is missing.',
             3),
            
            ('lag_l2_q4', 'lag_logic_001_lvl2',
             'TRUE AND TRUE equals:',
             'multiple_choice',
             '[{"id":"a","text":"TRUE"},{"id":"b","text":"FALSE"},{"id":"c","text":"MAYBE"},{"id":"d","text":"ERROR"}]',
             'a', 20,
             'In logic, AND returns TRUE only when both values are TRUE. TRUE AND TRUE = TRUE.',
             'Both must be true for AND to be true.',
             4),
            
            ('lag_l2_q5', 'lag_logic_001_lvl2',
             'TRUE OR FALSE equals:',
             'multiple_choice',
             '[{"id":"a","text":"TRUE"},{"id":"b","text":"FALSE"},{"id":"c","text":"MAYBE"},{"id":"d","text":"ERROR"}]',
             'a', 20,
             'In logic, OR returns TRUE if at least one value is TRUE. TRUE OR FALSE = TRUE.',
             'Only one needs to be true for OR to be true.',
             5);
    "#).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    Ok(())
}

fn seed_default_user(conn: &Connection) -> Result<(), DatabaseError> {
    // First create the user
    conn.execute(
        "INSERT OR REPLACE INTO users (id, display_name, avatar_json, total_xp, current_level, cowrie_shells, current_zone)
         VALUES (1, 'Student', '{\"skin\":\"tone_3\",\"head\":\"style_1\",\"top\":\"shirt_default\",\"accessory\":null}', 0, 1, 100, 'heritage')",
        []
    ).map_err(|e| DatabaseError::QueryError(format!("Failed to create user: {}", e)))?;
    
    // Then set initial progress (Abuja unlocked for the default user)
    conn.execute(
        "INSERT OR REPLACE INTO user_progress (user_id, state_id, stars, is_completed, best_score, attempts)
         VALUES (1, 'ABJ', 0, 0, 0, 0)",
        []
    ).map_err(|e| DatabaseError::QueryError(format!("Failed to create user progress: {}", e)))?;
    
    Ok(())
}

/// Checks if the curriculum has been seeded (verifies all required data exists)
pub fn is_curriculum_seeded(conn: &Connection) -> Result<bool, DatabaseError> {
    // Check modules exist
    let module_count: i32 = conn
        .query_row("SELECT COUNT(*) FROM modules", [], |row| row.get(0))
        .unwrap_or(0);
    
    // Check user exists
    let user_count: i32 = conn
        .query_row("SELECT COUNT(*) FROM users WHERE id = 1", [], |row| row.get(0))
        .unwrap_or(0);
    
    // Check avatar items exist (new tables)
    let avatar_items_count: i32 = conn
        .query_row("SELECT COUNT(*) FROM avatar_items", [], |row| row.get(0))
        .unwrap_or(0);
    
    // Check quests exist
    let quests_count: i32 = conn
        .query_row("SELECT COUNT(*) FROM quests", [], |row| row.get(0))
        .unwrap_or(0);
    
    // Only return true if ALL required data exists
    Ok(module_count > 0 && user_count > 0 && avatar_items_count > 0 && quests_count > 0)
}

// ============================================
// THE SABI CODEX - Encyclopedia Seed Data
// ============================================

fn seed_encyclopedia(conn: &Connection) -> Result<(), DatabaseError> {
    // Seed Folklore entries
    seed_folklore_entries(conn)?;
    
    // Seed History entries
    seed_history_entries(conn)?;
    
    // Seed Famous Nigerians entries
    seed_famous_nigerians(conn)?;
    
    // Seed Culture entries
    seed_culture_entries(conn)?;
    
    log::info!("Encyclopedia entries seeded successfully");
    Ok(())
}

fn seed_folklore_entries(conn: &Connection) -> Result<(), DatabaseError> {
    conn.execute_batch(r#"
        -- =====================================================
        -- FOLKLORE ENTRIES - Tier 1 (Open) and Tier 2 (Unlockable)
        -- =====================================================
        
        -- The Tortoise and the Wisdom Pot (Tier 1 - Always accessible)
        INSERT OR REPLACE INTO encyclopedia_entries (
            id, category, title, subtitle, summary, image_url, audio_url,
            associated_state, tier, unlock_condition, xp_reward, reading_time, tags, content_md
        ) VALUES (
            'folk_tortoise_wisdom',
            'folklore',
            'The Tortoise and the Wisdom Pot',
            'A Yoruba Folktale',
            'When Tortoise tried to keep all the wisdom in the world for himself, he learned a valuable lesson about sharing.',
            '/assets/images/codex/tortoise-wisdom.png',
            '/assets/audio/tortoise-wisdom.mp3',
            'LAG',
            1,
            NULL,
            15,
            5,
            '["yoruba","folktale","wisdom","tortoise","ijapa","moral","animals"]',
            '# The Tortoise and the Wisdom Pot

## 🐢 The Story

Long ago, when the world was young and animals could speak, there lived a **clever tortoise** named Ijapa. Ijapa was known throughout the land for his cunning mind, but he was also known for his greed.

One day, the **Sky God Olorun** decided to give the gift of wisdom to the world. He gathered all the wisdom into a great **calabash** — a dried gourd container — and entrusted it to Ijapa.

*"Take this to the people,"* Olorun commanded. *"Share it fairly among all creatures."*

But Ijapa had other plans. *"If I keep all this wisdom for myself,"* he thought, *"I will be the smartest creature in all the land. Everyone will have to come to ME for answers!"*

---

## 🌳 The Iroko Tree

Ijapa decided to hide the calabash at the top of the **tallest iroko tree** in the forest, where no one could reach it.

He tied the calabash to his belly and began to climb. But with the pot in front of him, he could not get a grip on the tree trunk. He slipped and fell. He tried again and again, but each time the calabash got in the way.

His young son, who had been watching from below, called out: *"Father, why don''t you tie the calabash to your BACK? Then you can climb easily!"*

Ijapa stopped. He realized his own son — who had none of the stolen wisdom — had thought of something he had not.

---

## 💡 The Lesson

In his anger and shame, Ijapa smashed the calabash against the iroko tree. The wisdom scattered to the **four winds** and spread throughout the world.

And that is why **no one person** has all the wisdom — it belongs to everyone, a little piece here and a little piece there.

---

## 📚 What This Story Teaches Us

> **"A single hand cannot hold all the wisdom in the world."**

This Yoruba proverb reminds us that:
- 🤝 **Wisdom is meant to be shared**, not hoarded
- 👂 **Listen to others** — even young people have valuable insights
- 🧠 **Being clever is not the same as being wise**
- 💎 **Greed can make us foolish**

---

## 🎭 Cultural Artifacts

**The Calabash (Igbá)**: In Yoruba culture, the calabash represents the universe — the top half is the sky, the bottom half is earth. Breaking it symbolized releasing wisdom to all creation.

**Ijapa the Tortoise**: In Yoruba folklore, Ijapa appears in hundreds of stories, sometimes as a hero, often as a trickster whose schemes backfire. He represents the dangers of excessive cleverness without wisdom.

---

## 🌍 Did You Know?

Similar stories exist in many African cultures:
- The **Igbo** tell of "Mbe" the tortoise
- The **Hausa** have "Kunkuru" in their tales
- Across West Africa, tortoise is often the trickster figure!

---

## 🎯 Quick Quiz

What did Ijapa learn from this experience?
- [ ] That he was the smartest animal
- [x] That wisdom belongs to everyone
- [ ] That he should have climbed faster
- [ ] That calabashes are hard to carry
'
        );
        
        -- Why the Sun and Moon Live in the Sky (Tier 2 - Unlockable)
        INSERT OR REPLACE INTO encyclopedia_entries (
            id, category, title, subtitle, summary, image_url, audio_url,
            associated_state, tier, unlock_condition, xp_reward, reading_time, tags, content_md
        ) VALUES (
            'folk_sun_moon',
            'folklore',
            'Why the Sun and Moon Live in the Sky',
            'An Efik/Ibibio Folktale',
            'Discover how the Sun and Moon came to live in the sky after inviting Water to their home.',
            '/assets/images/codex/sun-moon-sky.png',
            '/assets/audio/sun-moon.mp3',
            NULL,
            2,
            'Complete Akwa Ibom Module',
            20,
            6,
            '["efik","ibibio","sun","moon","water","creation","sky","friendship"]',
            '# Why the Sun and Moon Live in the Sky

## ☀️🌙 The Story

Long ago, the **Sun** and his wife, the **Moon**, lived on Earth in a beautiful compound. They had a very dear friend called **Water**.

Every day, Sun and Moon would visit Water at his home by the river. But Water never came to visit them.

*"My friend,"* said the Sun one day, *"why do you never visit us at our home?"*

Water replied sadly, *"I would love to visit, but my family is very large. We would need an enormous compound to fit us all."*

*"Build it!"* said the Sun eagerly. *"Build the biggest compound ever, and come visit us!"*

---

## 🏠 The Grand Invitation

Sun and Moon worked day and night, building a compound larger than any seen before. The walls stretched high, the courtyard spread wide, and they were proud of what they had created.

Finally, they sent word to Water: *"Come! Our home is ready for you!"*

Water arrived at the gate. *"Are you sure you want me to come in?"* he asked.

*"Of course!"* said Sun and Moon together.

So Water began to flow into the compound...

---

## 🌊 The Flood

First came Water himself, then his **fish** family, then the **crabs**, the **sea creatures**, the **waves**, and more. Water rose to the ankles, then the knees, then the waist.

*"Should I keep coming?"* Water called out.

*"Yes, yes!"* cried Sun, standing on a chair. *"You are welcome!"*

Water kept flowing — past the windows, past the roof beams. Sun and Moon climbed to the rooftop.

*"Shall I keep coming?"* Water asked once more.

*"Yes... of course..."* Sun said weakly.

Finally, there was nowhere left to go. Sun and Moon **leapt into the sky** to escape the rising flood.

---

## ✨ Forever in the Sky

And there they have remained ever since. The **Sun** shines during the day, and the **Moon** glows at night — watching over Water from a safe distance.

They learned that some friendships are better enjoyed from afar!

---

## 📚 What This Story Teaches Us

> **"Know the nature of your guest before you open your door wide."**

- 🤔 **Think before you promise** — can you really deliver?
- 🌊 **Respect the power of nature** — Water cannot be contained
- 🏠 **Some things are too big for our homes** — and that''s okay
- ⚖️ **Balance in relationships** — not all friendships work the same way

---

## 🎭 Cultural Significance

This story from the **Efik** and **Ibibio** peoples of the **Cross River region** explains natural phenomena through human relationships. It teaches children about:
- **Hospitality** and its limits
- **Understanding consequences**
- **The relationship between sky, water, and land**

---

## 🌍 Did You Know?

The Efik people are one of Nigeria''s oldest coastal communities. The story reflects their deep connection to **water** — the rivers, creeks, and the Atlantic Ocean that shaped their lives as fishers and traders.
'
        );
        
        -- The Disobedient Daughter (Tier 1 - Hausa)
        INSERT OR REPLACE INTO encyclopedia_entries (
            id, category, title, subtitle, summary, image_url, audio_url,
            associated_state, tier, unlock_condition, xp_reward, reading_time, tags, content_md
        ) VALUES (
            'folk_disobedient_daughter',
            'folklore',
            'The Disobedient Daughter Who Married a Skull',
            'A Hausa Folktale',
            'A cautionary tale about a girl who ignored her parents'' advice and faced supernatural consequences.',
            '/assets/images/codex/skull-story.png',
            NULL,
            NULL,
            1,
            NULL,
            15,
            7,
            '["hausa","folktale","obedience","parents","supernatural","marriage","skull"]',
            '# The Disobedient Daughter Who Married a Skull

## 👻 A Hausa Cautionary Tale

Long ago in a Hausa village, there lived a beautiful young woman named **Ladi** who was known far and wide for her beauty — but also for her **stubbornness**.

Her parents, like all good parents, wished to find her a suitable husband from their village. But Ladi refused every suitor they suggested.

*"He is too short!"* she would say of one.
*"His family is not wealthy enough!"* she would say of another.
*"I want to choose my OWN husband,"* she declared.

---

## 💀 The Handsome Stranger

One day, a **magnificent stranger** appeared in the market. He was taller than any man in the village, dressed in the finest embroidered robes, and spoke with a voice like honey.

Ladi was enchanted. *"THAT is the man I will marry!"*

Her parents were worried. *"But daughter, we know nothing about him. Where does he come from? Who is his family?"*

*"I don''t care!"* Ladi replied. *"He is the most handsome man I have ever seen."*

Against her parents'' wishes, she agreed to follow the stranger to his home.

---

## 🛤️ The Journey

As they walked deeper into the forest, strange things began to happen.

The handsome stranger stopped at a crossroads. *"This is where I borrowed these fine robes,"* he said — and his clothes disappeared, leaving him in rags.

They walked further. *"This is where I borrowed these long legs,"* he said — and suddenly he was short.

Again: *"This is where I borrowed this handsome face."*

Ladi watched in horror as the flesh fell away, revealing **nothing but a SKULL** — a skull that floated in the air, grinning at her.

*"Come, wife,"* said the skull. *"We are almost home."*

---

## 🏃‍♀️ The Escape

Ladi''s screams echoed through the forest. She ran as fast as she could back toward her village, the skull floating after her.

But an old woman appeared on the path — a **wise spirit** who took pity on the terrified girl.

*"Why do you run?"* asked the old woman.

Between sobs, Ladi told her story.

*"You did not listen to your parents,"* said the old woman. *"But perhaps you have learned."* She gave Ladi a magic charm and showed her a secret path home.

---

## 💡 The Lesson

Ladi returned to her village, humbled and grateful. She apologized to her parents and became known for her wisdom and her willingness to listen to good advice.

---

## 📚 What This Story Teaches Us

> **"Uwar mugu, uban mugu — sun fi uwar kirki, uban kirki."**
> *(A cruel mother and father are better than kind strangers.)*

This Hausa proverb means:
- 👨‍👩‍👧 **Parents want what''s best for you** — even when it feels restrictive
- 👀 **Appearances can deceive** — beauty is not the same as goodness
- 🏠 **Know someone before trusting them** with your future
- 🎭 **True character is revealed over time**, not at first sight

---

## 🎭 Cultural Note

This is a **tatsuniya** (plural: tatsuniyoyi) — a type of Hausa folktale traditionally told by grandmothers to children at night. These stories always end with moral lessons about proper behavior, respect for elders, and the dangers of the unknown world.
'
        );
        
        -- The Spider and the Feast (Anansi-style, Igbo version)
        INSERT OR REPLACE INTO encyclopedia_entries (
            id, category, title, subtitle, summary, image_url, audio_url,
            associated_state, tier, unlock_condition, xp_reward, reading_time, tags, content_md
        ) VALUES (
            'folk_spider_feast',
            'folklore',
            'Ududo the Spider''s Greedy Feast',
            'An Igbo Spider Tale',
            'When Ududo tries to attend two feasts at once, his greed catches up with him in an unexpected way.',
            '/assets/images/codex/spider-feast.png',
            NULL,
            NULL,
            1,
            NULL,
            15,
            5,
            '["igbo","spider","ududo","greed","feast","humor","trickster"]',
            '# Ududo the Spider''s Greedy Feast

## 🕷️ An Igbo Tale of Greed

In the days when animals could speak, there lived a spider named **Ududo** who was known throughout the land for two things: his **cleverness** and his **enormous appetite**.

One morning, Ududo received wonderful news: TWO villages were hosting feasts on the SAME day!

*"This is perfect!"* Ududo exclaimed, rubbing his eight legs together. *"I shall eat at BOTH feasts and have twice as much food as anyone else!"*

---

## 🍖 The Plan

But there was a problem — the two villages were in **opposite directions**, and Ududo didn''t know which feast would begin first.

Being clever (or so he thought), Ududo came up with a plan.

He called his two sons. *"I want each of you to go to one of the villages. Tie a rope around my waist before you go. When the feast begins in your village, PULL the rope, and I will come running!"*

His sons did as they were told, and Ududo sat in the middle of the crossroads, waiting.

---

## 😰 The Tug of War

As luck would have it, **BOTH feasts began at exactly the same time**.

Both sons pulled their ropes.

Ududo felt himself being yanked in BOTH directions at once! The ropes pulled and pulled, squeezing his middle tighter and tighter.

*"Stop! STOP!"* Ududo cried, but his sons were too far away to hear.

---

## 🕸️ The Consequence

When the sons finally returned, they found their father lying in the road, exhausted — and with a **very thin waist** where the ropes had squeezed him.

Ududo survived, but from that day forward, **all spiders have tiny waists** connecting their front and back sections.

And Ududo? He missed BOTH feasts entirely and went to bed hungry that night.

---

## 📚 What This Story Teaches Us

> **"Onye hụrụ nwa ya ukwu, ọ bụrụ ọgba aghara."**
> *(He who wants to do two things at once will do neither well.)*

- 🍽️ **Greed leads to loss** — trying to have everything, you may end up with nothing
- 🎯 **Focus on one goal** — divided attention brings divided results
- 🤣 **Sometimes cleverness backfires** — simple honesty is often better

---

## 🎭 Why Spiders Have Thin Waists

This "pourquoi" story (a story that explains why things are the way they are) is common across Africa. It uses humor to teach lessons while also explaining natural phenomena — like the spider''s distinctive body shape!

---

## 🌍 Did You Know?

Spider trickster tales appear in many cultures:
- **Anansi** in Akan/Ashanti stories (Ghana)
- **Ududo** in Igbo tradition
- **Gizo** in Hausa folklore

These clever spiders traveled with enslaved Africans to the Americas, where Anansi stories remain popular today!
'
        );
    "#).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    Ok(())
}

fn seed_history_entries(conn: &Connection) -> Result<(), DatabaseError> {
    conn.execute_batch(r#"
        -- =====================================================
        -- HISTORY ENTRIES - Nigerian Historical Events
        -- =====================================================
        
        -- Independence Day (Tier 1 - Always accessible)
        INSERT OR REPLACE INTO encyclopedia_entries (
            id, category, title, subtitle, summary, image_url, audio_url,
            associated_state, tier, unlock_condition, xp_reward, reading_time, tags, content_md
        ) VALUES (
            'hist_independence',
            'history',
            'October 1, 1960: Nigeria''s Independence',
            'The Day Nigeria Became Free',
            'Learn about the historic moment when Nigeria gained independence from British colonial rule.',
            '/assets/images/codex/independence-day.png',
            '/assets/audio/independence.mp3',
            NULL,
            1,
            NULL,
            20,
            8,
            '["independence","1960","british","colonial","tafawa balewa","nnamdi azikiwe","history"]',
            '# October 1, 1960: Nigeria''s Independence

## 🇳🇬 The Birth of a Nation

On **October 1, 1960**, at exactly **midnight**, the British Union Jack was lowered for the last time, and the **green-white-green** flag of Nigeria was raised. After nearly 100 years of colonial rule, Nigeria was finally **FREE**.

> *"Though tribe and tongue may differ, in brotherhood we stand."*
> — From the Nigerian National Anthem (1960-1978)

---

## 📜 The Road to Freedom

### Early Colonial Period (1861-1914)
Britain first arrived in Nigeria in the 1800s, starting with Lagos in 1861. By 1914, they had combined the Northern and Southern regions into one country — **"Nigeria"** — a name suggested by British journalist Flora Shaw.

### The Rise of Nationalism (1920s-1950s)
Nigerian leaders began demanding independence:
- **Herbert Macaulay** — "Father of Nigerian Nationalism"
- **Nnamdi Azikiwe** — "Zik of Africa"
- **Obafemi Awolowo** — Leader of the Action Group
- **Ahmadu Bello** — Sardauna of Sokoto

### Constitutional Conferences (1957-1959)
Nigerian leaders met with British officials to plan the transition to independence, including the **1957 Constitutional Conference** in London.

---

## 🎉 Independence Day: What Happened

### The Ceremony
- 🌙 **Midnight, September 30/October 1, 1960**
- 📍 **Location**: Race Course, Lagos (now Tafawa Balewa Square)
- 👥 **50,000 Nigerians** attended
- 🇬🇧 **Princess Alexandra** represented Queen Elizabeth II

### The New Government
- **Prime Minister**: Sir Abubakar Tafawa Balewa
- **Governor-General**: Dr. Nnamdi Azikiwe (representing the Queen)
- **Premier of the North**: Sir Ahmadu Bello
- **Premier of the West**: Chief Obafemi Awolowo
- **Premier of the East**: Dr. Michael Okpara

---

## 🗣️ Historic Words

Prime Minister Tafawa Balewa declared:

> *"This is a wonderful day, and it is all the more wonderful because we have achieved our freedom without a bloodshed. We are grateful to the British Officers who served us so faithfully... Let us forget our differences... We are one."*

---

## 📊 Then vs Now

| **1960** | **Today** |
|----------|-----------|
| Population: 45 million | Population: 220+ million |
| 3 Regions | 36 States + FCT |
| Agriculture-based economy | Oil & diverse economy |
| Few universities | 170+ universities |
| Lagos as capital | Abuja as capital (since 1991) |

---

## 🎯 Why Independence Matters

Independence Day reminds us that:
- 🦁 **Nigerians fought for freedom** through peaceful negotiation
- 🤝 **Unity in diversity** is possible
- 📚 **Education and organization** were key to freedom
- 🌟 **Every generation** must work to build the nation

---

## 🎭 Cultural Note

Every October 1st, Nigeria celebrates with:
- 🎌 Flag raising ceremonies
- 🎖️ Presidential addresses
- 🎪 Parades and cultural displays
- 🎵 Musical performances

---

## 🎯 Quick Quiz

Who lowered the British flag at the independence ceremony?
- [ ] Queen Elizabeth II
- [x] A British official as Princess Alexandra watched
- [ ] Nnamdi Azikiwe
- [ ] Tafawa Balewa
'
        );
        
        -- The Benin Empire (Tier 2 - Unlockable)
        INSERT OR REPLACE INTO encyclopedia_entries (
            id, category, title, subtitle, summary, image_url, audio_url,
            associated_state, tier, unlock_condition, xp_reward, reading_time, tags, content_md
        ) VALUES (
            'hist_benin_empire',
            'history',
            'The Great Benin Empire',
            'When Benin Amazed the World (1180-1897)',
            'Discover the powerful kingdom that created the famous Benin Bronzes and amazed European visitors.',
            '/assets/images/codex/benin-empire.png',
            NULL,
            NULL,
            2,
            'Visit Edo State',
            25,
            10,
            '["benin","empire","edo","bronzes","oba","art","walls","history"]',
            '# The Great Benin Empire

## 🏰 A Kingdom That Amazed the World

When Portuguese explorers first visited **Benin City** in 1485, they were shocked. They expected to find a small village — instead, they found a **magnificent city** with wide streets, grand buildings, and a palace larger than any in Europe.

The Benin Empire (c. 1180-1897) was one of the most **advanced civilizations** in pre-colonial Africa.

---

## 👑 The Obas of Benin

The kingdom was ruled by the **Oba** (King), who was considered semi-divine — a living connection between the people and their ancestors.

### Notable Obas:
- **Eweka I** (c. 1180) — First Oba, founded the current dynasty
- **Ewuare the Great** (c. 1440-1480) — Expanded the empire, built the famous walls
- **Esigie** (c. 1504-1547) — Established relations with Portugal, promoted arts

---

## 🎨 The Benin Bronzes

The empire is world-famous for the **Benin Bronzes** — thousands of metal sculptures made using the lost-wax casting technique.

These weren''t just decorations — they were:
- 📚 **Historical records** — documenting kings, wars, and ceremonies
- 👑 **Symbols of power** — decorating the palace
- 🙏 **Religious objects** — honoring ancestors

> The quality of these bronzes amazed Europeans. Some called them the finest metal sculptures ever made.

### Where Are They Now?
In 1897, British forces invaded Benin and took thousands of bronzes. Today, many are in museums worldwide, though **Nigeria is working to bring them home**.

---

## 🧱 The Walls of Benin

Perhaps more impressive than the bronzes were the **Walls of Benin** — the largest earthwork structure built before the modern era.

**Statistics:**
- 🏗️ **16,000 km** of walls in total (longer than the Great Wall of China!)
- 📏 Some walls were **20 meters high**
- 🚧 Took **several hundred years** to build
- 🛡️ Protected the city and surrounding villages

The Guinness Book of Records recognized it as **"the largest earthwork in the world."**

---

## 🌍 A Trading Power

Benin was a major trading hub:
- 🌶️ **Pepper** and palm oil to Europe
- 🧵 **Textiles** and cloths
- 🐚 **Ivory** carvings
- 🔩 **Metal goods**

The Portuguese were so impressed they established a trading post and embassy!

---

## 📉 The Fall of Benin (1897)

In February 1897, British forces invaded after a conflict over trade. The city was:
- 🔥 Burned
- 🏛️ Looted of thousands of artworks
- 👑 The Oba was exiled

This is known as the **Benin Punitive Expedition**.

---

## ✊ Legacy Today

The Benin Empire lives on:
- 👑 The **Oba of Benin** still holds cultural authority
- 🎨 **Bronze casting** continues as traditional art
- 🏛️ **Benin City** is the capital of Edo State
- 🔄 The **repatriation movement** is bringing bronzes home

---

## 📚 What This Teaches Us

- 🌟 **African civilizations** were advanced long before European contact
- 🎨 **Art and technology** can achieve incredible heights
- 📜 **History** can be preserved through visual art
- 💪 **Cultural pride** survives even after conquest
'
        );
    "#).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    Ok(())
}

fn seed_famous_nigerians(conn: &Connection) -> Result<(), DatabaseError> {
    conn.execute_batch(r#"
        -- =====================================================
        -- FAMOUS NIGERIANS ENTRIES
        -- =====================================================
        
        -- Wole Soyinka (Tier 1)
        INSERT OR REPLACE INTO encyclopedia_entries (
            id, category, title, subtitle, summary, image_url, audio_url,
            associated_state, tier, unlock_condition, xp_reward, reading_time, tags, content_md
        ) VALUES (
            'famous_wole_soyinka',
            'famous_nigerians',
            'Wole Soyinka',
            'Africa''s First Nobel Laureate in Literature',
            'Meet the playwright, poet, and activist who became the first African to win the Nobel Prize in Literature.',
            '/assets/images/codex/wole-soyinka.png',
            NULL,
            'OGU',
            1,
            NULL,
            20,
            7,
            '["wole soyinka","nobel prize","literature","playwright","poet","ogun","abeokuta"]',
            '# Wole Soyinka

## 📚 Africa''s Literary Giant

**Akinwande Oluwole "Wole" Soyinka** (born July 13, 1934) became the **first African** to win the **Nobel Prize in Literature** in 1986.

> *"The man dies in all who keep silent in the face of tyranny."*
> — Wole Soyinka

---

## 🎭 Career Card

| **Field** | **Achievement** |
|-----------|-----------------|
| 🎓 **Education** | University of Ibadan, University of Leeds |
| ✍️ **Profession** | Playwright, Poet, Novelist, Essayist |
| 🏆 **Major Award** | Nobel Prize in Literature (1986) |
| 🌍 **Impact** | First African Nobel laureate in Literature |

---

## 📖 Early Life

Soyinka was born in **Abeokuta, Ogun State** into a Yoruba family. His father was a school headmaster, and his mother was a shopkeeper and activist whom he nicknamed **"Wild Christian"**.

From an early age, Soyinka showed:
- 📚 A love for books and writing
- 🎭 Interest in traditional Yoruba stories
- ✊ A willingness to stand up for what''s right

---

## 🎪 Famous Works

### Plays
- **The Lion and the Jewel** (1959) — Comedy about tradition vs. modernity
- **Death and the King''s Horseman** (1975) — Based on real events, his masterpiece
- **A Dance of the Forests** (1960) — Written for Nigeria''s independence

### Books
- **The Interpreters** (1965) — Novel about young Nigerian intellectuals
- **Aké: The Years of Childhood** (1981) — Beloved memoir of his youth

---

## ✊ Activism & Prison

Soyinka has never just been a writer — he''s also a **fighter for justice**:

- **1967**: Imprisoned for 22 months during the Nigerian Civil War for trying to make peace
- **1994**: Fled Nigeria and criticized military dictator General Abacha
- **1997**: Sentenced to death in absentia (sentence later lifted)
- **Present**: Continues speaking against corruption and injustice

---

## 🏆 The Nobel Prize

In **1986**, Soyinka received the Nobel Prize for Literature. The committee praised him for creating works that:

> *"...in a wide cultural perspective and with poetic overtones, fashions the drama of existence."*

He donated part of his prize money to **theater development** in Africa.

---

## 💡 What Makes Him Special

- 🌍 **Bridged cultures** — combined Yoruba mythology with Western drama
- ✊ **Used art for change** — his writing challenged injustice
- 🎓 **Inspired generations** — opened doors for African writers worldwide
- 🎭 **Multiple talents** — playwright, poet, novelist, activist, professor

---

## 📚 Lesson from Soyinka

> **"Art must confront tyranny."**

Soyinka shows us that:
- ✍️ **Words have power** to change society
- 🦁 **Courage matters** — speak truth even when dangerous
- 🎭 **Culture is strength** — our traditions inspire great art
'
        );
        
        -- Funmilayo Ransome-Kuti (Tier 1)
        INSERT OR REPLACE INTO encyclopedia_entries (
            id, category, title, subtitle, summary, image_url, audio_url,
            associated_state, tier, unlock_condition, xp_reward, reading_time, tags, content_md
        ) VALUES (
            'famous_funmilayo_kuti',
            'famous_nigerians',
            'Funmilayo Ransome-Kuti',
            'Mother of African Feminism',
            'The fearless activist who fought for women''s rights and challenged both colonial and Nigerian authorities.',
            '/assets/images/codex/funmilayo-kuti.png',
            NULL,
            'OGU',
            1,
            NULL,
            20,
            8,
            '["funmilayo","ransome-kuti","feminism","women","activism","abeokuta","fela"]',
            '# Funmilayo Ransome-Kuti

## ✊ The Lioness of Lisabi

**Chief Funmilayo Ransome-Kuti** (1900-1978) was a Nigerian educator, political campaigner, and women''s rights activist who became known as the **"Mother of Africa"** and the **"Lioness of Lisabi"**.

> *"We are not asking for the right to be equal to men. We are asking for our due rights as women."*

---

## 🎭 Career Card

| **Field** | **Achievement** |
|-----------|-----------------|
| 🎓 **Education** | Abeokuta Grammar School, England |
| ✍️ **Profession** | Teacher, Activist, Political Leader |
| ✊ **Organization** | Nigerian Women''s Union |
| 🏆 **Recognition** | Lenin Peace Prize (1970) |

---

## 📖 Early Life & Education

Born **Frances Abigail Olufunmilayo Thomas** in Abeokuta on October 25, 1900, she was:
- 🏫 One of the **first girls** to attend Abeokuta Grammar School
- 🇬🇧 Studied in **England** (where she dropped her English name "Frances")
- 💑 Married Rev. Israel Oludotun Ransome-Kuti, a school principal

She was the mother of **Fela Kuti** (Afrobeat legend), **Beko Ransome-Kuti** (human rights activist), and **Olikoye Ransome-Kuti** (health minister).

---

## 💪 The Abeokuta Women''s Revolt (1947-1949)

Funmilayo''s greatest achievement was organizing the **Abeokuta Women''s Union** (with over 20,000 members!) to protest against:
- 💰 **Unfair taxes** on market women
- 👑 **Corrupt local chiefs**
- 🇬🇧 **Colonial exploitation**

### What They Did:
- 📢 **Organized protests** outside the palace of the Alake (king)
- 🚶‍♀️ **Led marches** of thousands of women
- 🎵 **Used songs** to spread their message
- ⚖️ **Petitioned** the British colonial government

### The Result:
The **Alake was forced to abdicate** (temporarily), and women gained **representation** in local government. This was the first time organized women''s protest succeeded in Nigerian history!

---

## 🌍 International Recognition

Funmilayo traveled the world to fight for women''s rights:
- 🇨🇳 Visited **China** (1956) — met with Mao Zedong
- 🇷🇺 Visited **Soviet Union** — received the Lenin Peace Prize
- 🇬🇧 Spoke at **British Parliament**
- 🌍 Attended the **World Congress of Women**

---

## 💔 Tragic End

In 1977, soldiers attacked her son Fela''s commune (Kalakuta Republic). The 77-year-old Funmilayo was thrown from a window by soldiers. She never recovered and died in 1978 from her injuries.

---

## 🏆 Legacy

- ✊ **Pioneer of women''s rights** in Nigeria
- 🗳️ Fought for **women''s suffrage** (right to vote)
- 📚 Proved that **organized action** brings change
- 💪 Showed that **women can lead**

---

## 📚 What She Teaches Us

> **"I fight for what I believe in."**

- ✊ **Stand up for the vulnerable**
- 🤝 **Organize collectively** — together we''re stronger
- 🦁 **Courage has no age** — she protested into her 70s
- 👩‍👧 **Mother and activist** — you can be both
'
        );
        
        -- Chinua Achebe (Tier 2)
        INSERT OR REPLACE INTO encyclopedia_entries (
            id, category, title, subtitle, summary, image_url, audio_url,
            associated_state, tier, unlock_condition, xp_reward, reading_time, tags, content_md
        ) VALUES (
            'famous_chinua_achebe',
            'famous_nigerians',
            'Chinua Achebe',
            'Father of Modern African Literature',
            'The author of "Things Fall Apart" changed how the world sees African literature and culture.',
            '/assets/images/codex/chinua-achebe.png',
            NULL,
            NULL,
            2,
            'Complete 3 Folklore entries',
            25,
            8,
            '["chinua achebe","things fall apart","literature","igbo","novelist","author"]',
            '# Chinua Achebe

## 📖 Father of Modern African Literature

**Albert Chinualumogu Achebe** (1930-2013) is considered the **father of modern African literature**. His novel *Things Fall Apart* has sold over **20 million copies** and been translated into **60+ languages**.

> *"Until the lions have their own historians, the history of the hunt will always glorify the hunter."*
> — Chinua Achebe

---

## 🎭 Career Card

| **Field** | **Achievement** |
|-----------|-----------------|
| 🎓 **Education** | University of Ibadan |
| ✍️ **Profession** | Novelist, Poet, Professor, Critic |
| 📚 **Most Famous Work** | Things Fall Apart (1958) |
| 🌍 **Impact** | Changed world perception of African literature |

---

## 📖 Early Life

Born November 16, 1930 in **Ogidi, Anambra State** to Christian Igbo parents:
- 👨‍👩‍👦 His father was a catechist (church teacher)
- 📚 He grew up between traditional Igbo culture and Christianity
- 🏫 Attended Government College Umuahia (one of Nigeria''s best schools)
- 🎓 Was among the first graduates of University College, Ibadan

---

## 📕 Things Fall Apart (1958)

This masterpiece tells the story of **Okonkwo**, a respected leader in an Igbo village, as colonialism arrives and transforms his world.

### Why It Matters:
- 📜 **Told Africa''s story** from an African perspective
- 🔄 **Challenged racist stereotypes** in Western literature
- 🎭 **Showed complexity** of pre-colonial African societies
- 📚 **Required reading** in schools worldwide

### Famous Opening:
> *"Okonkwo was well known throughout the nine villages and even beyond..."*

---

## 📚 Other Important Works

- **No Longer at Ease** (1960) — Corruption in modern Nigeria
- **Arrow of God** (1964) — Traditional religion vs. Christianity
- **A Man of the People** (1966) — Political corruption (predicted the 1966 coup!)
- **There Was a Country** (2012) — Memoir of the Biafran War

---

## 🎖️ The Heart of Darkness Controversy

Achebe was famous for his 1975 lecture criticizing Joseph Conrad''s *Heart of Darkness* as racist. He argued that:
- 📖 The novel portrayed Africa as **"the other"**
- 🗣️ Africans were **denied voices** in the story
- 🧠 Western literature needed **African perspectives**

This critique changed how literature is taught worldwide.

---

## ✊ Civil War & Politics

During the **Nigerian Civil War** (1967-1970):
- 🇧🇫 Supported **Biafra** (the breakaway Igbo state)
- 🌍 Traveled as a **diplomat** for Biafra
- 📰 Wrote poems about the war''s tragedy
- 💔 Witnessed the death of his close friend, poet **Christopher Okigbo**

---

## 🏆 Awards & Recognition

- 📚 **Man Booker International Prize** (2007) — Lifetime achievement
- 🏛️ **Nigerian National Order of Merit** — Nigeria''s highest intellectual honor
- 🎓 **30+ honorary doctorates** from universities worldwide
- ❌ **Turned down national honors twice** to protest government corruption

---

## 📚 What He Teaches Us

> **"Writers don''t give prescriptions. They give headaches."**

- ✍️ **Tell your own story** — don''t let others define you
- 🌍 **Challenge stereotypes** with truth
- 📖 **Literature can change minds**
- ✊ **Integrity matters** — he refused honors from corrupt governments
'
        );
    "#).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    Ok(())
}

fn seed_culture_entries(conn: &Connection) -> Result<(), DatabaseError> {
    conn.execute_batch(r#"
        -- =====================================================
        -- CULTURE ENTRIES - Nigerian Cultural Practices
        -- =====================================================
        
        -- Nigerian Traditional Attire (Tier 1)
        INSERT OR REPLACE INTO encyclopedia_entries (
            id, category, title, subtitle, summary, image_url, audio_url,
            associated_state, tier, unlock_condition, xp_reward, reading_time, tags, content_md
        ) VALUES (
            'culture_traditional_attire',
            'culture',
            'Traditional Nigerian Attire',
            'Wearing Culture with Pride',
            'Explore the rich variety of traditional clothing across Nigeria''s diverse ethnic groups.',
            '/assets/images/codex/traditional-attire.png',
            NULL,
            NULL,
            1,
            NULL,
            15,
            6,
            '["attire","fashion","agbada","buba","wrapper","gele","clothing","culture"]',
            '# Traditional Nigerian Attire

## 👗 Wearing Culture with Pride

Nigeria''s traditional clothing is as diverse as its people — with each ethnic group having distinctive styles that reflect their history, values, and artistic traditions.

> *"Clothes do not just cover the body; they tell a story."*

---

## 🎭 Yoruba Attire (South West)

### For Men: Agbada
- 📏 **Wide-sleeved** flowing robe
- 🧵 Worn over **buba** (inner shirt) and **sokoto** (trousers)
- 🎩 Completed with **fila** (cap)
- 💎 Often richly embroidered

### For Women: Iro and Buba
- 👚 **Buba** — Loose blouse
- 🎀 **Iro** — Wrapper tied around the waist
- 👑 **Gele** — Elaborate headwrap (can take 20 minutes to tie!)
- 🌈 Often made from **Aso-Oke** (hand-woven cloth)

---

## 🎭 Igbo Attire (South East)

### For Men: Isiagu
- 🦁 Features **lion head patterns**
- 👔 Worn with **trousers** or **wrapper**
- 🎩 **Red cap** (ochirikpa) shows title status
- 🪶 **Beads** and coral necklaces for chiefs

### For Women: George Wrapper
- ✨ **Shimmering** fabric (originally from India!)
- 👚 Two pieces: wrapper and blouse
- 💍 Heavy **coral beads** (essential for married women)
- 👑 Elaborate headgear for ceremonies

---

## 🎭 Hausa/Fulani Attire (North)

### For Men: Babban Riga
- 📐 **Flowing robe** with wide sleeves
- 🧢 **Hula** cap (small, round)
- 🎨 Often **white** or with intricate embroidery
- 📿 Simple but elegant

### For Women: Abaya and Hijab
- 👗 **Long, flowing** dresses
- 🧕 **Head covering** (hijab or turban)
- 💍 Gold jewelry and henna decorations
- 🌺 Often in bright, beautiful colors

---

## 🎨 Special Fabrics

| **Fabric** | **Origin** | **Used For** |
|------------|------------|--------------|
| **Aso-Oke** | Yoruba | Ceremonies, weddings |
| **Adire** | Yoruba | Tie-dye everyday wear |
| **Akwete** | Igbo | Traditional ceremonies |
| **Ankara** | All Nigeria | Modern fashion |
| **George** | Igbo (origin India) | Special occasions |

---

## 🎉 When Traditional Attire is Worn

- 💒 **Weddings** — Most elaborate outfits
- 🎂 **Naming ceremonies**
- 👑 **Chieftaincy titles**
- 🕌 **Religious festivals**
- 🇳🇬 **National holidays**
- 📅 **Fridays** — Many offices have "Traditional Attire Fridays"

---

## 💡 Fashion Tips

- 🎨 **Mix traditional and modern** — it''s very Nigerian!
- 🧵 **Quality matters** — good fabric lasts longer
- 👗 **Fit is important** — get a good tailor
- 🌈 **Express yourself** — colors show personality

---

## 📚 What Traditional Attire Teaches Us

- 🎭 **Identity** — Clothes connect us to our heritage
- 🤝 **Unity in diversity** — Each group has unique beauty
- 🎨 **Creativity** — Nigerian fashion influences the world
- 💪 **Pride** — Wearing traditional clothes celebrates who we are
'
        );
        
        -- Nigerian Food Culture (Tier 1)
        INSERT OR REPLACE INTO encyclopedia_entries (
            id, category, title, subtitle, summary, image_url, audio_url,
            associated_state, tier, unlock_condition, xp_reward, reading_time, tags, content_md
        ) VALUES (
            'culture_nigerian_food',
            'culture',
            'Nigerian Food Culture',
            'A Feast for the Senses',
            'Discover the delicious variety of Nigerian cuisine, from jollof rice to pounded yam.',
            '/assets/images/codex/nigerian-food.png',
            NULL,
            NULL,
            1,
            NULL,
            15,
            7,
            '["food","cuisine","jollof","pounded yam","egusi","suya","cooking","culture"]',
            '# Nigerian Food Culture

## 🍚 A Feast for the Senses

Nigerian cuisine is as diverse as its 250+ ethnic groups — rich, flavorful, and always meant to be **shared**. Food in Nigeria isn''t just about eating; it''s about **community**.

> *"Food that is not shared has no blessing."*
> — Nigerian Proverb

---

## 🍛 The Famous Dishes

### 🍅 Jollof Rice
The **king of Nigerian parties**! A one-pot rice dish cooked in tomato sauce with peppers and spices.

- 🔥 **Origin**: West Africa (Nigeria, Ghana, Senegal all claim it!)
- 🎉 **Served at**: Every celebration
- 🏆 **The Debate**: Ghana vs. Nigeria — whose jollof is better? (It''s Nigeria''s 😉)

### 🥣 Pounded Yam & Egusi Soup
- **Pounded Yam**: Smooth, stretchy, eaten with fingers
- **Egusi Soup**: Made from melon seeds, vegetables, and meat
- **How to eat**: Pinch a small ball, dip in soup, swallow!

### 🍖 Suya
Spiced grilled meat on skewers — Nigeria''s favorite street food!
- 🌶️ **Yaji**: The special spice mix (groundnuts, ginger, peppers)
- 🌙 **Best time**: Evening, from roadside vendors
- 🧅 **Served with**: Sliced onions, cabbage, tomatoes

---

## 🗺️ Regional Specialties

| **Region** | **Famous Dishes** |
|------------|-------------------|
| **South West (Yoruba)** | Amala, Ewedu, Gbegiri |
| **South East (Igbo)** | Ofe Nsala, Abacha (African Salad) |
| **South South** | Afang Soup, Banga Soup, Fisherman Soup |
| **North** | Tuwo Shinkafa, Miyan Kuka, Kilishi |
| **Middle Belt** | Masa, Tuwo Masara |

---

## 🍲 Soups & Stews

Nigerian soups are **thick and hearty** — nothing like Western soups!

- **Egusi** — Melon seed soup with vegetables
- **Okra** — Slimy but delicious!
- **Ogbono** — Draw soup (very stretchy)
- **Efo Riro** — Spinach stew
- **Bitterleaf** — Named for the leaf (wash it well!)
- **Pepper Soup** — Spicy broth, great for colds

---

## 🥤 Drinks

- 🌴 **Palm Wine** — Fresh from the palm tree
- 🌿 **Zobo** — Hibiscus drink (red and tangy)
- 🥜 **Kunu** — Millet-based drink
- 🍊 **Chapman** — Nigerian cocktail/mocktail

---

## 🍽️ Eating Culture

### **Hospitality Rules:**
- 👐 **Always offer food** to guests
- 🍚 **Serve elders first**
- 🤲 **Eating with right hand** is traditional
- 🙏 **Never finish everything** — leave some for the cook!

### **Special Occasion Foods:**
- 🎂 **Weddings**: Jollof rice, small chops, cake
- 🍼 **Naming ceremonies**: Honey, kola nuts, palm wine
- ⚰️ **Funerals**: Plenty of food for mourners
- 🎄 **Christmas**: Rice, chicken, fried meat

---

## 🧑‍🍳 Kitchen Wisdom

- 🌶️ **"No pepper, no food"** — Nigerians love spice!
- 🍲 **One pot = less washing**
- 🥘 **Cook in bulk** — always ready for visitors
- 💪 **Food must be filling** — light food is not "real food"

---

## 📚 What Nigerian Food Teaches Us

- 🤝 **Community** — Food is always shared
- 🎨 **Creativity** — Same ingredients, many dishes
- 💚 **Pride** — Nigerian food is recognized worldwide
- 🌍 **Connection** — Food links us to home, wherever we are
'
        );
    "#).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    Ok(())
}

// =====================================================
// AVATAR ITEMS SEEDING
// =====================================================

fn seed_avatar_items(conn: &Connection) -> Result<(), DatabaseError> {
    conn.execute_batch(r#"
        -- =====================================================
        -- SKIN TONES (All starter/free)
        -- =====================================================
        INSERT OR REPLACE INTO avatar_items (id, category, name, description, image_key, rarity, unlock_cost, sort_order)
        VALUES 
            ('skin_1', 'skin_tone', 'Light Brown', 'A warm light brown skin tone', 'skin_1', 'starter', 0, 1),
            ('skin_2', 'skin_tone', 'Golden Brown', 'A rich golden brown skin tone', 'skin_2', 'starter', 0, 2),
            ('skin_3', 'skin_tone', 'Medium Brown', 'A classic medium brown skin tone', 'skin_3', 'starter', 0, 3),
            ('skin_4', 'skin_tone', 'Dark Brown', 'A deep dark brown skin tone', 'skin_4', 'starter', 0, 4),
            ('skin_5', 'skin_tone', 'Ebony', 'A beautiful ebony skin tone', 'skin_5', 'starter', 0, 5),
            ('skin_6', 'skin_tone', 'Chocolate', 'A rich chocolate skin tone', 'skin_6', 'starter', 0, 6);

        -- =====================================================
        -- HAIRSTYLES
        -- =====================================================
        INSERT OR REPLACE INTO avatar_items (id, category, name, description, image_key, rarity, unlock_cost, sort_order)
        VALUES 
            -- Starter hairstyles (free)
            ('hair_1', 'hairstyle', 'Low Cut', 'Clean and fresh low cut fade', 'hair_low_cut', 'starter', 0, 1),
            ('hair_2', 'hairstyle', 'Afro', 'Classic natural afro', 'hair_afro', 'starter', 0, 2),
            ('hair_3', 'hairstyle', 'Braids', 'Traditional cornrow braids', 'hair_braids', 'starter', 0, 3),
            ('hair_4', 'hairstyle', 'Twist Out', 'Natural twist out style', 'hair_twist', 'starter', 0, 4),
            -- Unlockable hairstyles
            ('hair_5', 'hairstyle', 'Gele', 'Elegant traditional headwrap', 'hair_gele', 'rare', 100, 5),
            ('hair_6', 'hairstyle', 'Dreadlocks', 'Stylish dreadlocks', 'hair_dreads', 'common', 50, 6),
            ('hair_7', 'hairstyle', 'Bantu Knots', 'Beautiful Bantu knots', 'hair_bantu', 'rare', 80, 7),
            ('hair_8', 'hairstyle', 'Fulani Braids', 'Elegant Fulani braids with beads', 'hair_fulani', 'epic', 150, 8);

        -- =====================================================
        -- OUTFITS
        -- =====================================================
        INSERT OR REPLACE INTO avatar_items (id, category, name, description, image_key, rarity, unlock_cost, sort_order)
        VALUES 
            -- Starter outfits (free)
            ('outfit_school', 'outfit', 'School Uniform', 'Nigerian secondary school uniform', 'outfit_school', 'starter', 0, 1),
            ('outfit_casual', 'outfit', 'Casual Wear', 'Comfortable everyday clothes', 'outfit_casual', 'starter', 0, 2),
            -- Traditional outfits (unlockable)
            ('outfit_ankara_1', 'outfit', 'Ankara Casual', 'Colorful Ankara print casual wear', 'outfit_ankara1', 'common', 50, 3),
            ('outfit_ankara_2', 'outfit', 'Ankara Formal', 'Elegant Ankara formal attire', 'outfit_ankara2', 'rare', 100, 4),
            ('outfit_agbada', 'outfit', 'Agbada', 'Flowing Yoruba ceremonial robe', 'outfit_agbada', 'epic', 200, 5),
            ('outfit_isiagu', 'outfit', 'Isiagu', 'Igbo lion-patterned attire', 'outfit_isiagu', 'epic', 200, 6),
            ('outfit_kaftan', 'outfit', 'Kaftan', 'Northern Nigerian formal kaftan', 'outfit_kaftan', 'epic', 200, 7),
            ('outfit_aso_oke', 'outfit', 'Aso Oke', 'Premium hand-woven ceremonial cloth', 'outfit_asooke', 'legendary', 500, 8),
            ('outfit_buba', 'outfit', 'Buba & Iro', 'Traditional Yoruba female attire', 'outfit_buba', 'rare', 120, 9),
            ('outfit_george', 'outfit', 'George Wrapper', 'Elegant Igbo ceremonial fabric', 'outfit_george', 'epic', 250, 10),
            ('outfit_senator', 'outfit', 'Senator Style', 'Modern Nigerian formal wear', 'outfit_senator', 'rare', 150, 11),
            ('outfit_hausa_f', 'outfit', 'Abaya & Hijab', 'Modest Northern female attire', 'outfit_abaya', 'rare', 120, 12);

        -- =====================================================
        -- ACCESSORIES
        -- =====================================================
        INSERT OR REPLACE INTO avatar_items (id, category, name, description, image_key, rarity, unlock_cost, sort_order)
        VALUES 
            ('acc_cap_red', 'accessory', 'Red Chief Cap', 'Traditional red cap worn by chiefs', 'acc_cap_red', 'rare', 100, 1),
            ('acc_cap_white', 'accessory', 'White Fila Cap', 'Traditional Northern fila cap', 'acc_cap_white', 'common', 50, 2),
            ('acc_beads', 'accessory', 'Coral Beads', 'Traditional coral bead necklace', 'acc_beads', 'epic', 200, 3),
            ('acc_glasses', 'accessory', 'Stylish Glasses', 'Modern eyeglasses', 'acc_glasses', 'common', 30, 4),
            ('acc_earrings', 'accessory', 'Gold Earrings', 'Beautiful gold earrings', 'acc_earrings', 'rare', 80, 5),
            ('acc_watch', 'accessory', 'Wrist Watch', 'Smart wrist watch', 'acc_watch', 'common', 40, 6),
            ('acc_bag', 'accessory', 'School Bag', 'Backpack for your adventures', 'acc_bag', 'starter', 0, 7),
            ('acc_ankara_bag', 'accessory', 'Ankara Bag', 'Stylish Ankara print bag', 'acc_ankara_bag', 'rare', 90, 8);

        -- =====================================================
        -- BACKGROUNDS
        -- =====================================================
        INSERT OR REPLACE INTO avatar_items (id, category, name, description, image_key, rarity, unlock_cost, sort_order)
        VALUES 
            ('bg_default', 'background', 'Savanna Sunset', 'Beautiful African savanna at sunset', 'bg_savanna', 'starter', 0, 1),
            ('bg_lagos', 'background', 'Lagos Skyline', 'Bustling Lagos city skyline', 'bg_lagos', 'common', 50, 2),
            ('bg_village', 'background', 'Nigerian Village', 'Peaceful rural village scene', 'bg_village', 'common', 50, 3),
            ('bg_market', 'background', 'Market Scene', 'Colorful Nigerian market', 'bg_market', 'rare', 100, 4),
            ('bg_palace', 'background', 'Royal Palace', 'Traditional Oba palace', 'bg_palace', 'epic', 200, 5),
            ('bg_forest', 'background', 'Rainforest', 'Cross River tropical forest', 'bg_forest', 'rare', 120, 6);
    "#).map_err(|e| DatabaseError::QueryError(e.to_string()))?;

    // Auto-unlock starter items for user 1
    conn.execute_batch(r#"
        INSERT OR IGNORE INTO user_avatar_items (user_id, item_id, is_equipped)
        SELECT 1, id, CASE WHEN id IN ('skin_3', 'hair_1', 'outfit_school', 'bg_default') THEN 1 ELSE 0 END
        FROM avatar_items WHERE rarity = 'starter';
    "#).map_err(|e| DatabaseError::QueryError(e.to_string()))?;

    Ok(())
}

// =====================================================
// CULTURAL GUIDES SEEDING
// =====================================================

fn seed_cultural_guides(conn: &Connection) -> Result<(), DatabaseError> {
    conn.execute_batch(r#"
        INSERT OR REPLACE INTO cultural_guides (id, name, title, description, personality, avatar_image, state_id, region, greeting, catchphrase, voice_style)
        VALUES 
            ('guide_abuja', 'Chief Emeka', 'Guardian of Federal Knowledge',
             'A wise elder who has served in various government positions and loves teaching young Nigerians about their rights and responsibilities.',
             'Patient, dignified, encouraging',
             'guide_emeka.png', 'ABJ', 'North Central',
             'Welcome, young citizen! Ready to learn how our great nation works?',
             'Knowledge is power, and power belongs to the people!',
             'wise'),
             
            ('guide_lagos', 'Mama Sisi', 'Queen of Market Mathematics',
             'A successful market woman who started from nothing and built a business empire through sharp mathematics and hard work.',
             'Energetic, witty, no-nonsense',
             'guide_sisi.png', 'LAG', 'South West',
             'Ah-ah! Come come, let me teach you how to count money like a real Lagosian!',
             'Quick calculation, quick profit!',
             'energetic'),
             
            ('guide_kano', 'Alhaji Musa', 'Keeper of Northern Heritage',
             'A respected Islamic scholar and historian who has dedicated his life to preserving Northern Nigerian traditions and wisdom.',
             'Calm, scholarly, spiritual',
             'guide_musa.png', 'KAN', 'North West',
             'As-salamu alaykum, young learner. Let us explore the wisdom of our ancestors.',
             'May Allah guide your learning journey.',
             'calm'),
             
            ('guide_calabar', 'Mama Calabar', 'Guardian of Cross River Secrets',
             'A legendary storyteller and keeper of Efik traditions, known for her knowledge of ancient festivals and the forest mysteries.',
             'Mystical, warm, theatrical',
             'guide_calabar.png', 'CRS', 'South South',
             'Ekabo! Welcome to the land of the great carnival! I have stories that will make your eyes shine.',
             'Every story has a lesson, every lesson has a story.',
             'warm'),
             
            ('guide_benin', 'Oba Junior', 'Prince of Bronze Heritage',
             'A young prince from the Benin royal family who teaches about the great Benin Empire and its magnificent bronze artworks.',
             'Royal, proud, inspiring',
             'guide_benin.png', 'EDO', 'South South',
             'Welcome to the ancient Benin Kingdom! Our ancestors created wonders that amazed the world.',
             'The Oba is the custodian of culture.',
             'proud'),
             
            ('guide_owerri', 'Nne Chukwu', 'Mother of Eastern Wisdom',
             'An Igbo businesswoman and cultural ambassador who teaches the values of hard work, trade, and community.',
             'Ambitious, sharp, supportive',
             'guide_owerri.png', 'IMO', 'South East',
             'Nno! Welcome! In Igbo land, we say "Onye aghana nwanne ya" - be your brother''s keeper!',
             'Work hard, pray harder, succeed always!',
             'energetic');
    "#).map_err(|e| DatabaseError::QueryError(e.to_string()))?;

    Ok(())
}

// =====================================================
// ARTIFACTS SEEDING
// =====================================================

fn seed_artifacts(conn: &Connection) -> Result<(), DatabaseError> {
    conn.execute_batch(r#"
        INSERT OR REPLACE INTO artifacts (id, name, description, long_description, category, state_id, region, color_primary, color_secondary, rarity, historical_period, cultural_significance, unlock_type, unlock_source_id, cowrie_cost, sort_order)
        VALUES 
            -- MASKS
            ('art_eyo_mask', 'Eyo Masquerade Mask', 
             'Iconic white mask from Lagos Island festivals',
             'The Eyo masquerade is one of the most recognizable symbols of Lagos. These white-clad figures appear during important ceremonies and the death of prominent Lagos citizens. The pristine white represents purity and the ancestral spirits.',
             'mask', 'LAG', 'South West', '#FFFFFF', '#FFD700', 'rare', 'Traditional', 
             'Represents ancestral spirits and cultural identity of Lagos indigenes', 
             'module', 'LAG_MOD_1', 0, 1),
             
            ('art_ekpe_mask', 'Ekpe Society Mask',
             'Sacred mask of the Cross River secret society',
             'The Ekpe (Leopard) society is one of the most powerful traditional institutions in Cross River State. Their masks represent the spirit of the leopard and are used in ceremonies, justice administration, and community governance.',
             'mask', 'CRS', 'South South', '#8B4513', '#000000', 'epic', 'Pre-colonial',
             'Symbol of authority and justice in Efik/Calabar culture',
             'quest', 'quest_calabar_1', 0, 2),
             
            ('art_benin_mask', 'Benin Bronze Mask',
             'Royal commemorative mask from the Benin Kingdom',
             'The Benin Bronzes are among the most celebrated artworks in African history. This mask represents the Queen Mother and showcases the incredible metalworking skills of Benin artisans. Many originals were taken during the 1897 British invasion.',
             'mask', 'EDO', 'South South', '#CD7F32', '#FFD700', 'legendary', '16th Century',
             'Symbol of the Great Benin Empire''s artistic and political sophistication',
             'achievement', 'ach_benin_complete', 0, 3),
             
            -- TEXTILES
            ('art_aso_oke', 'Aso Oke Cloth',
             'Hand-woven Yoruba ceremonial fabric',
             'Aso Oke (top cloth) is a hand-woven fabric made by Yoruba weavers, primarily men. It comes in three main types: Etu (dark blue), Sanyan (light brown), and Alaari (red). It''s worn at important ceremonies like weddings, chieftaincy titles, and funerals.',
             'textile', 'OYO', 'South West', '#4169E1', '#FFD700', 'rare', 'Traditional',
             'Represents Yoruba weaving tradition and ceremonial importance',
             'module', 'OYO_MOD_1', 0, 4),
             
            ('art_akwete_cloth', 'Akwete Cloth',
             'Intricately woven Igbo textile',
             'Akwete cloth is woven exclusively by women in Akwete town, Abia State. The complex patterns are created from memory without written patterns, passed down through generations. Each design tells a story or represents proverbs.',
             'textile', 'IMO', 'South East', '#228B22', '#FFD700', 'rare', 'Traditional',
             'Symbol of Igbo women''s artistic prowess and cultural heritage',
             'quest', 'quest_imo_1', 0, 5),
             
            ('art_kano_dye', 'Kano Indigo Dye Pit Sample',
             'Traditional indigo dyed cloth from ancient dye pits',
             'The Kofar Mata Dye Pits in Kano are over 500 years old and still in use today. Men work in teams to dye cloth in deep indigo vats, creating the distinctive dark blue fabric that made Kano famous along trans-Saharan trade routes.',
             'textile', 'KAN', 'North West', '#191970', '#FFFFFF', 'epic', '15th Century',
             'Represents Kano''s ancient trade heritage and industrial history',
             'module', 'KAN_MOD_1', 0, 6),
             
            -- INSTRUMENTS
            ('art_talking_drum', 'Talking Drum (Dundun)',
             'Hour-glass shaped drum that mimics Yoruba tones',
             'The talking drum can replicate the tones and rhythms of Yoruba language. Skilled drummers can send complex messages across villages. It''s used in ceremonies, announcements, and praise singing. The drum is played while held under the arm.',
             'instrument', 'OYO', 'South West', '#8B4513', '#DEB887', 'common', 'Traditional',
             'Essential communication and entertainment instrument in Yoruba culture',
             'module', 'ABJ_MOD_1', 0, 7),
             
            ('art_ogene', 'Ogene Bell',
             'Igbo metal gong for music and announcements',
             'The Ogene is an iron gong that forms the backbone of Igbo highlife music. It''s also used to call village meetings and make important announcements. The "Ogene" music genre is named after this instrument.',
             'instrument', 'IMO', 'South East', '#B87333', '#C0C0C0', 'common', 'Traditional',
             'Central to Igbo music and community communication',
             'quest', 'quest_imo_2', 0, 8),
             
            ('art_shekere', 'Shekere',
             'Beaded gourd rattle',
             'The Shekere is made from a dried gourd covered with a net of beads or shells. When shaken or struck, it produces a distinctive rattling sound. It''s used in both traditional ceremonies and modern Nigerian music.',
             'instrument', 'LAG', 'South West', '#DEB887', '#FFFFFF', 'common', 'Traditional',
             'Versatile percussion instrument used across Nigerian music genres',
             'purchase', NULL, 100, 9),
             
            -- SCULPTURES
            ('art_nok_head', 'Nok Terracotta Head',
             'Ancient sculpture from Nigeria''s oldest known civilization',
             'The Nok civilization (1500 BC - 500 AD) created remarkable terracotta sculptures, making them the oldest known figurative sculptures in sub-Saharan Africa. Found in Central Nigeria, they show advanced artistic skill thousands of years ago.',
             'sculpture', 'NIG', 'North Central', '#D2691E', '#8B4513', 'legendary', '500 BC - 200 AD',
             'Evidence of Nigeria''s ancient sophisticated civilizations',
             'achievement', 'ach_historian', 0, 10),
             
            ('art_igbo_ukwu', 'Igbo Ukwu Bronze',
             'Sophisticated bronze vessel from ancient Igbo civilization',
             'Discovered in 1938, the Igbo Ukwu bronzes (9th-10th century) proved that sophisticated metalworking existed in Nigeria before European contact. The intricate designs show a wealthy, organized society with advanced technology.',
             'sculpture', 'IMO', 'South East', '#CD7F32', '#228B22', 'legendary', '9th Century',
             'Proof of ancient Igbo technological sophistication',
             'quest', 'quest_imo_complete', 0, 11),
             
            -- JEWELRY
            ('art_coral_beads', 'Royal Coral Beads',
             'Traditional coral bead jewelry worn by royalty',
             'Coral beads are symbols of royalty and wealth in many Nigerian cultures, especially Benin and Yoruba kingdoms. The Oba of Benin wears coral from head to toe during ceremonies. Red coral represents life force and connection to the sea god Olokun.',
             'jewelry', 'EDO', 'South South', '#FF6347', '#FFD700', 'epic', 'Traditional',
             'Symbol of royalty, wealth, and divine connection',
             'purchase', NULL, 250, 12),
             
            ('art_waist_beads', 'Traditional Waist Beads',
             'Decorative beads worn around the waist',
             'Waist beads have been worn by African women for centuries. They serve various purposes: tracking weight changes, celebrating femininity, spiritual protection, and as private adornment between couples. Colors and materials carry meaning.',
             'jewelry', 'LAG', 'South West', '#FF69B4', '#40E0D0', 'common', 'Traditional',
             'Symbol of femininity, beauty, and cultural identity',
             'module', 'LAG_MOD_2', 0, 13),
             
            -- POTTERY
            ('art_calabash', 'Decorated Calabash',
             'Ornately carved gourd container',
             'Calabashes (dried gourds) serve countless purposes in Nigerian life: drinking cups, food storage, musical instruments, and ceremonial objects. They are often decorated with carvings or pyrography (burn designs).',
             'pottery', 'NIG', 'North Central', '#DAA520', '#8B4513', 'common', 'Traditional',
             'Versatile container central to Nigerian domestic life',
             'purchase', NULL, 75, 14),
             
            ('art_water_pot', 'Nupe Water Pot',
             'Elegant terracotta water vessel',
             'Nupe pottery is famous for its elegant shapes and distinctive black coloring achieved through special firing techniques. These pots keep water naturally cool and are prized across Nigeria.',
             'pottery', 'NIG', 'North Central', '#2F4F4F', '#8B4513', 'rare', 'Traditional',
             'Represents Nupe mastery of pottery and practical design',
             'module', 'NIG_MOD_1', 0, 15),
             
            -- DOCUMENTS/SYMBOLS
            ('art_nsibidi', 'Nsibidi Symbols Tablet',
             'Ancient Ekoi/Ejagham writing system',
             'Nsibidi is an indigenous writing system from southeastern Nigeria, one of the few independent African scripts. Originally used by the Ekpe society, the symbols convey concepts, proverbs, and messages that could be read across different languages.',
             'document', 'CRS', 'South South', '#000000', '#FFD700', 'epic', 'Pre-colonial',
             'Evidence of indigenous African literacy and communication',
             'quest', 'quest_calabar_complete', 0, 16),
             
            ('art_kola_nut', 'Ceremonial Kola Nut',
             'Sacred nut used in all important occasions',
             'The kola nut (obi) is central to Nigerian hospitality and ceremony. Breaking kola involves prayers and is required at meetings, naming ceremonies, and marriage negotiations. "He who brings kola, brings life" is a famous saying.',
             'document', 'ABJ', 'North Central', '#800000', '#FFFFFF', 'common', 'Traditional',
             'Symbol of hospitality, respect, and covenant',
             'module', 'ABJ_MOD_1', 0, 17);
    "#).map_err(|e| DatabaseError::QueryError(e.to_string()))?;

    Ok(())
}

// =====================================================
// QUESTS SEEDING
// =====================================================

fn seed_quests(conn: &Connection) -> Result<(), DatabaseError> {
    conn.execute_batch(r#"
        -- =====================================================
        -- MAIN STORY QUESTS
        -- =====================================================
        INSERT OR REPLACE INTO quests (id, title, description, quest_type, category, state_id, guide_id, required_level, requirements_json, xp_reward, cowrie_reward, artifact_reward_id, intro_dialogue, completion_dialogue, icon, sort_order)
        VALUES 
            ('quest_welcome', 'Welcome to Nigeria!', 
             'Begin your journey through Nigeria by completing the tutorial in Abuja.',
             'main', 'exploration', 'ABJ', 'guide_abuja', 1,
             '[{"requirement_type": "complete_module", "target": "ABJ_MOD_1", "count": 1}]',
             100, 50, 'art_kola_nut',
             'Welcome, young explorer! I am Chief Emeka, and I will guide you on your first steps. Every great journey begins with understanding where you stand. Let us explore how Nigeria works!',
             'Excellent! You have taken your first steps as a true Nigerian citizen. This kola nut is a symbol of our hospitality - may it remind you that knowledge is always shared. Now, greater adventures await!',
             'flag', 1),
             
            ('quest_lagos_challenge', 'The Lagos Challenge',
             'Prove your mathematical skills in the bustling markets of Lagos.',
             'main', 'mastery', 'LAG', 'guide_lagos', 2,
             '[{"requirement_type": "complete_module", "target": "LAG_MOD_1", "count": 1}, {"requirement_type": "score_quiz", "target": "LAG_MOD_1", "count": 80}]',
             200, 100, 'art_eyo_mask',
             'Ah-ah! You want to survive in Lagos? Then you must be sharp-sharp! Come, let me teach you how we calculate in the market. No calculator allowed - only your brain!',
             'Chai! You are now a true Lagosian! Quick calculation, quick profit - you have learned well. Take this Eyo mask as proof of your Lagos certification. The city has accepted you!',
             'calculator', 2),
             
            ('quest_codex_scholar', 'Codex Scholar',
             'Read 5 entries in the Sabi Codex to expand your knowledge.',
             'main', 'learning', NULL, NULL, 1,
             '[{"requirement_type": "read_codex", "target": "any", "count": 5}]',
             75, 30, NULL,
             'The Sabi Codex contains the wisdom of our ancestors. A true scholar reads widely and deeply. Begin your journey through Nigerian knowledge!',
             'Your mind grows with every page turned. You are becoming a true Sabi - one who knows! Continue your scholarly journey.',
             'book', 3);

        -- =====================================================
        -- SIDE QUESTS
        -- =====================================================
        INSERT OR REPLACE INTO quests (id, title, description, quest_type, category, state_id, guide_id, required_level, requirements_json, xp_reward, cowrie_reward, intro_dialogue, completion_dialogue, icon, sort_order)
        VALUES 
            ('quest_perfect_score', 'Perfect Score',
             'Get 100% on any quiz to show your mastery.',
             'side', 'mastery', NULL, NULL, 1,
             '[{"requirement_type": "score_quiz", "target": "any", "count": 100}]',
             150, 75,
             'A true master makes no mistakes. Can you achieve perfection?',
             'Incredible! A perfect score shows true dedication. You have earned this reward!',
             'star', 1),
             
            ('quest_collector_begin', 'Budding Collector',
             'Collect your first 3 cultural artifacts.',
             'side', 'collection', NULL, NULL, 1,
             '[{"requirement_type": "collect_artifact", "target": "any", "count": 3}]',
             100, 50,
             'Nigeria''s cultural treasures await discovery. Begin building your museum collection!',
             'Your museum is taking shape! Each artifact tells a story of our great heritage.',
             'trophy', 2),
             
            ('quest_codex_master', 'Codex Master',
             'Read 20 entries in the Sabi Codex.',
             'side', 'learning', NULL, NULL, 3,
             '[{"requirement_type": "read_codex", "target": "any", "count": 20}]',
             200, 100,
             'Knowledge has no end. The Codex holds many more secrets for the dedicated student.',
             'You have read far and wide! Your knowledge of Nigeria is becoming encyclopedic!',
             'book-open', 3);

        -- =====================================================
        -- DAILY QUESTS
        -- =====================================================
        INSERT OR REPLACE INTO quests (id, title, description, quest_type, category, required_level, requirements_json, xp_reward, cowrie_reward, icon, is_repeatable, cooldown_hours, sort_order)
        VALUES 
            ('quest_daily_quiz', 'Daily Quiz',
             'Complete any quiz today.',
             'daily', 'mastery', 1,
             '[{"requirement_type": "complete_quiz", "target": "any", "count": 1}]',
             50, 25, 'calendar', 1, 24, 1),
             
            ('quest_daily_read', 'Daily Reading',
             'Read 2 Codex entries today.',
             'daily', 'learning', 1,
             '[{"requirement_type": "read_codex", "target": "any", "count": 2}]',
             30, 15, 'book', 1, 24, 2);
    "#).map_err(|e| DatabaseError::QueryError(e.to_string()))?;

    Ok(())
}
