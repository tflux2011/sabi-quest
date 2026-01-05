import React, { useEffect, useState, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { motion, AnimatePresence } from 'framer-motion';
import Confetti, { playCelebrationSound } from './components/Confetti';
import UpdateChecker from './components/UpdateChecker';
import { 
  AlertTriangle, 
  Lock, 
  Landmark, 
  BookOpen, 
  Star, 
  CheckCircle,
  Shell,
  MapPin,
  Flame,
  Backpack,
  X,
  ChevronRight,
  ChevronLeft,
  Sparkles,
  Trophy,
  Compass,
  Sword,
  Palette,
  BookMarked,
  Crown,
  GraduationCap,
  Theater,
  Globe,
  UtensilsCrossed,
  Music,
  MessageCircle,
  Clock,
  Zap,
  Volume2,
  VolumeX,
  Map,
  Book,
  Bookmark,
  ArrowLeft,
  ScrollText,
  Ghost,
  Award,
  Drum,
  LayoutGrid,
  // New icons for RPG features
  User,
  Shirt,
  Scissors,
  Circle,
  Image,
  Gem,
  Target,
  Gift,
  Scroll,
  Building2,
  Heart,
  Check,
  Pencil,
  Save
} from 'lucide-react';
import ChibiAvatar from './components/ChibiAvatar';
import JourneyMap from './components/JourneyMap';

// Sound effects utility
const createSound = (frequency: number, duration: number, type: OscillatorType = 'sine') => {
  return () => {
    try {
      const audioContext = new (window.AudioContext || (window as any).webkitAudioContext)();
      const oscillator = audioContext.createOscillator();
      const gainNode = audioContext.createGain();
      
      oscillator.connect(gainNode);
      gainNode.connect(audioContext.destination);
      
      oscillator.frequency.value = frequency;
      oscillator.type = type;
      
      gainNode.gain.setValueAtTime(0.1, audioContext.currentTime);
      gainNode.gain.exponentialRampToValueAtTime(0.01, audioContext.currentTime + duration);
      
      oscillator.start(audioContext.currentTime);
      oscillator.stop(audioContext.currentTime + duration);
    } catch (e) {
      // Audio not supported
    }
  };
};

const sounds = {
  click: createSound(800, 0.1, 'sine'),
  hover: createSound(600, 0.05, 'sine'),
  success: () => {
    createSound(523, 0.1, 'sine')();
    setTimeout(() => createSound(659, 0.1, 'sine')(), 100);
    setTimeout(() => createSound(784, 0.15, 'sine')(), 200);
  },
  unlock: () => {
    createSound(392, 0.1, 'triangle')();
    setTimeout(() => createSound(523, 0.1, 'triangle')(), 100);
    setTimeout(() => createSound(659, 0.15, 'triangle')(), 200);
  },
  pop: createSound(1000, 0.05, 'sine'),
  correct: () => {
    // Happy ascending melody
    createSound(523, 0.12, 'sine')(); // C5
    setTimeout(() => createSound(659, 0.12, 'sine')(), 80); // E5
    setTimeout(() => createSound(784, 0.12, 'sine')(), 160); // G5
    setTimeout(() => createSound(1047, 0.2, 'sine')(), 240); // C6
  },
  incorrect: () => {
    // Gentle descending tone
    createSound(400, 0.15, 'triangle')();
    setTimeout(() => createSound(300, 0.2, 'triangle')(), 150);
  },
  levelComplete: () => {
    // Victory fanfare
    createSound(523, 0.1, 'sine')();
    setTimeout(() => createSound(659, 0.1, 'sine')(), 100);
    setTimeout(() => createSound(784, 0.1, 'sine')(), 200);
    setTimeout(() => createSound(1047, 0.3, 'sine')(), 300);
    setTimeout(() => createSound(784, 0.1, 'sine')(), 500);
    setTimeout(() => createSound(1047, 0.4, 'sine')(), 600);
  },
  startup: () => {
    // Cheerful welcome melody - African-inspired pentatonic
    const playNote = (freq: number, delay: number, duration: number, volume: number = 0.12) => {
      setTimeout(() => {
        try {
          const audioContext = new (window.AudioContext || (window as any).webkitAudioContext)();
          const oscillator = audioContext.createOscillator();
          const gainNode = audioContext.createGain();
          
          oscillator.connect(gainNode);
          gainNode.connect(audioContext.destination);
          
          oscillator.frequency.value = freq;
          oscillator.type = 'sine';
          
          gainNode.gain.setValueAtTime(volume, audioContext.currentTime);
          gainNode.gain.exponentialRampToValueAtTime(0.01, audioContext.currentTime + duration);
          
          oscillator.start(audioContext.currentTime);
          oscillator.stop(audioContext.currentTime + duration);
        } catch (e) {}
      }, delay);
    };
    
    // Uplifting melody
    playNote(392, 0, 0.2);     // G4
    playNote(440, 150, 0.15);  // A4
    playNote(523, 300, 0.2);   // C5
    playNote(587, 450, 0.15);  // D5
    playNote(659, 600, 0.3);   // E5
    playNote(523, 850, 0.15);  // C5
    playNote(659, 1000, 0.4);  // E5 (hold)
    playNote(784, 1300, 0.5);  // G5 (final)
  },
};

// Avatar options for selection
const avatarOptions = [
  { id: 'explorer', icon: Compass, label: 'Explorer', color: '#00C896' },
  { id: 'scholar', icon: GraduationCap, label: 'Scholar', color: '#4D5382' },
  { id: 'warrior', icon: Sword, label: 'Warrior', color: '#EF476F' },
  { id: 'artist', icon: Palette, label: 'Artist', color: '#FFD166' },
  { id: 'storyteller', icon: BookMarked, label: 'Storyteller', color: '#06D6A0' },
  { id: 'chief', icon: Crown, label: 'Chief', color: '#FF9F1C' },
];

// Interest options
const interestOptions = [
  { id: 'history', label: 'History', icon: Landmark, color: '#4D5382' },
  { id: 'culture', label: 'Culture & Traditions', icon: Theater, color: '#EF476F' },
  { id: 'geography', label: 'Geography', icon: Globe, color: '#00C896' },
  { id: 'food', label: 'Food & Cuisine', icon: UtensilsCrossed, color: '#FF9F1C' },
  { id: 'music', label: 'Music & Arts', icon: Music, color: '#06D6A0' },
  { id: 'languages', label: 'Languages', icon: MessageCircle, color: '#FFD166' },
];

// Education level options (Nigerian school system)
const educationLevelOptions = [
  { id: 'primary_lower', label: 'Primary 1-3', ageRange: '6-8 years', icon: '🌱', color: '#06D6A0' },
  { id: 'primary_upper', label: 'Primary 4-6', ageRange: '9-11 years', icon: '🌿', color: '#00C896' },
  { id: 'jss', label: 'JSS 1-3', ageRange: '12-14 years', icon: '🌳', color: '#4D5382' },
  { id: 'sss', label: 'SS 1-3', ageRange: '15-17 years', icon: '🎓', color: '#EF476F' },
];

// Age options (6-18)
const ageOptions = Array.from({ length: 13 }, (_, i) => i + 6);

// Onboarding Form Steps
type OnboardingStep = 'name' | 'age' | 'education' | 'avatar' | 'interests' | 'complete';

interface UserInfo {
  displayName: string;
  age: number | null;
  educationLevel: string;
  avatar: string;
  interests: string[];
}

// Typeform-style Onboarding Screen Component
const OnboardingScreen = ({ onComplete }: { onComplete: (userInfo: UserInfo) => void }) => {
  const [step, setStep] = useState<OnboardingStep>('name');
  const [userInfo, setUserInfo] = useState<UserInfo>({
    displayName: '',
    age: null,
    educationLevel: '',
    avatar: '',
    interests: []
  });
  const [inputValue, setInputValue] = useState('');

  const handleNameSubmit = () => {
    if (inputValue.trim().length >= 2) {
      setUserInfo(prev => ({ ...prev, displayName: inputValue.trim() }));
      setStep('age');
    }
  };

  const handleAgeSelect = (age: number) => {
    setUserInfo(prev => ({ ...prev, age }));
    setStep('education');
  };

  const handleEducationSelect = (educationLevel: string) => {
    setUserInfo(prev => ({ ...prev, educationLevel }));
    setStep('avatar');
  };

  const handleAvatarSelect = (avatarId: string) => {
    setUserInfo(prev => ({ ...prev, avatar: avatarId }));
    setStep('interests');
  };

  const handleInterestToggle = (interestId: string) => {
    setUserInfo(prev => ({
      ...prev,
      interests: prev.interests.includes(interestId)
        ? prev.interests.filter(i => i !== interestId)
        : [...prev.interests, interestId]
    }));
  };

  const handleComplete = () => {
    if (userInfo.interests.length > 0) {
      onComplete(userInfo);
    }
  };

  const handleKeyPress = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && step === 'name') {
      handleNameSubmit();
    }
  };

  // Progress calculation: 5 steps now (name, age, education, avatar, interests)
  const progressMap: Record<OnboardingStep, number> = {
    'name': 20,
    'age': 40,
    'education': 60,
    'avatar': 80,
    'interests': 100,
    'complete': 100
  };
  const progress = progressMap[step];

  return (
    <motion.div 
      className="onboarding-screen typeform-style"
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      exit={{ opacity: 0 }}
    >
      {/* Background layers */}
      <div className="onboarding-bg" />
      
      {/* Form container with map background */}
      <motion.div 
        className="onboarding-container"
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.6 }}
      >
        {/* Map background inside form */}
        <div className="onboarding-map-bg">
          <motion.img 
            src="/assets/images/nigeria-map-isometric.png" 
            alt=""
            className="onboarding-map-image"
            animate={{ 
              y: [0, -10, 0],
            }}
            transition={{ 
              duration: 6, 
              repeat: Infinity, 
              ease: "easeInOut" 
            }}
          />
        </div>

        {/* Progress bar */}
        <div className="typeform-progress">
          <motion.div 
            className="typeform-progress-fill"
            initial={{ width: 0 }}
            animate={{ width: `${progress}%` }}
            transition={{ duration: 0.5 }}
          />
        </div>

        {/* Form content */}
        <div className="typeform-content">
          <AnimatePresence mode="wait">
            {/* Step 1: Name */}
            {step === 'name' && (
              <motion.div 
                key="name"
                className="typeform-step"
                initial={{ opacity: 0, y: 30 }}
                animate={{ opacity: 1, y: 0 }}
                exit={{ opacity: 0, y: -30 }}
                transition={{ duration: 0.4 }}
              >
                <span className="step-number">1 →</span>
                <h2 className="typeform-question">What should we call you?</h2>
                <p className="typeform-hint">This is how you'll appear in the app</p>
                
                <div className="typeform-input-wrapper">
                  <input
                    type="text"
                    className="typeform-input"
                    placeholder="Enter your name..."
                    value={inputValue}
                    onChange={(e) => setInputValue(e.target.value)}
                    onKeyPress={handleKeyPress}
                    autoFocus
                    maxLength={20}
                  />
                </div>
                
                {/* Only show submit when there's input */}
                <AnimatePresence>
                  {inputValue.trim().length >= 2 && (
                    <motion.div 
                      className="typeform-submit-row"
                      initial={{ opacity: 0, y: 10 }}
                      animate={{ opacity: 1, y: 0 }}
                      exit={{ opacity: 0, y: 10 }}
                      transition={{ duration: 0.3 }}
                    >
                      <span className="typeform-helper">Press Enter ↵</span>
                      <motion.button
                        className="typeform-submit"
                        onClick={handleNameSubmit}
                        whileHover={{ scale: 1.05 }}
                        whileTap={{ scale: 0.95 }}
                      >
                        <span>OK</span>
                        <CheckCircle size={18} />
                      </motion.button>
                    </motion.div>
                  )}
                </AnimatePresence>
              </motion.div>
            )}

            {/* Step 2: Age */}
            {step === 'age' && (
              <motion.div 
                key="age"
                className="typeform-step"
                initial={{ opacity: 0, y: 30 }}
                animate={{ opacity: 1, y: 0 }}
                exit={{ opacity: 0, y: -30 }}
                transition={{ duration: 0.4 }}
              >
                <span className="step-number">2 →</span>
                <h2 className="typeform-question">
                  Hey {userInfo.displayName}! How old are you? 🎂
                </h2>
                <p className="typeform-hint">This helps us show you the right content</p>
                
                <div className="age-selector-grid">
                  {ageOptions.map((age, i) => (
                    <motion.button
                      key={age}
                      className={`age-option ${userInfo.age === age ? 'selected' : ''}`}
                      onClick={() => handleAgeSelect(age)}
                      initial={{ opacity: 0, scale: 0.8 }}
                      animate={{ opacity: 1, scale: 1 }}
                      transition={{ delay: i * 0.03 }}
                      whileHover={{ scale: 1.1, y: -3 }}
                      whileTap={{ scale: 0.95 }}
                    >
                      <span className="age-number">{age}</span>
                    </motion.button>
                  ))}
                </div>
              </motion.div>
            )}

            {/* Step 3: Education Level */}
            {step === 'education' && (
              <motion.div 
                key="education"
                className="typeform-step"
                initial={{ opacity: 0, y: 30 }}
                animate={{ opacity: 1, y: 0 }}
                exit={{ opacity: 0, y: -30 }}
                transition={{ duration: 0.4 }}
              >
                <span className="step-number">3 →</span>
                <h2 className="typeform-question">
                  What class are you in? 📚
                </h2>
                <p className="typeform-hint">We'll customize lessons for your level</p>
                
                <div className="education-grid">
                  {educationLevelOptions.map((level, i) => (
                    <motion.button
                      key={level.id}
                      className={`education-option ${userInfo.educationLevel === level.id ? 'selected' : ''}`}
                      onClick={() => handleEducationSelect(level.id)}
                      initial={{ opacity: 0, x: -20 }}
                      animate={{ opacity: 1, x: 0 }}
                      transition={{ delay: i * 0.1 }}
                      whileHover={{ scale: 1.03, x: 5 }}
                      whileTap={{ scale: 0.98 }}
                      style={{ '--education-color': level.color } as React.CSSProperties}
                    >
                      <span className="education-icon">{level.icon}</span>
                      <div className="education-info">
                        <span className="education-label">{level.label}</span>
                        <span className="education-age">{level.ageRange}</span>
                      </div>
                      {userInfo.educationLevel === level.id && (
                        <motion.span 
                          className="education-check"
                          initial={{ scale: 0 }}
                          animate={{ scale: 1 }}
                        >
                          <CheckCircle size={24} />
                        </motion.span>
                      )}
                    </motion.button>
                  ))}
                </div>
              </motion.div>
            )}

            {/* Step 4: Avatar */}
            {step === 'avatar' && (
              <motion.div 
                key="avatar"
                className="typeform-step"
                initial={{ opacity: 0, y: 30 }}
                animate={{ opacity: 1, y: 0 }}
                exit={{ opacity: 0, y: -30 }}
                transition={{ duration: 0.4 }}
              >
                <span className="step-number">4 →</span>
                <h2 className="typeform-question">
                  Awesome! Choose your adventurer type ⚔️
                </h2>
                <p className="typeform-hint">This represents you on your journey</p>
                
                <div className="avatar-grid">
                  {avatarOptions.map((avatar, i) => (
                    <motion.button
                      key={avatar.id}
                      className={`avatar-option ${userInfo.avatar === avatar.id ? 'selected' : ''}`}
                      onClick={() => handleAvatarSelect(avatar.id)}
                      initial={{ opacity: 0, scale: 0.8 }}
                      animate={{ opacity: 1, scale: 1 }}
                      transition={{ delay: i * 0.1 }}
                      whileHover={{ scale: 1.05, y: -5 }}
                      whileTap={{ scale: 0.95 }}
                      style={{ '--avatar-color': avatar.color } as React.CSSProperties}
                    >
                      <span className="avatar-icon">
                        <avatar.icon size={32} />
                      </span>
                      <span className="avatar-label">{avatar.label}</span>
                    </motion.button>
                  ))}
                </div>
              </motion.div>
            )}

            {/* Step 5: Interests */}
            {step === 'interests' && (
              <motion.div 
                key="interests"
                className="typeform-step"
                initial={{ opacity: 0, y: 30 }}
                animate={{ opacity: 1, y: 0 }}
                exit={{ opacity: 0, y: -30 }}
                transition={{ duration: 0.4 }}
              >
                <span className="step-number">5 →</span>
                <h2 className="typeform-question">What interests you most?</h2>
                <p className="typeform-hint">Select all that apply (at least one)</p>
                
                <div className="interests-grid">
                  {interestOptions.map((interest, i) => (
                    <motion.button
                      key={interest.id}
                      className={`interest-option ${userInfo.interests.includes(interest.id) ? 'selected' : ''}`}
                      onClick={() => handleInterestToggle(interest.id)}
                      initial={{ opacity: 0, x: -20 }}
                      animate={{ opacity: 1, x: 0 }}
                      transition={{ delay: i * 0.08 }}
                      whileHover={{ scale: 1.02 }}
                      whileTap={{ scale: 0.98 }}
                      style={{ '--interest-color': interest.color } as React.CSSProperties}
                    >
                      <span className="interest-icon-wrapper">
                        <interest.icon size={20} />
                      </span>
                      <span className="interest-label">{interest.label}</span>
                      {userInfo.interests.includes(interest.id) && (
                        <motion.span 
                          className="interest-check"
                          initial={{ scale: 0 }}
                          animate={{ scale: 1 }}
                        >
                          <CheckCircle size={20} />
                        </motion.span>
                      )}
                    </motion.button>
                  ))}
                </div>

                <motion.button
                  className="typeform-complete-btn"
                  onClick={handleComplete}
                  disabled={userInfo.interests.length === 0}
                  initial={{ opacity: 0 }}
                  animate={{ opacity: 1 }}
                  transition={{ delay: 0.5 }}
                  whileHover={{ scale: 1.05 }}
                  whileTap={{ scale: 0.95 }}
                >
                  <span>Start My Adventure</span>
                <ChevronRight size={24} />
              </motion.button>
            </motion.div>
          )}
        </AnimatePresence>
      </div>
    </motion.div>
  </motion.div>
  );
};

