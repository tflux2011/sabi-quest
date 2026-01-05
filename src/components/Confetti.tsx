import React, { useEffect, useState, useCallback } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import './Confetti.css';

interface ConfettiPiece {
  id: number;
  x: number;
  y: number;
  rotation: number;
  scale: number;
  color: string;
  shape: 'square' | 'circle' | 'triangle' | 'star';
  delay: number;
  duration: number;
}

interface ConfettiProps {
  isActive: boolean;
  duration?: number; // How long the confetti shows (ms)
  pieceCount?: number;
  colors?: string[];
  onComplete?: () => void;
}

const defaultColors = [
  '#FFD700', // Gold
  '#FF6B35', // Orange
  '#00A86B', // Jade Green (Nigeria)
  '#FFFFFF', // White (Nigeria)
  '#FF1493', // Deep Pink
  '#00CED1', // Dark Turquoise
  '#FF4500', // Orange Red
  '#32CD32', // Lime Green
  '#FF69B4', // Hot Pink
  '#7B68EE', // Medium Slate Blue
];

const shapes = ['square', 'circle', 'triangle', 'star'] as const;

const Confetti: React.FC<ConfettiProps> = ({
  isActive,
  duration = 3000,
  pieceCount = 100,
  colors = defaultColors,
  onComplete
}) => {
  const [pieces, setPieces] = useState<ConfettiPiece[]>([]);
  const [isVisible, setIsVisible] = useState(false);

  const generatePieces = useCallback(() => {
    const newPieces: ConfettiPiece[] = [];
    for (let i = 0; i < pieceCount; i++) {
      newPieces.push({
        id: i,
        x: Math.random() * 100, // percentage across screen
        y: -10 - Math.random() * 20, // start above screen
        rotation: Math.random() * 360,
        scale: 0.5 + Math.random() * 1,
        color: colors[Math.floor(Math.random() * colors.length)],
        shape: shapes[Math.floor(Math.random() * shapes.length)],
        delay: Math.random() * 0.5,
        duration: 2 + Math.random() * 2,
      });
    }
    return newPieces;
  }, [pieceCount, colors]);

  useEffect(() => {
    if (isActive) {
      setIsVisible(true);
      setPieces(generatePieces());

      const timer = setTimeout(() => {
        setIsVisible(false);
        setPieces([]);
        onComplete?.();
      }, duration);

      return () => clearTimeout(timer);
    }
  }, [isActive, duration, generatePieces, onComplete]);

  const renderShape = (shape: ConfettiPiece['shape'], color: string) => {
    switch (shape) {
      case 'circle':
        return <div className="confetti-circle" style={{ backgroundColor: color }} />;
      case 'triangle':
        return (
          <div 
            className="confetti-triangle" 
            style={{ borderBottomColor: color }} 
          />
        );
      case 'star':
        return <span className="confetti-star" style={{ color }}>★</span>;
      default:
        return <div className="confetti-square" style={{ backgroundColor: color }} />;
    }
  };

  return (
    <AnimatePresence>
      {isVisible && (
        <div className="confetti-container">
          {pieces.map((piece) => (
            <motion.div
              key={piece.id}
              className="confetti-piece"
              initial={{
                left: `${piece.x}%`,
                top: `${piece.y}%`,
                rotate: piece.rotation,
                scale: piece.scale,
                opacity: 1,
              }}
              animate={{
                top: '110%',
                rotate: piece.rotation + 720,
                opacity: [1, 1, 1, 0],
              }}
              transition={{
                duration: piece.duration,
                delay: piece.delay,
                ease: 'linear',
              }}
            >
              {renderShape(piece.shape, piece.color)}
            </motion.div>
          ))}
          
          {/* Burst particles from center */}
          {[...Array(20)].map((_, i) => {
            const angle = (i / 20) * Math.PI * 2;
            const distance = 30 + Math.random() * 20;
            return (
              <motion.div
                key={`burst-${i}`}
                className="confetti-burst"
                style={{
                  backgroundColor: colors[i % colors.length],
                }}
                initial={{
                  left: '50%',
                  top: '40%',
                  scale: 0,
                  opacity: 1,
                }}
                animate={{
                  left: `calc(50% + ${Math.cos(angle) * distance}vw)`,
                  top: `calc(40% + ${Math.sin(angle) * distance}vh)`,
                  scale: [0, 1.5, 0],
                  opacity: [1, 1, 0],
                }}
                transition={{
                  duration: 1,
                  ease: 'easeOut',
                }}
              />
            );
          })}
        </div>
      )}
    </AnimatePresence>
  );
};

// Celebration sound effect - more elaborate than levelComplete
export const playCelebrationSound = () => {
  try {
    const audioContext = new (window.AudioContext || (window as any).webkitAudioContext)();
    
    const playNote = (freq: number, delay: number, duration: number, volume: number = 0.15, type: OscillatorType = 'sine') => {
      setTimeout(() => {
        const oscillator = audioContext.createOscillator();
        const gainNode = audioContext.createGain();
        
        oscillator.connect(gainNode);
        gainNode.connect(audioContext.destination);
        
        oscillator.frequency.value = freq;
        oscillator.type = type;
        
        gainNode.gain.setValueAtTime(volume, audioContext.currentTime);
        gainNode.gain.exponentialRampToValueAtTime(0.01, audioContext.currentTime + duration);
        
        oscillator.start(audioContext.currentTime);
        oscillator.stop(audioContext.currentTime + duration);
      }, delay);
    };
    
    // Triumphant fanfare melody
    // First phrase - ascending
    playNote(392, 0, 0.15, 0.12);      // G4
    playNote(440, 80, 0.15, 0.12);     // A4
    playNote(523, 160, 0.15, 0.12);    // C5
    playNote(587, 240, 0.2, 0.15);     // D5
    playNote(659, 320, 0.25, 0.15);    // E5
    
    // Second phrase - higher
    playNote(784, 500, 0.15, 0.15);    // G5
    playNote(880, 580, 0.15, 0.15);    // A5
    playNote(1047, 660, 0.4, 0.18);    // C6 (hold)
    
    // Sparkle effect
    playNote(1319, 900, 0.1, 0.08);    // E6
    playNote(1568, 950, 0.1, 0.08);    // G6
    playNote(2093, 1000, 0.15, 0.1);   // C7
    
    // Final chord (arpeggiated)
    playNote(523, 1200, 0.5, 0.1);     // C5
    playNote(659, 1250, 0.45, 0.1);    // E5
    playNote(784, 1300, 0.4, 0.1);     // G5
    playNote(1047, 1350, 0.6, 0.12);   // C6
    
  } catch (e) {
    // Audio not supported
  }
};

// Quick success chime
export const playSuccessChime = () => {
  try {
    const audioContext = new (window.AudioContext || (window as any).webkitAudioContext)();
    
    const playNote = (freq: number, delay: number, duration: number, volume: number = 0.12) => {
      setTimeout(() => {
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
      }, delay);
    };
    
    // Quick happy arpeggio
    playNote(523, 0, 0.1);     // C5
    playNote(659, 60, 0.1);    // E5
    playNote(784, 120, 0.1);   // G5
    playNote(1047, 180, 0.2);  // C6
    
  } catch (e) {
    // Audio not supported
  }
};

export default Confetti;
