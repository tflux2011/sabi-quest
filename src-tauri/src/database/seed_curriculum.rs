// Curriculum seed data for Project Nigeria
// Contains real Nigerian educational content for secondary school students

use rusqlite::Connection;
use super::DatabaseError;

/// Seeds the database with comprehensive curriculum for all Nigerian states
pub fn seed_curriculum(conn: &Connection) -> Result<(), DatabaseError> {
    // Seed states FIRST (required by user_progress foreign key)
    seed_states(conn)?;
    
    // Create default user (after states exist)
    seed_default_user(conn)?;
    
    // Seed items that can be unlocked
    seed_items(conn)?;
    
    // Seed Abuja modules (Heritage Zone - Tutorial)
    seed_abuja_modules(conn)?;
    
    // Seed Lagos modules (Mind Zone)
    seed_lagos_modules(conn)?;
    
    // Seed additional state modules
    seed_ogun_modules(conn)?;
    seed_oyo_modules(conn)?;
    seed_osun_modules(conn)?;
    seed_rivers_modules(conn)?;
    seed_crossriver_modules(conn)?;
    seed_sokoto_modules(conn)?;
    seed_borno_modules(conn)?;
    seed_taraba_modules(conn)?;
    seed_kano_modules(conn)?;
    seed_edo_modules(conn)?;
    seed_enugu_modules(conn)?;
    seed_plateau_modules(conn)?;
    seed_bauchi_modules(conn)?;
    seed_anambra_modules(conn)?;
    
    // Seed The Sabi Codex encyclopedia entries
    seed_encyclopedia(conn)?;
    
    // Seed RPG features
    seed_avatar_items(conn)?;
    seed_cultural_guides(conn)?;
    seed_artifacts(conn)?;
    seed_quests(conn)?;
    
    log::info!("Curriculum seeded successfully with all state modules");
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
             'Cross River contains the last remaining virgin tropical rainforest in Nigeria and hosts Africa''s biggest street party!'),
            
            -- =====================================================
            -- REMAINING 26 STATES
            -- =====================================================
            
            -- SOUTH EAST ZONE
            ('ABI', 'Abia', 'South East', 'spirit', 5, 'Ariaria Market', '/assets/images/abia-landmark.png',
             'Abia State - God''s Own State, known for its entrepreneurial spirit and the famous Aba market.',
             'Aba in Abia State is called the "Japan of Africa" because its artisans can replicate almost any product!'),
            ('ANA', 'Anambra', 'South East', 'mind', 5, 'Ogbunike Caves', '/assets/images/anambra-landmark.png',
             'Anambra State - Light of the Nation, home to Onitsha, one of Africa''s largest markets.',
             'Onitsha Main Market is the largest market in Africa by geographical size and number of traders!'),
            ('EBO', 'Ebonyi', 'South East', 'spirit', 6, 'Salt Lake Okposi', '/assets/images/ebonyi-landmark.png',
             'Ebonyi State - The Salt of the Nation, known for its natural salt lakes and rice production.',
             'Ebonyi has natural salt lakes that have been mined for centuries, giving the state its nickname!'),
            ('ENU', 'Enugu', 'South East', 'heritage', 5, 'Enugu Coal Mine', '/assets/images/enugu-landmark.png',
             'Enugu State - Coal City State, the former capital of Eastern Nigeria and home to Nigeria''s coal industry.',
             'Enugu means "hilltop" in Igbo. The city grew around coal mining and was once the most industrialized city in Nigeria!'),
            
            -- SOUTH WEST ZONE (Additional)
            ('EKI', 'Ekiti', 'South West', 'heritage', 5, 'Ikogosi Warm Springs', '/assets/images/ekiti-landmark.png',
             'Ekiti State - Land of Honor, known for its highly educated population and unique warm springs.',
             'Ikogosi has a natural wonder where warm and cold springs meet and flow side by side without mixing!'),
            ('OND', 'Ondo', 'South West', 'spirit', 5, 'Idanre Hills', '/assets/images/ondo-landmark.png',
             'Ondo State - Sunshine State, home to cocoa farming and the ancient Idanre Kingdom.',
             'Idanre Hills has 640 steps carved into ancient rock leading to a hilltop kingdom inhabited for over 800 years!'),
            ('OSU', 'Osun', 'South West', 'spirit', 4, 'Osun-Osogbo Sacred Grove', '/assets/images/osun-landmark.png',
             'Osun State - State of the Living Spring, home to the UNESCO World Heritage Osun-Osogbo Sacred Grove.',
             'The Osun-Osogbo Grove is one of the last remaining primary forests in southern Nigeria and a UNESCO World Heritage Site!'),
            
            -- SOUTH SOUTH ZONE (Additional)
            ('AKW', 'Akwa Ibom', 'South South', 'mind', 5, 'Ibom Plaza', '/assets/images/akwaibom-landmark.png',
             'Akwa Ibom State - Land of Promise, known for its beautiful beaches and rich oil resources.',
             'Akwa Ibom is the highest oil-producing state in Nigeria and has some of the most beautiful beaches in West Africa!'),
            ('BAY', 'Bayelsa', 'South South', 'spirit', 6, 'Ox-Bow Lake', '/assets/images/bayelsa-landmark.png',
             'Bayelsa State - Glory of All Lands, the heart of the Niger Delta with rich mangrove forests.',
             'Bayelsa was created in 1996 and is the youngest state in the South South region!'),
            ('DEL', 'Delta', 'South South', 'mind', 5, 'Lander Brothers Anchorage', '/assets/images/delta-landmark.png',
             'Delta State - The Big Heart, home to diverse ethnic groups and major oil production.',
             'Delta State has over 40 ethnic groups speaking different languages, one of the most diverse states in Nigeria!'),
            
            -- NORTH CENTRAL ZONE (Additional)
            ('BEN', 'Benue', 'North Central', 'heritage', 4, 'River Benue', '/assets/images/benue-landmark.png',
             'Benue State - Food Basket of the Nation, Nigeria''s leading producer of yams, cassava, and vegetables.',
             'Benue produces more food than any other state in Nigeria and is called the "Food Basket of the Nation"!'),
            ('KOG', 'Kogi', 'North Central', 'heritage', 4, 'Confluence of Niger and Benue', '/assets/images/kogi-landmark.png',
             'Kogi State - The Confluence State, where Nigeria''s two largest rivers meet at Lokoja.',
             'Lokoja in Kogi State is where the Rivers Niger and Benue meet - the only place in the world with this natural phenomenon!'),
            ('KWA', 'Kwara', 'North Central', 'mind', 4, 'Owu Waterfalls', '/assets/images/kwara-landmark.png',
             'Kwara State - State of Harmony, known for its cultural diversity and beautiful waterfalls.',
             'Owu Waterfalls in Kwara is the highest waterfall in West Africa at 120 meters!'),
            ('NAS', 'Nasarawa', 'North Central', 'heritage', 3, 'Farin Ruwa Falls', '/assets/images/nasarawa-landmark.png',
             'Nasarawa State - Home of Solid Minerals, rich in gemstones and natural resources.',
             'Nasarawa is called Nigeria''s "Jewel in the Savannah" due to its vast deposits of precious stones!'),
            ('PLA', 'Plateau', 'North Central', 'heritage', 4, 'Shere Hills', '/assets/images/plateau-landmark.png',
             'Plateau State - Home of Peace and Tourism, with a unique temperate climate in tropical Africa.',
             'Jos in Plateau State has the coolest weather in Nigeria and was a popular colonial retreat due to its temperate climate!'),
            
            -- NORTH WEST ZONE (Additional)
            ('JIG', 'Jigawa', 'North West', 'heritage', 5, 'Hadejia-Nguru Wetlands', '/assets/images/jigawa-landmark.png',
             'Jigawa State - The New World, known for its wetlands and groundnut farming.',
             'The Hadejia-Nguru Wetlands in Jigawa is one of Africa''s most important wetland ecosystems!'),
            ('KAT', 'Katsina', 'North West', 'heritage', 5, 'Gobarau Minaret', '/assets/images/katsina-landmark.png',
             'Katsina State - Home of Hospitality, ancient center of Islamic learning and culture.',
             'Katsina was a major stop on trans-Saharan trade routes and had one of the oldest universities in Africa!'),
            ('KEB', 'Kebbi', 'North West', 'heritage', 6, 'Argungu Fishing Festival', '/assets/images/kebbi-landmark.png',
             'Kebbi State - Land of Equity, famous for the spectacular Argungu Fishing Festival.',
             'The Argungu Fishing Festival is a 4-day event where thousands catch fish with their bare hands!'),
            ('SOK', 'Sokoto', 'North West', 'heritage', 6, 'Sultan''s Palace', '/assets/images/sokoto-landmark.png',
             'Sokoto State - Seat of the Caliphate, the spiritual center of Islam in Nigeria.',
             'The Sultan of Sokoto is considered the spiritual leader of Nigerian Muslims, continuing a 200-year-old tradition!'),
            ('ZAM', 'Zamfara', 'North West', 'heritage', 6, 'Gusau Dam', '/assets/images/zamfara-landmark.png',
             'Zamfara State - Farming is Our Pride, known for cotton and groundnut production.',
             'Zamfara was the first state in Nigeria to implement Sharia law in 2000!'),
            
            -- NORTH EAST ZONE
            ('ADA', 'Adamawa', 'North East', 'spirit', 6, 'Mandara Mountains', '/assets/images/adamawa-landmark.png',
             'Adamawa State - Land of Beauty, home to stunning mountain ranges and diverse cultures.',
             'The Mandara Mountains in Adamawa are home to unique communities who have lived there for thousands of years!'),
            ('BAU', 'Bauchi', 'North East', 'heritage', 5, 'Yankari Game Reserve', '/assets/images/bauchi-landmark.png',
             'Bauchi State - Pearl of Tourism, home to Nigeria''s premier wildlife reserve.',
             'Yankari Game Reserve has Africa''s largest herd of forest elephants and natural warm springs!'),
            ('BOR', 'Borno', 'North East', 'heritage', 7, 'Shehu''s Palace', '/assets/images/borno-landmark.png',
             'Borno State - Home of Peace, the ancient seat of the Kanem-Bornu Empire that lasted 1000 years.',
             'The Kanem-Bornu Empire was one of the longest-lasting empires in African history, spanning over 1000 years!'),
            ('GOM', 'Gombe', 'North East', 'heritage', 6, 'Tula Plateau', '/assets/images/gombe-landmark.png',
             'Gombe State - Jewel in the Savannah, known for its unique Tangale culture.',
             'Gombe is home to the Tula people who built their homes on mountain peaks for protection!'),
            ('TAR', 'Taraba', 'North East', 'spirit', 6, 'Mambilla Plateau', '/assets/images/taraba-landmark.png',
             'Taraba State - Nature''s Gift to the Nation, home to Nigeria''s highest plateau.',
             'Mambilla Plateau at 1,800 meters is Nigeria''s highest point and has a climate similar to European countries!'),
            ('YOB', 'Yobe', 'North East', 'heritage', 7, 'Dufuna Canoe Site', '/assets/images/yobe-landmark.png',
             'Yobe State - Pride of the Sahel, where Africa''s oldest boat was discovered.',
             'The Dufuna Canoe found in Yobe is over 8,000 years old - the oldest boat in Africa and third oldest in the world!');
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
    // Module 1: The People's Court (Social Studies/Civic Education) - All education levels (tutorial)
    conn.execute_batch(r#"
        -- =====================================================
        -- ABUJA MODULE 1: THE PEOPLE'S COURT (Social Studies)
        -- =====================================================
        INSERT OR REPLACE INTO modules (id, state_id, subject, title, description, required_level, total_xp, estimated_time, icon, education_level, interest_tags)
        VALUES ('abj_civics_001', 'ABJ', 'Social Studies', 'The People''s Court', 
                'Learn how Nigeria''s government works! Pass bills through the National Assembly and understand your rights as a citizen.',
                1, 500, 20, 'gavel', 'all', '["history", "culture"]');
        
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

    // Module 2: Nigerian Geography (Basic Studies)
    conn.execute_batch(r#"
        -- =====================================================
        -- ABUJA MODULE 2: DISCOVER NIGERIA (Geography)
        -- =====================================================
        INSERT OR REPLACE INTO modules (id, state_id, subject, title, description, required_level, total_xp, estimated_time, icon, education_level, interest_tags)
        VALUES ('abj_geo_001', 'ABJ', 'Geography', 'Discover Nigeria: Our Land', 
                'Explore the geography of Nigeria! Learn about our states, rivers, mountains, and why Abuja became our capital city.',
                1, 450, 20, 'globe', 'all', '["geography"]');
        
        -- Module Context
        INSERT OR REPLACE INTO module_context (module_id, did_you_know, fun_fact, intro_text, historical_note)
        VALUES ('abj_geo_001',
                'Nigeria is about twice the size of California, USA, and is the most populous country in Africa with over 200 million people!',
                'If Nigeria were a rectangle, it would be roughly 1,200 km from east to west and 1,050 km from north to south. That''s bigger than many European countries combined!',
                'Welcome, young explorer! In this module, you will discover the amazing geography of Nigeria - from the sands of the Sahel in the north to the mangrove swamps of the Niger Delta. Learn about our rivers, mountains, and the 36 states that make up our great nation.',
                'Abuja was chosen as Nigeria''s capital in 1976 because of its central location. Before that, Lagos was the capital, but it became too crowded!');
        
        -- Level 1: Nigeria on the Map
        INSERT OR REPLACE INTO levels (id, module_id, title, difficulty, order_index, xp_reward, unlock_item_id)
        VALUES ('abj_geo_001_lvl1', 'abj_geo_001', 'Nigeria on the Map', 'easy', 1, 100, 'badge_explorer');
        
        -- Level 1 Questions
        INSERT OR REPLACE INTO questions (id, level_id, question_text, question_type, options_json, correct_answer, xp_reward, explanation, hint, order_index)
        VALUES 
            ('abj_g1_q1', 'abj_geo_001_lvl1', 
             'On which continent is Nigeria located?',
             'multiple_choice',
             '[{"id":"a","text":"Asia"},{"id":"b","text":"Europe"},{"id":"c","text":"Africa"},{"id":"d","text":"South America"}]',
             'c', 15,
             'Nigeria is located in West Africa, on the African continent. Africa is the second-largest continent in the world.',
             'Nigeria is known as the "Giant of Africa."',
             1),
            
            ('abj_g1_q2', 'abj_geo_001_lvl1',
             'How many states does Nigeria have (not counting the FCT)?',
             'multiple_choice',
             '[{"id":"a","text":"30 states"},{"id":"b","text":"36 states"},{"id":"c","text":"40 states"},{"id":"d","text":"24 states"}]',
             'b', 15,
             'Nigeria has 36 states plus the Federal Capital Territory (Abuja). The states are grouped into 6 geopolitical zones.',
             'It''s more than 30 but less than 40.',
             2),
            
            ('abj_g1_q3', 'abj_geo_001_lvl1',
             'Which body of water borders Nigeria to the south?',
             'multiple_choice',
             '[{"id":"a","text":"Indian Ocean"},{"id":"b","text":"Mediterranean Sea"},{"id":"c","text":"Atlantic Ocean"},{"id":"d","text":"Red Sea"}]',
             'c', 15,
             'The Atlantic Ocean (specifically the Gulf of Guinea and Bight of Benin) borders Nigeria to the south. This is why states like Lagos and Rivers have coastal areas.',
             'It''s the same ocean that borders the eastern coast of the Americas.',
             3),
            
            ('abj_g1_q4', 'abj_geo_001_lvl1',
             'Which country shares a border with Nigeria to the west?',
             'multiple_choice',
             '[{"id":"a","text":"Ghana"},{"id":"b","text":"Benin Republic"},{"id":"c","text":"Togo"},{"id":"d","text":"South Africa"}]',
             'b', 15,
             'Benin Republic borders Nigeria to the west. Nigeria also shares borders with Niger (north), Chad (northeast), and Cameroon (east).',
             'It shares a similar name with a historic kingdom in Nigeria.',
             4),
            
            ('abj_g1_q5', 'abj_geo_001_lvl1',
             'True or False: Nigeria is the most populous country in Africa.',
             'true_false',
             '[{"id":"true","text":"True"},{"id":"false","text":"False"}]',
             'true', 10,
             'True! Nigeria has over 200 million people, making it the most populous country in Africa and the 7th most populous in the world.',
             NULL,
             5);
        
        -- Level 2: Rivers and Natural Features
        INSERT OR REPLACE INTO levels (id, module_id, title, difficulty, order_index, xp_reward, unlock_item_id)
        VALUES ('abj_geo_001_lvl2', 'abj_geo_001', 'Rivers and Natural Wonders', 'medium', 2, 150, 'badge_naturalist');
        
        -- Level 2 Questions
        INSERT OR REPLACE INTO questions (id, level_id, question_text, question_type, options_json, correct_answer, xp_reward, explanation, hint, order_index)
        VALUES 
            ('abj_g2_q1', 'abj_geo_001_lvl2',
             'Nigeria gets its name from which river?',
             'multiple_choice',
             '[{"id":"a","text":"River Benue"},{"id":"b","text":"River Niger"},{"id":"c","text":"River Nile"},{"id":"d","text":"River Congo"}]',
             'b', 20,
             'Nigeria is named after the River Niger, which flows through the western part of the country. The name was suggested by Flora Shaw, a British journalist, in 1897.',
             'The country and the river share almost the same spelling.',
             1),
            
            ('abj_g2_q2', 'abj_geo_001_lvl2',
             'Where do the Rivers Niger and Benue meet?',
             'multiple_choice',
             '[{"id":"a","text":"Lagos"},{"id":"b","text":"Abuja"},{"id":"c","text":"Lokoja"},{"id":"d","text":"Port Harcourt"}]',
             'c', 20,
             'The Rivers Niger and Benue meet at Lokoja in Kogi State. This confluence is a famous landmark and makes Lokoja a historically significant city.',
             'This city is the capital of Kogi State.',
             2),
            
            ('abj_g2_q3', 'abj_geo_001_lvl2',
             'What is the name of the plateau that Abuja is built on?',
             'multiple_choice',
             '[{"id":"a","text":"Mambilla Plateau"},{"id":"b","text":"Jos Plateau"},{"id":"c","text":"Obudu Plateau"},{"id":"d","text":"Gwagwalada Plateau"}]',
             'b', 20,
             'Abuja is located on the Jos Plateau region (specifically the Abuja Plateau). The Jos Plateau in Plateau State is known for its cool climate and tin mining.',
             'This plateau shares its name with a major city in north-central Nigeria.',
             3),
            
            ('abj_g2_q4', 'abj_geo_001_lvl2',
             'What is the Niger Delta famous for?',
             'multiple_choice',
             '[{"id":"a","text":"Gold mining"},{"id":"b","text":"Oil and gas production"},{"id":"c","text":"Diamond mining"},{"id":"d","text":"Coffee farming"}]',
             'b', 20,
             'The Niger Delta region is famous for oil and gas production. Nigeria is one of the largest oil producers in Africa, and most of the oil comes from states in the Niger Delta.',
             'Nigeria''s main export comes from this region.',
             4),
            
            ('abj_g2_q5', 'abj_geo_001_lvl2',
             'Which Nigerian plateau is home to the Obudu Mountain Resort?',
             'multiple_choice',
             '[{"id":"a","text":"Jos Plateau"},{"id":"b","text":"Mambilla Plateau"},{"id":"c","text":"Obudu Plateau"},{"id":"d","text":"Biu Plateau"}]',
             'c', 20,
             'The Obudu Mountain Resort is located on the Obudu Plateau in Cross River State. It''s famous for its cable cars and cool mountain climate.',
             'The resort and plateau share the same name.',
             5);

        -- Level 3: States and Regions
        INSERT OR REPLACE INTO levels (id, module_id, title, difficulty, order_index, xp_reward, unlock_item_id)
        VALUES ('abj_geo_001_lvl3', 'abj_geo_001', 'States and Geopolitical Zones', 'medium', 3, 150, NULL);
        
        -- Level 3 Questions
        INSERT OR REPLACE INTO questions (id, level_id, question_text, question_type, options_json, correct_answer, xp_reward, explanation, hint, order_index)
        VALUES 
            ('abj_g3_q1', 'abj_geo_001_lvl3',
             'How many geopolitical zones does Nigeria have?',
             'multiple_choice',
             '[{"id":"a","text":"4"},{"id":"b","text":"5"},{"id":"c","text":"6"},{"id":"d","text":"7"}]',
             'c', 25,
             'Nigeria has 6 geopolitical zones: North-Central, North-East, North-West, South-East, South-South, and South-West.',
             'Each zone has 6 states on average.',
             1),
            
            ('abj_g3_q2', 'abj_geo_001_lvl3',
             'Which geopolitical zone is Abuja (FCT) located in?',
             'multiple_choice',
             '[{"id":"a","text":"North-West"},{"id":"b","text":"North-Central"},{"id":"c","text":"South-West"},{"id":"d","text":"North-East"}]',
             'b', 20,
             'Abuja (FCT) is in the North-Central geopolitical zone, also known as the Middle Belt. This zone includes states like Niger, Kogi, Kwara, Nasarawa, Benue, and Plateau.',
             'It''s in the middle of the country, which matches its name.',
             2),
            
            ('abj_g3_q3', 'abj_geo_001_lvl3',
             'Which state is known as the "Centre of Excellence"?',
             'multiple_choice',
             '[{"id":"a","text":"Kano"},{"id":"b","text":"Rivers"},{"id":"c","text":"Lagos"},{"id":"d","text":"Kaduna"}]',
             'c', 20,
             'Lagos State is known as the "Centre of Excellence." It was Nigeria''s former capital and remains the economic hub of the country.',
             'It''s Nigeria''s most populous city and commercial capital.',
             3),
            
            ('abj_g3_q4', 'abj_geo_001_lvl3',
             'True or False: Sokoto State is located in the South-South geopolitical zone.',
             'true_false',
             '[{"id":"true","text":"True"},{"id":"false","text":"False"}]',
             'false', 15,
             'False! Sokoto State is in the North-West geopolitical zone, not South-South. Sokoto is known as the seat of the Caliphate.',
             NULL,
             4),
            
            ('abj_g3_q5', 'abj_geo_001_lvl3',
             'Which state in Nigeria has the largest land area?',
             'multiple_choice',
             '[{"id":"a","text":"Lagos"},{"id":"b","text":"Kano"},{"id":"c","text":"Niger"},{"id":"d","text":"Borno"}]',
             'c', 25,
             'Niger State is the largest state in Nigeria by land area, covering about 76,363 km². Despite being the largest, it''s not the most populous.',
             'It shares its name with the river and a neighboring country.',
             5);
    "#).map_err(|e| DatabaseError::QueryError(e.to_string()))?;

    // Module 3: Nigerian Languages and Culture
    conn.execute_batch(r#"
        -- =====================================================
        -- ABUJA MODULE 3: UNITY IN DIVERSITY (Languages & Culture)
        -- =====================================================
        INSERT OR REPLACE INTO modules (id, state_id, subject, title, description, required_level, total_xp, estimated_time, icon, education_level, interest_tags)
        VALUES ('abj_culture_001', 'ABJ', 'Cultural Studies', 'Unity in Diversity', 
                'Learn about Nigeria''s rich cultural tapestry! Discover our major languages, ethnic groups, and what makes us "Unity in Diversity."',
                1, 400, 18, 'users', 'all', '["culture", "languages"]');
        
        -- Module Context
        INSERT OR REPLACE INTO module_context (module_id, did_you_know, fun_fact, intro_text, historical_note)
        VALUES ('abj_culture_001',
                'Nigeria has over 500 different languages! The three major ones are Yoruba, Igbo, and Hausa, but English is the official language that unites us all.',
                'The Nigerian coat of arms features two horses, an eagle, and a black shield with a white Y-shape representing the Rivers Niger and Benue. Our motto is "Unity and Faith, Peace and Progress!"',
                'Welcome, young Nigerian! Our country is a beautiful mix of over 250 ethnic groups, each with their own language, food, clothing, and traditions. Yet, we are all united as one Nigeria. Let''s explore what makes us special!',
                'Nigeria''s diverse ethnic groups have lived together for centuries. In 1914, the Northern and Southern Protectorates were merged to form what we now call Nigeria.');
        
        -- Level 1: Languages of Nigeria
        INSERT OR REPLACE INTO levels (id, module_id, title, difficulty, order_index, xp_reward, unlock_item_id)
        VALUES ('abj_culture_001_lvl1', 'abj_culture_001', 'Our Languages', 'easy', 1, 100, 'badge_linguist');
        
        -- Level 1 Questions
        INSERT OR REPLACE INTO questions (id, level_id, question_text, question_type, options_json, correct_answer, xp_reward, explanation, hint, order_index)
        VALUES 
            ('abj_cu1_q1', 'abj_culture_001_lvl1', 
             'What is the official language of Nigeria?',
             'multiple_choice',
             '[{"id":"a","text":"Yoruba"},{"id":"b","text":"Hausa"},{"id":"c","text":"English"},{"id":"d","text":"Igbo"}]',
             'c', 15,
             'English is Nigeria''s official language. It was inherited from British colonial rule and is used in government, schools, and business to unite our diverse nation.',
             'It''s a language that came from our colonial history.',
             1),
            
            ('abj_cu1_q2', 'abj_culture_001_lvl1',
             'Which of these is NOT one of Nigeria''s three major indigenous languages?',
             'multiple_choice',
             '[{"id":"a","text":"Hausa"},{"id":"b","text":"Yoruba"},{"id":"c","text":"French"},{"id":"d","text":"Igbo"}]',
             'c', 15,
             'French is not a Nigerian language. Nigeria''s three major indigenous languages are Hausa (North), Yoruba (South-West), and Igbo (South-East).',
             'One of these is a European language.',
             2),
            
            ('abj_cu1_q3', 'abj_culture_001_lvl1',
             '"Sannu" is a greeting in which Nigerian language?',
             'multiple_choice',
             '[{"id":"a","text":"Yoruba"},{"id":"b","text":"Igbo"},{"id":"c","text":"Hausa"},{"id":"d","text":"Ijaw"}]',
             'c', 15,
             '"Sannu" means "Hello" in Hausa. Hausa is widely spoken in Northern Nigeria and is one of the most spoken languages in Africa.',
             'This language is dominant in the Northern region.',
             3),
            
            ('abj_cu1_q4', 'abj_culture_001_lvl1',
             '"Bawo ni?" means "How are you?" in which language?',
             'multiple_choice',
             '[{"id":"a","text":"Hausa"},{"id":"b","text":"Yoruba"},{"id":"c","text":"Igbo"},{"id":"d","text":"Tiv"}]',
             'b', 15,
             '"Bawo ni?" is Yoruba for "How are you?" The proper response is "Mo wa" (I am fine) or "Dada" (Fine).',
             'This is the dominant language in the South-West.',
             4),
            
            ('abj_cu1_q5', 'abj_culture_001_lvl1',
             '"Kedu?" is a greeting in which language?',
             'multiple_choice',
             '[{"id":"a","text":"Hausa"},{"id":"b","text":"Yoruba"},{"id":"c","text":"Igbo"},{"id":"d","text":"Fulani"}]',
             'c', 15,
             '"Kedu?" means "How are you?" in Igbo. It can also mean "Hello." The response is often "Ọ dị mma" (It is well).',
             'This language is dominant in the South-East region.',
             5);
        
        -- Level 2: Ethnic Groups and Traditions
        INSERT OR REPLACE INTO levels (id, module_id, title, difficulty, order_index, xp_reward, unlock_item_id)
        VALUES ('abj_culture_001_lvl2', 'abj_culture_001', 'People and Traditions', 'medium', 2, 150, 'badge_cultural_ambassador');
        
        -- Level 2 Questions
        INSERT OR REPLACE INTO questions (id, level_id, question_text, question_type, options_json, correct_answer, xp_reward, explanation, hint, order_index)
        VALUES 
            ('abj_cu2_q1', 'abj_culture_001_lvl2',
             'What is a "Gele"?',
             'multiple_choice',
             '[{"id":"a","text":"A type of Nigerian soup"},{"id":"b","text":"A traditional headtie worn by women"},{"id":"c","text":"A musical instrument"},{"id":"d","text":"A type of dance"}]',
             'b', 20,
             'A Gele is a traditional headtie or head wrap worn by Nigerian women, especially at celebrations. Tying a Gele is an art form!',
             'It''s something women wear on their heads at parties.',
             1),
            
            ('abj_cu2_q2', 'abj_culture_001_lvl2',
             'Which festival celebrates the New Yam harvest and is popular in Igbo culture?',
             'multiple_choice',
             '[{"id":"a","text":"Eyo Festival"},{"id":"b","text":"Durbar Festival"},{"id":"c","text":"New Yam Festival (Iri Ji)"},{"id":"d","text":"Argungu Fishing Festival"}]',
             'c', 20,
             'The New Yam Festival (Iri Ji or Iwa Ji) is an Igbo celebration of the yam harvest. Yam is so important it''s called the "King of Crops" in Igbo land!',
             'The festival name mentions the crop being celebrated.',
             2),
            
            ('abj_cu2_q3', 'abj_culture_001_lvl2',
             'The "Agbada" is a traditional flowing robe worn mainly by:',
             'multiple_choice',
             '[{"id":"a","text":"Women only"},{"id":"b","text":"Men only"},{"id":"c","text":"Children only"},{"id":"d","text":"Men, especially for important occasions"}]',
             'd', 20,
             'Agbada is a wide-sleeved flowing robe traditionally worn by Yoruba, Hausa, and other Nigerian men for important occasions. It''s a symbol of dignity and wealth.',
             'You often see important men wearing this at ceremonies.',
             3),
            
            ('abj_cu2_q4', 'abj_culture_001_lvl2',
             'True or False: The Tiv people are famous for their "Kwagh-hir" puppet theatre.',
             'true_false',
             '[{"id":"true","text":"True"},{"id":"false","text":"False"}]',
             'true', 15,
             'True! The Tiv people of Benue State are known for Kwagh-hir, an elaborate puppet theatre that tells stories through carved figures, dance, and music.',
             NULL,
             4),
            
            ('abj_cu2_q5', 'abj_culture_001_lvl2',
             'The Durbar Festival features colorful horse parades and is celebrated mainly in which region?',
             'multiple_choice',
             '[{"id":"a","text":"South-West"},{"id":"b","text":"South-East"},{"id":"c","text":"Northern Nigeria"},{"id":"d","text":"South-South"}]',
             'c', 20,
             'The Durbar Festival is celebrated in Northern Nigerian cities like Kano, Katsina, and Zaria. It features magnificent horse parades to honor the Emir after Eid prayers.',
             'This festival is associated with Islamic celebrations in the north.',
             5);

        -- Level 3: National Symbols
        INSERT OR REPLACE INTO levels (id, module_id, title, difficulty, order_index, xp_reward, unlock_item_id)
        VALUES ('abj_culture_001_lvl3', 'abj_culture_001', 'Our National Symbols', 'medium', 3, 150, 'badge_patriot');
        
        -- Level 3 Questions
        INSERT OR REPLACE INTO questions (id, level_id, question_text, question_type, options_json, correct_answer, xp_reward, explanation, hint, order_index)
        VALUES 
            ('abj_cu3_q1', 'abj_culture_001_lvl3',
             'What do the green stripes on Nigeria''s flag represent?',
             'multiple_choice',
             '[{"id":"a","text":"The rivers"},{"id":"b","text":"Agriculture and natural wealth"},{"id":"c","text":"Peace"},{"id":"d","text":"The military"}]',
             'b', 20,
             'The green stripes represent Nigeria''s agriculture, natural wealth, and forests. The white stripe represents peace and unity.',
             'Think about what color plants are.',
             1),
            
            ('abj_cu3_q2', 'abj_culture_001_lvl3',
             'What animal is at the top of Nigeria''s Coat of Arms?',
             'multiple_choice',
             '[{"id":"a","text":"Lion"},{"id":"b","text":"Horse"},{"id":"c","text":"Eagle"},{"id":"d","text":"Elephant"}]',
             'c', 20,
             'An eagle sits at the top of Nigeria''s Coat of Arms, representing strength. The two horses represent dignity, and the black shield with the white Y represents the Niger and Benue rivers.',
             'It''s a bird that symbolizes strength and power.',
             2),
            
            ('abj_cu3_q3', 'abj_culture_001_lvl3',
             'What is Nigeria''s national motto?',
             'multiple_choice',
             '[{"id":"a","text":"One Nigeria, Great Nation"},{"id":"b","text":"Unity and Faith, Peace and Progress"},{"id":"c","text":"Together We Stand"},{"id":"d","text":"In God We Trust"}]',
             'b', 25,
             '"Unity and Faith, Peace and Progress" is Nigeria''s national motto. It appears on the Coat of Arms and represents our national aspirations.',
             'It has four key values separated by commas.',
             3),
            
            ('abj_cu3_q4', 'abj_culture_001_lvl3',
             'Who wrote Nigeria''s current National Anthem "Arise, O Compatriots"?',
             'multiple_choice',
             '[{"id":"a","text":"Wole Soyinka"},{"id":"b","text":"Ben Odiase"},{"id":"c","text":"A collective of Nigerian authors"},{"id":"d","text":"Chinua Achebe"}]',
             'c', 20,
             'Nigeria''s current anthem was written collectively by John A. Ilechukwu, Eme Etim Akpan, B.A. Ogunnaike, Sota Omoigui, and P.O. Aderibigbe. The music was composed by Benedict Elide Odiase.',
             'It wasn''t written by one person.',
             4),
            
            ('abj_cu3_q5', 'abj_culture_001_lvl3',
             'When did Nigeria gain independence from Britain?',
             'multiple_choice',
             '[{"id":"a","text":"October 1, 1960"},{"id":"b","text":"May 29, 1999"},{"id":"c","text":"January 1, 1970"},{"id":"d","text":"June 12, 1993"}]',
             'a', 25,
             'Nigeria gained independence on October 1, 1960. This date is celebrated annually as Independence Day. May 29 is Democracy Day, marking our return to civilian rule in 1999.',
             'We celebrate this date every year in October.',
             5);
    "#).map_err(|e| DatabaseError::QueryError(e.to_string()))?;

    // Module 4: Nigerian History Basics
    conn.execute_batch(r#"
        -- =====================================================
        -- ABUJA MODULE 4: ROOTS OF NIGERIA (Basic History)
        -- =====================================================
        INSERT OR REPLACE INTO modules (id, state_id, subject, title, description, required_level, total_xp, estimated_time, icon, education_level, interest_tags)
        VALUES ('abj_history_001', 'ABJ', 'History', 'Roots of Nigeria', 
                'Travel back in time to discover how Nigeria came to be! Learn about ancient kingdoms, colonial history, and our path to independence.',
                1, 500, 22, 'clock', 'all', '["history"]');
        
        -- Module Context
        INSERT OR REPLACE INTO module_context (module_id, did_you_know, fun_fact, intro_text, historical_note)
        VALUES ('abj_history_001',
                'Before colonization, the area now called Nigeria was home to powerful kingdoms and empires like the Benin Empire, Oyo Empire, Sokoto Caliphate, and the Kanem-Bornu Empire!',
                'The name "Nigeria" was coined by Flora Shaw in an 1897 newspaper article. She was the girlfriend (and later wife) of Lord Lugard, who merged Northern and Southern Nigeria in 1914.',
                'Welcome, young historian! Nigeria has a rich history that stretches back thousands of years. From ancient kingdoms with bronze sculptures that amazed the world, to our struggle for independence and becoming Africa''s most populous nation - let''s explore it all!',
                'The amalgamation of Northern and Southern Nigeria on January 1, 1914, by Lord Lugard created the country we know today as Nigeria.');
        
        -- Level 1: Ancient Kingdoms
        INSERT OR REPLACE INTO levels (id, module_id, title, difficulty, order_index, xp_reward, unlock_item_id)
        VALUES ('abj_history_001_lvl1', 'abj_history_001', 'Ancient Kingdoms', 'easy', 1, 100, 'badge_historian');
        
        -- Level 1 Questions
        INSERT OR REPLACE INTO questions (id, level_id, question_text, question_type, options_json, correct_answer, xp_reward, explanation, hint, order_index)
        VALUES 
            ('abj_h1_q1', 'abj_history_001_lvl1', 
             'The ancient Benin Kingdom was famous for creating beautiful artworks made of:',
             'multiple_choice',
             '[{"id":"a","text":"Gold"},{"id":"b","text":"Bronze and brass"},{"id":"c","text":"Silver"},{"id":"d","text":"Iron"}]',
             'b', 15,
             'The Benin Kingdom (in present-day Edo State) was world-famous for its bronze and brass sculptures. These "Benin Bronzes" showed amazing artistic skill and many are now in museums worldwide.',
             'These metals can be mixed to make sculptures.',
             1),
            
            ('abj_h1_q2', 'abj_history_001_lvl1',
             'The Oyo Empire was one of the largest empires in which region of Nigeria?',
             'multiple_choice',
             '[{"id":"a","text":"North-East"},{"id":"b","text":"South-West"},{"id":"c","text":"South-East"},{"id":"d","text":"North-West"}]',
             'b', 15,
             'The Oyo Empire was a powerful Yoruba empire in present-day South-West Nigeria. At its peak, it was one of the largest West African states.',
             'This is Yoruba land.',
             2),
            
            ('abj_h1_q3', 'abj_history_001_lvl1',
             'Which ancient Nigerian culture is famous for creating terracotta (clay) sculptures over 2,000 years ago?',
             'multiple_choice',
             '[{"id":"a","text":"Ife Culture"},{"id":"b","text":"Nok Culture"},{"id":"c","text":"Benin Culture"},{"id":"d","text":"Igbo-Ukwu Culture"}]',
             'b', 20,
             'The Nok Culture (from around present-day Plateau and Kaduna states) created terracotta sculptures as early as 500 BC. These are among the oldest sculptures in sub-Saharan Africa!',
             'It shares its name with a place in Kaduna State.',
             3),
            
            ('abj_h1_q4', 'abj_history_001_lvl1',
             'True or False: The Sokoto Caliphate was founded by Usman dan Fodio in 1804.',
             'true_false',
             '[{"id":"true","text":"True"},{"id":"false","text":"False"}]',
             'true', 15,
             'True! Usman dan Fodio led a jihad (religious reform movement) that established the Sokoto Caliphate in 1804. It became one of the largest states in 19th century Africa.',
             NULL,
             4),
            
            ('abj_h1_q5', 'abj_history_001_lvl1',
             'The Kanem-Bornu Empire was located in which part of present-day Nigeria?',
             'multiple_choice',
             '[{"id":"a","text":"South-West"},{"id":"b","text":"North-East"},{"id":"c","text":"South-South"},{"id":"d","text":"North-Central"}]',
             'b', 15,
             'The Kanem-Bornu Empire was centered around present-day Borno State in North-East Nigeria. It lasted for over 1,000 years!',
             'Look at the name of the empire - it matches a current state.',
             5);
        
        -- Level 2: Colonial Period
        INSERT OR REPLACE INTO levels (id, module_id, title, difficulty, order_index, xp_reward, unlock_item_id)
        VALUES ('abj_history_001_lvl2', 'abj_history_001', 'The Colonial Era', 'medium', 2, 150, 'badge_freedom_scholar');
        
        -- Level 2 Questions
        INSERT OR REPLACE INTO questions (id, level_id, question_text, question_type, options_json, correct_answer, xp_reward, explanation, hint, order_index)
        VALUES 
            ('abj_h2_q1', 'abj_history_001_lvl2',
             'Which European country colonized Nigeria?',
             'multiple_choice',
             '[{"id":"a","text":"France"},{"id":"b","text":"Portugal"},{"id":"c","text":"Britain"},{"id":"d","text":"Germany"}]',
             'c', 20,
             'Britain colonized Nigeria. The British first established trading posts on the coast, then gradually took control of the entire region.',
             'This is why English is our official language.',
             1),
            
            ('abj_h2_q2', 'abj_history_001_lvl2',
             'In what year were the Northern and Southern Protectorates merged to form Nigeria?',
             'multiple_choice',
             '[{"id":"a","text":"1900"},{"id":"b","text":"1914"},{"id":"c","text":"1960"},{"id":"d","text":"1861"}]',
             'b', 20,
             'The Northern and Southern Protectorates were merged on January 1, 1914, by Lord Frederick Lugard. This amalgamation created the entity we now know as Nigeria.',
             'It''s during World War I.',
             2),
            
            ('abj_h2_q3', 'abj_history_001_lvl2',
             'Which Nigerian city was the main port for the transatlantic slave trade?',
             'multiple_choice',
             '[{"id":"a","text":"Kano"},{"id":"b","text":"Calabar"},{"id":"c","text":"Abuja"},{"id":"d","text":"Sokoto"}]',
             'b', 20,
             'Calabar (in present-day Cross River State) was a major port for the transatlantic slave trade. The slave history museum in Calabar preserves this painful history.',
             'It''s a coastal city in the South-South region.',
             3),
            
            ('abj_h2_q4', 'abj_history_001_lvl2',
             'The Royal Niger Company was a British trading company that helped colonize Nigeria. What did it mainly trade?',
             'multiple_choice',
             '[{"id":"a","text":"Gold and diamonds"},{"id":"b","text":"Palm oil and other products"},{"id":"c","text":"Rubber only"},{"id":"d","text":"Cattle"}]',
             'b', 20,
             'The Royal Niger Company mainly traded palm oil, palm kernels, and other agricultural products. Palm oil was called "red gold" because of its high value.',
             'Think about what grows in the forests of Southern Nigeria.',
             4),
            
            ('abj_h2_q5', 'abj_history_001_lvl2',
             'Who was the first Nigerian to be appointed to the Legislative Council in 1923?',
             'multiple_choice',
             '[{"id":"a","text":"Nnamdi Azikiwe"},{"id":"b","text":"Herbert Macaulay"},{"id":"c","text":"Obafemi Awolowo"},{"id":"d","text":"Ahmadu Bello"}]',
             'b', 25,
             'Herbert Macaulay is considered the "Father of Nigerian Nationalism." He founded Nigeria''s first political party (NNDP) and fought for Nigerian representation in government.',
             'He is called the "Father of Nigerian Nationalism."',
             5);

        -- Level 3: Independence and After
        INSERT OR REPLACE INTO levels (id, module_id, title, difficulty, order_index, xp_reward, unlock_item_id)
        VALUES ('abj_history_001_lvl3', 'abj_history_001', 'Independence and Beyond', 'medium', 3, 200, 'badge_independence');
        
        -- Level 3 Questions
        INSERT OR REPLACE INTO questions (id, level_id, question_text, question_type, options_json, correct_answer, xp_reward, explanation, hint, order_index)
        VALUES 
            ('abj_h3_q1', 'abj_history_001_lvl3',
             'Who was Nigeria''s first Prime Minister at independence?',
             'multiple_choice',
             '[{"id":"a","text":"Nnamdi Azikiwe"},{"id":"b","text":"Obafemi Awolowo"},{"id":"c","text":"Abubakar Tafawa Balewa"},{"id":"d","text":"Ahmadu Bello"}]',
             'c', 25,
             'Sir Abubakar Tafawa Balewa became Nigeria''s first Prime Minister on October 1, 1960. He was from Bauchi State.',
             'His image was on the old Nigerian ₦5 note.',
             1),
            
            ('abj_h3_q2', 'abj_history_001_lvl3',
             'Who was Nigeria''s first President (ceremonial)?',
             'multiple_choice',
             '[{"id":"a","text":"Obafemi Awolowo"},{"id":"b","text":"Nnamdi Azikiwe"},{"id":"c","text":"Ahmadu Bello"},{"id":"d","text":"Shehu Shagari"}]',
             'b', 25,
             'Dr. Nnamdi Azikiwe became Nigeria''s first President (ceremonial role) in 1963 when Nigeria became a Republic. He was known as "Zik of Africa."',
             'He is called "Zik of Africa."',
             2),
            
            ('abj_h3_q3', 'abj_history_001_lvl3',
             'The Nigerian Civil War (1967-1970) involved which region attempting to secede?',
             'multiple_choice',
             '[{"id":"a","text":"Western Region as Oduduwa"},{"id":"b","text":"Northern Region as Arewa"},{"id":"c","text":"Eastern Region as Biafra"},{"id":"d","text":"Mid-Western Region as Bendel"}]',
             'c', 25,
             'The Eastern Region, led by Colonel Odumegwu Ojukwu, declared independence as the Republic of Biafra in 1967. The civil war lasted until January 1970.',
             'This word starts with "B" and was led by Ojukwu.',
             3),
            
            ('abj_h3_q4', 'abj_history_001_lvl3',
             'In what year did Nigeria become a Republic?',
             'multiple_choice',
             '[{"id":"a","text":"1960"},{"id":"b","text":"1963"},{"id":"c","text":"1979"},{"id":"d","text":"1999"}]',
             'b', 20,
             'Nigeria became a Republic on October 1, 1963, exactly three years after independence. The Queen of England was no longer the Head of State.',
             'It was exactly 3 years after independence.',
             4),
            
            ('abj_h3_q5', 'abj_history_001_lvl3',
             'Which Nigerian head of state moved the capital from Lagos to Abuja?',
             'multiple_choice',
             '[{"id":"a","text":"Shehu Shagari"},{"id":"b","text":"Ibrahim Babangida"},{"id":"c","text":"Olusegun Obasanjo"},{"id":"d","text":"Sani Abacha"}]',
             'b', 25,
             'General Ibrahim Babangida officially moved the federal capital from Lagos to Abuja on December 12, 1991, though the plan was conceived under Murtala Muhammed in 1976.',
             'He was known as "IBB" and ruled from 1985-1993.',
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
        INSERT OR REPLACE INTO modules (id, state_id, subject, title, description, required_level, total_xp, estimated_time, icon, education_level, interest_tags)
        VALUES ('lag_math_001', 'LAG', 'Mathematics', 'The Balogun Market Challenge', 
                'Master the art of buying and selling in West Africa''s biggest market! Learn arithmetic, percentages, and financial calculations.',
                2, 600, 25, 'calculator', 'all', '["food"]');
        
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
        INSERT OR REPLACE INTO modules (id, state_id, subject, title, description, required_level, total_xp, estimated_time, icon, education_level, interest_tags)
        VALUES ('lag_logic_001', 'LAG', 'Logic & Coding', 'Yaba Tech: Logic Puzzles', 
                'Welcome to Nigeria''s tech hub! Train your brain with logic puzzles and algorithmic thinking.',
                2, 450, 20, 'cpu', 'all', '[]');
        
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
    
    // Seed additional state modules
    seed_kano_modules(conn)?;
    seed_edo_modules(conn)?;
    seed_enugu_modules(conn)?;
    seed_plateau_modules(conn)?;
    seed_bauchi_modules(conn)?;
    seed_anambra_modules(conn)?;
    
    Ok(())
}

// =====================================================
// KANO STATE MODULES
// =====================================================
fn seed_kano_modules(conn: &Connection) -> Result<(), DatabaseError> {
    conn.execute_batch(r#"
        -- =====================================================
        -- KANO MODULE 1: THE ANCIENT TRADE ROUTES (History/Commerce)
        -- =====================================================
        INSERT OR REPLACE INTO modules (id, state_id, subject, title, description, required_level, total_xp, estimated_time, icon, education_level, interest_tags)
        VALUES ('kan_history_001', 'KAN', 'History', 'The Ancient Trade Routes', 
                'Discover the history of trans-Saharan trade that made Kano one of Africa''s greatest commercial cities!',
                4, 550, 22, 'map', 'all', '["history", "geography"]');
        
        INSERT OR REPLACE INTO module_context (module_id, did_you_know, fun_fact, intro_text, historical_note)
        VALUES ('kan_history_001',
                'Kano has been a trading center for over 1000 years! Merchants from across the Sahara brought salt, cloth, and books in exchange for gold, kola nuts, and leather.',
                'The famous Kano Groundnut Pyramids were so large they appeared on Nigerian currency! At their peak, they contained millions of bags of groundnuts.',
                'Welcome to ancient Kano! Walk the same paths that merchants from Morocco, Egypt, and Arabia traveled centuries ago. Learn about the goods they traded and the cultures that connected across the desert.',
                'The Hausa city-states, including Kano, were established around 1000 CE. By the 1400s, Kano was one of the most important cities in the trans-Saharan trade network.');
        
        INSERT OR REPLACE INTO levels (id, module_id, title, difficulty, order_index, xp_reward, unlock_item_id)
        VALUES ('kan_history_001_lvl1', 'kan_history_001', 'Merchants of the Sahara', 'medium', 1, 120, NULL);
        
        INSERT OR REPLACE INTO questions (id, level_id, question_text, question_type, options_json, correct_answer, xp_reward, explanation, hint, order_index)
        VALUES 
            ('kan_h1_q1', 'kan_history_001_lvl1',
             'What valuable item was brought FROM the Sahara to Kano for trade?',
             'multiple_choice',
             '[{"id":"a","text":"Gold"},{"id":"b","text":"Salt"},{"id":"c","text":"Kola nuts"},{"id":"d","text":"Groundnuts"}]',
             'b', 20,
             'Salt was extremely valuable and was mined in the Sahara Desert. It was traded southward for gold, kola nuts, and other goods.',
             'This mineral is essential for preserving food and was rare in the savanna.',
             1),
            
            ('kan_h1_q2', 'kan_history_001_lvl1',
             'Kurmi Market in Kano is famous for being:',
             'multiple_choice',
             '[{"id":"a","text":"The newest market in Nigeria"},{"id":"b","text":"One of the oldest markets in West Africa"},{"id":"c","text":"Only for selling electronics"},{"id":"d","text":"A market that only opens once a year"}]',
             'b', 20,
             'Kurmi Market has been in continuous operation for over 500 years, making it one of the oldest and most historic markets in West Africa.',
             'Think about the word "ancient" in connection to Kano.',
             2),
            
            ('kan_h1_q3', 'kan_history_001_lvl1',
             'The famous Kano dye pits are used to create which color?',
             'multiple_choice',
             '[{"id":"a","text":"Red"},{"id":"b","text":"Yellow"},{"id":"c","text":"Indigo (deep blue)"},{"id":"d","text":"Green"}]',
             'c', 20,
             'Kano is famous for its indigo dye pits, some over 500 years old. The deep blue fabric produced here is prized across Africa.',
             'Think of a deep blue color - the same as blue jeans originally used.',
             3),
            
            ('kan_h1_q4', 'kan_history_001_lvl1',
             'What desert did traders cross to reach Kano from North Africa?',
             'multiple_choice',
             '[{"id":"a","text":"Gobi Desert"},{"id":"b","text":"Sahara Desert"},{"id":"c","text":"Kalahari Desert"},{"id":"d","text":"Arabian Desert"}]',
             'b', 20,
             'The Sahara Desert is the world''s largest hot desert and separates North Africa from sub-Saharan Africa. Trade routes crossed it for thousands of years.',
             'It''s the largest hot desert in the world, located in Africa.',
             4),
            
            ('kan_h1_q5', 'kan_history_001_lvl1',
             'True or False: Camels were essential for trans-Saharan trade.',
             'true_false',
             '[{"id":"true","text":"True"},{"id":"false","text":"False"}]',
             'true', 15,
             'Camels, known as "ships of the desert," could travel long distances without water and carry heavy loads, making trans-Saharan trade possible.',
             NULL,
             5);
    "#).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    Ok(())
}

// =====================================================
// EDO STATE MODULES
// =====================================================
fn seed_edo_modules(conn: &Connection) -> Result<(), DatabaseError> {
    conn.execute_batch(r#"
        -- =====================================================
        -- EDO MODULE 1: THE BENIN BRONZES (Art/History)
        -- =====================================================
        INSERT OR REPLACE INTO modules (id, state_id, subject, title, description, required_level, total_xp, estimated_time, icon, education_level, interest_tags)
        VALUES ('edo_art_001', 'EDO', 'Art & History', 'The Benin Bronzes', 
                'Explore the magnificent art of the Benin Kingdom - masterpieces that amazed the world!',
                5, 600, 25, 'crown', 'all', '["history", "music", "culture"]');
        
        INSERT OR REPLACE INTO module_context (module_id, did_you_know, fun_fact, intro_text, historical_note)
        VALUES ('edo_art_001',
                'Benin bronze-casting technique is so sophisticated that European scientists initially refused to believe Africans made them! The lost-wax casting method used is extremely complex.',
                'The Oba (King) of Benin commissioned artworks to record history. Some bronze plaques show Portuguese traders who arrived in the 15th century - one of the earliest visual records of European contact!',
                'Welcome to the Royal Palace of Benin! Here, master craftsmen created bronze sculptures so beautiful that museums around the world still treasure them. Learn about this incredible artistic tradition.',
                'The Benin Kingdom flourished from the 13th century until 1897. At its height, the capital Benin City was larger than London and had electric street lighting using palm oil lamps!');
        
        INSERT OR REPLACE INTO levels (id, module_id, title, difficulty, order_index, xp_reward, unlock_item_id)
        VALUES ('edo_art_001_lvl1', 'edo_art_001', 'Royal Craftsmen', 'medium', 1, 130, NULL);
        
        INSERT OR REPLACE INTO questions (id, level_id, question_text, question_type, options_json, correct_answer, xp_reward, explanation, hint, order_index)
        VALUES 
            ('edo_a1_q1', 'edo_art_001_lvl1',
             'The "Benin Bronzes" are famous artworks made primarily from:',
             'multiple_choice',
             '[{"id":"a","text":"Wood"},{"id":"b","text":"Brass and bronze"},{"id":"c","text":"Clay"},{"id":"d","text":"Gold"}]',
             'b', 20,
             'Despite being called "bronzes," most Benin artworks are actually made of brass (a mixture of copper and zinc). The term "bronze" was applied by Europeans.',
             'The name gives a hint, though the actual metal is slightly different.',
             1),
            
            ('edo_a1_q2', 'edo_art_001_lvl1',
             'What is the title of the King of Benin?',
             'multiple_choice',
             '[{"id":"a","text":"Sultan"},{"id":"b","text":"Chief"},{"id":"c","text":"Oba"},{"id":"d","text":"Emir"}]',
             'c', 20,
             'The Oba is the traditional ruler of the Benin Kingdom. The title has been held continuously for over 700 years, making it one of Africa''s oldest monarchies.',
             'This three-letter title is unique to Benin Kingdom.',
             2),
            
            ('edo_a1_q3', 'edo_art_001_lvl1',
             'The technique used to create Benin bronzes is called:',
             'multiple_choice',
             '[{"id":"a","text":"Pottery throwing"},{"id":"b","text":"Lost-wax casting"},{"id":"c","text":"Wood carving"},{"id":"d","text":"Stone cutting"}]',
             'b', 25,
             'Lost-wax casting involves creating a wax model, covering it in clay, then melting out the wax and pouring in molten metal. It produces incredibly detailed sculptures.',
             'A wax model is used and then "lost" in the process.',
             3),
            
            ('edo_a1_q4', 'edo_art_001_lvl1',
             'True or False: Many Benin Bronzes are now in museums outside Nigeria.',
             'true_false',
             '[{"id":"true","text":"True"},{"id":"false","text":"False"}]',
             'true', 15,
             'Thousands of Benin artworks were taken during the 1897 British invasion and are now in museums worldwide. Nigeria and these museums are discussing returning them.',
             NULL,
             4),
            
            ('edo_a1_q5', 'edo_art_001_lvl1',
             'Benin City''s ancient walls (Iya) were once comparable in size to:',
             'multiple_choice',
             '[{"id":"a","text":"A football field"},{"id":"b","text":"The Great Wall of China"},{"id":"c","text":"A house"},{"id":"d","text":"A small village"}]',
             'b', 25,
             'The Benin Moat (Iya) consisted of over 16,000 km of walls and ditches - it was described as the world''s largest man-made earthwork before modern times!',
             'Think VERY large - one of the biggest structures in history.',
             5);
    "#).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    Ok(())
}

// =====================================================
// ENUGU STATE MODULES
// =====================================================
fn seed_enugu_modules(conn: &Connection) -> Result<(), DatabaseError> {
    conn.execute_batch(r#"
        -- =====================================================
        -- ENUGU MODULE 1: THE COAL CITY (Science/Industry)
        -- =====================================================
        INSERT OR REPLACE INTO modules (id, state_id, subject, title, description, required_level, total_xp, estimated_time, icon, education_level, interest_tags)
        VALUES ('enu_science_001', 'ENU', 'Science', 'The Coal City Story', 
                'Discover how coal shaped Nigeria''s industrial history and learn about energy and resources!',
                5, 500, 20, 'flame', 'all', '["history"]');
        
        INSERT OR REPLACE INTO module_context (module_id, did_you_know, fun_fact, intro_text, historical_note)
        VALUES ('enu_science_001',
                'Coal was so important to colonial Nigeria that the railway was built specifically to transport it from Enugu to Port Harcourt. The whole city grew around coal mining!',
                'Nigerian coal miners went on strike in 1949 at Iva Valley, and 21 miners were killed by police. This event sparked nationwide protests and helped speed up Nigerian independence!',
                'Welcome to the Coal City! Enugu''s story is one of how a natural resource changed history. Learn about coal, energy, and how mining shaped a nation.',
                'Coal was discovered in Enugu in 1909 by a British geological survey team. By the 1950s, Enugu was producing over 900,000 tons of coal per year.');
        
        INSERT OR REPLACE INTO levels (id, module_id, title, difficulty, order_index, xp_reward, unlock_item_id)
        VALUES ('enu_science_001_lvl1', 'enu_science_001', 'Underground Treasures', 'medium', 1, 110, NULL);
        
        INSERT OR REPLACE INTO questions (id, level_id, question_text, question_type, options_json, correct_answer, xp_reward, explanation, hint, order_index)
        VALUES 
            ('enu_s1_q1', 'enu_science_001_lvl1',
             'Coal is formed from the remains of:',
             'multiple_choice',
             '[{"id":"a","text":"Dinosaurs"},{"id":"b","text":"Ancient plants and trees"},{"id":"c","text":"Sea creatures"},{"id":"d","text":"Volcanic rocks"}]',
             'b', 20,
             'Coal is a fossil fuel formed from ancient plants that died millions of years ago. Over time, heat and pressure transformed them into coal.',
             'Think about what grows in forests and swamps.',
             1),
            
            ('enu_s1_q2', 'enu_science_001_lvl1',
             'Why is Enugu called "Coal City"?',
             'multiple_choice',
             '[{"id":"a","text":"It sells charcoal"},{"id":"b","text":"It was built around coal mines"},{"id":"c","text":"Buildings are black"},{"id":"d","text":"It''s very hot there"}]',
             'b', 15,
             'Enugu literally grew because of coal mining. The discovery of coal led to the building of railways, workers'' housing, and eventually a major city.',
             'The city exists because of what was found underground.',
             2),
            
            ('enu_s1_q3', 'enu_science_001_lvl1',
             'Coal is used mainly as a source of:',
             'multiple_choice',
             '[{"id":"a","text":"Water"},{"id":"b","text":"Energy"},{"id":"c","text":"Food"},{"id":"d","text":"Medicine"}]',
             'b', 15,
             'Coal is burned to produce heat energy, which can power steam engines, generate electricity, and heat homes.',
             'When something burns, what does it release?',
             3),
            
            ('enu_s1_q4', 'enu_science_001_lvl1',
             'The Iva Valley incident in 1949 involved:',
             'multiple_choice',
             '[{"id":"a","text":"A flood"},{"id":"b","text":"Coal miners'' strike"},{"id":"c","text":"A festival"},{"id":"d","text":"A football match"}]',
             'b', 20,
             'The Iva Valley Massacre occurred when coal miners striking for better conditions were shot by colonial police. This tragedy became a turning point toward independence.',
             'Workers who dig coal wanted better conditions.',
             4),
            
            ('enu_s1_q5', 'enu_science_001_lvl1',
             'True or False: Fossil fuels like coal take millions of years to form.',
             'true_false',
             '[{"id":"true","text":"True"},{"id":"false","text":"False"}]',
             'true', 15,
             'Coal, oil, and natural gas are fossil fuels that formed over millions of years from dead organisms. That''s why they''re considered non-renewable resources.',
             NULL,
             5);
    "#).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    Ok(())
}

// =====================================================
// PLATEAU STATE MODULES
// =====================================================
fn seed_plateau_modules(conn: &Connection) -> Result<(), DatabaseError> {
    conn.execute_batch(r#"
        -- =====================================================
        -- PLATEAU MODULE 1: THE NOK CIVILIZATION (History)
        -- =====================================================
        INSERT OR REPLACE INTO modules (id, state_id, subject, title, description, required_level, total_xp, estimated_time, icon, education_level, interest_tags)
        VALUES ('pla_history_001', 'PLA', 'History', 'The Nok Civilization', 
                'Journey back 2,500 years to discover Africa''s oldest known sculpture tradition!',
                4, 550, 22, 'landmark', 'all', '["history", "culture"]');
        
        INSERT OR REPLACE INTO module_context (module_id, did_you_know, fun_fact, intro_text, historical_note)
        VALUES ('pla_history_001',
                'Nok terracotta sculptures are so old that they predate the Roman Empire! The earliest pieces date to around 500 BCE.',
                'The Nok people were among the first in sub-Saharan Africa to smelt iron - a technological achievement that changed human history!',
                'Welcome, archaeologist! You are about to discover one of Africa''s most mysterious ancient civilizations. The Nok people created incredible art and technology thousands of years ago.',
                'The Nok culture was discovered by accident in 1928 when tin miners found terracotta sculptures. The civilization existed from about 1500 BCE to 500 CE.');
        
        INSERT OR REPLACE INTO levels (id, module_id, title, difficulty, order_index, xp_reward, unlock_item_id)
        VALUES ('pla_history_001_lvl1', 'pla_history_001', 'Ancient Sculptors', 'medium', 1, 120, NULL);
        
        INSERT OR REPLACE INTO questions (id, level_id, question_text, question_type, options_json, correct_answer, xp_reward, explanation, hint, order_index)
        VALUES 
            ('pla_h1_q1', 'pla_history_001_lvl1',
             'Nok sculptures are made from:',
             'multiple_choice',
             '[{"id":"a","text":"Bronze"},{"id":"b","text":"Terracotta (baked clay)"},{"id":"c","text":"Stone"},{"id":"d","text":"Wood"}]',
             'b', 20,
             'Terracotta means "baked earth" in Italian. The Nok people created their sculptures from clay and fired them at high temperatures.',
             'This material comes from the earth and is hardened by fire.',
             1),
            
            ('pla_h1_q2', 'pla_history_001_lvl1',
             'How old are the oldest Nok sculptures?',
             'multiple_choice',
             '[{"id":"a","text":"About 100 years"},{"id":"b","text":"About 500 years"},{"id":"c","text":"About 2,500 years"},{"id":"d","text":"About 50 years"}]',
             'c', 20,
             'The oldest Nok sculptures date to around 500 BCE, making them about 2,500 years old - older than many ancient civilizations!',
             'They''re VERY old - even older than some famous ancient empires.',
             2),
            
            ('pla_h1_q3', 'pla_history_001_lvl1',
             'The Nok were pioneers in:',
             'multiple_choice',
             '[{"id":"a","text":"Building pyramids"},{"id":"b","text":"Iron smelting"},{"id":"c","text":"Making paper"},{"id":"d","text":"Sailing ships"}]',
             'b', 25,
             'The Nok were among the first people in sub-Saharan Africa to smelt iron - heating iron ore to extract metal for tools and weapons.',
             'This technology involves heating metal from rocks.',
             3),
            
            ('pla_h1_q4', 'pla_history_001_lvl1',
             'Nok sculptures were discovered by:',
             'multiple_choice',
             '[{"id":"a","text":"Archaeologists looking for them"},{"id":"b","text":"Tin miners by accident"},{"id":"c","text":"Farmers"},{"id":"d","text":"Fishermen"}]',
             'b', 20,
             'The first Nok sculpture was found accidentally in 1928 by workers mining for tin in present-day Plateau State.',
             'They were looking for something else entirely.',
             4),
            
            ('pla_h1_q5', 'pla_history_001_lvl1',
             'True or False: Jos, capital of Plateau State, has a museum dedicated to Nok art.',
             'true_false',
             '[{"id":"true","text":"True"},{"id":"false","text":"False"}]',
             'true', 15,
             'The Jos Museum was established in 1952 and houses one of the finest collections of Nok terracotta sculptures in the world.',
             NULL,
             5);
    "#).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    Ok(())
}

// =====================================================
// BAUCHI STATE MODULES
// =====================================================
fn seed_bauchi_modules(conn: &Connection) -> Result<(), DatabaseError> {
    conn.execute_batch(r#"
        -- =====================================================
        -- BAUCHI MODULE 1: YANKARI WILDLIFE (Science/Nature)
        -- =====================================================
        INSERT OR REPLACE INTO modules (id, state_id, subject, title, description, required_level, total_xp, estimated_time, icon, education_level, interest_tags)
        VALUES ('bau_science_001', 'BAU', 'Science', 'Yankari Wildlife Safari', 
                'Explore Nigeria''s premier wildlife reserve and learn about animal conservation!',
                5, 550, 24, 'paw-print', 'all', '["geography"]');
        
        INSERT OR REPLACE INTO module_context (module_id, did_you_know, fun_fact, intro_text, historical_note)
        VALUES ('bau_science_001',
                'Yankari has warm springs called Wikki Warm Springs where the water stays at 31°C all year round! Animals and humans have bathed there for centuries.',
                'Yankari has the largest remaining herd of African forest elephants in Nigeria - over 300 elephants call the reserve home!',
                'Welcome to Yankari Game Reserve, young ranger! Get ready for a safari where you''ll learn about Nigeria''s amazing wildlife and why conservation matters.',
                'Yankari was established as a game reserve in 1962 and became a National Park in 1991. It covers over 2,244 square kilometers.');
        
        INSERT OR REPLACE INTO levels (id, module_id, title, difficulty, order_index, xp_reward, unlock_item_id)
        VALUES ('bau_science_001_lvl1', 'bau_science_001', 'Safari Guide', 'medium', 1, 120, NULL);
        
        INSERT OR REPLACE INTO questions (id, level_id, question_text, question_type, options_json, correct_answer, xp_reward, explanation, hint, order_index)
        VALUES 
            ('bau_s1_q1', 'bau_science_001_lvl1',
             'Which animal is Yankari most famous for protecting?',
             'multiple_choice',
             '[{"id":"a","text":"Lions"},{"id":"b","text":"Elephants"},{"id":"c","text":"Zebras"},{"id":"d","text":"Giraffes"}]',
             'b', 20,
             'Yankari has Nigeria''s largest population of elephants. These gentle giants are a major attraction for visitors.',
             'Think of the largest land animal.',
             1),
            
            ('bau_s1_q2', 'bau_science_001_lvl1',
             'Wikki Warm Springs gets its heat from:',
             'multiple_choice',
             '[{"id":"a","text":"The sun"},{"id":"b","text":"Underground volcanic activity"},{"id":"c","text":"A heating machine"},{"id":"d","text":"Hot air"}]',
             'b', 20,
             'Warm springs are heated by geothermal activity deep underground where hot rocks heat the water naturally.',
             'The heat comes from deep inside the Earth.',
             2),
            
            ('bau_s1_q3', 'bau_science_001_lvl1',
             'What does "conservation" mean?',
             'multiple_choice',
             '[{"id":"a","text":"Destroying nature"},{"id":"b","text":"Protecting and preserving nature"},{"id":"c","text":"Building cities"},{"id":"d","text":"Hunting animals"}]',
             'b', 15,
             'Conservation means taking care of nature so that plants, animals, and natural places survive for future generations.',
             'It''s about keeping things safe and preserved.',
             3),
            
            ('bau_s1_q4', 'bau_science_001_lvl1',
             'Elephants are herbivores, which means they eat:',
             'multiple_choice',
             '[{"id":"a","text":"Only meat"},{"id":"b","text":"Only plants"},{"id":"c","text":"Both meat and plants"},{"id":"d","text":"Nothing"}]',
             'b', 15,
             'Herbivores only eat plants. Elephants eat grass, leaves, bark, and fruit - an adult elephant can eat up to 300 kg of food per day!',
             'The prefix "herb" relates to plants.',
             4),
            
            ('bau_s1_q5', 'bau_science_001_lvl1',
             'True or False: National Parks help protect endangered animals.',
             'true_false',
             '[{"id":"true","text":"True"},{"id":"false","text":"False"}]',
             'true', 15,
             'National Parks create safe spaces where animals can live without threats from hunting or habitat destruction.',
             NULL,
             5);
    "#).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    Ok(())
}

// =====================================================
// ANAMBRA STATE MODULES
// =====================================================
fn seed_anambra_modules(conn: &Connection) -> Result<(), DatabaseError> {
    conn.execute_batch(r#"
        -- =====================================================
        -- ANAMBRA MODULE 1: ONITSHA MARKET MATHEMATICS
        -- =====================================================
        INSERT OR REPLACE INTO modules (id, state_id, subject, title, description, required_level, total_xp, estimated_time, icon, education_level, interest_tags)
        VALUES ('ana_math_001', 'ANA', 'Mathematics', 'Onitsha Market Mathematics', 
                'Learn percentages and profit calculations at Africa''s largest market!',
                5, 600, 25, 'calculator', 'all', '["food"]');
        
        INSERT OR REPLACE INTO module_context (module_id, did_you_know, fun_fact, intro_text, historical_note)
        VALUES ('ana_math_001',
                'Onitsha Main Market has over 10,000 shops and is the largest market in Africa by the number of traders! You can find literally anything for sale.',
                'The Igbo people have a saying: "Ahịa adịghị egbu onye ohu" - meaning "trade doesn''t kill the diligent." Business skills are highly valued!',
                'Welcome to Onitsha Market! Here, fortunes are made by those who master the mathematics of business. Learn to calculate profit, loss, and percentages!',
                'Onitsha has been a trading center since before the colonial era. Its location on the River Niger made it perfect for commerce.');
        
        INSERT OR REPLACE INTO levels (id, module_id, title, difficulty, order_index, xp_reward, unlock_item_id)
        VALUES ('ana_math_001_lvl1', 'ana_math_001', 'Market Mathematics', 'medium', 1, 130, NULL);
        
        INSERT OR REPLACE INTO questions (id, level_id, question_text, question_type, options_json, correct_answer, xp_reward, explanation, hint, order_index)
        VALUES 
            ('ana_m1_q1', 'ana_math_001_lvl1',
             'A trader buys goods for ₦10,000 and sells for ₦12,000. What is the profit?',
             'multiple_choice',
             '[{"id":"a","text":"₦1,000"},{"id":"b","text":"₦2,000"},{"id":"c","text":"₦10,000"},{"id":"d","text":"₦22,000"}]',
             'b', 20,
             'Profit = Selling Price - Cost Price. So ₦12,000 - ₦10,000 = ₦2,000 profit.',
             'Subtract what you paid from what you received.',
             1),
            
            ('ana_m1_q2', 'ana_math_001_lvl1',
             'If something costs ₦500 and you mark it up by 20%, what is the selling price?',
             'multiple_choice',
             '[{"id":"a","text":"₦520"},{"id":"b","text":"₦600"},{"id":"c","text":"₦700"},{"id":"d","text":"₦400"}]',
             'b', 25,
             '20% of ₦500 = ₦100. So selling price = ₦500 + ₦100 = ₦600.',
             '20% of 500 is 100. Add that to the original price.',
             2),
            
            ('ana_m1_q3', 'ana_math_001_lvl1',
             'A trader sold goods for ₦8,000 but made a loss of ₦2,000. What was the cost price?',
             'multiple_choice',
             '[{"id":"a","text":"₦6,000"},{"id":"b","text":"₦10,000"},{"id":"c","text":"₦8,000"},{"id":"d","text":"₦16,000"}]',
             'b', 25,
             'If there was a loss, the cost was higher than the selling price. Cost = Selling Price + Loss = ₦8,000 + ₦2,000 = ₦10,000.',
             'Loss means they paid more than they received back.',
             3),
            
            ('ana_m1_q4', 'ana_math_001_lvl1',
             'What is 25% of ₦4,000?',
             'multiple_choice',
             '[{"id":"a","text":"₦500"},{"id":"b","text":"₦1,000"},{"id":"c","text":"₦2,500"},{"id":"d","text":"₦250"}]',
             'b', 20,
             '25% is the same as 1/4 or 0.25. So 25% of ₦4,000 = ₦4,000 × 0.25 = ₦1,000.',
             '25% is one quarter. What is one quarter of 4000?',
             4),
            
            ('ana_m1_q5', 'ana_math_001_lvl1',
             'True or False: If you buy at ₦100 and sell at ₦80, you make a loss.',
             'true_false',
             '[{"id":"true","text":"True"},{"id":"false","text":"False"}]',
             'true', 15,
             'When you sell for less than you bought, you make a loss. Here the loss is ₦100 - ₦80 = ₦20.',
             NULL,
             5);
    "#).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    Ok(())
}

// =====================================================
// OGUN STATE MODULES
// =====================================================
fn seed_ogun_modules(conn: &Connection) -> Result<(), DatabaseError> {
    conn.execute_batch(r#"
        INSERT OR REPLACE INTO modules (id, state_id, subject, title, description, required_level, total_xp, estimated_time, icon, education_level, interest_tags)
        VALUES ('ogu_english_001', 'OGU', 'English', 'Words of Wole Soyinka', 
                'Learn the power of language from Africa''s first Nobel Laureate in Literature!',
                4, 550, 22, 'book-open', 'all', '["languages", "culture"]');
        
        INSERT OR REPLACE INTO module_context (module_id, did_you_know, fun_fact, intro_text, historical_note)
        VALUES ('ogu_english_001',
                'Wole Soyinka was born in Abeokuta, Ogun State in 1934. He won the Nobel Prize for Literature in 1986 - the first African to win this prestigious award!',
                'Soyinka once took over a radio station at gunpoint to prevent false election results from being broadcast! He spent years in prison for his activism.',
                'Welcome to the literary heart of Nigeria! Here, we explore the power of words through the lens of our greatest writer. Learn vocabulary, expression, and the art of storytelling.',
                'Abeokuta means "Under the Rock" - referring to Olumo Rock, where the Egba people took refuge from slave raiders.');
        
        INSERT OR REPLACE INTO levels (id, module_id, title, difficulty, order_index, xp_reward, unlock_item_id)
        VALUES ('ogu_english_001_lvl1', 'ogu_english_001', 'Power of Words', 'medium', 1, 120, NULL);
        
        INSERT OR REPLACE INTO questions (id, level_id, question_text, question_type, options_json, correct_answer, xp_reward, explanation, hint, order_index)
        VALUES 
            ('ogu_e1_q1', 'ogu_english_001_lvl1',
             'What is a synonym for "brave"?',
             'multiple_choice',
             '[{"id":"a","text":"Scared"},{"id":"b","text":"Courageous"},{"id":"c","text":"Weak"},{"id":"d","text":"Tired"}]',
             'b', 15,
             'Courageous means showing courage or bravery. It''s a synonym (word with similar meaning) to brave.',
             'Which word means showing no fear?',
             1),
            
            ('ogu_e1_q2', 'ogu_english_001_lvl1',
             'Identify the verb in: "The eagle soars above the clouds."',
             'multiple_choice',
             '[{"id":"a","text":"eagle"},{"id":"b","text":"soars"},{"id":"c","text":"above"},{"id":"d","text":"clouds"}]',
             'b', 20,
             'A verb is an action word. "Soars" describes what the eagle does - the action of flying high.',
             'What is the eagle doing?',
             2),
            
            ('ogu_e1_q3', 'ogu_english_001_lvl1',
             'What is the opposite (antonym) of "ancient"?',
             'multiple_choice',
             '[{"id":"a","text":"Old"},{"id":"b","text":"Modern"},{"id":"c","text":"Historic"},{"id":"d","text":"Traditional"}]',
             'b', 15,
             'Modern means relating to present time, the opposite of ancient which means very old.',
             'If ancient means very old, what means very new?',
             3),
            
            ('ogu_e1_q4', 'ogu_english_001_lvl1',
             'Which sentence uses correct punctuation?',
             'multiple_choice',
             '[{"id":"a","text":"where are you going"},{"id":"b","text":"Where are you going?"},{"id":"c","text":"Where are you going"},{"id":"d","text":"where are you going."}]',
             'b', 20,
             'Questions need a capital letter at the start and a question mark at the end.',
             'Questions need special punctuation at the end.',
             4),
            
            ('ogu_e1_q5', 'ogu_english_001_lvl1',
             'True or False: Wole Soyinka won the Nobel Prize for Peace.',
             'true_false',
             '[{"id":"true","text":"True"},{"id":"false","text":"False"}]',
             'false', 15,
             'Wole Soyinka won the Nobel Prize for Literature, not Peace. He was honored for his dramatic works.',
             NULL,
             5);
    "#).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    Ok(())
}

// =====================================================
// OYO STATE MODULES  
// =====================================================
fn seed_oyo_modules(conn: &Connection) -> Result<(), DatabaseError> {
    conn.execute_batch(r#"
        INSERT OR REPLACE INTO modules (id, state_id, subject, title, description, required_level, total_xp, estimated_time, icon, education_level, interest_tags)
        VALUES ('oyo_history_001', 'OYO', 'History', 'The Oyo Empire', 
                'Discover one of Africa''s most powerful empires that ruled for over 400 years!',
                4, 600, 25, 'crown', 'all', '["history"]');
        
        INSERT OR REPLACE INTO module_context (module_id, did_you_know, fun_fact, intro_text, historical_note)
        VALUES ('oyo_history_001',
                'The Oyo Empire had a powerful cavalry (horse soldiers) that made it one of the most feared military forces in West Africa!',
                'The Alaafin of Oyo was not an absolute ruler - a council called the Oyo Mesi could even remove him from power. This was an early form of checks and balances!',
                'Welcome to the seat of an empire! The Oyo Empire once controlled trade routes across West Africa. Learn about its rise, governance, and legacy.',
                'At its peak in the 17th-18th centuries, the Oyo Empire stretched from western Nigeria to modern-day Togo and Ghana.');
        
        INSERT OR REPLACE INTO levels (id, module_id, title, difficulty, order_index, xp_reward, unlock_item_id)
        VALUES ('oyo_history_001_lvl1', 'oyo_history_001', 'Rise of an Empire', 'medium', 1, 130, NULL);
        
        INSERT OR REPLACE INTO questions (id, level_id, question_text, question_type, options_json, correct_answer, xp_reward, explanation, hint, order_index)
        VALUES 
            ('oyo_h1_q1', 'oyo_history_001_lvl1',
             'What is the title of the king of the Oyo Empire?',
             'multiple_choice',
             '[{"id":"a","text":"Oba"},{"id":"b","text":"Alaafin"},{"id":"c","text":"Sultan"},{"id":"d","text":"Emir"}]',
             'b', 20,
             'The Alaafin (meaning "Owner of the Palace") is the title of the king of the Oyo Empire. The current Alaafin still holds this traditional title.',
             'This title is unique to Oyo.',
             1),
            
            ('oyo_h1_q2', 'oyo_history_001_lvl1',
             'What military advantage helped the Oyo Empire dominate West Africa?',
             'multiple_choice',
             '[{"id":"a","text":"Ships"},{"id":"b","text":"Cavalry (horses)"},{"id":"c","text":"Cannons"},{"id":"d","text":"Elephants"}]',
             'b', 20,
             'The Oyo cavalry was the key to their military success. Horses allowed them to move quickly across the savanna and overwhelm enemies.',
             'These animals are fast and can carry riders.',
             2),
            
            ('oyo_h1_q3', 'oyo_history_001_lvl1',
             'The Oyo Mesi was:',
             'multiple_choice',
             '[{"id":"a","text":"A type of food"},{"id":"b","text":"A council of chiefs who advised the Alaafin"},{"id":"c","text":"A weapon"},{"id":"d","text":"A festival"}]',
             'b', 25,
             'The Oyo Mesi was a powerful council of seven chiefs who could check the Alaafin''s power and even remove him if necessary.',
             'Think of a group of advisors.',
             3),
            
            ('oyo_h1_q4', 'oyo_history_001_lvl1',
             'Ibadan, the capital of Oyo State, was once the largest city in Africa by:',
             'multiple_choice',
             '[{"id":"a","text":"Population"},{"id":"b","text":"Geographical area"},{"id":"c","text":"Number of buildings"},{"id":"d","text":"Wealth"}]',
             'b', 20,
             'Ibadan was once the largest city in Africa by geographical area, sprawling across seven hills.',
             'Think about space, not people.',
             4),
            
            ('oyo_h1_q5', 'oyo_history_001_lvl1',
             'True or False: The Oyo Empire traded slaves.',
             'true_false',
             '[{"id":"true","text":"True"},{"id":"false","text":"False"}]',
             'true', 20,
             'Unfortunately, the Oyo Empire was heavily involved in the Atlantic slave trade, which contributed to both its wealth and eventual decline.',
             NULL,
             5);
    "#).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    Ok(())
}

// =====================================================
// OSUN STATE MODULES
// =====================================================
fn seed_osun_modules(conn: &Connection) -> Result<(), DatabaseError> {
    conn.execute_batch(r#"
        INSERT OR REPLACE INTO modules (id, state_id, subject, title, description, required_level, total_xp, estimated_time, icon, education_level, interest_tags)
        VALUES ('osu_culture_001', 'OSU', 'Culture', 'The Sacred Grove', 
                'Explore the UNESCO World Heritage Site and learn about Yoruba spirituality!',
                4, 500, 20, 'trees', 'all', '["culture", "history"]');
        
        INSERT OR REPLACE INTO module_context (module_id, did_you_know, fun_fact, intro_text, historical_note)
        VALUES ('osu_culture_001',
                'The Osun-Osogbo Sacred Grove is one of the last remaining primary high forests in southern Nigeria. It became a UNESCO World Heritage Site in 2005!',
                'The annual Osun Festival attracts hundreds of thousands of visitors, including Yoruba descendants from Brazil, Cuba, and the Caribbean!',
                'Welcome to the sacred forest! The Osun-Osogbo Grove is where nature, art, and spirituality meet. Learn about Yoruba beliefs and the importance of protecting sacred spaces.',
                'The grove is dedicated to Osun, the Yoruba goddess of fertility, love, and the river. It contains sculptures by Austrian artist Susanne Wenger who devoted her life to preserving Yoruba culture.');
        
        INSERT OR REPLACE INTO levels (id, module_id, title, difficulty, order_index, xp_reward, unlock_item_id)
        VALUES ('osu_culture_001_lvl1', 'osu_culture_001', 'Secrets of the Grove', 'medium', 1, 110, NULL);
        
        INSERT OR REPLACE INTO questions (id, level_id, question_text, question_type, options_json, correct_answer, xp_reward, explanation, hint, order_index)
        VALUES 
            ('osu_c1_q1', 'osu_culture_001_lvl1',
             'Osun is the Yoruba goddess of:',
             'multiple_choice',
             '[{"id":"a","text":"War"},{"id":"b","text":"Fertility and rivers"},{"id":"c","text":"The sky"},{"id":"d","text":"Fire"}]',
             'b', 20,
             'Osun (also spelled Oshun) is the goddess of fertility, love, fresh water, and the Osun River. She is one of the most beloved Yoruba deities.',
             'Think about water and life.',
             1),
            
            ('osu_c1_q2', 'osu_culture_001_lvl1',
             'What international organization recognized the Osun-Osogbo Grove?',
             'multiple_choice',
             '[{"id":"a","text":"FIFA"},{"id":"b","text":"UNESCO"},{"id":"c","text":"WHO"},{"id":"d","text":"UNICEF"}]',
             'b', 15,
             'UNESCO (United Nations Educational, Scientific and Cultural Organization) designated it a World Heritage Site in 2005.',
             'This organization protects cultural sites.',
             2),
            
            ('osu_c1_q3', 'osu_culture_001_lvl1',
             'What does "World Heritage Site" mean?',
             'multiple_choice',
             '[{"id":"a","text":"A place owned by the United Nations"},{"id":"b","text":"A place of global cultural or natural importance"},{"id":"c","text":"A tourist attraction"},{"id":"d","text":"An ancient ruin"}]',
             'b', 20,
             'World Heritage Sites are places of outstanding cultural or natural importance that belong to all humanity and deserve protection.',
             'It means important to the whole world.',
             3),
            
            ('osu_c1_q4', 'osu_culture_001_lvl1',
             'When is the Osun Festival celebrated?',
             'multiple_choice',
             '[{"id":"a","text":"January"},{"id":"b","text":"August"},{"id":"c","text":"December"},{"id":"d","text":"March"}]',
             'b', 15,
             'The Osun Festival is held in August each year. It includes a grand procession to the river where offerings are made.',
             'It''s during the rainy season.',
             4),
            
            ('osu_c1_q5', 'osu_culture_001_lvl1',
             'True or False: The Osun Grove is an artificial (man-made) forest.',
             'true_false',
             '[{"id":"true","text":"True"},{"id":"false","text":"False"}]',
             'false', 15,
             'The Osun Grove is a natural primary forest - one of the last remaining ancient forests in southern Nigeria.',
             NULL,
             5);
    "#).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    Ok(())
}

// =====================================================
// RIVERS STATE MODULES
// =====================================================
fn seed_rivers_modules(conn: &Connection) -> Result<(), DatabaseError> {
    conn.execute_batch(r#"
        INSERT OR REPLACE INTO modules (id, state_id, subject, title, description, required_level, total_xp, estimated_time, icon, education_level, interest_tags)
        VALUES ('riv_science_001', 'RIV', 'Science', 'Oil and Energy', 
                'Discover how oil powers Nigeria and learn about energy resources!',
                5, 600, 25, 'fuel', 'all', '["geography"]');
        
        INSERT OR REPLACE INTO module_context (module_id, did_you_know, fun_fact, intro_text, historical_note)
        VALUES ('riv_science_001',
                'Nigeria is Africa''s largest oil producer and one of the top 10 producers in the world! Most of this oil comes from the Niger Delta region.',
                'Oil was first discovered in commercial quantities in Nigeria at Oloibiri, Bayelsa State in 1956. This changed Nigeria''s economy forever!',
                'Welcome to the Treasure Base of the Nation! Rivers State is at the heart of Nigeria''s oil industry. Learn about petroleum, energy, and environmental responsibility.',
                'Port Harcourt was founded in 1912 as a port for exporting coal. Today it''s the center of Nigeria''s oil industry.');
        
        INSERT OR REPLACE INTO levels (id, module_id, title, difficulty, order_index, xp_reward, unlock_item_id)
        VALUES ('riv_science_001_lvl1', 'riv_science_001', 'Black Gold', 'medium', 1, 130, NULL);
        
        INSERT OR REPLACE INTO questions (id, level_id, question_text, question_type, options_json, correct_answer, xp_reward, explanation, hint, order_index)
        VALUES 
            ('riv_s1_q1', 'riv_science_001_lvl1',
             'Oil is often called "black gold" because:',
             'multiple_choice',
             '[{"id":"a","text":"It is actually black gold metal"},{"id":"b","text":"It is very valuable like gold"},{"id":"c","text":"It was discovered in a gold mine"},{"id":"d","text":"It can be turned into gold"}]',
             'b', 15,
             'Oil is called "black gold" because it is black in color and extremely valuable - it has made countries rich!',
             'Think about value and color.',
             1),
            
            ('riv_s1_q2', 'riv_science_001_lvl1',
             'What products come from petroleum (crude oil)?',
             'multiple_choice',
             '[{"id":"a","text":"Only petrol"},{"id":"b","text":"Petrol, diesel, plastics, and more"},{"id":"c","text":"Only plastics"},{"id":"d","text":"Only kerosene"}]',
             'b', 20,
             'Crude oil is refined into many products: petrol, diesel, kerosene, plastics, chemicals, and even medicines!',
             'Oil is used to make many different things.',
             2),
            
            ('riv_s1_q3', 'riv_science_001_lvl1',
             'Oil spills can harm the environment by:',
             'multiple_choice',
             '[{"id":"a","text":"Making the land more fertile"},{"id":"b","text":"Killing fish and polluting water"},{"id":"c","text":"Creating new islands"},{"id":"d","text":"Cooling the climate"}]',
             'b', 20,
             'Oil spills are devastating to the environment. They kill fish, poison water, and destroy farmland for years.',
             'Oil is toxic to living things.',
             3),
            
            ('riv_s1_q4', 'riv_science_001_lvl1',
             'NNPC stands for:',
             'multiple_choice',
             '[{"id":"a","text":"Nigerian National Petroleum Corporation"},{"id":"b","text":"National Nigerian Petrol Company"},{"id":"c","text":"Niger National Power Corporation"},{"id":"d","text":"Nigerian Natural Products Company"}]',
             'a', 20,
             'NNPC (Nigerian National Petroleum Corporation) is the government company that manages Nigeria''s oil resources.',
             'The P stands for Petroleum.',
             4),
            
            ('riv_s1_q5', 'riv_science_001_lvl1',
             'True or False: Oil is a renewable resource.',
             'true_false',
             '[{"id":"true","text":"True"},{"id":"false","text":"False"}]',
             'false', 20,
             'Oil is NOT renewable - it takes millions of years to form. Once we use it all, it''s gone. That''s why we need to find alternative energy sources.',
             NULL,
             5);
    "#).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    Ok(())
}

// =====================================================
// CROSS RIVER STATE MODULES
// =====================================================
fn seed_crossriver_modules(conn: &Connection) -> Result<(), DatabaseError> {
    conn.execute_batch(r#"
        INSERT OR REPLACE INTO modules (id, state_id, subject, title, description, required_level, total_xp, estimated_time, icon, education_level, interest_tags)
        VALUES ('crs_culture_001', 'CRS', 'Culture', 'Carnival and Conservation', 
                'Experience Africa''s biggest street party and learn about rainforest conservation!',
                6, 650, 28, 'party-popper', 'all', '["culture", "music", "geography"]');
        
        INSERT OR REPLACE INTO module_context (module_id, did_you_know, fun_fact, intro_text, historical_note)
        VALUES ('crs_culture_001',
                'The Calabar Carnival attracts over 2 million visitors each December! It features elaborate costumes, dance competitions, and street parties.',
                'Cross River State contains the last remaining virgin tropical rainforest in Nigeria - home to rare gorillas, drills, and hundreds of bird species!',
                'Welcome to the People''s Paradise! From the colorful Calabar Carnival to the ancient rainforests, Cross River State is a land of celebration and nature.',
                'Calabar was one of the earliest points of contact between Nigeria and Europe. The old colonial buildings still stand in the city.');
        
        INSERT OR REPLACE INTO levels (id, module_id, title, difficulty, order_index, xp_reward, unlock_item_id)
        VALUES ('crs_culture_001_lvl1', 'crs_culture_001', 'Festival and Forest', 'hard', 1, 140, NULL);
        
        INSERT OR REPLACE INTO questions (id, level_id, question_text, question_type, options_json, correct_answer, xp_reward, explanation, hint, order_index)
        VALUES 
            ('crs_c1_q1', 'crs_culture_001_lvl1',
             'When is the Calabar Carnival held?',
             'multiple_choice',
             '[{"id":"a","text":"January"},{"id":"b","text":"August"},{"id":"c","text":"December"},{"id":"d","text":"March"}]',
             'c', 15,
             'The Calabar Carnival runs throughout December, culminating in a grand finale on December 28th.',
             'It''s during the Christmas season.',
             1),
            
            ('crs_c1_q2', 'crs_culture_001_lvl1',
             'What endangered animal is protected in Cross River forests?',
             'multiple_choice',
             '[{"id":"a","text":"Lions"},{"id":"b","text":"Cross River Gorilla"},{"id":"c","text":"Polar bears"},{"id":"d","text":"Tigers"}]',
             'b', 25,
             'The Cross River Gorilla is one of the world''s most endangered primates with only about 300 left in the wild.',
             'This great ape shares its name with the state.',
             2),
            
            ('crs_c1_q3', 'crs_culture_001_lvl1',
             '"Conservation" of rainforests means:',
             'multiple_choice',
             '[{"id":"a","text":"Cutting down all trees"},{"id":"b","text":"Protecting and preserving them"},{"id":"c","text":"Building cities in them"},{"id":"d","text":"Burning them"}]',
             'b', 15,
             'Conservation means protecting natural resources and environments so they survive for future generations.',
             'Think about keeping something safe.',
             3),
            
            ('crs_c1_q4', 'crs_culture_001_lvl1',
             'Why are rainforests important?',
             'multiple_choice',
             '[{"id":"a","text":"They produce oxygen and absorb carbon dioxide"},{"id":"b","text":"They are only for tourists"},{"id":"c","text":"They have no importance"},{"id":"d","text":"They block the sun"}]',
             'a', 20,
             'Rainforests are the "lungs of the Earth" - they produce oxygen, absorb CO2, and are home to millions of species.',
             'Think about what plants do with air.',
             4),
            
            ('crs_c1_q5', 'crs_culture_001_lvl1',
             'True or False: The Calabar Carnival is Africa''s biggest street party.',
             'true_false',
             '[{"id":"true","text":"True"},{"id":"false","text":"False"}]',
             'true', 15,
             'The Calabar Carnival is proudly called "Africa''s Biggest Street Party" - it rivals the carnivals of Rio and Trinidad!',
             NULL,
             5);
    "#).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    Ok(())
}

// =====================================================
// SOKOTO STATE MODULES
// =====================================================
fn seed_sokoto_modules(conn: &Connection) -> Result<(), DatabaseError> {
    conn.execute_batch(r#"
        INSERT OR REPLACE INTO modules (id, state_id, subject, title, description, required_level, total_xp, estimated_time, icon, education_level, interest_tags)
        VALUES ('sok_history_001', 'SOK', 'History', 'The Sokoto Caliphate', 
                'Learn about one of the largest empires in 19th century Africa!',
                6, 600, 25, 'building-2', 'all', '["history"]');
        
        INSERT OR REPLACE INTO module_context (module_id, did_you_know, fun_fact, intro_text, historical_note)
        VALUES ('sok_history_001',
                'The Sokoto Caliphate was the largest state in Africa in the 19th century! It included most of northern Nigeria and parts of neighboring countries.',
                'The Sultan of Sokoto is still regarded as the spiritual leader of Muslims in Nigeria today - a tradition that has continued for over 200 years!',
                'Welcome to the Seat of the Caliphate! Sokoto is the spiritual heart of Islam in Nigeria. Learn about Usman dan Fodio, the jihad, and the empire he built.',
                'Usman dan Fodio founded the Sokoto Caliphate after a religious jihad in 1804. His descendants still hold the title of Sultan.');
        
        INSERT OR REPLACE INTO levels (id, module_id, title, difficulty, order_index, xp_reward, unlock_item_id)
        VALUES ('sok_history_001_lvl1', 'sok_history_001', 'Rise of the Caliphate', 'hard', 1, 130, NULL);
        
        INSERT OR REPLACE INTO questions (id, level_id, question_text, question_type, options_json, correct_answer, xp_reward, explanation, hint, order_index)
        VALUES 
            ('sok_h1_q1', 'sok_history_001_lvl1',
             'Who founded the Sokoto Caliphate?',
             'multiple_choice',
             '[{"id":"a","text":"Ahmadu Bello"},{"id":"b","text":"Usman dan Fodio"},{"id":"c","text":"Muhammad Bello"},{"id":"d","text":"Sani Abacha"}]',
             'b', 20,
             'Usman dan Fodio (1754-1817) was a religious teacher who led a jihad and founded the Sokoto Caliphate in 1804.',
             'His name starts with "Usman".',
             1),
            
            ('sok_h1_q2', 'sok_history_001_lvl1',
             'A "caliphate" is:',
             'multiple_choice',
             '[{"id":"a","text":"A type of building"},{"id":"b","text":"An Islamic state led by a caliph"},{"id":"c","text":"A market"},{"id":"d","text":"A river"}]',
             'b', 20,
             'A caliphate is a form of Islamic government led by a caliph (successor to the Prophet Muhammad).',
             'It relates to Islamic leadership.',
             2),
            
            ('sok_h1_q3', 'sok_history_001_lvl1',
             'What year was the Sokoto Caliphate founded?',
             'multiple_choice',
             '[{"id":"a","text":"1804"},{"id":"b","text":"1960"},{"id":"c","text":"1500"},{"id":"d","text":"1900"}]',
             'a', 20,
             'The Sokoto Caliphate was established in 1804 after Usman dan Fodio''s successful jihad.',
             'It was early in the 19th century.',
             3),
            
            ('sok_h1_q4', 'sok_history_001_lvl1',
             'The Sultan of Sokoto today is the:',
             'multiple_choice',
             '[{"id":"a","text":"President of Nigeria"},{"id":"b","text":"Spiritual leader of Nigerian Muslims"},{"id":"c","text":"Governor of Sokoto"},{"id":"d","text":"Head of the Army"}]',
             'b', 20,
             'The Sultan of Sokoto is the highest traditional Islamic authority in Nigeria and leads prayers at major Muslim festivals.',
             'He leads in religious, not political matters.',
             4),
            
            ('sok_h1_q5', 'sok_history_001_lvl1',
             'True or False: The Sokoto Caliphate ended when the British conquered it in 1903.',
             'true_false',
             '[{"id":"true","text":"True"},{"id":"false","text":"False"}]',
             'true', 20,
             'The British defeated the Sokoto Caliphate in 1903, but allowed the Sultan to continue as a religious and traditional ruler.',
             NULL,
             5);
    "#).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    Ok(())
}

// =====================================================
// BORNO STATE MODULES
// =====================================================
fn seed_borno_modules(conn: &Connection) -> Result<(), DatabaseError> {
    conn.execute_batch(r#"
        INSERT OR REPLACE INTO modules (id, state_id, subject, title, description, required_level, total_xp, estimated_time, icon, education_level, interest_tags)
        VALUES ('bor_history_001', 'BOR', 'History', 'The Kanem-Bornu Empire', 
                'Discover Africa''s longest-lasting empire - over 1000 years of history!',
                7, 700, 30, 'landmark', 'all', '["history"]');
        
        INSERT OR REPLACE INTO module_context (module_id, did_you_know, fun_fact, intro_text, historical_note)
        VALUES ('bor_history_001',
                'The Kanem-Bornu Empire lasted for over 1000 years (from about 700 CE to 1900 CE) - one of the longest-lasting empires in world history!',
                'The mai (king) of Kanem-Bornu went on pilgrimage to Mecca with thousands of followers. They were so wealthy they gave away gold along the way!',
                'Welcome to Home of Peace! Borno State was the center of one of Africa''s greatest empires. Learn about the Kanem-Bornu legacy.',
                'The Kanuri people built this great empire around Lake Chad. At its height, it controlled trade routes across the Sahara.');
        
        INSERT OR REPLACE INTO levels (id, module_id, title, difficulty, order_index, xp_reward, unlock_item_id)
        VALUES ('bor_history_001_lvl1', 'bor_history_001', 'A Thousand Year Empire', 'hard', 1, 150, NULL);
        
        INSERT OR REPLACE INTO questions (id, level_id, question_text, question_type, options_json, correct_answer, xp_reward, explanation, hint, order_index)
        VALUES 
            ('bor_h1_q1', 'bor_history_001_lvl1',
             'How long did the Kanem-Bornu Empire last?',
             'multiple_choice',
             '[{"id":"a","text":"About 100 years"},{"id":"b","text":"About 500 years"},{"id":"c","text":"Over 1000 years"},{"id":"d","text":"About 50 years"}]',
             'c', 20,
             'The Kanem-Bornu Empire lasted over 1000 years - from about 700 CE to 1900 CE!',
             'It''s the longest option.',
             1),
            
            ('bor_h1_q2', 'bor_history_001_lvl1',
             'Lake Chad was important to the empire because:',
             'multiple_choice',
             '[{"id":"a","text":"It provided water for people and trade"},{"id":"b","text":"It was full of gold"},{"id":"c","text":"It was used for swimming only"},{"id":"d","text":"It had no importance"}]',
             'a', 20,
             'Lake Chad provided water, fish, and fertile land. It was also a crossroads for trade routes.',
             'Think about what people need to survive.',
             2),
            
            ('bor_h1_q3', 'bor_history_001_lvl1',
             'The king of Kanem-Bornu was called:',
             'multiple_choice',
             '[{"id":"a","text":"Oba"},{"id":"b","text":"Mai"},{"id":"c","text":"Sultan"},{"id":"d","text":"Alaafin"}]',
             'b', 20,
             'The king was called Mai (also spelled "Mai" or "May"). This title was used for centuries.',
             'It''s a short, three-letter title.',
             3),
            
            ('bor_h1_q4', 'bor_history_001_lvl1',
             'Which ethnic group founded the Kanem-Bornu Empire?',
             'multiple_choice',
             '[{"id":"a","text":"Yoruba"},{"id":"b","text":"Igbo"},{"id":"c","text":"Kanuri"},{"id":"d","text":"Hausa"}]',
             'c', 20,
             'The Kanuri people founded and ruled the Kanem-Bornu Empire. They are still the dominant group in Borno State today.',
             'The state is named after them (Bornu = Borno).',
             4),
            
            ('bor_h1_q5', 'bor_history_001_lvl1',
             'True or False: Kanem-Bornu was one of the first African kingdoms to adopt Islam.',
             'true_false',
             '[{"id":"true","text":"True"},{"id":"false","text":"False"}]',
             'true', 20,
             'Kanem adopted Islam in the 11th century, making it one of the earliest Islamic states in sub-Saharan Africa.',
             NULL,
             5);
    "#).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    Ok(())
}

// =====================================================
// TARABA STATE MODULES
// =====================================================
fn seed_taraba_modules(conn: &Connection) -> Result<(), DatabaseError> {
    conn.execute_batch(r#"
        INSERT OR REPLACE INTO modules (id, state_id, subject, title, description, required_level, total_xp, estimated_time, icon, education_level, interest_tags)
        VALUES ('tar_geography_001', 'TAR', 'Geography', 'The Mambilla Plateau', 
                'Explore Nigeria''s highest point and learn about highland geography!',
                6, 550, 22, 'mountain', 'all', '["geography"]');
        
        INSERT OR REPLACE INTO module_context (module_id, did_you_know, fun_fact, intro_text, historical_note)
        VALUES ('tar_geography_001',
                'The Mambilla Plateau is at 1,800 meters (about 6,000 feet) above sea level - so high that it gets cold and foggy, unlike the rest of Nigeria!',
                'Because of its cool climate, Mambilla grows tea - one of the few places in Nigeria where tea is cultivated!',
                'Welcome to Nature''s Gift to the Nation! Taraba State has Nigeria''s highest plateau. Learn about mountains, climate, and highland life.',
                'The Mambilla Plateau is home to the Mambilla people who have lived at these heights for centuries.');
        
        INSERT OR REPLACE INTO levels (id, module_id, title, difficulty, order_index, xp_reward, unlock_item_id)
        VALUES ('tar_geography_001_lvl1', 'tar_geography_001', 'Highland Adventures', 'hard', 1, 120, NULL);
        
        INSERT OR REPLACE INTO questions (id, level_id, question_text, question_type, options_json, correct_answer, xp_reward, explanation, hint, order_index)
        VALUES 
            ('tar_g1_q1', 'tar_geography_001_lvl1',
             'A plateau is:',
             'multiple_choice',
             '[{"id":"a","text":"A deep valley"},{"id":"b","text":"A flat area of high land"},{"id":"c","text":"A river"},{"id":"d","text":"A desert"}]',
             'b', 15,
             'A plateau is a flat, elevated area of land - like a table top raised high above the surrounding areas.',
             'Think of a flat-topped mountain.',
             1),
            
            ('tar_g1_q2', 'tar_geography_001_lvl1',
             'Why is the Mambilla Plateau cooler than most of Nigeria?',
             'multiple_choice',
             '[{"id":"a","text":"It has air conditioning"},{"id":"b","text":"It is at high altitude"},{"id":"c","text":"It is near the sea"},{"id":"d","text":"It has no sun"}]',
             'b', 20,
             'Temperature drops as altitude increases. At 1,800 meters high, Mambilla is much cooler than lowland Nigeria.',
             'Height affects temperature.',
             2),
            
            ('tar_g1_q3', 'tar_geography_001_lvl1',
             'What crop is grown on the Mambilla Plateau because of its cool climate?',
             'multiple_choice',
             '[{"id":"a","text":"Rice"},{"id":"b","text":"Tea"},{"id":"c","text":"Cassava"},{"id":"d","text":"Palm oil"}]',
             'b', 20,
             'Tea needs cool, misty conditions - perfect for the Mambilla Plateau! Nigeria produces tea there.',
             'This drink is usually served hot.',
             3),
            
            ('tar_g1_q4', 'tar_geography_001_lvl1',
             'Approximately how high is the Mambilla Plateau?',
             'multiple_choice',
             '[{"id":"a","text":"100 meters"},{"id":"b","text":"500 meters"},{"id":"c","text":"1,800 meters"},{"id":"d","text":"5,000 meters"}]',
             'c', 20,
             'The Mambilla Plateau reaches about 1,800 meters (6,000 feet) above sea level.',
             'It''s nearly 2 kilometers high.',
             4),
            
            ('tar_g1_q5', 'tar_geography_001_lvl1',
             'True or False: The Mambilla Plateau is Nigeria''s highest point.',
             'true_false',
             '[{"id":"true","text":"True"},{"id":"false","text":"False"}]',
             'true', 15,
             'The Mambilla Plateau contains Chappal Waddi, the highest peak in Nigeria at about 2,419 meters.',
             NULL,
             5);
    "#).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
    
    Ok(())
}

fn seed_default_user(conn: &Connection) -> Result<(), DatabaseError> {
    // First create the user (only if they don't exist - preserve existing user data)
    conn.execute(
        "INSERT OR IGNORE INTO users (id, display_name, avatar_json, total_xp, current_level, cowrie_shells, current_zone)
         VALUES (1, 'Student', '{\"skin\":\"tone_3\",\"head\":\"style_1\",\"top\":\"shirt_default\",\"accessory\":null}', 0, 1, 100, 'heritage')",
        []
    ).map_err(|e| DatabaseError::QueryError(format!("Failed to create user: {}", e)))?;
    
    // Then set initial progress (Abuja unlocked for the default user) - also ignore if exists
    conn.execute(
        "INSERT OR IGNORE INTO user_progress (user_id, state_id, stars, is_completed, best_score, attempts)
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