const FloatingElements = () => (
  <div className="floating-elements">
    {[...Array(12)].map((_, i) => (
      <motion.div
        key={i}
        className={`floating-shape shape-${i % 4}`}
        initial={{ 
          x: Math.random() * 100 - 50,
          y: Math.random() * 100 - 50,
          opacity: 0,
          scale: 0
        }}
        animate={{ 
          x: [Math.random() * 40 - 20, Math.random() * 40 - 20],
          y: [Math.random() * 40 - 20, Math.random() * 40 - 20],
          opacity: [0.3, 0.6, 0.3],
          scale: 1,
          rotate: [0, 360]
        }}
        transition={{
          duration: 8 + Math.random() * 4,
          repeat: Infinity,
          repeatType: "reverse",
          delay: i * 0.2
        }}
        style={{
          left: `${10 + (i % 4) * 25}%`,
          top: `${15 + Math.floor(i / 4) * 30}%`,
        }}
      />
    ))}
  </div>
);

// Startup/Splash Screen Component
const StartupScreen = ({ onStart }: { onStart: (forceOnboarding?: boolean) => void }) => {
  const [loadingProgress, setLoadingProgress] = useState(0);
  const [isReady, setIsReady] = useState(false);

  useEffect(() => {
    // Play startup melody
    const timer = setTimeout(() => {
      sounds.startup();
    }, 300);

    const interval = setInterval(() => {
      setLoadingProgress(prev => {
        if (prev >= 100) {
          clearInterval(interval);
          setTimeout(() => setIsReady(true), 500);
          return 100;
        }
        return prev + 2;
      });
    }, 50);

    return () => {
      clearTimeout(timer);
      clearInterval(interval);
    };
  }, []);

  const handleClick = (e: React.MouseEvent) => {
    // Hold Shift to force onboarding (for development/testing)
    onStart(e.shiftKey);
  };

  return (
    <motion.div 
      className="startup-screen"
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      exit={{ opacity: 0, scale: 1.1 }}
      transition={{ duration: 0.5 }}
    >
      {/* Background gradient layers */}
      <div className="startup-bg-layer layer-1" />
      <div className="startup-bg-layer layer-2" />
      <div className="startup-bg-layer layer-3" />
      
      {/* Nigeria Map Background */}
      <motion.div 
        className="map-background"
        initial={{ opacity: 0, scale: 1.1 }}
        animate={{ opacity: 1, scale: 1 }}
        transition={{ duration: 1.5, ease: "easeOut" }}
      >
        <motion.img 
          src="/assets/images/nigeria-map-isometric.png" 
          alt=""
          className="map-bg-image"
          animate={{ 
            y: [0, -10, 0],
          }}
          transition={{ 
            duration: 6, 
            repeat: Infinity, 
            ease: "easeInOut" 
          }}
        />
      </motion.div>
      
      {/* Floating decorative elements */}
      <FloatingElements />
      
      {/* Main content */}
      <div className="startup-content">
        {/* Logo/Title area */}
        <motion.div 
          className="startup-header"
          initial={{ y: -50, opacity: 0 }}
          animate={{ y: 0, opacity: 1 }}
          transition={{ delay: 0.3, duration: 0.8, type: "spring" }}
        >
          <h1 className="startup-title">
            <span className="title-text">Sabi Quest</span>
          </h1>
          <p className="startup-tagline">Learn Nigeria, One Adventure at a Time!</p>
        </motion.div>

        {/* Loading / Start section */}
        <motion.div 
          className="startup-footer"
          initial={{ y: 30, opacity: 0 }}
          animate={{ y: 0, opacity: 1 }}
          transition={{ delay: 2, duration: 0.6 }}
        >
          {!isReady ? (
            <div className="loading-section">
              <div className="progress-bar-container">
                <motion.div 
                  className="progress-bar-fill"
                  initial={{ width: 0 }}
                  animate={{ width: `${loadingProgress}%` }}
                  transition={{ duration: 0.1 }}
                />
              </div>
              <span className="loading-text">
                {loadingProgress < 100 ? `Loading adventure... ${loadingProgress}%` : 'Ready!'}
              </span>
            </div>
          ) : (
            <motion.button
              className="start-adventure-btn"
              onClick={handleClick}
              initial={{ scale: 0, opacity: 0 }}
              animate={{ scale: 1, opacity: 1 }}
              transition={{ type: "spring", stiffness: 300, damping: 20 }}
              whileHover={{ scale: 1.05, boxShadow: "0 12px 40px rgba(0, 200, 150, 0.4)" }}
              whileTap={{ scale: 0.95 }}
            >
              <Compass size={24} />
              <span>Start Adventure</span>
              <ChevronRight size={20} />
            </motion.button>
          )}
        </motion.div>

        {/* Bottom decorative text */}
        <motion.p 
          className="startup-credit"
          initial={{ opacity: 0 }}
          animate={{ opacity: 0.6 }}
          transition={{ delay: 2.5 }}
        >
          An Educational Journey Through Nigeria
        </motion.p>
      </div>
    </motion.div>
  );
};

// TypeScript interfaces matching Rust models
interface User {
  id: number;
  display_name: string;
  avatar: AvatarConfig;
  birth_year: number | null;
  education_level: string | null; // "primary_lower", "primary_upper", "jss", "sss"
  interests: string[]; // ["history", "culture", "geography", "food", "music", "languages"]
  total_xp: number;
  current_level: number;
  cowrie_shells: number;
  streak_days: number;
  last_login_at: string | null;
  created_at: string;
}

interface AvatarConfig {
  skin: string;
  head: string;
  top: string;
  accessory: string | null;
}

interface GameState {
  id: string;
  name: string;
  region: string | null;
  zone: string | null;
  unlock_level: number;
  landmark_name: string | null;
  landmark_image: string | null;
  description: string | null;
  fun_fact: string | null;
}

interface UserProgress {
  user_id: number;
  state_id: string;
  stars: number;
  is_completed: boolean;
  best_score: number;
  attempts: number;
  last_played_at: string | null;
}

interface StateWithProgress {
  state: GameState;
  progress: UserProgress | null;
  is_unlocked: boolean;
  lessons_count: number;
  modules_count: number;
}

interface Lesson {
  id: string;
  state_id: string;
  subject: string;
  title: string;
  content_md: string | null;
}

interface LessonContent {
  lesson: Lesson;
  questions_count: number;
  total_xp: number;
}

// New Module types
interface Module {
  id: string;
  state_id: string;
  subject: string;
  title: string;
  description: string | null;
  required_level: number;
  total_xp: number;
  estimated_time: number | null;
  icon: string | null;
  education_level: string; // "primary_lower", "primary_upper", "jss", "sss", "all"
  interest_tags: string[]; // ["history", "culture", "geography", "food", "music", "languages"]
}

interface ModuleContext {
  module_id: string;
  did_you_know: string | null;
  fun_fact: string | null;
  intro_text: string | null;
  historical_note: string | null;
  intro_image_url: string | null;
  intro_video_url: string | null;
}

interface UserModuleProgress {
  user_id: number;
  module_id: string;
  current_level_id: string | null;
  is_completed: boolean;
  stars: number;
  total_xp_earned: number;
  best_score: number;
  attempts: number;
  last_played_at: string | null;
}

interface ModuleWithProgress {
  module: Module;
  context: ModuleContext | null;
  progress: UserModuleProgress | null;
  is_unlocked: boolean;
}

// Quiz/Level related interfaces
interface QuestionOption {
  id: string;
  text: string;
}

interface Question {
  id: string;
  level_id: string;
  question_text: string;
  question_type: string;
  options: QuestionOption[] | null;
  correct_answer: string;
  xp_reward: number;
  explanation: string | null;
  hint: string | null;
  image_url: string | null;
  order_index: number;
}

interface Level {
  id: string;
  module_id: string;
  title: string;
  difficulty: number;
  order_index: number;
  xp_reward: number;
  unlock_item_id: string | null;
}

interface LevelWithQuestions {
  level: Level;
  questions: Question[];
}

interface ModuleWithContent {
  module: Module;
  context: ModuleContext | null;
  levels: LevelWithQuestions[];
}

interface LevelResult {
  passed: boolean;
  stars_earned: number;
  xp_earned: number;
  correct_answers: number;
  total_questions: number;
  is_new_best: boolean;
  unlocked_item: string | null;
}

// Encyclopedia/Codex types
interface EncyclopediaEntry {
  id: string;
  category: string;
  title: string;
  subtitle: string | null;
  content_md: string;
  summary: string | null;
  image_url: string | null;
  audio_url: string | null;
  associated_state: string | null;
  tier: number;
  unlock_condition: string | null;
  xp_reward: number;
  reading_time: number | null;
  tags: string | null;
}

interface UserEncyclopediaProgress {
  entry_id: string;
  is_unlocked: boolean;
  is_read: boolean;
  is_bookmarked: boolean;
  unlocked_at: string | null;
  first_read_at: string | null;
  read_count: number;
}

interface EncyclopediaEntryWithProgress {
  entry: EncyclopediaEntry;
  progress: UserEncyclopediaProgress | null;
  is_accessible: boolean;
}

interface CodexStats {
  total_entries: number;
  unlocked_entries: number;
  read_entries: number;
  bookmarked_entries: number;
  entries_by_category: CategoryCount[];
}

interface CategoryCount {
  category: string;
  total: number;
  unlocked: number;
  read: number;
}

interface MarkReadResult {
  success: boolean;
  xp_awarded: number;
  was_first_read: boolean;
}

const modalVariants = {
  hidden: { opacity: 0, scale: 0.9, y: 50 },
  visible: { 
    opacity: 1, 
    scale: 1, 
    y: 0,
    transition: { 
      type: 'spring' as const, 
      damping: 25, 
      stiffness: 300 
    }
  },
  exit: { 
    opacity: 0, 
    scale: 0.9, 
    y: 50,
    transition: { duration: 0.2 }
  }
};

const overlayVariants = {
  hidden: { opacity: 0 },
  visible: { opacity: 1 },
  exit: { opacity: 0 }
};

const buttonVariants = {
  hover: { scale: 1.05 },
  tap: { scale: 0.95 }
};

// App screens enum
type AppScreen = 'startup' | 'onboarding' | 'main';

// Main view within the main screen (bottom navigation)
type MainView = 'map' | 'quests' | 'character' | 'museum';

// =====================================================
// CHARACTER / AVATAR SYSTEM TYPES
// =====================================================

interface AvatarItem {
  id: string;
  category: string;
  name: string;
  description: string;
  image_key: string;
  rarity: string;
  unlock_cost: number;
  unlock_condition: string | null;
  is_premium: boolean;
  sort_order: number;
}

interface AvatarItemWithStatus {
  item: AvatarItem;
  is_unlocked: boolean;
  is_equipped: boolean;
  can_afford: boolean;
}

interface UserAvatar {
  user_id: number;
  character_name: string | null;
  skin_tone: string;
  hairstyle: string;
  outfit: string;
  accessory: string | null;
  background: string;
}

// =====================================================
// QUEST SYSTEM TYPES
// =====================================================

interface Quest {
  id: string;
  title: string;
  description: string;
  quest_type: string;
  category: string;
  state_id: string | null;
  guide_id: string | null;
  required_level: number;
  requirements_json: string;
  xp_reward: number;
  cowrie_reward: number;
  artifact_reward_id: string | null;
  intro_dialogue: string | null;
  completion_dialogue: string | null;
  icon: string;
  is_repeatable: boolean;
  cooldown_hours: number | null;
  sort_order: number;
}

interface QuestWithProgress {
  quest: Quest;
  status: string;
  current_progress: number;
  is_available: boolean;
}

interface CulturalGuide {
  id: string;
  name: string;
  title: string;
  description: string;
  personality: string;
  avatar_image: string;
  state_id: string;
  region: string;
  greeting: string;
  catchphrase: string;
  voice_style: string;
}

// =====================================================
// ARTIFACT / MUSEUM SYSTEM TYPES
// =====================================================

interface Artifact {
  id: string;
  name: string;
  description: string;
  long_description: string;
  category: string;
  state_id: string | null;
  region: string | null;
  color_primary: string;
  color_secondary: string;
  rarity: string;
  historical_period: string | null;
  cultural_significance: string;
  unlock_type: string;
  unlock_source_id: string | null;
  cowrie_cost: number;
  sort_order: number;
}

interface ArtifactWithStatus {
  artifact: Artifact;
  is_unlocked: boolean;
  is_new: boolean;
  is_favorite: boolean;
  unlocked_at: string | null;
}

interface CollectionStats {
  total_artifacts: number;
  unlocked_artifacts: number;
  completion_percentage: number;
  by_rarity: { rarity: string; total: number; unlocked: number }[];
  by_category: { category: string; total: number; unlocked: number }[];
  by_state: { state_id: string; state_name: string; total: number; unlocked: number }[];
}

