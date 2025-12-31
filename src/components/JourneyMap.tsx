import { motion, AnimatePresence } from 'framer-motion';
import { Star, Trophy, Gift, ChevronRight, Home, Sparkles, MapPin } from 'lucide-react';
import ChibiAvatar from './ChibiAvatar';
import './JourneyMap.css';

interface JourneyMapProps {
  fromState: {
    id: string;
    name: string;
    region: string;
  };
  toState: {
    id: string;
    name: string;
    region: string;
    landmark_name?: string;
  } | null;
  rewards: {
    stars: number;
    xp: number;
    items: string[];
    badges: string[];
  };
  avatar: {
    skin_tone: string;
    hair_style: string;
    hair_color: string;
    outfit: string;
    accessories: string[];
  } | null;
  onContinue: () => void;
  onReturnHome: () => void;
  isVisible: boolean;
}

// Nigerian states with approximate map positions (percentage based)
const statePositions: Record<string, { x: number; y: number; region: string }> = {
  'ABJ': { x: 52, y: 48, region: 'North Central' },
  'LAG': { x: 28, y: 72, region: 'South West' },
  'KAN': { x: 55, y: 15, region: 'North West' },
  'RIV': { x: 48, y: 85, region: 'South South' },
  'OYO': { x: 30, y: 62, region: 'South West' },
  'KAD': { x: 52, y: 28, region: 'North West' },
  'ENE': { x: 58, y: 75, region: 'South East' },
  'ANA': { x: 45, y: 78, region: 'South East' },
  'BEN': { x: 38, y: 72, region: 'South South' },
  'NIG': { x: 42, y: 52, region: 'North Central' },
  'OGU': { x: 26, y: 68, region: 'South West' },
  'KWA': { x: 38, y: 55, region: 'North Central' },
  'BOR': { x: 78, y: 20, region: 'North East' },
  'SOK': { x: 38, y: 12, region: 'North West' },
  'PLA': { x: 58, y: 42, region: 'North Central' },
  'CRS': { x: 58, y: 82, region: 'South South' },
};

// Default position for unknown states
const defaultPosition = { x: 50, y: 50, region: 'Nigeria' };

