import React from 'react';
import './ChibiAvatar.css';

interface ChibiAvatarProps {
  skinTone?: string;
  hairStyle?: string;
  hairColor?: string;
  outfit?: string;
  accessory?: string;
  expression?: 'happy' | 'neutral' | 'excited' | 'thinking';
  size?: number;
  className?: string;
  animated?: boolean;
}

const SKIN_TONES: Record<string, string> = {
  skin_1: '#FFDCB8',
  skin_2: '#E8B796',
  skin_3: '#C68642',
  skin_4: '#8D5524',
  skin_5: '#6B4423',
  skin_6: '#3C2415',
};

const HAIR_COLORS: Record<string, string> = {
  black: '#1a1a1a',
  dark_brown: '#2D1B0E',
  brown: '#4A3728',
  default: '#1a1a1a',
};

export const ChibiAvatar: React.FC<ChibiAvatarProps> = ({
  skinTone = 'skin_3',
  hairStyle = 'hair_1',
  hairColor = 'default',
  outfit = 'outfit_school',
  accessory,
  expression = 'happy',
  size = 200,
  className = '',
  animated = true,
}) => {
  const skin = SKIN_TONES[skinTone] || SKIN_TONES.skin_3;
  const hair = HAIR_COLORS[hairColor] || HAIR_COLORS.default;

  const avatarClasses = ['chibi-avatar'];
  if (animated) avatarClasses.push('chibi-animated');
  if (className) avatarClasses.push(className);

  return (
    <svg
      viewBox="0 0 100 120"
      width={size}
      height={size * 1.2}
      className={avatarClasses.join(' ')}
    >
      <g className="chibi-body">
        {/* OUTFIT */}
        {outfit === 'outfit_school' && (
          <g>
            <path d="M35 85 L30 115 L70 115 L65 85 Q50 90 35 85Z" fill="#FFF" stroke="#E0E0E0" strokeWidth="1" />
            <path d="M42 85 L50 95 L58 85" fill="#FFF" stroke="#E0E0E0" strokeWidth="1" />
            <polygon points="50,95 47,115 50,112 53,115" fill="#2E7D32" />
          </g>
        )}
        {outfit === 'outfit_agbada' && (
          <g>
            <path d="M28 85 L22 115 L78 115 L72 85 Q50 92 28 85Z" fill="#7B1FA2" />
            <line x1="50" y1="90" x2="50" y2="108" stroke="#FFD700" strokeWidth="2" />
            <line x1="46" y1="95" x2="54" y2="95" stroke="#FFD700" strokeWidth="1.5" />
            <line x1="46" y1="100" x2="54" y2="100" stroke="#FFD700" strokeWidth="1.5" />
          </g>
        )}
        {outfit === 'outfit_ankara' && (
          <g>
            <path d="M32 85 L28 115 L72 115 L68 85 Q50 90 32 85Z" fill="#E65100" />
            <circle cx="42" cy="98" r="4" fill="#FFC107" />
            <circle cx="50" cy="92" r="4" fill="#C62828" />
            <circle cx="58" cy="98" r="4" fill="#FFC107" />
            <circle cx="46" cy="107" r="3" fill="#1565C0" />
            <circle cx="54" cy="107" r="3" fill="#1565C0" />
          </g>
        )}
        {outfit === 'outfit_traditional' && (
          <g>
            <path d="M30 85 L26 115 L74 115 L70 85 Q50 92 30 85Z" fill="#1B5E20" />
            <line x1="36" y1="88" x2="36" y2="115" stroke="#FFD700" strokeWidth="2" />
            <line x1="46" y1="87" x2="46" y2="115" stroke="#FFD700" strokeWidth="2" />
            <line x1="54" y1="87" x2="54" y2="115" stroke="#FFD700" strokeWidth="2" />
            <line x1="64" y1="88" x2="64" y2="115" stroke="#FFD700" strokeWidth="2" />
          </g>
        )}
        {outfit === 'outfit_casual' && (
          <g>
            <path d="M35 85 L30 115 L70 115 L65 85 Q50 90 35 85Z" fill="#1976D2" />
            <rect x="42" y="94" width="16" height="12" rx="2" fill="#FFF" />
            <rect x="42" y="94" width="5" height="12" fill="#008751" />
            <rect x="53" y="94" width="5" height="12" fill="#008751" />
          </g>
        )}
        {!['outfit_school', 'outfit_agbada', 'outfit_ankara', 'outfit_traditional', 'outfit_casual'].includes(outfit) && (
          <path d="M35 85 L30 115 L70 115 L65 85 Q50 90 35 85Z" fill="#607D8B" />
        )}

        {/* Neck */}
        <rect x="44" y="78" width="12" height="10" rx="2" fill={skin} />

        {/* HAIR BACK for voluminous styles */}
        {hairStyle === 'hair_2' && (
          <g className="hair-back">
            {/* Huey Freeman big round afro - fills top of head */}
            <circle cx="50" cy="25" r="40" fill={hair} />
          </g>
        )}
        {hairStyle === 'hair_3' && (
          <g className="hair-back">
            <path d="M20 50 Q15 70 18 90" stroke={hair} strokeWidth="6" fill="none" strokeLinecap="round" />
            <path d="M80 50 Q85 70 82 90" stroke={hair} strokeWidth="6" fill="none" strokeLinecap="round" />
            <path d="M28 52 Q24 72 26 88" stroke={hair} strokeWidth="5" fill="none" strokeLinecap="round" />
            <path d="M72 52 Q76 72 74 88" stroke={hair} strokeWidth="5" fill="none" strokeLinecap="round" />
          </g>
        )}
        {hairStyle === 'hair_6' && (
          <g className="hair-back">
            <path d="M18 48 Q10 68 15 92" stroke={hair} strokeWidth="5" fill="none" strokeLinecap="round" />
            <path d="M82 48 Q90 68 85 92" stroke={hair} strokeWidth="5" fill="none" strokeLinecap="round" />
            <path d="M26 50 Q20 70 24 90" stroke={hair} strokeWidth="4" fill="none" strokeLinecap="round" />
            <path d="M74 50 Q80 70 76 90" stroke={hair} strokeWidth="4" fill="none" strokeLinecap="round" />
            <path d="M34 50 Q30 68 32 85" stroke={hair} strokeWidth="4" fill="none" strokeLinecap="round" />
            <path d="M66 50 Q70 68 68 85" stroke={hair} strokeWidth="4" fill="none" strokeLinecap="round" />
          </g>
        )}

        {/* Head */}
        <ellipse cx="50" cy="50" rx="30" ry="28" fill={skin} />

        {/* Ears */}
        <ellipse cx="20" cy="52" rx="4" ry="6" fill={skin} />
        <ellipse cx="80" cy="52" rx="4" ry="6" fill={skin} />

        {/* HAIR FRONT */}
        {hairStyle === 'hair_1' && (
          <path d="M22 52 C22 32 34 24 50 24 C66 24 78 32 78 52 C72 44 62 40 50 40 C38 40 28 44 22 52Z" fill={hair} />
        )}
        {hairStyle === 'hair_2' && (
          <g>
            {/* Huey hairline - comes down on forehead a bit */}
            <path d="M26 48 C28 44 38 42 50 42 C62 42 72 44 74 48 C70 46 60 44 50 44 C40 44 30 46 26 48Z" fill={hair} />
          </g>
        )}
        {hairStyle === 'hair_3' && (
          <g>
            <path d="M22 54 C22 34 34 26 50 26 C66 26 78 34 78 54 C72 46 62 42 50 42 C38 42 28 46 22 54Z" fill={hair} />
            <circle cx="18" cy="88" r="3" fill="#FFD700" />
            <circle cx="82" cy="88" r="3" fill="#FFD700" />
            <circle cx="26" cy="86" r="2.5" fill="#E53935" />
            <circle cx="74" cy="86" r="2.5" fill="#E53935" />
          </g>
        )}
        {hairStyle === 'hair_4' && (
          <g>
            <path d="M18 55 C16 35 30 22 50 22 C70 22 84 35 82 55 C76 46 64 42 50 42 C36 42 24 46 18 55Z" fill={hair} />
            <circle cx="28" cy="32" r="7" fill={hair} />
            <circle cx="42" cy="26" r="7" fill={hair} />
            <circle cx="58" cy="26" r="7" fill={hair} />
            <circle cx="72" cy="32" r="7" fill={hair} />
          </g>
        )}
        {hairStyle === 'hair_5' && (
          <g>
            <path d="M18 55 C14 42 28 28 50 24 C72 28 86 42 82 55 C76 48 64 44 50 44 C36 44 24 48 18 55Z" fill="#E65100" />
            <ellipse cx="50" cy="28" rx="14" ry="8" fill="#FFB300" />
            <path d="M28 40 Q50 32 72 40" stroke="#FFB300" strokeWidth="4" fill="none" />
          </g>
        )}
        {hairStyle === 'hair_6' && (
          <path d="M22 54 C22 36 34 28 50 28 C66 28 78 36 78 54 C72 46 62 42 50 42 C38 42 28 46 22 54Z" fill={hair} />
        )}
        {hairStyle === 'hair_7' && (
          <g>
            <path d="M26 54 C26 40 38 34 50 34 C62 34 74 40 74 54 C68 48 60 44 50 44 C40 44 32 48 26 54Z" fill={hair} />
            <circle cx="34" cy="30" r="9" fill={hair} />
            <circle cx="50" cy="24" r="10" fill={hair} />
            <circle cx="66" cy="30" r="9" fill={hair} />
          </g>
        )}
        {hairStyle === 'hair_8' && (
          <g>
            <path d="M24 54 C24 36 36 28 50 28 C64 28 76 36 76 54 C70 46 62 42 50 42 C38 42 30 46 24 54Z" fill={hair} />
            <path d="M50 32 L50 58" stroke={hair} strokeWidth="5" fill="none" strokeLinecap="round" />
            <path d="M32 42 Q28 58 30 75" stroke={hair} strokeWidth="4" fill="none" strokeLinecap="round" />
            <path d="M68 42 Q72 58 70 75" stroke={hair} strokeWidth="4" fill="none" strokeLinecap="round" />
            <circle cx="50" cy="48" r="3" fill="#FFD700" />
            <circle cx="30" cy="65" r="3" fill="#FFD700" />
            <circle cx="70" cy="65" r="3" fill="#FFD700" />
          </g>
        )}
        {!['hair_1', 'hair_2', 'hair_3', 'hair_4', 'hair_5', 'hair_6', 'hair_7', 'hair_8'].includes(hairStyle) && (
          <path d="M22 52 C22 32 34 24 50 24 C66 24 78 32 78 52 C72 44 62 40 50 40 C38 40 28 44 22 52Z" fill={hair} />
        )}

        {/* Blush */}
        <ellipse cx="30" cy="62" rx="5" ry="2.5" fill="#FFAB91" opacity="0.4" />
        <ellipse cx="70" cy="62" rx="5" ry="2.5" fill="#FFAB91" opacity="0.4" />

        {/* Eyes - large round chibi eyes */}
        <ellipse cx="38" cy="54" rx="8" ry="9" fill="#FFF" />
        <ellipse cx="62" cy="54" rx="8" ry="9" fill="#FFF" />
        {/* Iris - dark brown */}
        <circle cx="38" cy="55" r="6" fill="#3E2723" />
        <circle cx="62" cy="55" r="6" fill="#3E2723" />
        {/* Pupil */}
        <circle cx="38" cy="55" r="3" fill="#1a1a1a" />
        <circle cx="62" cy="55" r="3" fill="#1a1a1a" />
        {/* Eye shine */}
        <circle cx="40" cy="53" r="2.5" fill="#FFF" />
        <circle cx="64" cy="53" r="2.5" fill="#FFF" />
        <circle cx="36" cy="57" r="1" fill="#FFF" opacity="0.6" />
        <circle cx="60" cy="57" r="1" fill="#FFF" opacity="0.6" />

        {/* Eyebrows */}
        <path d="M30 45 Q38 42 44 45" stroke="#5D4037" strokeWidth="2" fill="none" strokeLinecap="round" />
        <path d="M56 45 Q62 42 70 45" stroke="#5D4037" strokeWidth="2" fill="none" strokeLinecap="round" />

        {/* Nose */}
        <ellipse cx="50" cy="62" rx="2" ry="1.5" fill="rgba(0,0,0,0.12)" />

        {/* Mouth */}
        {expression === 'happy' && (
          <path d="M44 68 Q50 74 56 68" stroke="#6D4C41" strokeWidth="2.5" fill="none" strokeLinecap="round" />
        )}
        {expression === 'excited' && (
          <ellipse cx="50" cy="70" rx="6" ry="4" fill="#6D4C41" />
        )}
        {expression === 'thinking' && (
          <g>
            <path d="M46 69 Q50 67 54 69" stroke="#6D4C41" strokeWidth="2" fill="none" strokeLinecap="round" />
            <circle cx="78" cy="40" r="4" fill="#FFF" stroke="#DDD" strokeWidth="1" />
            <circle cx="85" cy="32" r="3" fill="#FFF" stroke="#DDD" strokeWidth="1" />
          </g>
        )}
        {expression === 'neutral' && (
          <path d="M45 69 L55 69" stroke="#6D4C41" strokeWidth="2.5" strokeLinecap="round" />
        )}

        {/* Accessories */}
        {accessory === 'acc_beads' && (
          <g>
            <circle cx="36" cy="80" r="3" fill="#E53935" />
            <circle cx="44" cy="82" r="3" fill="#E53935" />
            <circle cx="50" cy="83" r="3" fill="#E53935" />
            <circle cx="56" cy="82" r="3" fill="#E53935" />
            <circle cx="64" cy="80" r="3" fill="#E53935" />
          </g>
        )}
        {accessory === 'acc_cap_red' && (
          <g>
            <ellipse cx="50" cy="26" rx="18" ry="8" fill="#B71C1C" />
            <path d="M32 26 Q32 18 50 15 Q68 18 68 26" fill="#B71C1C" />
          </g>
        )}
        {accessory === 'acc_cap_white' && (
          <g>
            <ellipse cx="50" cy="24" rx="20" ry="9" fill="#FAFAFA" stroke="#E0E0E0" strokeWidth="1" />
            <path d="M30 24 Q30 14 50 12 Q70 14 70 24" fill="#FAFAFA" stroke="#E0E0E0" strokeWidth="1" />
          </g>
        )}
        {accessory === 'acc_glasses' && (
          <g>
            <circle cx="38" cy="54" r="9" fill="none" stroke="#424242" strokeWidth="2" />
            <circle cx="62" cy="54" r="9" fill="none" stroke="#424242" strokeWidth="2" />
            <line x1="47" y1="54" x2="53" y2="54" stroke="#424242" strokeWidth="2" />
            <line x1="29" y1="54" x2="22" y2="52" stroke="#424242" strokeWidth="2" />
            <line x1="71" y1="54" x2="78" y2="52" stroke="#424242" strokeWidth="2" />
          </g>
        )}
        {accessory === 'acc_earrings' && (
          <g>
            <circle cx="20" cy="58" r="4" fill="#FFD700" />
            <circle cx="80" cy="58" r="4" fill="#FFD700" />
          </g>
        )}
        {accessory === 'acc_bag' && (
          <g>
            <path d="M68 86 Q76 88 78 95" stroke="#1565C0" strokeWidth="3" fill="none" strokeLinecap="round" />
            <rect x="76" y="92" width="18" height="20" rx="4" fill="#2196F3" />
            <rect x="78" y="95" width="14" height="9" rx="2" fill="#1976D2" />
            <line x1="85" y1="96" x2="85" y2="102" stroke="#FFC107" strokeWidth="2" />
          </g>
        )}
        {accessory === 'acc_ankara_bag' && (
          <g>
            <path d="M68 86 Q76 88 78 95" stroke="#E65100" strokeWidth="3" fill="none" strokeLinecap="round" />
            <rect x="76" y="92" width="18" height="20" rx="4" fill="#FF9800" />
            <circle cx="85" cy="100" r="5" fill="#E53935" />
            <circle cx="85" cy="100" r="2.5" fill="#FF9800" />
          </g>
        )}
      </g>
    </svg>
  );
};

export default ChibiAvatar;