function App() {
  const [currentScreen, setCurrentScreen] = useState<AppScreen>('startup');
  const [isLoading, setIsLoading] = useState(true);
  const [user, setUser] = useState<User | null>(null);
  const [states, setStates] = useState<StateWithProgress[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [selectedState, setSelectedState] = useState<StateWithProgress | null>(null);
  const [_lessonContent, setLessonContent] = useState<LessonContent[] | null>(null);
  const [moduleContent, setModuleContent] = useState<ModuleWithProgress[] | null>(null);
  const [isModalLoading, setIsModalLoading] = useState(false);
  const [showInventory, setShowInventory] = useState(false);
  
  // Sabi Codex (Encyclopedia) state
  const [showCodex, setShowCodex] = useState(false);
  const [codexEntries, setCodexEntries] = useState<EncyclopediaEntryWithProgress[]>([]);
  const [codexStats, setCodexStats] = useState<CodexStats | null>(null);
  const [selectedCodexCategory, setSelectedCodexCategory] = useState<string>('all');
  const [selectedCodexEntry, setSelectedCodexEntry] = useState<EncyclopediaEntryWithProgress | null>(null);
  const [isCodexLoading, setIsCodexLoading] = useState(false);
  
  // Quiz state
  const [activeQuiz, setActiveQuiz] = useState<ModuleWithContent | null>(null);
  const [currentLevelIndex, setCurrentLevelIndex] = useState(0);
  const [currentQuestionIndex, setCurrentQuestionIndex] = useState(0);
  const [selectedAnswer, setSelectedAnswer] = useState<string | null>(null);
  const [showExplanation, setShowExplanation] = useState(false);
  const [correctAnswers, setCorrectAnswers] = useState(0);
  const [quizCompleted, setQuizCompleted] = useState(false);
  const [levelResult, setLevelResult] = useState<LevelResult | null>(null);
  const [_isQuizLoading, setIsQuizLoading] = useState(false);
  const [soundEnabled, setSoundEnabled] = useState(true);
  const [answerFeedback, setAnswerFeedback] = useState<'correct' | 'incorrect' | null>(null);
  const [robotMessage, setRobotMessage] = useState<string | null>(null);
  const [showCelebration, setShowCelebration] = useState(false);

  // Main view navigation (bottom nav)
  const [mainView, setMainView] = useState<MainView>('map');
  
  // Character/Avatar state
  const [userAvatar, setUserAvatar] = useState<UserAvatar | null>(null);
  const [avatarItems, setAvatarItems] = useState<AvatarItemWithStatus[]>([]);
  const [hasCreatedCharacter, setHasCreatedCharacter] = useState(false);
  const [isCharacterLoading, setIsCharacterLoading] = useState(false);
  const [selectedAvatarCategory, setSelectedAvatarCategory] = useState<string>('skin_tone');
  const [isEditingName, setIsEditingName] = useState(false);
  const [tempAvatarSelections, setTempAvatarSelections] = useState<{
    skin_tone_id: string;
    hairstyle_id: string;
    outfit_id: string;
    accessory_id: string | null;
    background_id: string;
    character_name: string;
  } | null>(null);
  
  // Quest state
  const [quests, setQuests] = useState<QuestWithProgress[]>([]);
  const [selectedQuestType, setSelectedQuestType] = useState<string>('main');
  const [_activeGuide, _setActiveGuide] = useState<CulturalGuide | null>(null);
  const [isQuestsLoading, setIsQuestsLoading] = useState(false);
  
  // Artifact/Museum state
  const [artifacts, setArtifacts] = useState<ArtifactWithStatus[]>([]);
  const [collectionStats, setCollectionStats] = useState<CollectionStats | null>(null);
  const [selectedArtifactCategory, setSelectedArtifactCategory] = useState<string>('all');
  const [_selectedArtifact, setSelectedArtifact] = useState<ArtifactWithStatus | null>(null);
  const [isMuseumLoading, setIsMuseumLoading] = useState(false);

  // Journey Map state (shown when completing a state)
  const [showJourneyMap, setShowJourneyMap] = useState(false);
  const [journeyData, setJourneyData] = useState<{
    fromState: { id: string; name: string; region: string };
    toState: { id: string; name: string; region: string; landmark_name?: string } | null;
    rewards: { stars: number; xp: number; items: string[]; badges: string[] };
  } | null>(null);

  // Recommended modules based on user interests
  const [recommendedModules, setRecommendedModules] = useState<ModuleWithProgress[]>([]);
  const [isLoadingRecommended, setIsLoadingRecommended] = useState(false);

  // Play sound helper
  const playSound = (soundName: keyof typeof sounds) => {
    if (soundEnabled) {
      sounds[soundName]();
    }
  };

  useEffect(() => {
    // Pre-initialize app data while startup screen is showing
    initializeApp();
  }, []);

  const handleStartAdventure = useCallback((forceOnboarding = false) => {
    // For development: allow forcing onboarding by passing true
    if (forceOnboarding) {
      localStorage.removeItem('hasSeenOnboarding');
      localStorage.removeItem('userPreferences');
    }
    
    // Check if user has seen onboarding before (could store in localStorage or DB)
    const hasSeenOnboarding = localStorage.getItem('hasSeenOnboarding');
    if (hasSeenOnboarding && !forceOnboarding) {
      setCurrentScreen('main');
    } else {
      setCurrentScreen('onboarding');
    }
  }, []);

  const handleOnboardingComplete = useCallback(async (userInfo: UserInfo) => {
    // Store user preferences
    localStorage.setItem('hasSeenOnboarding', 'true');
    localStorage.setItem('userPreferences', JSON.stringify(userInfo));
    
    // Update user profile in database with age, education level, and interests
    if (user) {
      try {
        // Calculate birth year from age
        const currentYear = new Date().getFullYear();
        const birthYear = userInfo.age ? currentYear - userInfo.age : null;
        
        // Call the update_user_profile command with interests
        const updatedUser = await invoke<User>('update_user_profile', {
          userId: user.id,
          displayName: userInfo.displayName,
          birthYear: birthYear,
          educationLevel: userInfo.educationLevel || null,
          interests: userInfo.interests.length > 0 ? userInfo.interests : null,
        });
        
        setUser(updatedUser);
        console.log('User profile updated:', updatedUser);
      } catch (err) {
        console.error('Failed to update user profile:', err);
        // Still update locally even if database update fails
        setUser(prev => prev ? { ...prev, display_name: userInfo.displayName } : prev);
      }
    }
    
    console.log('User onboarding complete:', userInfo);
    setCurrentScreen('main');
  }, [user]);

  const initializeApp = async () => {
    try {
      setIsLoading(true);
      
      // Initialize database
      const initResult = await invoke<string>('init_database');
      console.log('DB Init:', initResult);
      
      // Seed if needed
      const seedResult = await invoke<string>('seed_database');
      console.log('DB Seed:', seedResult);
      
      // Load user data (default user ID is 1)
      const userData = await invoke<User>('get_user', { userId: 1 });
      setUser(userData);
      
      // Load states with progress
      const statesData = await invoke<StateWithProgress[]>('get_all_states', { userId: 1 });
      setStates(statesData);
      
      // Check if character has been created
      try {
        const hasChar = await invoke<boolean>('has_created_character', { userId: 1 });
        setHasCreatedCharacter(hasChar);
        void hasCreatedCharacter; // Acknowledge the variable
        if (hasChar) {
          const avatar = await invoke<UserAvatar>('get_user_avatar', { userId: 1 });
          setUserAvatar(avatar);
        }
      } catch (err) {
        console.log('Character check error (may be new user):', err);
      }
      
    } catch (err) {
      console.error('Initialization error:', err);
      setError(String(err));
    } finally {
      setIsLoading(false);
    }
  };

  // Load avatar items for character customization
  // Always load ALL items so avatar preview can access any category
  const loadAvatarItems = useCallback(async () => {
    setIsCharacterLoading(true);
    try {
      const items = await invoke<AvatarItemWithStatus[]>('get_avatar_items', { 
        userId: 1, 
        category: null  // Load all categories
      });
      setAvatarItems(items);
    } catch (err) {
      console.error('Error loading avatar items:', err);
    } finally {
      setIsCharacterLoading(false);
    }
  }, []);

  // Load quests
  const loadQuests = useCallback(async (questType?: string) => {
    setIsQuestsLoading(true);
    try {
      const questsData = await invoke<QuestWithProgress[]>('get_quests', { 
        userId: 1, 
        questType: questType || null 
      });
      setQuests(questsData);
    } catch (err) {
      console.error('Error loading quests:', err);
    } finally {
      setIsQuestsLoading(false);
    }
  }, []);

  // Load recommended modules based on user interests
  const loadRecommendedModules = useCallback(async () => {
    setIsLoadingRecommended(true);
    try {
      const modules = await invoke<ModuleWithProgress[]>('get_recommended_modules', { 
        userId: 1, 
        limit: 6 
      });
      setRecommendedModules(modules);
    } catch (err) {
      console.error('Error loading recommended modules:', err);
    } finally {
      setIsLoadingRecommended(false);
    }
  }, []);

  // Load recommended modules when user data is available
  useEffect(() => {
    if (user && currentScreen === 'main') {
      loadRecommendedModules();
    }
  }, [user, currentScreen, loadRecommendedModules]);

  // Load artifacts/museum
  const loadArtifacts = useCallback(async (category?: string) => {
    setIsMuseumLoading(true);
    try {
      const [artifactsData, statsData] = await Promise.all([
        invoke<ArtifactWithStatus[]>('get_artifacts', { 
          userId: 1, 
          category: category !== 'all' ? category : null,
          stateId: null,
          unlockedOnly: false
        }),
        invoke<CollectionStats>('get_collection_stats', { userId: 1 })
      ]);
      setArtifacts(artifactsData);
      setCollectionStats(statsData);
    } catch (err) {
      console.error('Error loading artifacts:', err);
    } finally {
      setIsMuseumLoading(false);
    }
  }, []);

  // Save character avatar
  const handleSaveAvatar = useCallback(async () => {
    if (!tempAvatarSelections) return;
    
    try {
      console.log('Saving avatar with:', {
        characterName: tempAvatarSelections.character_name,
        skinTone: tempAvatarSelections.skin_tone_id,
        hairstyle: tempAvatarSelections.hairstyle_id,
      });
      
      const avatar = await invoke<UserAvatar>('update_user_avatar', {
        userId: 1,
        characterName: tempAvatarSelections.character_name,
        skinTone: tempAvatarSelections.skin_tone_id,
        hairstyle: tempAvatarSelections.hairstyle_id,
        outfit: tempAvatarSelections.outfit_id,
        accessory: tempAvatarSelections.accessory_id,
        background: tempAvatarSelections.background_id
      });
      
      console.log('Avatar saved, response:', avatar);
      
      setUserAvatar(avatar);
      setHasCreatedCharacter(true);
      setTempAvatarSelections(null);
      
      // Also update the user's display name to match the character name
      if (tempAvatarSelections.character_name) {
        setUser(prev => prev ? { ...prev, display_name: tempAvatarSelections.character_name } : prev);
      }
      
      playSound('success');
    } catch (err) {
      console.error('Error saving avatar:', err);
    }
  }, [tempAvatarSelections, playSound]);

  // Handle main view change
  const handleViewChange = useCallback(async (view: MainView) => {
    setMainView(view);
    
    // Load data for the view
    if (view === 'character' && avatarItems.length === 0) {
      // Load all avatar items when entering character view (only if not already loaded)
      loadAvatarItems();
    } else if (view === 'quests' && quests.length === 0) {
      loadQuests();
    } else if (view === 'museum' && artifacts.length === 0) {
      loadArtifacts();
    }
  }, [avatarItems.length, quests.length, artifacts.length, loadAvatarItems, loadQuests, loadArtifacts]);

  const handleExploreState = useCallback(async (stateItem: StateWithProgress) => {
    if (!stateItem.is_unlocked) return;
    
    setSelectedState(stateItem);
    document.body.classList.add('modal-open');
    setIsModalLoading(true);
    
    try {
      console.log('Fetching modules for state:', stateItem.state.id);
      // Fetch modules for this state using the new curriculum system
      const modules = await invoke<ModuleWithProgress[]>('get_modules_for_state', { 
        stateId: stateItem.state.id,
        userId: 1
      });
      console.log('Modules received:', modules);
      setModuleContent(modules);
      setLessonContent(null); // Clear old lesson content
    } catch (err) {
      console.error('Error loading module content:', err);
      setModuleContent([]);
    } finally {
      setIsModalLoading(false);
    }
  }, []);

  const handleCloseModal = useCallback(() => {
    setSelectedState(null);
    setLessonContent(null);
    setModuleContent(null);
    document.body.classList.remove('modal-open');
  }, []);

  // Journey Map handlers
  const handleJourneyContinue = useCallback(async () => {
    if (!journeyData?.toState) return;
    
    // Find the next state in our states array and open it
    const nextStateData = states.find(s => s.state.id === journeyData.toState?.id);
    if (nextStateData) {
      setShowJourneyMap(false);
      setJourneyData(null);
      // Small delay before opening next state
      setTimeout(() => {
        handleExploreState(nextStateData);
      }, 300);
    }
  }, [journeyData, states, handleExploreState]);

  const handleJourneyReturnHome = useCallback(() => {
    setShowJourneyMap(false);
    setJourneyData(null);
  }, []);

  const handleStartModule = useCallback(async (module: Module) => {
    console.log('Starting module:', module.id);
    setIsQuizLoading(true);
    
    try {
      // Fetch full module content with levels and questions
      const content = await invoke<ModuleWithContent>('get_module_content', { 
        moduleId: module.id 
      });
      console.log('Module content loaded:', content);
      
      if (content.levels.length === 0) {
        alert('No levels available for this module yet!');
        return;
      }
      
      // Reset quiz state
      setActiveQuiz(content);
      document.body.classList.add('modal-open');
      setCurrentLevelIndex(0);
      setCurrentQuestionIndex(0);
      setSelectedAnswer(null);
      setShowExplanation(false);
      setCorrectAnswers(0);
      setQuizCompleted(false);
      setLevelResult(null);
      
    } catch (err) {
      console.error('Error loading module content:', err);
      alert('Failed to load module. Please try again.');
    } finally {
      setIsQuizLoading(false);
    }
  }, []);

  const handleAnswerSelect = useCallback((answerId: string) => {
    if (showExplanation) return; // Can't change answer after submitting
    setSelectedAnswer(answerId);
  }, [showExplanation]);

  const handleSubmitAnswer = useCallback(() => {
    if (!activeQuiz || !selectedAnswer) return;
    
    const currentLevel = activeQuiz.levels[currentLevelIndex];
    const currentQuestion = currentLevel.questions[currentQuestionIndex];
    
    const isCorrect = selectedAnswer === currentQuestion.correct_answer;
    
    // Set feedback state for animations
    setAnswerFeedback(isCorrect ? 'correct' : 'incorrect');
    
    // Play sound
    if (soundEnabled) {
      if (isCorrect) {
        sounds.correct();
      } else {
        sounds.incorrect();
      }
    }
    
    // Set robot message based on result
    if (isCorrect) {
      const correctMessages = [
        "Excellent! You've got it! 🎉",
        "Amazing work! You're a quick learner! ⭐",
        "That's right! Keep going! 🚀",
        "Brilliant! You're on fire! 🔥",
        "Perfect! You really know your stuff! 💪"
      ];
      setRobotMessage(correctMessages[Math.floor(Math.random() * correctMessages.length)]);
    } else {
      const incorrectMessages = [
        "Not quite, but don't give up! Let me explain...",
        "Good try! Here's what you need to know...",
        "Almost there! Let's learn together...",
        "No worries! Learning takes time. Here's why..."
      ];
      setRobotMessage(incorrectMessages[Math.floor(Math.random() * incorrectMessages.length)]);
    }
    
    setShowExplanation(true);
    
    // Clear feedback animation after a delay
    setTimeout(() => setAnswerFeedback(null), 1000);
  }, [activeQuiz, selectedAnswer, currentLevelIndex, currentQuestionIndex, soundEnabled]);

  const handleNextQuestion = useCallback(async () => {
    if (!activeQuiz) return;
    
    const currentLevel = activeQuiz.levels[currentLevelIndex];
    const isLastQuestion = currentQuestionIndex >= currentLevel.questions.length - 1;
    
    console.log('handleNextQuestion called:', {
      currentQuestionIndex,
      totalQuestions: currentLevel.questions.length,
      isLastQuestion
    });
    
    // Track if current answer is correct
    const currentQuestion = currentLevel.questions[currentQuestionIndex];
    const currentAnswerCorrect = selectedAnswer === currentQuestion.correct_answer;
    const finalCorrectCount = correctAnswers + (currentAnswerCorrect ? 1 : 0);
    
    if (isLastQuestion) {
      // Level completed - submit progress
      try {
        console.log('Submitting level progress:', {
          levelId: currentLevel.level.id,
          correctAnswers: finalCorrectCount,
          totalQuestions: currentLevel.questions.length
        });
        
        const result = await invoke<LevelResult>('update_level_progress', {
          userId: 1,
          levelId: currentLevel.level.id,
          correctAnswers: finalCorrectCount,
          totalQuestions: currentLevel.questions.length
        });
        
        console.log('Level result:', result);
        
        // Play celebration sound and show confetti if passed
        if (soundEnabled) {
          if (result.passed) {
            playCelebrationSound();
            setShowCelebration(true);
          } else {
            sounds.levelComplete();
          }
        }
        
        setLevelResult(result);
        setQuizCompleted(true);
        setRobotMessage(null);
        
        // Refresh user data
        const updatedUser = await invoke<User>('get_user', { userId: 1 });
        setUser(updatedUser);
        
        // Refresh states to show updated stars/progress
        const updatedStates = await invoke<StateWithProgress[]>('get_all_states', { userId: 1 });
        setStates(updatedStates);
        
      } catch (err) {
        console.error('Error updating progress:', err);
      }
    } else {
      // Move to next question - update correct count first
      if (currentAnswerCorrect) {
        setCorrectAnswers(prev => prev + 1);
      }
      console.log('Moving to next question:', currentQuestionIndex + 1);
      setCurrentQuestionIndex(prev => prev + 1);
      setSelectedAnswer(null);
      setShowExplanation(false);
      setRobotMessage(null);
    }
  }, [activeQuiz, currentLevelIndex, currentQuestionIndex, correctAnswers, selectedAnswer, soundEnabled]);

  const handleExitQuiz = useCallback(async () => {
    setActiveQuiz(null);
    setQuizCompleted(false);
    setLevelResult(null);
    setShowCelebration(false);
    document.body.classList.remove('modal-open');
    
    // Refresh module content to show updated progress
    if (selectedState) {
      try {
        const updatedModules = await invoke<ModuleWithProgress[]>('get_modules_for_state', { 
          stateId: selectedState.state.id,
          userId: 1
        });
        setModuleContent(updatedModules);
        
        // Also refresh states to update the state card
        const updatedStates = await invoke<StateWithProgress[]>('get_all_states', { userId: 1 });
        setStates(updatedStates);
        
        // Update selectedState with fresh data
        const freshSelectedState = updatedStates.find(s => s.state.id === selectedState.state.id);
        if (freshSelectedState) {
          setSelectedState(freshSelectedState);
        }
      } catch (err) {
        console.error('Error refreshing module content:', err);
      }
    }
  }, [selectedState]);

  const handleContinueToNextLevel = useCallback(async () => {
    if (!activeQuiz) return;
    
    const isLastLevel = currentLevelIndex >= activeQuiz.levels.length - 1;
    
    if (isLastLevel) {
      // Module completed - check if STATE is complete (all modules done)
      if (selectedState && moduleContent) {
        // Refresh module content to check completion status
        try {
          const updatedModules = await invoke<ModuleWithProgress[]>('get_modules_for_state', { 
            stateId: selectedState.state.id,
            userId: 1
          });
          
          // Check if all modules in this state are completed
          const allModulesComplete = updatedModules.every(m => m.progress?.is_completed);
          
          if (allModulesComplete) {
            // STATE COMPLETED! Show Journey Map
            // Find the next unlocked state
            const updatedStates = await invoke<StateWithProgress[]>('get_all_states', { userId: 1 });
            setStates(updatedStates);
            
            const currentStateIndex = updatedStates.findIndex(s => s.state.id === selectedState.state.id);
            const nextState = updatedStates.find((s, i) => i > currentStateIndex && s.is_unlocked && !s.progress?.is_completed);
            
            // Calculate total rewards from this state
            const totalXpEarned = updatedModules.reduce((sum, m) => sum + (m.progress?.total_xp_earned || 0), 0);
            const maxStars = Math.max(...updatedModules.map(m => m.progress?.stars || 0));
            
            setJourneyData({
              fromState: {
                id: selectedState.state.id,
                name: selectedState.state.name,
                region: selectedState.state.region || 'Nigeria'
              },
              toState: nextState ? {
                id: nextState.state.id,
                name: nextState.state.name,
                region: nextState.state.region || 'Nigeria',
                landmark_name: nextState.state.landmark_name || undefined
              } : null,
              rewards: {
                stars: maxStars,
                xp: totalXpEarned,
                items: levelResult?.unlocked_item ? [levelResult.unlocked_item] : [],
                badges: []
              }
            });
            
            // Play a victory sound
            if (soundEnabled) {
              sounds.levelComplete();
              setTimeout(() => sounds.levelComplete(), 300);
            }
            
            handleExitQuiz();
            setSelectedState(null);
            setModuleContent(null);
            setShowJourneyMap(true);
            return;
          }
        } catch (err) {
          console.error('Error checking state completion:', err);
        }
      }
      
      // Not all modules complete - just go back to module list
      handleExitQuiz();
    } else {
      // Move to next level
      setCurrentLevelIndex(prev => prev + 1);
      setCurrentQuestionIndex(0);
      setSelectedAnswer(null);
      setShowExplanation(false);
      setCorrectAnswers(0);
      setQuizCompleted(false);
      setLevelResult(null);
    }
  }, [activeQuiz, currentLevelIndex, handleExitQuiz, selectedState, moduleContent, levelResult, soundEnabled]);

  // ============================================
  // THE SABI CODEX - Encyclopedia Handlers
  // ============================================

  const loadCodexEntries = useCallback(async (category?: string) => {
    setIsCodexLoading(true);
    try {
      let entries: EncyclopediaEntryWithProgress[];
      
      if (category && category !== 'all') {
        entries = await invoke<EncyclopediaEntryWithProgress[]>('get_encyclopedia_by_category', {
          userId: 1,
          category
        });
      } else {
        entries = await invoke<EncyclopediaEntryWithProgress[]>('get_all_encyclopedia_entries', {
          userId: 1
        });
      }
      
      setCodexEntries(entries);
      
      // Also load stats
      const stats = await invoke<CodexStats>('get_codex_stats', { userId: 1 });
      setCodexStats(stats);
    } catch (err) {
      console.error('Error loading codex entries:', err);
    } finally {
      setIsCodexLoading(false);
    }
  }, []);

  const handleOpenCodex = useCallback(async () => {
    playSound('click');
    setShowCodex(true);
    document.body.classList.add('modal-open');
    await loadCodexEntries();
  }, [loadCodexEntries, playSound]);

  const handleCloseCodex = useCallback(() => {
    setShowCodex(false);
    setSelectedCodexEntry(null);
    setSelectedCodexCategory('all');
    document.body.classList.remove('modal-open');
  }, []);

  const handleOpenInventory = useCallback(() => {
    setShowInventory(true);
    document.body.classList.add('modal-open');
  }, []);

  const handleCloseInventory = useCallback(() => {
    setShowInventory(false);
    document.body.classList.remove('modal-open');
  }, []);

  const handleSelectCodexCategory = useCallback(async (category: string) => {
    playSound('pop');
    setSelectedCodexCategory(category);
    await loadCodexEntries(category);
  }, [loadCodexEntries, playSound]);

  const handleSelectCodexEntry = useCallback((entry: EncyclopediaEntryWithProgress) => {
    if (!entry.is_accessible) {
      // Entry is locked
      playSound('hover');
      return;
    }
    playSound('click');
    setSelectedCodexEntry(entry);
  }, [playSound]);

  const handleCloseCodexEntry = useCallback(() => {
    setSelectedCodexEntry(null);
  }, []);

  const handleMarkEntryRead = useCallback(async (entryId: string) => {
    try {
      const result = await invoke<MarkReadResult>('mark_encyclopedia_read', {
        userId: 1,
        entryId
      });
      
      if (result.was_first_read && result.xp_awarded > 0) {
        playSound('success');
        // Refresh user data for XP update
        const updatedUser = await invoke<User>('get_user', { userId: 1 });
        setUser(updatedUser);
      }
      
      // Refresh entries
      await loadCodexEntries(selectedCodexCategory);
      
      // Update the selected entry's progress to reflect the read status
      if (selectedCodexEntry && selectedCodexEntry.progress) {
        setSelectedCodexEntry({
          ...selectedCodexEntry,
          progress: {
            entry_id: selectedCodexEntry.progress.entry_id,
            is_read: true,
            is_unlocked: selectedCodexEntry.progress.is_unlocked,
            is_bookmarked: selectedCodexEntry.progress.is_bookmarked,
            unlocked_at: selectedCodexEntry.progress.unlocked_at,
            first_read_at: selectedCodexEntry.progress.first_read_at || new Date().toISOString(),
            read_count: (selectedCodexEntry.progress.read_count || 0) + 1
          }
        });
      }
    } catch (err) {
      console.error('Error marking entry as read:', err);
    }
  }, [loadCodexEntries, selectedCodexCategory, playSound]);

  const handleToggleBookmark = useCallback(async (entryId: string) => {
    try {
      playSound('pop');
      await invoke<boolean>('toggle_encyclopedia_bookmark', {
        userId: 1,
        entryId
      });
      // Refresh entries
      await loadCodexEntries(selectedCodexCategory);
    } catch (err) {
      console.error('Error toggling bookmark:', err);
    }
  }, [loadCodexEntries, selectedCodexCategory, playSound]);

  // Calculate XP progress percentage
  const xpProgress = user ? ((user.total_xp % 100) / 100) * 100 : 0;

  // Show startup screen first
  if (currentScreen === 'startup') {
    return (
      <AnimatePresence mode="wait">
        <StartupScreen onStart={handleStartAdventure} />
      </AnimatePresence>
    );
  }

  // Show onboarding screen
  if (currentScreen === 'onboarding') {
    return (
      <AnimatePresence mode="wait">
        <OnboardingScreen onComplete={handleOnboardingComplete} />
      </AnimatePresence>
    );
  }

  if (isLoading) {
    return (
      <div className="loading-screen">
        <motion.div 
          className="loading-container"
          initial={{ opacity: 0, scale: 0.8 }}
          animate={{ opacity: 1, scale: 1 }}
          transition={{ duration: 0.5 }}
        >
          <motion.div 
            className="loading-mascot"
            animate={{ 
              y: [0, -15, 0],
              rotate: [0, 5, -5, 0]
            }}
            transition={{ 
              duration: 2, 
              repeat: Infinity, 
              ease: "easeInOut" 
            }}
          >
            🦎
          </motion.div>
          <motion.div 
            className="loading-bar-container"
            initial={{ width: 0 }}
            animate={{ width: '200px' }}
            transition={{ duration: 0.3 }}
          >
            <motion.div 
              className="loading-bar"
              initial={{ width: '0%' }}
              animate={{ width: '100%' }}
              transition={{ duration: 2, ease: 'easeInOut' }}
            />
          </motion.div>
          <motion.h2
            animate={{ opacity: [1, 0.5, 1] }}
            transition={{ duration: 1.5, repeat: Infinity }}
          >
            Loading Sabi Quest...
          </motion.h2>
        </motion.div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="error-screen">
        <motion.div 
          className="error-container"
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
        >
          <AlertTriangle size={48} className="error-icon" />
          <h2>Oops! Something went wrong</h2>
          <p className="error-message">{error}</p>
          <motion.button 
            className="btn-primary"
            onClick={initializeApp}
            variants={buttonVariants}
            whileHover="hover"
            whileTap="tap"
          >
            Try Again
          </motion.button>
        </motion.div>
      </div>
    );
  }

  return (
    <div className="app">
      {/* Update Checker - Shows banner when update is available */}
      <UpdateChecker />

      {/* Immersive Background */}
      <div className="app-background">
        <div className="bg-gradient" />
        <div className="bg-pattern" />
      </div>

      {/* HUD - Floating Top Bar */}
      <header className="hud-floating">
        {/* Left: Avatar & User Info */}
        <motion.div 
          className="user-card"
          initial={{ opacity: 0, x: -20 }}
          animate={{ opacity: 1, x: 0 }}
          whileHover={{ scale: 1.02 }}
        >
          <div className="avatar-wrapper">
            <div className="avatar-glow" />
            <div className="avatar-circle-new">
              {userAvatar ? (
                <ChibiAvatar
                  skinTone={userAvatar.skin_tone}
                  hairStyle={userAvatar.hairstyle}
                  outfit={userAvatar.outfit}
                  accessory={userAvatar.accessory || undefined}
                  size={48}
                />
              ) : (
                <span>{(user?.display_name || 'E').charAt(0)}</span>
              )}
            </div>
            <div className="level-ring">
              <svg viewBox="0 0 36 36" className="level-progress-ring">
                <path
                  className="ring-bg"
                  d="M18 2.0845 a 15.9155 15.9155 0 0 1 0 31.831 a 15.9155 15.9155 0 0 1 0 -31.831"
                />
                <path
                  className="ring-fill"
                  strokeDasharray={`${xpProgress}, 100`}
                  d="M18 2.0845 a 15.9155 15.9155 0 0 1 0 31.831 a 15.9155 15.9155 0 0 1 0 -31.831"
                />
              </svg>
            </div>
          </div>
          <div className="user-details">
            <span className="greeting">Welcome back,</span>
            <span className="user-name-large">{userAvatar?.character_name || user?.display_name || 'Explorer'}!</span>
            <div className="xp-bar-mini">
              <div className="xp-fill" style={{ width: `${xpProgress}%` }} />
              <span className="xp-text">{user?.total_xp || 0} XP</span>
            </div>
          </div>
        </motion.div>

        {/* Right: Stats Pills */}
        <div className="stats-row">
          <motion.div 
            className="stat-card level"
            whileHover={{ scale: 1.05, y: -2 }}
            whileTap={{ scale: 0.95 }}
          >
            <div className="stat-icon-wrapper">
              <Trophy size={18} />
            </div>
            <div className="stat-info">
              <span className="stat-label">Level</span>
              <span className="stat-number">{user?.current_level || 1}</span>
            </div>
          </motion.div>
          
          <motion.div 
            className="stat-card cowries"
            whileHover={{ scale: 1.05, y: -2 }}
            whileTap={{ scale: 0.95 }}
          >
            <div className="stat-icon-wrapper">
              <Shell size={18} />
            </div>
            <div className="stat-info">
              <span className="stat-label">Cowries</span>
              <span className="stat-number">{user?.cowrie_shells || 0}</span>
            </div>
          </motion.div>
          
          <motion.div 
            className="stat-card streak"
            whileHover={{ scale: 1.05, y: -2 }}
            whileTap={{ scale: 0.95 }}
          >
            <div className="stat-icon-wrapper fire">
              <Flame size={18} />
            </div>
            <div className="stat-info">
              <span className="stat-label">Streak</span>
              <span className="stat-number">{user?.streak_days || 0} 🔥</span>
            </div>
          </motion.div>
          
          {/* Sabi Codex Button */}
          <motion.button
            className="codex-btn"
            onClick={handleOpenCodex}
            whileHover={{ scale: 1.08, y: -2 }}
            whileTap={{ scale: 0.95 }}
            title="The Sabi Codex - Encyclopedia"
          >
            <Book size={20} />
            <span className="codex-label">Codex</span>
          </motion.button>
        </div>
      </header>

      {/* Main Content Area - Map View */}
      {mainView === 'map' && (
      <main className="explore-main">
        {/* Clean Explore Header */}
        <motion.section 
          className="explore-header"
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          transition={{ duration: 0.4 }}
        >
          <div className="explore-header-left">
            <motion.div 
              className="explore-icon"
              whileHover={{ scale: 1.05 }}
              whileTap={{ scale: 0.95 }}
              onClick={() => playSound('pop')}
            >
              <Map size={24} />
            </motion.div>
            <div className="explore-text">
              <h1>Explore Nigeria</h1>
              <p>Discover stories from {states.length} amazing states</p>
            </div>
          </div>
          
          <div className="explore-header-right">
            <div className="stat-badge completed">
              <CheckCircle size={14} />
              <span>{states.filter(s => s.progress?.is_completed).length}</span>
            </div>
            <div className="stat-badge available">
              <Sparkles size={14} />
              <span>{states.filter(s => s.is_unlocked && !s.progress?.is_completed).length}</span>
            </div>
            <div className="stat-badge locked">
              <Lock size={14} />
              <span>{states.filter(s => !s.is_unlocked).length}</span>
            </div>
            <motion.button
              className="sound-btn"
              onClick={() => {
                setSoundEnabled(!soundEnabled);
                if (!soundEnabled) sounds.pop();
              }}
              whileHover={{ scale: 1.1 }}
              whileTap={{ scale: 0.9 }}
            >
              {soundEnabled ? <Volume2 size={16} /> : <VolumeX size={16} />}
            </motion.button>
          </div>
        </motion.section>

        {/* Recommended For You Section */}
        {recommendedModules.length > 0 && (
          <motion.section 
            className="recommended-section"
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.5, delay: 0.2 }}
          >
            <div className="recommended-header">
              <div className="recommended-title">
                <Sparkles size={20} className="sparkle-icon" />
                <h2>Recommended for You</h2>
              </div>
              <p className="recommended-subtitle">
                Based on your interests: {user?.interests?.map(i => 
                  i.charAt(0).toUpperCase() + i.slice(1)
                ).join(', ') || 'All topics'}
              </p>
            </div>
            
            <div className="recommended-grid">
              {recommendedModules.map((module, index) => {
                // Get matching interest tags
                const matchingTags = (module.module.interest_tags || []).filter(
                  tag => user?.interests?.includes(tag)
                );
                
                // Interest tag icons
                const tagIcons: Record<string, React.ReactNode> = {
                  history: <BookMarked size={12} />,
                  culture: <Theater size={12} />,
                  geography: <Globe size={12} />,
                  food: <UtensilsCrossed size={12} />,
                  music: <Music size={12} />,
                  languages: <MessageCircle size={12} />,
                };
                
                return (
                  <motion.div
                    key={module.module.id}
                    className={`recommended-card ${module.progress?.is_completed ? 'completed' : ''}`}
                    initial={{ opacity: 0, scale: 0.9 }}
                    animate={{ opacity: 1, scale: 1 }}
                    transition={{ delay: index * 0.1, duration: 0.3 }}
                    whileHover={{ scale: 1.03, y: -4 }}
                    whileTap={{ scale: 0.98 }}
                    onClick={() => {
                      // Find the state this module belongs to
                      const state = states.find(s => s.state.id === module.module.state_id);
                      if (state && state.is_unlocked) {
                        playSound('click');
                        handleExploreState(state);
                      }
                    }}
                  >
                    <div className="recommended-card-header">
                      <span className="recommended-state-name">
                        {states.find(s => s.state.id === module.module.state_id)?.state.name || 'Unknown State'}
                      </span>
                      {module.progress?.is_completed && (
                        <CheckCircle size={14} className="completed-check" />
                      )}
                    </div>
                    
                    <h3 className="recommended-module-title">{module.module.title}</h3>
                    
                    <div className="recommended-tags">
                      {matchingTags.length > 0 ? (
                        matchingTags.map(tag => (
                          <span key={tag} className="interest-tag matched">
                            {tagIcons[tag] || <Star size={12} />}
                            {tag}
                          </span>
                        ))
                      ) : (
                        (module.module.interest_tags || []).slice(0, 2).map(tag => (
                          <span key={tag} className="interest-tag">
                            {tagIcons[tag] || <Star size={12} />}
                            {tag}
                          </span>
                        ))
                      )}
                    </div>
                    
                    <div className="recommended-meta">
                      <span className="xp-reward">
                        <Zap size={12} />
                        {module.module.total_xp} XP
                      </span>
                    </div>
                    
                    {!states.find(s => s.state.id === module.module.state_id)?.is_unlocked && (
                      <div className="recommended-locked-overlay">
                        <Lock size={16} />
                        <span>State Locked</span>
                      </div>
                    )}
                  </motion.div>
                );
              })}
            </div>
            
            {isLoadingRecommended && (
              <div className="recommended-loading">
                <motion.div
                  animate={{ rotate: 360 }}
                  transition={{ duration: 1, repeat: Infinity, ease: "linear" }}
                >
                  <Sparkles size={24} />
                </motion.div>
                <span>Finding modules for you...</span>
              </div>
            )}
          </motion.section>
        )}

        {/* States Grid */}
        <section className="states-section">
          <div className="states-grid-new">
            {states.map((item, index) => {
              // Generate landmark image path - uses state name lowercase
              const stateLower = item.state.name.toLowerCase().replace(/\s+/g, '-');
              const landmarkImage = `/assets/images/${stateLower}-landmark.png`;
              
              return (
                <motion.div
                  key={item.state.id}
                  className={`state-card-new ${item.is_unlocked ? 'unlocked' : 'locked'} ${item.progress?.is_completed ? 'completed' : ''}`}
                  custom={index}
                  initial={{ opacity: 0, y: 40 }}
                  animate={{ opacity: 1, y: 0 }}
                  transition={{ delay: index * 0.08, duration: 0.5, ease: "easeOut" }}
                  whileHover={item.is_unlocked ? { y: -8, scale: 1.02 } : undefined}
                  whileTap={item.is_unlocked ? { scale: 0.98 } : undefined}
                  onHoverStart={() => item.is_unlocked && playSound('hover')}
                  onClick={() => {
                    if (item.is_unlocked) {
                      playSound('click');
                      handleExploreState(item);
                    }
                  }}
                >
                  {/* Landmark Image Area */}
                  <div className="landmark-image-wrapper">
                    {/* Try to load landmark image, fallback to placeholder */}
                    <img 
                      src={landmarkImage}
                      alt={`${item.state.name} landmark`}
                      className="landmark-image"
                      onError={(e) => {
                        // Hide broken image and show placeholder
                        e.currentTarget.style.display = 'none';
                        const placeholder = e.currentTarget.nextElementSibling as HTMLElement;
                        if (placeholder) placeholder.style.display = 'flex';
                      }}
                    />
                    {/* Placeholder shown when image fails to load */}
                    <div className="landmark-placeholder" style={{ display: 'none' }}>
                      <div className="landmark-placeholder-icon">
                        <Landmark size={28} />
                      </div>
                      <span className="landmark-placeholder-text">{item.state.name}</span>
                    </div>
                    
                    {/* Gradient Overlay */}
                    <div className="image-overlay" />
                    
                    {/* State Name on Image */}
                    <div className="state-name-overlay">
                      <div className="region-badge">{item.state.region || 'Nigeria'}</div>
                      <h3 className="state-name-large">{item.state.name}</h3>
                    </div>
                  </div>
                  
                  {/* Status Badge */}
                  {item.progress?.is_completed && (
                    <motion.div 
                      className="status-badge completed"
                      initial={{ scale: 0 }}
                      animate={{ scale: 1 }}
                      transition={{ delay: index * 0.08 + 0.3, type: "spring" }}
                    >
                      <CheckCircle size={18} />
                    </motion.div>
                  )}
                  
                  {!item.is_unlocked && (
                    <div className="status-badge locked">
                      <Lock size={16} />
                    </div>
                  )}

                  {/* Card Content - Bottom */}
                  <div className="card-content-new">
                    {/* Info Row */}
                    <div className="card-info-row">
                      {item.state.landmark_name && (
                        <div className="landmark-name">
                          <MapPin size={14} />
                          <span>{item.state.landmark_name}</span>
                        </div>
                      )}
                      <div className="stars-row">
                        {[...Array(3)].map((_, i) => (
                          <Star
                            key={i}
                            size={16}
                            className={i < (item.progress?.stars || 0) ? 'star-earned' : 'star-dim'}
                            fill={i < (item.progress?.stars || 0) ? '#FFD700' : 'none'}
                          />
                        ))}
                      </div>
                    </div>
                    
                    {/* Meta Row */}
                    {item.is_unlocked ? (
                      <div className="card-meta">
                        <div className="meta-item">
                          <BookOpen size={14} />
                          <span>{item.modules_count || item.lessons_count} modules</span>
                        </div>
                        <div className="meta-item xp">
                          <Zap size={14} />
                          <span>{(item.modules_count || item.lessons_count || 1) * 100} XP</span>
                        </div>
                      </div>
                    ) : (
                      <div className="locked-footer">
                        <Lock size={14} />
                        <span>Level {item.state.unlock_level} to unlock</span>
                      </div>
                    )}
                  </div>
                </motion.div>
              );
            })}
          </div>
        </section>
      </main>
      )}

      {/* Character/Avatar View */}
      {mainView === 'character' && (
        <motion.main 
          className="character-view"
          initial={{ opacity: 0, x: 20 }}
          animate={{ opacity: 1, x: 0 }}
          exit={{ opacity: 0, x: -20 }}
        >
          <div className="character-header">
            <h1><User size={28} /> My Character</h1>
          </div>

          {isCharacterLoading ? (
            <div className="view-loading">
              <div className="loading-spinner" />
              <span>Loading customization options...</span>
            </div>
          ) : (
            <div className="avatar-preview-container">
              {/* Enhanced Avatar Preview */}
              <div className="avatar-preview">
                {/* Dynamic Background */}
                <div 
                  className="avatar-preview-bg"
                  style={{
                    background: (() => {
                      const bgId = tempAvatarSelections?.background_id || userAvatar?.background || 'bg_default';
                      const bgItem = avatarItems.find(i => i.item.id === bgId);
                      const bgName = bgItem?.item.name?.toLowerCase() || '';
                      
                      // Background gradients based on name or ID
                      if (bgName.includes('savanna') || bgName.includes('sunset') || bgId === 'bg_default') {
                        return 'linear-gradient(180deg, #FF6B35 0%, #FFB347 30%, #FFCC70 60%, #8B7355 100%)';
                      }
                      if (bgName.includes('rainforest') || bgName.includes('forest') || bgId === 'bg_forest') {
                        return 'linear-gradient(180deg, #1B4332 0%, #2D6A4F 30%, #40916C 60%, #52B788 100%)';
                      }
                      if (bgName.includes('village') || bgId === 'bg_village') {
                        return 'linear-gradient(180deg, #87CEEB 0%, #E8D4B8 50%, #A67B5B 100%)';
                      }
                      if (bgName.includes('palace') || bgName.includes('royal') || bgId === 'bg_palace') {
                        return 'linear-gradient(180deg, #8B0000 0%, #B22222 30%, #FFD700 60%, #5C4033 100%)';
                      }
                      if (bgName.includes('market') || bgId === 'bg_market') {
                        return 'linear-gradient(180deg, #FF6B6B 0%, #FFA500 25%, #FFD700 50%, #00CED1 75%, #4169E1 100%)';
                      }
                      if (bgName.includes('lagos') || bgName.includes('skyline') || bgId === 'bg_lagos') {
                        return 'linear-gradient(180deg, #1a1a2e 0%, #16213e 30%, #0f3460 60%, #e94560 100%)';
                      }
                      return 'linear-gradient(135deg, #E8F5F1 0%, #B8E6D8 50%, #F0FDF9 100%)';
                    })()
                  }}
                >
                  {/* Background decorations */}
                  <div className="bg-decoration bg-dec-1" />
                  <div className="bg-decoration bg-dec-2" />
                  <div className="bg-decoration bg-dec-3" />
                </div>
                
                {/* Chibi Avatar Character Display */}
                <div className="avatar-character-wrapper">
                  <ChibiAvatar
                    skinTone={tempAvatarSelections?.skin_tone_id || userAvatar?.skin_tone || 'skin_3'}
                    hairStyle={tempAvatarSelections?.hairstyle_id || userAvatar?.hairstyle || 'hair_1'}
                    outfit={tempAvatarSelections?.outfit_id || userAvatar?.outfit || 'outfit_school'}
                    accessory={tempAvatarSelections?.accessory_id || userAvatar?.accessory || undefined}
                    expression="happy"
                    size={220}
                  />
                </div>
                
                {/* Character Info - Closer to avatar */}
                <div className="avatar-info">
                  <div className="avatar-name-row">
                    {isEditingName ? (
                      <input
                        type="text"
                        className="avatar-name-input"
                        value={tempAvatarSelections?.character_name || userAvatar?.character_name || ''}
                        onChange={(e) => {
                          const baseSelections = tempAvatarSelections || {
                            skin_tone_id: userAvatar?.skin_tone || 'skin_3',
                            hairstyle_id: userAvatar?.hairstyle || 'hair_1',
                            outfit_id: userAvatar?.outfit || 'outfit_school',
                            accessory_id: userAvatar?.accessory || null,
                            background_id: userAvatar?.background || 'bg_default',
                            character_name: userAvatar?.character_name || user?.display_name || 'Explorer'
                          };
                          setTempAvatarSelections({
                            ...baseSelections,
                            character_name: e.target.value
                          });
                        }}
                        maxLength={20}
                        autoFocus
                        onBlur={() => setIsEditingName(false)}
                        onKeyDown={(e) => e.key === 'Enter' && setIsEditingName(false)}
                      />
                    ) : (
                      <span className="avatar-preview-name">
                        {tempAvatarSelections?.character_name || userAvatar?.character_name || user?.display_name || 'Your Character'}
                      </span>
                    )}
                    <button 
                      className="edit-name-btn"
                      onClick={() => setIsEditingName(!isEditingName)}
                    >
                      {isEditingName ? <Check size={14} /> : <Pencil size={14} />}
                    </button>
                  </div>
                  <span className="avatar-preview-title">
                    Level {user?.current_level || 1} Explorer
                  </span>
                </div>
                
                {/* Save Button - Top Right */}
                {tempAvatarSelections && (
                  <motion.button
                    className="avatar-save-btn"
                    onClick={handleSaveAvatar}
                    initial={{ opacity: 0, scale: 0.8 }}
                    animate={{ opacity: 1, scale: 1 }}
                    whileHover={{ scale: 1.05 }}
                    whileTap={{ scale: 0.95 }}
                  >
                    <Save size={16} />
                    Save
                  </motion.button>
                )}
              </div>

              {/* Customization Panel */}
              <div className="avatar-customization">
                <div className="customization-tabs">
                  {[
                    { id: 'skin_tone', label: 'Skin', icon: Circle },
                    { id: 'hairstyle', label: 'Hair', icon: Scissors },
                    { id: 'outfit', label: 'Outfit', icon: Shirt },
                    { id: 'accessory', label: 'Accessories', icon: Gem },
                    { id: 'background', label: 'Background', icon: Image }
                  ].map(tab => (
                    <button 
                      key={tab.id}
                      className={`customization-tab ${selectedAvatarCategory === tab.id ? 'active' : ''}`}
                      onClick={() => {
                        setSelectedAvatarCategory(tab.id);
                        // Items are already loaded, just switch the category filter
                      }}
                    >
                      <tab.icon size={16} />
                      {tab.label}
                    </button>
                  ))}
                </div>

                <div className="avatar-items-grid">
                  {avatarItems
                    .filter(item => item.item.category === selectedAvatarCategory)
                    .map(item => {
                      const isSelected = tempAvatarSelections 
                        ? (selectedAvatarCategory === 'skin_tone' && tempAvatarSelections.skin_tone_id === item.item.id) ||
                          (selectedAvatarCategory === 'hairstyle' && tempAvatarSelections.hairstyle_id === item.item.id) ||
                          (selectedAvatarCategory === 'outfit' && tempAvatarSelections.outfit_id === item.item.id) ||
                          (selectedAvatarCategory === 'accessory' && tempAvatarSelections.accessory_id === item.item.id) ||
                          (selectedAvatarCategory === 'background' && tempAvatarSelections.background_id === item.item.id)
                        : (selectedAvatarCategory === 'skin_tone' && userAvatar?.skin_tone === item.item.id) ||
                          (selectedAvatarCategory === 'hairstyle' && userAvatar?.hairstyle === item.item.id) ||
                          (selectedAvatarCategory === 'outfit' && userAvatar?.outfit === item.item.id) ||
                          (selectedAvatarCategory === 'accessory' && userAvatar?.accessory === item.item.id) ||
                          (selectedAvatarCategory === 'background' && userAvatar?.background === item.item.id);

                      return (
                        <motion.button
                          key={item.item.id}
                          className={`avatar-item-card ${isSelected ? 'selected' : ''} ${!item.is_unlocked ? 'locked' : ''}`}
                          onClick={() => {
                            if (!item.is_unlocked) return;
                            playSound('click');
                            
                            const baseSelections = tempAvatarSelections || {
                              skin_tone_id: userAvatar?.skin_tone || 'skin_3',
                              hairstyle_id: userAvatar?.hairstyle || 'hair_1',
                              outfit_id: userAvatar?.outfit || 'outfit_school',
                              accessory_id: userAvatar?.accessory || null,
                              background_id: userAvatar?.background || 'bg_default',
                              character_name: userAvatar?.character_name || user?.display_name || 'Explorer'
                            };

                            setTempAvatarSelections({
                              ...baseSelections,
                              [selectedAvatarCategory === 'skin_tone' ? 'skin_tone_id' :
                               selectedAvatarCategory === 'hairstyle' ? 'hairstyle_id' :
                               selectedAvatarCategory === 'outfit' ? 'outfit_id' :
                               selectedAvatarCategory === 'accessory' ? 'accessory_id' :
                               'background_id']: item.item.id
                            });
                          }}
                          whileHover={{ scale: item.is_unlocked ? 1.05 : 1 }}
                          whileTap={{ scale: item.is_unlocked ? 0.95 : 1 }}
                        >
                          <div 
                            className={`avatar-item-preview ${selectedAvatarCategory === 'hairstyle' ? 'hair-preview' : ''}`}
                            style={{
                              background: (() => {
                                if (selectedAvatarCategory === 'skin_tone') {
                                  // Direct ID to color mapping for skin tones
                                  const skinColors: Record<string, string> = {
                                    'skin_1': '#D4A574',
                                    'skin_2': '#C68642',
                                    'skin_3': '#8D5524',
                                    'skin_4': '#5C3317',
                                    'skin_5': '#3B2314',
                                    'skin_6': '#4A2C2A',
                                  };
                                  return skinColors[item.item.id] || '#8D5524';
                                }
                                if (selectedAvatarCategory === 'hairstyle') {
                                  const hairName = item.item.name.toLowerCase();
                                  if (hairName.includes('gele')) return 'linear-gradient(135deg, #FFD700 0%, #FF6B6B 100%)';
                                  return '#1C1C1C';
                                }
                                if (selectedAvatarCategory === 'outfit') {
                                  const outfitName = item.item.name.toLowerCase();
                                  if (outfitName.includes('agbada')) return 'linear-gradient(135deg, #FFD700 0%, #DAA520 100%)';
                                  if (outfitName.includes('ankara')) return 'linear-gradient(135deg, #FF6B6B 0%, #FFA500 50%, #00CED1 100%)';
                                  if (outfitName.includes('isiagu')) return 'linear-gradient(135deg, #8B0000 0%, #DC143C 100%)';
                                  if (outfitName.includes('warrior')) return 'linear-gradient(135deg, #8B4513 0%, #A0522D 100%)';
                                  if (outfitName.includes('royal')) return 'linear-gradient(135deg, #4B0082 0%, #8A2BE2 100%)';
                                  if (outfitName.includes('festival')) return 'linear-gradient(135deg, #FF1493 0%, #FFD700 50%, #00FF7F 100%)';
                                  if (outfitName.includes('school')) return 'linear-gradient(135deg, #4169E1 0%, #1E90FF 100%)';
                                  if (outfitName.includes('aso') || outfitName.includes('oke')) return 'linear-gradient(135deg, #C0C0C0 0%, #FFD700 50%, #4169E1 100%)';
                                  return 'linear-gradient(135deg, #00C896 0%, #00A080 100%)';
                                }
                                if (selectedAvatarCategory === 'background') {
                                  const bgId = item.item.id;
                                  const bgName = item.item.name.toLowerCase();
                                  if (bgName.includes('savanna') || bgName.includes('sunset') || bgId === 'bg_default') {
                                    return 'linear-gradient(180deg, #FF6B35 0%, #FFB347 50%, #8B7355 100%)';
                                  }
                                  if (bgName.includes('rainforest') || bgName.includes('forest') || bgId === 'bg_forest') {
                                    return 'linear-gradient(180deg, #1B4332 0%, #40916C 100%)';
                                  }
                                  if (bgName.includes('village') || bgId === 'bg_village') {
                                    return 'linear-gradient(180deg, #87CEEB 0%, #A67B5B 100%)';
                                  }
                                  if (bgName.includes('palace') || bgName.includes('royal') || bgId === 'bg_palace') {
                                    return 'linear-gradient(180deg, #8B0000 0%, #FFD700 100%)';
                                  }
                                  if (bgName.includes('market') || bgId === 'bg_market') {
                                    return 'linear-gradient(180deg, #FF6B6B 0%, #FFD700 50%, #4169E1 100%)';
                                  }
                                  if (bgName.includes('lagos') || bgName.includes('skyline') || bgId === 'bg_lagos') {
                                    return 'linear-gradient(180deg, #1a1a2e 0%, #e94560 100%)';
                                  }
                                  return 'linear-gradient(135deg, #E8F5F1 0%, #B8E6D8 100%)';
                                }
                                return 'var(--color-background)';
                              })()
                            }}
                          >
                            {selectedAvatarCategory === 'hairstyle' && (
                              <span className="hair-icon">
                                {item.item.name.toLowerCase().includes('afro') ? '🌀' :
                                 item.item.name.toLowerCase().includes('gele') ? '👳‍♀️' :
                                 item.item.name.toLowerCase().includes('dread') ? '〰️' :
                                 item.item.name.toLowerCase().includes('bantu') ? '⭕' :
                                 item.item.name.toLowerCase().includes('braid') ? '〽️' :
                                 '✂️'}
                              </span>
                            )}
                            {selectedAvatarCategory === 'accessory' && '✨'}
                          </div>
                          <span className="avatar-item-name">{item.item.name}</span>
                          {!item.is_unlocked && (
                            <>
                              <div className="avatar-item-lock">
                                <Lock size={12} />
                              </div>
                              <div className="avatar-item-cost">
                                <Shell size={12} />
                                {item.item.unlock_cost}
                              </div>
                            </>
                          )}
                          {item.item.rarity !== 'starter' && (
                            <span className={`avatar-item-rarity ${item.item.rarity}`}>
                              {item.item.rarity}
                            </span>
                          )}
                        </motion.button>
                      );
                    })}
                </div>
              </div>
            </div>
          )}
        </motion.main>
      )}

      {/* Quests View */}
      {mainView === 'quests' && (
        <motion.main 
          className="quests-view"
          initial={{ opacity: 0, x: 20 }}
          animate={{ opacity: 1, x: 0 }}
          exit={{ opacity: 0, x: -20 }}
        >
          <div className="quests-header">
            <h1><Target size={28} /> Quests</h1>
          </div>

          <div className="quest-type-tabs">
            {[
              { id: 'main', label: 'Main Story' },
              { id: 'side', label: 'Side Quests' },
              { id: 'daily', label: 'Daily' },
              { id: 'weekly', label: 'Weekly' },
              { id: 'achievement', label: 'Achievements' }
            ].map(tab => (
              <button 
                key={tab.id}
                className={`quest-type-tab ${selectedQuestType === tab.id ? 'active' : ''}`}
                onClick={() => {
                  setSelectedQuestType(tab.id);
                  loadQuests(tab.id);
                }}
              >
                {tab.label}
              </button>
            ))}
          </div>

          {isQuestsLoading ? (
            <div className="view-loading">
              <div className="loading-spinner" />
              <span>Loading quests...</span>
            </div>
          ) : quests.length === 0 ? (
            <div className="empty-state">
              <Target size={64} />
              <h3>No Quests Available</h3>
              <p>Check back later for new adventures!</p>
            </div>
          ) : (
            <div className="quests-list">
              {quests.map((quest, index) => (
                <motion.div
                  key={quest.quest.id}
                  className={`quest-card ${quest.quest.quest_type} ${quest.status === 'completed' ? 'completed' : ''}`}
                  initial={{ opacity: 0, y: 20 }}
                  animate={{ opacity: 1, y: 0 }}
                  transition={{ delay: index * 0.05 }}
                  whileHover={{ scale: 1.01 }}
                >
                  <div className="quest-icon">
                    {quest.quest.quest_type === 'main' && <Scroll size={24} />}
                    {quest.quest.quest_type === 'side' && <Compass size={24} />}
                    {quest.quest.quest_type === 'daily' && <Clock size={24} />}
                    {quest.quest.quest_type === 'weekly' && <Target size={24} />}
                    {quest.quest.quest_type === 'achievement' && <Trophy size={24} />}
                  </div>
                  <div className="quest-content">
                    <h3 className="quest-title">{quest.quest.title}</h3>
                    <p className="quest-description">{quest.quest.description}</p>
                    <div className="quest-rewards">
                      <span className="quest-reward xp">
                        <Zap size={14} />
                        {quest.quest.xp_reward} XP
                      </span>
                      <span className="quest-reward cowries">
                        <Shell size={14} />
                        {quest.quest.cowrie_reward}
                      </span>
                      {quest.quest.artifact_reward_id && (
                        <span className="quest-reward artifact">
                          <Gift size={14} />
                          Artifact
                        </span>
                      )}
                    </div>
                  </div>
                  <div className="quest-progress">
                    <span className={`quest-status-badge ${quest.status}`}>
                      {quest.status === 'completed' && 'Completed'}
                      {quest.status === 'in_progress' && 'In Progress'}
                      {quest.status === 'not_started' && 'Available'}
                      {quest.status === 'locked' && 'Locked'}
                    </span>
                  </div>
                </motion.div>
              ))}
            </div>
          )}
        </motion.main>
      )}

      {/* Museum/Artifacts View */}
      {mainView === 'museum' && (
        <motion.main 
          className="museum-view"
          initial={{ opacity: 0, x: 20 }}
          animate={{ opacity: 1, x: 0 }}
          exit={{ opacity: 0, x: -20 }}
        >
          <div className="museum-header">
            <div className="museum-header-left">
              <h1><Building2 size={28} /> Cultural Museum</h1>
              <p className="museum-subtitle">Discover Nigeria's rich heritage through cultural artifacts</p>
            </div>
            {collectionStats && (
              <div className="collection-stats">
                <div className="collection-stat">
                  <span className="collection-stat-value">{collectionStats.unlocked_artifacts}</span>
                  <span className="collection-stat-label">Collected</span>
                </div>
                <div className="collection-stat">
                  <span className="collection-stat-value">{collectionStats.completion_percentage}%</span>
                  <span className="collection-stat-label">Complete</span>
                </div>
              </div>
            )}
          </div>

          <div className="artifact-categories">
            {[
              { id: 'all', label: 'All' },
              { id: 'mask', label: 'Masks' },
              { id: 'textile', label: 'Textiles' },
              { id: 'instrument', label: 'Instruments' },
              { id: 'sculpture', label: 'Sculptures' },
              { id: 'jewelry', label: 'Jewelry' },
              { id: 'pottery', label: 'Pottery' },
              { id: 'document', label: 'Documents' }
            ].map(cat => (
              <button
                key={cat.id}
                className={`artifact-category-btn ${selectedArtifactCategory === cat.id ? 'active' : ''}`}
                onClick={() => {
                  setSelectedArtifactCategory(cat.id);
                  loadArtifacts(cat.id === 'all' ? undefined : cat.id);
                }}
              >
                {cat.label}
              </button>
            ))}
          </div>

          {isMuseumLoading ? (
            <div className="view-loading">
              <div className="loading-spinner" />
              <span>Loading artifacts...</span>
            </div>
          ) : artifacts.length === 0 ? (
            <div className="empty-state">
              <Building2 size={64} />
              <h3>No Artifacts Yet</h3>
              <p>Complete quests and modules to discover cultural treasures!</p>
            </div>
          ) : (
            <div className="artifacts-grid">
              {artifacts.map((artifact, index) => (
                <motion.div
                  key={artifact.artifact.id}
                  className={`artifact-card ${!artifact.is_unlocked ? 'locked' : ''}`}
                  initial={{ opacity: 0, scale: 0.9 }}
                  animate={{ opacity: 1, scale: 1 }}
                  transition={{ delay: index * 0.03 }}
                  whileHover={{ scale: artifact.is_unlocked ? 1.03 : 1 }}
                  onClick={() => artifact.is_unlocked && setSelectedArtifact(artifact)}
                >
                  {artifact.is_new && <span className="artifact-new-badge">New!</span>}
                  
                  {artifact.is_unlocked && (
                    <button 
                      className={`artifact-favorite-btn ${artifact.is_favorite ? 'favorited' : ''}`}
                      onClick={(e) => {
                        e.stopPropagation();
                        // Toggle favorite
                        invoke('toggle_artifact_favorite', { 
                          userId: 1, 
                          artifactId: artifact.artifact.id 
                        });
                      }}
                    >
                      <Heart size={16} fill={artifact.is_favorite ? 'currentColor' : 'none'} />
                    </button>
                  )}

                  <div 
                    className="artifact-image"
                    style={{ background: `linear-gradient(135deg, ${artifact.artifact.color_primary}20, ${artifact.artifact.color_secondary}20)` }}
                  >
                    {artifact.artifact.category === 'mask' && '🎭'}
                    {artifact.artifact.category === 'textile' && '🧵'}
                    {artifact.artifact.category === 'instrument' && '🥁'}
                    {artifact.artifact.category === 'sculpture' && '🗿'}
                    {artifact.artifact.category === 'jewelry' && '💎'}
                    {artifact.artifact.category === 'pottery' && '🏺'}
                    {artifact.artifact.category === 'document' && '📜'}
                    
                    {!artifact.is_unlocked && (
                      <div className="artifact-lock-overlay">
                        <Lock size={32} />
                        <span className="artifact-lock-text">
                          {artifact.artifact.unlock_type === 'module' && 'Complete Module'}
                          {artifact.artifact.unlock_type === 'quest' && 'Complete Quest'}
                          {artifact.artifact.unlock_type === 'purchase' && `${artifact.artifact.cowrie_cost} Cowries`}
                          {artifact.artifact.unlock_type === 'achievement' && 'Earn Achievement'}
                        </span>
                      </div>
                    )}
                  </div>

                  <div className="artifact-info">
                    <h4 className="artifact-name">{artifact.artifact.name}</h4>
                    <p className="artifact-description">{artifact.artifact.description}</p>
                    <div className="artifact-meta">
                      <span className={`artifact-rarity ${artifact.artifact.rarity}`}>
                        {artifact.artifact.rarity}
                      </span>
                      {artifact.artifact.state_id && (
                        <span className="artifact-state">
                          <MapPin size={12} />
                          {artifact.artifact.state_id}
                        </span>
                      )}
                    </div>
                  </div>
                </motion.div>
              ))}
            </div>
          )}
        </motion.main>
      )}

      {/* Bottom Navigation */}
      <nav className="bottom-nav">
        {[
          { id: 'map' as MainView, icon: Map, label: 'Explore' },
          { id: 'quests' as MainView, icon: Target, label: 'Quests' },
          { id: 'character' as MainView, icon: User, label: 'Character' },
          { id: 'museum' as MainView, icon: Building2, label: 'Museum' }
        ].map(item => (
          <motion.button
            key={item.id}
            className={`nav-item ${mainView === item.id ? 'active' : ''}`}
            onClick={() => {
              playSound('click');
              handleViewChange(item.id);
            }}
            whileTap={{ scale: 0.9 }}
          >
            <div className="nav-item-icon">
              <item.icon size={22} />
            </div>
            <span className="nav-item-label">{item.label}</span>
          </motion.button>
        ))}
      </nav>

      {/* Floating Action Button - Only show on map view */}
      {mainView === 'map' && (
        <motion.button
          className="fab-button"
          onClick={() => showInventory ? handleCloseInventory() : handleOpenInventory()}
          whileHover={{ scale: 1.1 }}
          whileTap={{ scale: 0.9 }}
          initial={{ opacity: 0, scale: 0 }}
          animate={{ opacity: 1, scale: 1 }}
          transition={{ delay: 0.5, type: "spring" }}
        >
          <Backpack size={24} />
          <span className="fab-tooltip">Backpack</span>
        </motion.button>
      )}

      {/* Explore Screen - Encarta-Inspired Design */}
      <AnimatePresence>
        {selectedState && (
          <motion.div 
            className="encarta-explore"
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
          >
            {/* Sticky Navigation */}
            <motion.nav 
              className="encarta-nav"
              initial={{ y: -60 }}
              animate={{ y: 0 }}
              transition={{ delay: 0.2 }}
            >
              <button className="nav-back" onClick={handleCloseModal}>
                <ChevronLeft size={20} />
                <span>Back to Map</span>
              </button>
              <div className="nav-breadcrumb">
                <span>Nigeria</span>
                <ChevronRight size={14} />
                <span className="current">{selectedState.state.name}</span>
              </div>
              <div className="nav-progress">
                {[...Array(3)].map((_, i) => (
                  <div 
                    key={i} 
                    className={`progress-pip ${i < (selectedState.progress?.stars || 0) ? 'filled' : ''}`}
                  />
                ))}
              </div>
            </motion.nav>

            {/* Hero Article Header */}
            <header className="encarta-header">
              <motion.div 
                className="header-illustration"
                initial={{ scale: 1.1, opacity: 0 }}
                animate={{ scale: 1, opacity: 1 }}
                transition={{ duration: 0.8 }}
              >
                {/* Hero Image - tries hero image first, then landmark, then pattern fallback */}
                {(() => {
                  const stateLower = selectedState.state.name.toLowerCase().replace(/\s+/g, '-');
                  const heroImage = `/assets/images/${stateLower}-hero.png`;
                  const landmarkImage = selectedState.state.landmark_image;
                  return (
                    <img 
                      src={heroImage}
                      alt={`${selectedState.state.name} hero`}
                      className="hero-image"
                      onError={(e) => {
                        const img = e.currentTarget;
                        if (landmarkImage && img.src !== landmarkImage) {
                          img.src = landmarkImage;
                        } else {
                          img.style.display = 'none';
                          const fallback = img.parentElement?.querySelector('.illustration-pattern');
                          if (fallback) (fallback as HTMLElement).style.display = 'flex';
                        }
                      }}
                    />
                  );
                })()}
                <div className="illustration-pattern" style={{ display: 'none' }}>
                  {/* Decorative Nigerian pattern fallback */}
                  <svg viewBox="0 0 100 100" className="adire-pattern">
                    <defs>
                      <pattern id="adire" patternUnits="userSpaceOnUse" width="20" height="20">
                        <circle cx="10" cy="10" r="3" fill="currentColor" opacity="0.3"/>
                        <circle cx="0" cy="0" r="2" fill="currentColor" opacity="0.2"/>
                        <circle cx="20" cy="20" r="2" fill="currentColor" opacity="0.2"/>
                      </pattern>
                    </defs>
                    <rect width="100%" height="100%" fill="url(#adire)"/>
                  </svg>
                </div>
                <div className="illustration-overlay" />
                
                {/* Header Content - Now inside the illustration */}
                <div className="header-content-overlay">
                  <motion.div 
                    className="header-badge"
                    initial={{ scale: 0 }}
                    animate={{ scale: 1 }}
                    transition={{ delay: 0.3, type: "spring" }}
                  >
                    <MapPin size={16} />
                    <span>{selectedState.state.region}</span>
                  </motion.div>
                  
                  <motion.h1
                    initial={{ opacity: 0, y: 20 }}
                    animate={{ opacity: 1, y: 0 }}
                    transition={{ delay: 0.4 }}
                  >
                    {selectedState.state.name}
                  </motion.h1>
                  
                  {selectedState.state.description && (
                    <motion.p 
                      className="header-intro"
                      initial={{ opacity: 0 }}
                      animate={{ opacity: 1 }}
                      transition={{ delay: 0.5 }}
                    >
                      {selectedState.state.description}
                    </motion.p>
                  )}
                </div>
              </motion.div>
            </header>

            {/* Main Content Area */}
            <main className="encarta-main">
              {/* Did You Know Sidebar */}
              {selectedState.state.fun_fact && (
                <motion.aside 
                  className="encarta-sidebar"
                  initial={{ opacity: 0, x: -20 }}
                  animate={{ opacity: 1, x: 0 }}
                  transition={{ delay: 0.6 }}
                >
                  <div className="sidebar-card did-you-know">
                    <div className="card-icon">
                      <Sparkles size={24} />
                    </div>
                    <h3>Did You Know?</h3>
                    <p>{selectedState.state.fun_fact}</p>
                  </div>
                </motion.aside>
              )}

              {/* Learning Modules */}
              <section className="encarta-content">
                <motion.div 
                  className="section-title"
                  initial={{ opacity: 0, y: 10 }}
                  animate={{ opacity: 1, y: 0 }}
                  transition={{ delay: 0.5 }}
                >
                  <div className="title-icon">
                    <BookOpen size={24} />
                  </div>
                  <div>
                    <h2>Learning Adventures</h2>
                    <p>Choose a module to begin your journey</p>
                  </div>
                </motion.div>

                {isModalLoading ? (
                  <div className="encarta-loading">
                    <motion.div 
                      className="loading-book"
                      animate={{ rotateY: [0, 180, 360] }}
                      transition={{ duration: 2, repeat: Infinity, ease: "easeInOut" }}
                    >
                      <BookOpen size={48} />
                    </motion.div>
                    <p>Preparing your adventure...</p>
                  </div>
                ) : moduleContent && moduleContent.length > 0 ? (
                  <div className="module-articles">
                    {moduleContent.map((item, index) => {
                      const subjectIcons: Record<string, React.ReactNode> = {
                        'Social Studies': <Globe size={28} />,
                        'Mathematics': <Trophy size={28} />,
                        'Logic & Coding': <Compass size={28} />,
                        'Science': <Sparkles size={28} />,
                        'History': <Crown size={28} />,
                        'Language': <MessageCircle size={28} />,
                      };
                      const subjectColors: Record<string, string> = {
                        'Social Studies': '#E85D04',
                        'Mathematics': '#2D6A4F',
                        'Logic & Coding': '#7B2CBF',
                        'Science': '#0077B6',
                        'History': '#9D4EDD',
                        'Language': '#E63946',
                      };
                      const color = subjectColors[item.module.subject] || '#00C896';
                      const icon = subjectIcons[item.module.subject] || <BookOpen size={28} />;
                      
                      return (
                        <motion.article 
                          key={item.module.id}
                          className={`module-article ${!item.is_unlocked ? 'locked' : ''} ${item.progress?.is_completed ? 'completed' : ''}`}
                          initial={{ opacity: 0, y: 30 }}
                          animate={{ opacity: 1, y: 0 }}
                          transition={{ delay: 0.6 + index * 0.15 }}
                          whileHover={item.is_unlocked ? { y: -8, transition: { duration: 0.2 } } : {}}
                          onClick={() => item.is_unlocked && handleStartModule(item.module)}
                          style={{ '--module-color': color } as React.CSSProperties}
                        >
                          {/* Article Header */}
                          <div className="article-header">
                            <div className="article-icon" style={{ background: color }}>
                              {item.progress?.is_completed ? (
                                <CheckCircle size={28} />
                              ) : !item.is_unlocked ? (
                                <Lock size={24} />
                              ) : (
                                icon
                              )}
                            </div>
                            <span className="article-subject" style={{ color }}>
                              {item.module.subject}
                            </span>
                            {item.progress?.is_completed && (
                              <span className="completed-tag">✓ Completed</span>
                            )}
                          </div>

                          {/* Article Body */}
                          <div className="article-body">
                            <h3>{item.module.title}</h3>
                            {item.module.description && (
                              <p className="article-excerpt">{item.module.description}</p>
                            )}
                            
                            {/* Context Preview */}
                            {item.context?.intro_text && (
                              <p className="article-preview">
                                {item.context.intro_text.substring(0, 100)}...
                              </p>
                            )}
                          </div>

                          {/* Article Footer */}
                          <div className="article-footer">
                            <div className="article-meta">
                              {item.module.estimated_time && (
                                <span className="meta-item">
                                  <Clock size={14} />
                                  {item.module.estimated_time} min
                                </span>
                              )}
                              <span className="meta-item xp">
                                <Star size={14} />
                                {item.module.total_xp} XP
                              </span>
                            </div>
                            
                            {item.is_unlocked ? (
                              <motion.button 
                                className="article-cta"
                                whileHover={{ scale: 1.05 }}
                                whileTap={{ scale: 0.95 }}
                                style={{ background: color }}
                              >
                                {item.progress && item.progress.attempts > 0 && !item.progress.is_completed 
                                  ? 'Continue Learning' 
                                  : 'Start Adventure'}
                                <ChevronRight size={18} />
                              </motion.button>
                            ) : (
                              <div className="locked-notice">
                                <Lock size={14} />
                                <span>Reach Level {item.module.required_level} to unlock</span>
                              </div>
                            )}
                          </div>

                          {/* Progress indicator */}
                          {item.progress && item.progress.attempts > 0 && !item.progress.is_completed && (
                            <div className="article-progress">
                              <div 
                                className="progress-bar" 
                                style={{ 
                                  width: `${Math.min((item.progress.total_xp_earned / item.module.total_xp) * 100, 100)}%`,
                                  background: color 
                                }}
                              />
                            </div>
                          )}
                        </motion.article>
                      );
                    })}
                  </div>
                ) : (
                  <motion.div 
                    className="coming-soon-card"
                    initial={{ opacity: 0, scale: 0.9 }}
                    animate={{ opacity: 1, scale: 1 }}
                    transition={{ delay: 0.5 }}
                  >
                    <motion.div 
                      className="coming-soon-icon"
                      animate={{ rotate: [0, 10, -10, 0] }}
                      transition={{ duration: 3, repeat: Infinity }}
                    >
                      📚
                    </motion.div>
                    <h3>Coming Soon!</h3>
                    <p>We're crafting amazing learning adventures for {selectedState.state.name}.</p>
                    <p className="subtitle">Check back soon for new content!</p>
                  </motion.div>
                )}
              </section>
            </main>

            {/* Footer */}
            <footer className="encarta-footer">
              <p>© Sabi Quest • Learn • Explore • Discover</p>
            </footer>
          </motion.div>
        )}
      </AnimatePresence>

      {/* Quiz Screen */}
      <AnimatePresence>
        {activeQuiz && (
          <motion.div 
            className="quiz-screen"
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
          >
            {!quizCompleted ? (
              <>
                {/* Quiz Header */}
                <header className="quiz-header">
                  <button className="quiz-exit" onClick={handleExitQuiz} aria-label="Exit quiz">
                    <X size={20} />
                  </button>
                  <div className="quiz-info">
                    <span className="quiz-module-title">{activeQuiz.module.title}</span>
                    <span className="quiz-level-title">
                      {activeQuiz.levels[currentLevelIndex]?.level.title}
                    </span>
                  </div>
                  <div className="quiz-progress-indicator">
                    <span>
                      {currentQuestionIndex + 1} / {activeQuiz.levels[currentLevelIndex]?.questions.length || 0}
                    </span>
                  </div>
                </header>

                {/* Progress Bar */}
                <div className="quiz-progress-bar">
                  <motion.div 
                    className="quiz-progress-fill"
                    initial={{ width: 0 }}
                    animate={{ 
                      width: `${((currentQuestionIndex + 1) / (activeQuiz.levels[currentLevelIndex]?.questions.length || 1)) * 100}%` 
                    }}
                    transition={{ duration: 0.3 }}
                  />
                </div>

                {/* Question Content */}
                {activeQuiz.levels[currentLevelIndex]?.questions[currentQuestionIndex] && (
                  <main className="quiz-content-wrapper">
                    {/* Answer Feedback Overlay */}
                    <AnimatePresence>
                      {answerFeedback && (
                        <motion.div 
                          className={`answer-feedback-overlay ${answerFeedback}`}
                          initial={{ opacity: 0, scale: 0.8 }}
                          animate={{ opacity: 1, scale: 1 }}
                          exit={{ opacity: 0, scale: 0.8 }}
                        >
                          <motion.div 
                            className="feedback-icon"
                            initial={{ scale: 0, rotate: -180 }}
                            animate={{ scale: 1, rotate: 0 }}
                            transition={{ type: 'spring', damping: 10 }}
                          >
                            {answerFeedback === 'correct' ? (
                              <CheckCircle size={80} />
                            ) : (
                              <X size={80} />
                            )}
                          </motion.div>
                        </motion.div>
                      )}
                    </AnimatePresence>

                    <div className="quiz-main-area">
                      <motion.div 
                        className={`question-card ${answerFeedback ? `shake-${answerFeedback}` : ''}`}
                        key={currentQuestionIndex}
                        initial={{ opacity: 0, x: 50 }}
                        animate={{ opacity: 1, x: 0 }}
                        exit={{ opacity: 0, x: -50 }}
                      >
                        <div className="question-number">
                          Question {currentQuestionIndex + 1}
                        </div>
                        <h2 className="question-text">
                          {activeQuiz.levels[currentLevelIndex].questions[currentQuestionIndex].question_text}
                        </h2>

                        {/* Answer Options */}
                        <div className="answer-options">
                          {activeQuiz.levels[currentLevelIndex].questions[currentQuestionIndex].options?.map((option, index) => {
                            const currentQuestion = activeQuiz.levels[currentLevelIndex].questions[currentQuestionIndex];
                            const isSelected = selectedAnswer === option.id;
                            const isCorrect = option.id === currentQuestion.correct_answer;
                            const showResult = showExplanation;
                            
                            let optionClass = 'answer-option';
                            if (showResult) {
                              if (isCorrect) optionClass += ' correct';
                              else if (isSelected) optionClass += ' incorrect';
                            } else if (isSelected) {
                              optionClass += ' selected';
                            }

                            return (
                              <motion.button
                                key={option.id}
                                className={optionClass}
                                onClick={() => handleAnswerSelect(option.id)}
                                disabled={showExplanation}
                                initial={{ opacity: 0, y: 20 }}
                                animate={{ 
                                  opacity: 1, 
                                  y: 0,
                                  scale: showResult && isCorrect ? [1, 1.05, 1] : 1
                                }}
                                transition={{ delay: index * 0.1 }}
                                whileHover={!showExplanation ? { scale: 1.02, x: 4 } : {}}
                                whileTap={!showExplanation ? { scale: 0.98 } : {}}
                              >
                                <span className="option-letter">
                                  {String.fromCharCode(65 + index)}
                                </span>
                                <span className="option-text">{option.text}</span>
                                {showResult && isCorrect && (
                                  <motion.span
                                    initial={{ scale: 0 }}
                                    animate={{ scale: 1 }}
                                    transition={{ type: 'spring', damping: 10 }}
                                  >
                                    <CheckCircle size={24} className="result-icon" />
                                  </motion.span>
                                )}
                                {showResult && isSelected && !isCorrect && (
                                  <motion.span
                                    initial={{ scale: 0 }}
                                    animate={{ scale: 1 }}
                                    transition={{ type: 'spring', damping: 10 }}
                                  >
                                    <X size={24} className="result-icon" />
                                  </motion.span>
                                )}
                              </motion.button>
                            );
                          })}
                        </div>
                      </motion.div>
                    </div>

                    {/* Robot Assistant Sidebar */}
                    <aside className="quiz-robot-sidebar">
                      <motion.div 
                        className="robot-container"
                        initial={{ opacity: 0, x: 50 }}
                        animate={{ opacity: 1, x: 0 }}
                        transition={{ delay: 0.3 }}
                      >
                        {/* Robot Character */}
                        <motion.div 
                          className={`robot-character ${showExplanation ? (selectedAnswer === activeQuiz.levels[currentLevelIndex].questions[currentQuestionIndex].correct_answer ? 'happy' : 'thinking') : 'idle'}`}
                          animate={{ 
                            y: [0, -4, 0],
                          }}
                          transition={{ 
                            duration: 3,
                            repeat: Infinity,
                            ease: 'easeInOut'
                          }}
                          whileHover={{ 
                            scale: 1.05,
                            transition: { duration: 0.2 }
                          }}
                          whileTap={{ 
                            scale: 0.95,
                            transition: { duration: 0.1 }
                          }}
                          onClick={() => {
                            playSound('pop');
                          }}
                          style={{ cursor: 'pointer' }}
                        >
                          <div className="robot-head">
                            <div className="robot-antenna">
                              <motion.div 
                                className="antenna-light"
                                animate={{ 
                                  opacity: [0.5, 1, 0.5],
                                  scale: [1, 1.2, 1]
                                }}
                                transition={{ duration: 1.5, repeat: Infinity }}
                              />
                            </div>
                            <div className="robot-face">
                              <motion.div 
                                className="robot-eyes"
                                animate={showExplanation && selectedAnswer === activeQuiz.levels[currentLevelIndex].questions[currentQuestionIndex].correct_answer ? {
                                  scaleY: [1, 0.1, 1]
                                } : {}}
                                transition={{ duration: 0.3 }}
                              >
                                <div className="robot-eye left">
                                  <div className="eye-pupil" />
                                </div>
                                <div className="robot-eye right">
                                  <div className="eye-pupil" />
                                </div>
                              </motion.div>
                              <motion.div 
                                className={`robot-mouth ${showExplanation ? (selectedAnswer === activeQuiz.levels[currentLevelIndex].questions[currentQuestionIndex].correct_answer ? 'happy' : 'thinking') : ''}`}
                              />
                            </div>
                          </div>
                          <div className="robot-body">
                            <div className="robot-chest">
                              <motion.div 
                                className="chest-light"
                                animate={{ 
                                  backgroundColor: showExplanation 
                                    ? (selectedAnswer === activeQuiz.levels[currentLevelIndex].questions[currentQuestionIndex].correct_answer 
                                      ? ['#00C896', '#00ff9d', '#00C896'] 
                                      : ['#ff6b6b', '#ff8787', '#ff6b6b'])
                                    : ['#4da6ff', '#80bfff', '#4da6ff']
                                }}
                                transition={{ duration: 1, repeat: Infinity }}
                              />
                            </div>
                          </div>
                        </motion.div>

                        {/* Robot Speech Bubble */}
                        <AnimatePresence mode="wait">
                          {!showExplanation ? (
                            <motion.div 
                              key="idle"
                              className="robot-speech idle"
                              initial={{ opacity: 0, y: 10 }}
                              animate={{ opacity: 1, y: 0 }}
                              exit={{ opacity: 0, y: -10 }}
                            >
                              <p>Take your time! Pick the answer you think is correct. 🤔</p>
                            </motion.div>
                          ) : (
                            <motion.div 
                              key="feedback"
                              className={`robot-speech ${selectedAnswer === activeQuiz.levels[currentLevelIndex].questions[currentQuestionIndex].correct_answer ? 'correct' : 'incorrect'}`}
                              initial={{ opacity: 0, y: 10, scale: 0.9 }}
                              animate={{ opacity: 1, y: 0, scale: 1 }}
                              exit={{ opacity: 0, y: -10 }}
                            >
                              {robotMessage && <p className="robot-reaction">{robotMessage}</p>}
                              {activeQuiz.levels[currentLevelIndex].questions[currentQuestionIndex].explanation && (
                                <p className="robot-explanation">
                                  {activeQuiz.levels[currentLevelIndex].questions[currentQuestionIndex].explanation}
                                </p>
                              )}
                            </motion.div>
                          )}
                        </AnimatePresence>

                        {/* Robot Name Tag */}
                        <div className="robot-name">
                          <Zap size={12} />
                          <span>Naija Bot</span>
                        </div>
                      </motion.div>
                    </aside>
                  </main>
                )}

                {/* Quiz Footer Actions */}
                <footer className="quiz-footer">
                  {!showExplanation ? (
                    <motion.button
                      className="quiz-submit-btn"
                      onClick={handleSubmitAnswer}
                      disabled={!selectedAnswer}
                      whileHover={{ scale: 1.02 }}
                      whileTap={{ scale: 0.98 }}
                    >
                      Check Answer
                    </motion.button>
                  ) : (
                    <motion.button
                      className="quiz-next-btn"
                      onClick={handleNextQuestion}
                      whileHover={{ scale: 1.02 }}
                      whileTap={{ scale: 0.98 }}
                    >
                      {currentQuestionIndex >= (activeQuiz.levels[currentLevelIndex]?.questions.length || 0) - 1
                        ? 'Complete Level'
                        : 'Next Question'}
                      <ChevronRight size={20} />
                    </motion.button>
                  )}
                </footer>
              </>
            ) : (
              /* Level Complete Screen */
              <motion.div 
                className="level-complete"
                initial={{ opacity: 0, scale: 0.9 }}
                animate={{ opacity: 1, scale: 1 }}
                style={{
                  backgroundImage: selectedState?.state.name 
                    ? `linear-gradient(135deg, rgba(0,0,0,0.5) 0%, rgba(0,0,0,0.3) 50%, rgba(0,0,0,0.5) 100%), url(/assets/images/${selectedState.state.name.toLowerCase().replace(/\s+/g, '-')}-wallpaper.png)`
                    : 'none',
                  backgroundSize: 'cover',
                  backgroundPosition: 'center',
                  backgroundRepeat: 'no-repeat',
                  backgroundColor: '#2D5016',
                }}
              >
                {/* Confetti celebration for passed levels */}
                <Confetti 
                  isActive={showCelebration && levelResult?.passed === true} 
                  duration={4000}
                  pieceCount={150}
                  onComplete={() => setShowCelebration(false)}
                />
                
                <motion.div 
                  className="complete-celebration"
                  animate={{ rotate: [0, 5, -5, 0] }}
                  transition={{ duration: 0.5, repeat: 3 }}
                >
                  {levelResult?.stars_earned === 3 ? '🌟' : levelResult?.stars_earned === 2 ? '⭐' : '✨'}
                </motion.div>
                
                <h1 className="complete-title">
                  {levelResult?.passed ? 'Level Complete!' : 'Keep Practicing!'}
                </h1>
                
                <div className="complete-stars">
                  {[...Array(3)].map((_, i) => (
                    <motion.div
                      key={i}
                      initial={{ scale: 0, rotate: -180 }}
                      animate={{ scale: 1, rotate: 0 }}
                      transition={{ delay: 0.2 + i * 0.2, type: 'spring' }}
                    >
                      <Star 
                        size={48} 
                        className={i < (levelResult?.stars_earned || 0) ? 'star-earned' : 'star-empty'}
                        fill={i < (levelResult?.stars_earned || 0) ? '#FFD700' : 'none'}
                      />
                    </motion.div>
                  ))}
                </div>

                <div className="complete-stats">
                  <div className="stat-item">
                    <span className="stat-label">Correct</span>
                    <span className="stat-value">
                      {levelResult?.correct_answers}/{levelResult?.total_questions}
                    </span>
                  </div>
                  <div className="stat-item xp">
                    <span className="stat-label">XP Earned</span>
                    <span className="stat-value">+{levelResult?.xp_earned}</span>
                  </div>
                </div>

                {levelResult?.is_new_best && (
                  <motion.div 
                    className="new-best-badge"
                    initial={{ scale: 0 }}
                    animate={{ scale: 1 }}
                    transition={{ delay: 0.5, type: 'spring' }}
                  >
                    🏆 New Best Score!
                  </motion.div>
                )}

                <div className="complete-actions">
                  <motion.button
                    className="btn-secondary"
                    onClick={handleExitQuiz}
                    whileHover={{ scale: 1.02 }}
                    whileTap={{ scale: 0.98 }}
                  >
                    Back to Modules
                  </motion.button>
                  {currentLevelIndex < activeQuiz.levels.length - 1 && levelResult?.passed && (
                    <motion.button
                      className="btn-primary"
                      onClick={handleContinueToNextLevel}
                      whileHover={{ scale: 1.02 }}
                      whileTap={{ scale: 0.98 }}
                    >
                      Next Level
                      <ChevronRight size={20} />
                    </motion.button>
                  )}
                </div>
              </motion.div>
            )}
          </motion.div>
        )}
      </AnimatePresence>

      {/* Inventory Modal (Placeholder) */}
      <AnimatePresence>
        {showInventory && (
          <>
            <motion.div 
              className="modal-overlay"
              variants={overlayVariants}
              initial="hidden"
              animate="visible"
              exit="exit"
              onClick={handleCloseInventory}
            />
            <motion.div 
              className="inventory-modal"
              variants={modalVariants}
              initial="hidden"
              animate="visible"
              exit="exit"
            >
              <button className="modal-close" onClick={handleCloseInventory} aria-label="Close inventory">
                <X size={24} />
              </button>
              <h2><Backpack size={28} /> Your Backpack</h2>
              <div className="inventory-empty">
                <Sparkles size={48} />
                <p>Your backpack is empty!</p>
                <p className="hint">Complete lessons to collect cultural artifacts.</p>
              </div>
            </motion.div>
          </>
        )}
      </AnimatePresence>

      {/* ============================================ */}
      {/* THE SABI CODEX - Encyclopedia Modal */}
      {/* ============================================ */}
      <AnimatePresence>
        {showCodex && (
          <div 
            className="codex-wrapper"
            onWheel={(e) => e.stopPropagation()}
            onTouchMove={(e) => {
              // Only prevent if not scrolling inside the entries grid
              const target = e.target as HTMLElement;
              if (!target.closest('.codex-entries-grid') && !target.closest('.entry-content')) {
                e.preventDefault();
              }
            }}
          >
            <motion.div 
              className="codex-overlay"
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              exit={{ opacity: 0 }}
              onClick={handleCloseCodex}
            />
            <motion.div 
              className="codex-modal"
              initial={{ opacity: 0, scale: 0.9, y: 30 }}
              animate={{ opacity: 1, scale: 1, y: 0 }}
              exit={{ opacity: 0, scale: 0.95, y: 20 }}
              transition={{ type: 'spring', damping: 25, stiffness: 300 }}
            >
              {/* Codex Header */}
              <div className="codex-header">
                <div className="codex-title-area">
                  <Book size={32} className="codex-icon" />
                  <div>
                    <h1 className="codex-title">The Sabi Codex</h1>
                    <p className="codex-subtitle">Encyclopedia of Nigerian Knowledge</p>
                  </div>
                </div>
                <button 
                  className="codex-close-btn" 
                  onClick={handleCloseCodex}
                  aria-label="Close Codex"
                >
                  <X size={24} />
                </button>
              </div>

              {/* Codex Stats Bar */}
              {codexStats && (
                <div className="codex-stats-bar">
                  <div className="codex-stat">
                    <BookOpen size={16} />
                    <span>{codexStats.read_entries}/{codexStats.total_entries} Read</span>
                  </div>
                  <div className="codex-stat">
                    <Lock size={16} />
                    <span>{codexStats.unlocked_entries} Unlocked</span>
                  </div>
                  <div className="codex-stat">
                    <Bookmark size={16} />
                    <span>{codexStats.bookmarked_entries} Saved</span>
                  </div>
                </div>
              )}

              {/* Codex Content */}
              <div className="codex-content">
                {/* Category Tabs */}
                <div className="codex-categories">
                  {[
                    { id: 'all', label: 'All', icon: LayoutGrid },
                    { id: 'folklore', label: 'Folklore', icon: Ghost },
                    { id: 'history', label: 'History', icon: ScrollText },
                    { id: 'famous_nigerians', label: 'Famous Nigerians', icon: Award },
                    { id: 'culture', label: 'Culture', icon: Drum },
                  ].map((cat) => (
                    <motion.button
                      key={cat.id}
                      className={`codex-category-btn ${selectedCodexCategory === cat.id ? 'active' : ''}`}
                      onClick={() => handleSelectCodexCategory(cat.id)}
                      whileHover={{ scale: 1.02 }}
                      whileTap={{ scale: 0.98 }}
                    >
                      <cat.icon size={18} />
                      <span>{cat.label}</span>
                    </motion.button>
                  ))}
                </div>

                {/* Entry Reading View */}
                {selectedCodexEntry ? (
                  <motion.div 
                    className="codex-entry-view"
                    initial={{ opacity: 0, x: 20 }}
                    animate={{ opacity: 1, x: 0 }}
                    exit={{ opacity: 0, x: -20 }}
                  >
                    <div className="entry-header">
                      <motion.button 
                        className="entry-back-btn"
                        onClick={handleCloseCodexEntry}
                        whileHover={{ x: -3 }}
                        whileTap={{ scale: 0.95 }}
                      >
                        <ArrowLeft size={20} />
                        <span>Back to entries</span>
                      </motion.button>
                      
                      <motion.button
                        className={`entry-bookmark-btn ${selectedCodexEntry.progress?.is_bookmarked ? 'bookmarked' : ''}`}
                        onClick={() => handleToggleBookmark(selectedCodexEntry.entry.id)}
                        whileHover={{ scale: 1.1 }}
                        whileTap={{ scale: 0.9 }}
                      >
                        <Bookmark size={20} fill={selectedCodexEntry.progress?.is_bookmarked ? 'currentColor' : 'none'} />
                      </motion.button>
                    </div>

                    <div className="entry-content">
                      {/* Entry Image */}
                      {selectedCodexEntry.entry.image_url && (
                        <div className="entry-image">
                          <img 
                            src={selectedCodexEntry.entry.image_url} 
                            alt={selectedCodexEntry.entry.title}
                            onError={(e) => {
                              e.currentTarget.style.display = 'none';
                            }}
                          />
                        </div>
                      )}

                      {/* Entry Title */}
                      <div className="entry-title-section">
                        <span className="entry-category-badge">{selectedCodexEntry.entry.category.replace('_', ' ')}</span>
                        <h2 className="entry-title">{selectedCodexEntry.entry.title}</h2>
                        {selectedCodexEntry.entry.subtitle && (
                          <p className="entry-subtitle">{selectedCodexEntry.entry.subtitle}</p>
                        )}
                        <div className="entry-meta">
                          {selectedCodexEntry.entry.reading_time && (
                            <span className="meta-item">
                              <Clock size={14} />
                              {selectedCodexEntry.entry.reading_time} min read
                            </span>
                          )}
                          <span className="meta-item xp">
                            <Star size={14} />
                            +{selectedCodexEntry.entry.xp_reward} XP
                          </span>
                        </div>
                      </div>

                      {/* Markdown Content */}
                      <div 
                        className="entry-markdown"
                        dangerouslySetInnerHTML={{ 
                          __html: parseMarkdownSimple(selectedCodexEntry.entry.content_md) 
                        }}
                        onClick={() => {
                          // Mark as read when user interacts with content
                          if (!selectedCodexEntry.progress?.is_read) {
                            handleMarkEntryRead(selectedCodexEntry.entry.id);
                          }
                        }}
                      />

                      {/* Mark as Read Button */}
                      {!selectedCodexEntry.progress?.is_read && (
                        <motion.button
                          className="mark-read-btn"
                          onClick={() => handleMarkEntryRead(selectedCodexEntry.entry.id)}
                          whileHover={{ scale: 1.02 }}
                          whileTap={{ scale: 0.98 }}
                        >
                          <CheckCircle size={20} />
                          <span>Mark as Read (+{selectedCodexEntry.entry.xp_reward} XP)</span>
                        </motion.button>
                      )}
                    </div>
                  </motion.div>
                ) : (
                  /* Entry Grid */
                  <div className="codex-entries-grid">
                    {isCodexLoading ? (
                      <div className="codex-loading">
                        <motion.div
                          animate={{ rotate: 360 }}
                          transition={{ duration: 1, repeat: Infinity, ease: 'linear' }}
                        >
                          <Book size={32} />
                        </motion.div>
                        <p>Loading entries...</p>
                      </div>
                    ) : codexEntries.length === 0 ? (
                      <div className="codex-empty">
                        <Book size={48} />
                        <p>No entries found in this category.</p>
                        <p className="hint">Complete quizzes and explore states to unlock more!</p>
                      </div>
                    ) : (
                      codexEntries.map((item, index) => (
                        <motion.div
                          key={item.entry.id}
                          className={`codex-entry-card ${item.is_accessible ? 'accessible' : 'locked'} ${item.progress?.is_read ? 'read' : ''}`}
                          initial={{ opacity: 0, y: 20 }}
                          animate={{ opacity: 1, y: 0 }}
                          transition={{ delay: index * 0.05 }}
                          whileHover={item.is_accessible ? { y: -4, scale: 1.02 } : undefined}
                          whileTap={item.is_accessible ? { scale: 0.98 } : undefined}
                          onClick={() => handleSelectCodexEntry(item)}
                        >
                          {/* Entry Card Image */}
                          <div className="entry-card-image">
                            {item.entry.image_url ? (
                              <img 
                                src={item.entry.image_url} 
                                alt={item.entry.title}
                                onError={(e) => {
                                  e.currentTarget.style.display = 'none';
                                  const placeholder = e.currentTarget.nextElementSibling as HTMLElement;
                                  if (placeholder) placeholder.style.display = 'flex';
                                }}
                              />
                            ) : null}
                            <div 
                              className={`entry-card-placeholder ${item.entry.category || 'default'}`} 
                              style={{ display: item.entry.image_url ? 'none' : 'flex' }}
                            >
                              {item.entry.category === 'folklore' && <Ghost size={36} />}
                              {item.entry.category === 'history' && <ScrollText size={36} />}
                              {item.entry.category === 'famous_nigerians' && <Award size={36} />}
                              {item.entry.category === 'culture' && <Drum size={36} />}
                              {!['folklore', 'history', 'famous_nigerians', 'culture'].includes(item.entry.category) && <Book size={36} />}
                            </div>
                            
                            {/* Lock Overlay */}
                            {!item.is_accessible && (
                              <div className="entry-card-lock">
                                <Lock size={24} />
                                <span>{item.entry.tier === 2 ? 'Locked' : 'Unlock Required'}</span>
                              </div>
                            )}
                            
                            {/* Read Badge */}
                            {item.progress?.is_read && (
                              <div className="entry-card-read-badge">
                                <CheckCircle size={16} />
                              </div>
                            )}
                            
                            {/* Bookmark Badge */}
                            {item.progress?.is_bookmarked && (
                              <div className="entry-card-bookmark-badge">
                                <Bookmark size={14} fill="currentColor" />
                              </div>
                            )}
                          </div>
                          
                          {/* Entry Card Info */}
                          <div className="entry-card-info">
                            <span className={`entry-card-category ${item.entry.category || 'default'}`}>
                              {item.entry.category.replace('_', ' ')}
                            </span>
                            <h4 className="entry-card-title">{item.entry.title}</h4>
                            {item.entry.summary && (
                              <p className="entry-card-summary">{item.entry.summary}</p>
                            )}
                            <div className="entry-card-footer">
                              {item.entry.reading_time && (
                                <span className="entry-card-time">
                                  <Clock size={12} />
                                  {item.entry.reading_time} min
                                </span>
                              )}
                              {item.entry.associated_state && (
                                <span className="entry-card-state">
                                  <MapPin size={12} />
                                  {item.entry.associated_state}
                                </span>
                              )}
                            </div>
                          </div>
                        </motion.div>
                      ))
                    )}
                  </div>
                )}
              </div>
            </motion.div>
          </div>
        )}
      </AnimatePresence>

      {/* Journey Map - State Completion Screen */}
      <JourneyMap
        isVisible={showJourneyMap}
        fromState={journeyData?.fromState || { id: '', name: '', region: '' }}
        toState={journeyData?.toState || null}
        rewards={journeyData?.rewards || { stars: 0, xp: 0, items: [], badges: [] }}
        avatar={userAvatar ? {
          skin_tone: userAvatar.skin_tone,
          hairstyle: userAvatar.hairstyle,
          outfit: userAvatar.outfit,
          accessory: userAvatar.accessory
        } : null}
        onContinue={handleJourneyContinue}
        onReturnHome={handleJourneyReturnHome}
      />
    </div>
  );
}