const JourneyMap: React.FC<JourneyMapProps> = ({
  fromState,
  toState,
  rewards,
  avatar,
  onContinue,
  onReturnHome,
  isVisible
}) => {
  const fromPos = statePositions[fromState.id] || defaultPosition;
  const toPos = toState ? (statePositions[toState.id] || defaultPosition) : null;

  return (
    <AnimatePresence>
      {isVisible && (
        <motion.div
          className="journey-map-overlay"
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          exit={{ opacity: 0 }}
        >
          <motion.div
            className="journey-map-container"
            initial={{ scale: 0.8, opacity: 0 }}
            animate={{ scale: 1, opacity: 1 }}
            transition={{ delay: 0.2, type: 'spring', damping: 20 }}
          >
            {/* Header */}
            <motion.div 
              className="journey-header"
              initial={{ y: -20, opacity: 0 }}
              animate={{ y: 0, opacity: 1 }}
              transition={{ delay: 0.4 }}
            >
              <Sparkles className="sparkle-icon" />
              <h1>State Complete!</h1>
              <Sparkles className="sparkle-icon" />
            </motion.div>

            {/* Map Area */}
            <div className="journey-map-area">
              {/* Stylized Nigeria Map Background */}
              <svg viewBox="0 0 100 100" className="nigeria-map-svg">
                {/* Simplified Nigeria outline */}
                <path
                  className="nigeria-outline"
                  d="M25,65 Q20,55 25,45 Q28,35 35,25 Q45,15 55,12 Q65,10 75,18 Q82,25 85,35 Q88,45 85,55 Q82,65 78,75 Q72,82 65,85 Q55,88 45,85 Q35,82 28,75 Z"
                  fill="none"
                  stroke="var(--color-primary)"
                  strokeWidth="0.5"
                  strokeDasharray="2,1"
                />
                
                {/* State dots */}
                {Object.entries(statePositions).map(([id, pos]) => (
                  <motion.circle
                    key={id}
                    cx={pos.x}
                    cy={pos.y}
                    r={id === fromState.id || id === toState?.id ? 3 : 1.5}
                    className={`state-dot ${id === fromState.id ? 'completed' : ''} ${id === toState?.id ? 'next' : ''}`}
                    initial={{ scale: 0 }}
                    animate={{ scale: 1 }}
                    transition={{ delay: 0.5 + Math.random() * 0.3 }}
                  />
                ))}

                {/* Journey Path */}
                {toPos && (
                  <motion.path
                    d={`M${fromPos.x},${fromPos.y} Q${(fromPos.x + toPos.x) / 2},${Math.min(fromPos.y, toPos.y) - 10} ${toPos.x},${toPos.y}`}
                    className="journey-path"
                    initial={{ pathLength: 0 }}
                    animate={{ pathLength: 1 }}
                    transition={{ delay: 1.5, duration: 2, ease: "easeInOut" }}
                  />
                )}

                {/* Footsteps along path */}
                {toPos && (
                  <>
                    {[0.2, 0.4, 0.6, 0.8].map((t, i) => {
                      const x = fromPos.x + (toPos.x - fromPos.x) * t;
                      const y = fromPos.y + (toPos.y - fromPos.y) * t - Math.sin(t * Math.PI) * 10;
                      return (
                        <motion.text
                          key={i}
                          x={x}
                          y={y}
                          className="footstep"
                          initial={{ opacity: 0, scale: 0 }}
                          animate={{ opacity: 0.3, scale: 1 }}
                          transition={{ delay: 1.8 + i * 0.4 }}
                        >
                          👣
                        </motion.text>
                      );
                    })}
                  </>
                )}
              </svg>

              {/* State Labels */}
              <motion.div
                className="state-label from-state"
                style={{ left: `${fromPos.x}%`, top: `${fromPos.y}%` }}
                initial={{ scale: 0 }}
                animate={{ scale: 1 }}
                transition={{ delay: 0.6, type: 'spring' }}
              >
                <div className="state-marker completed">
                  <Star size={16} fill="#FFD700" />
                </div>
                <span className="state-name">{fromState.name}</span>
              </motion.div>

              {toState && toPos && (
                <motion.div
                  className="state-label to-state"
                  style={{ left: `${toPos.x}%`, top: `${toPos.y}%` }}
                  initial={{ scale: 0, opacity: 0 }}
                  animate={{ scale: 1, opacity: 1 }}
                  transition={{ delay: 3.5, type: 'spring' }}
                >
                  <motion.div 
                    className="state-marker next"
                    animate={{ scale: [1, 1.2, 1] }}
                    transition={{ repeat: Infinity, duration: 1.5 }}
                  >
                    <MapPin size={16} />
                  </motion.div>
                  <span className="state-name">{toState.name}</span>
                  <span className="state-subtitle">Next Adventure!</span>
                </motion.div>
              )}

              {/* Animated Avatar traveling */}
              <motion.div
                className="traveling-avatar"
                initial={{ 
                  left: `${fromPos.x}%`, 
                  top: `${fromPos.y}%`,
                  scale: 1
                }}
                animate={toPos ? { 
                  left: `${toPos.x}%`, 
                  top: `${toPos.y}%`,
                  scale: [1, 1.2, 1]
                } : {}}
                transition={{ 
                  delay: 1.5, 
                  duration: 2.5, 
                  ease: "easeInOut"
                }}
              >
                {avatar ? (
                  <ChibiAvatar
                    skinTone={avatar.skin_tone}
                    hairStyle={avatar.hair_style}
                    hairColor={avatar.hair_color}
                    outfit={avatar.outfit}
                    accessories={avatar.accessories}
                    size={60}
                  />
                ) : (
                  <div className="avatar-placeholder">🧑‍🎓</div>
                )}
              </motion.div>
            </div>

            {/* Rewards Section */}
            <motion.div
              className="rewards-section"
              initial={{ y: 50, opacity: 0 }}
              animate={{ y: 0, opacity: 1 }}
              transition={{ delay: 0.8 }}
            >
              <h2>Rewards Earned</h2>
              <div className="rewards-grid">
                <motion.div 
                  className="reward-card stars"
                  whileHover={{ scale: 1.05 }}
                  initial={{ x: -20, opacity: 0 }}
                  animate={{ x: 0, opacity: 1 }}
                  transition={{ delay: 1 }}
                >
                  <div className="reward-icon">
                    {[...Array(rewards.stars)].map((_, i) => (
                      <Star key={i} size={20} fill="#FFD700" color="#FFD700" />
                    ))}
                  </div>
                  <span className="reward-label">{rewards.stars} Stars</span>
                </motion.div>

                <motion.div 
                  className="reward-card xp"
                  whileHover={{ scale: 1.05 }}
                  initial={{ y: 20, opacity: 0 }}
                  animate={{ y: 0, opacity: 1 }}
                  transition={{ delay: 1.2 }}
                >
                  <div className="reward-icon">
                    <Trophy size={24} />
                  </div>
                  <span className="reward-value">+{rewards.xp}</span>
                  <span className="reward-label">XP Earned</span>
                </motion.div>

                {rewards.badges.length > 0 && (
                  <motion.div 
                    className="reward-card badges"
                    whileHover={{ scale: 1.05 }}
                    initial={{ x: 20, opacity: 0 }}
                    animate={{ x: 0, opacity: 1 }}
                    transition={{ delay: 1.4 }}
                  >
                    <div className="reward-icon">
                      <Gift size={24} />
                    </div>
                    <span className="reward-value">{rewards.badges.length}</span>
                    <span className="reward-label">New Badge{rewards.badges.length > 1 ? 's' : ''}</span>
                  </motion.div>
                )}
              </div>
            </motion.div>

            {/* Action Buttons */}
            <motion.div
              className="journey-actions"
              initial={{ y: 30, opacity: 0 }}
              animate={{ y: 0, opacity: 1 }}
              transition={{ delay: 4 }}
            >
              {toState ? (
                <motion.button
                  className="journey-btn primary"
                  onClick={onContinue}
                  whileHover={{ scale: 1.05 }}
                  whileTap={{ scale: 0.95 }}
                >
                  Continue to {toState.name}
                  <ChevronRight size={20} />
                </motion.button>
              ) : (
                <motion.button
                  className="journey-btn primary"
                  onClick={onReturnHome}
                  whileHover={{ scale: 1.05 }}
                  whileTap={{ scale: 0.95 }}
                >
                  <Sparkles size={20} />
                  All States Complete!
                </motion.button>
              )}
              
              <motion.button
                className="journey-btn secondary"
                onClick={onReturnHome}
                whileHover={{ scale: 1.02 }}
                whileTap={{ scale: 0.98 }}
              >
                <Home size={18} />
                Return to Map
              </motion.button>
            </motion.div>
          </motion.div>
        </motion.div>
      )}
    </AnimatePresence>
  );
};

export default JourneyMap;