// Simple markdown parser (basic support)
function parseMarkdownSimple(md: string): string {
  if (!md) return '';
  
  let html = md
    // Headers
    .replace(/^### (.*$)/gim, '<h3>$1</h3>')
    .replace(/^## (.*$)/gim, '<h2>$1</h2>')
    .replace(/^# (.*$)/gim, '<h1>$1</h1>')
    // Bold
    .replace(/\*\*(.*?)\*\*/gim, '<strong>$1</strong>')
    // Italic
    .replace(/\*(.*?)\*/gim, '<em>$1</em>')
    // Blockquotes
    .replace(/^> (.*$)/gim, '<blockquote>$1</blockquote>')
    // Horizontal rules
    .replace(/^---$/gim, '<hr />')
    // Unordered lists
    .replace(/^- (.*$)/gim, '<li>$1</li>')
    // Tables (basic)
    .replace(/\| (.*?) \|/gim, '<tr><td>$1</td></tr>')
    // Line breaks
    .replace(/\n\n/gim, '</p><p>')
    .replace(/\n/gim, '<br />');
  
  // Wrap in paragraph
  html = '<p>' + html + '</p>';
  
  // Clean up empty paragraphs
  html = html.replace(/<p><\/p>/g, '');
  html = html.replace(/<p><br \/><\/p>/g, '');
  
  return html;
}

export default App;
