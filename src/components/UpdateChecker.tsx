import React, { useEffect, useState } from 'react';
import { check } from '@tauri-apps/plugin-updater';
import { relaunch } from '@tauri-apps/plugin-process';
import { motion, AnimatePresence } from 'framer-motion';
import { Download, RefreshCw, X, Sparkles, AlertCircle } from 'lucide-react';
import './UpdateChecker.css';

interface UpdateInfo {
  version: string;
  body: string | null;
  date: string | null;
}

interface UpdateCheckerProps {
  onDismiss?: () => void;
}

export const UpdateChecker: React.FC<UpdateCheckerProps> = ({ onDismiss }) => {
  const [updateAvailable, setUpdateAvailable] = useState<UpdateInfo | null>(null);
  const [isChecking, setIsChecking] = useState(false);
  const [isDownloading, setIsDownloading] = useState(false);
  const [downloadProgress, setDownloadProgress] = useState(0);
  const [error, setError] = useState<string | null>(null);
  const [showBanner, setShowBanner] = useState(false);

  // Check for updates on mount
  useEffect(() => {
    checkForUpdates();
  }, []);

  const checkForUpdates = async () => {
    setIsChecking(true);
    setError(null);
    
    try {
      const update = await check();
      
      if (update) {
        setUpdateAvailable({
          version: update.version,
          body: update.body || null,
          date: update.date || null,
        });
        setShowBanner(true);
      }
    } catch (err) {
      console.error('Update check failed:', err);
      // Don't show error for network issues - silent fail
      if (String(err).includes('endpoint')) {
        console.log('Update endpoint not configured yet');
      } else {
        setError('Could not check for updates');
      }
    } finally {
      setIsChecking(false);
    }
  };

  const downloadAndInstall = async () => {
    if (!updateAvailable) return;
    
    setIsDownloading(true);
    setError(null);
    
    try {
      const update = await check();
      
      if (update) {
        let downloaded = 0;
        let contentLength = 0;
        
        // Download with progress tracking
        await update.downloadAndInstall((event) => {
          switch (event.event) {
            case 'Started':
              contentLength = (event.data as { contentLength?: number }).contentLength || 0;
              setDownloadProgress(0);
              break;
            case 'Progress':
              downloaded += (event.data as { chunkLength: number }).chunkLength;
              if (contentLength > 0) {
                setDownloadProgress((downloaded / contentLength) * 100);
              }
              break;
            case 'Finished':
              setDownloadProgress(100);
              break;
          }
        });
        
        // Relaunch the app to apply update
        await relaunch();
      }
    } catch (err) {
      console.error('Update download failed:', err);
      setError('Failed to download update. Please try again.');
      setIsDownloading(false);
    }
  };

  const dismissUpdate = () => {
    setShowBanner(false);
    onDismiss?.();
  };

  if (!showBanner || !updateAvailable) {
    return null;
  }

  return (
    <AnimatePresence>
      <motion.div
        className="update-banner"
        initial={{ opacity: 0, y: -50 }}
        animate={{ opacity: 1, y: 0 }}
        exit={{ opacity: 0, y: -50 }}
        transition={{ type: "spring", damping: 20 }}
      >
        <div className="update-banner-content">
          <div className="update-icon">
            <Sparkles size={24} />
          </div>
          
          <div className="update-info">
            <h3>New Update Available! 🎉</h3>
            <p>Version {updateAvailable.version} is ready to download</p>
            {updateAvailable.body && (
              <p className="update-notes">{updateAvailable.body}</p>
            )}
          </div>

          <div className="update-actions">
            {isDownloading ? (
              <div className="download-progress">
                <div className="progress-bar">
                  <motion.div 
                    className="progress-fill"
                    initial={{ width: 0 }}
                    animate={{ width: `${downloadProgress}%` }}
                  />
                </div>
                <span>{Math.round(downloadProgress)}%</span>
              </div>
            ) : (
              <>
                <motion.button
                  className="update-btn primary"
                  onClick={downloadAndInstall}
                  whileHover={{ scale: 1.05 }}
                  whileTap={{ scale: 0.95 }}
                  disabled={isChecking}
                >
                  <Download size={16} />
                  Update Now
                </motion.button>
                
                <motion.button
                  className="update-btn secondary"
                  onClick={dismissUpdate}
                  whileHover={{ scale: 1.05 }}
                  whileTap={{ scale: 0.95 }}
                >
                  Later
                </motion.button>
              </>
            )}
          </div>

          {!isDownloading && (
            <motion.button
              className="update-close"
              onClick={dismissUpdate}
              whileHover={{ scale: 1.1 }}
              whileTap={{ scale: 0.9 }}
              aria-label="Dismiss update notification"
            >
              <X size={18} />
            </motion.button>
          )}
        </div>

        {error && (
          <motion.div 
            className="update-error"
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
          >
            <AlertCircle size={14} />
            {error}
          </motion.div>
        )}
      </motion.div>
    </AnimatePresence>
  );
};

// Manual update check button for settings
export const UpdateCheckButton: React.FC = () => {
  const [isChecking, setIsChecking] = useState(false);
  const [status, setStatus] = useState<'idle' | 'upToDate' | 'available' | 'error'>('idle');

  const handleCheck = async () => {
    setIsChecking(true);
    setStatus('idle');
    
    try {
      const update = await check();
      setStatus(update ? 'available' : 'upToDate');
    } catch (err) {
      setStatus('error');
    } finally {
      setIsChecking(false);
    }
  };

  return (
    <motion.button
      className="check-update-btn"
      onClick={handleCheck}
      disabled={isChecking}
      whileHover={{ scale: 1.02 }}
      whileTap={{ scale: 0.98 }}
    >
      <RefreshCw size={16} className={isChecking ? 'spinning' : ''} />
      {isChecking ? 'Checking...' : 'Check for Updates'}
      {status === 'upToDate' && <span className="status-badge success">✓ Up to date</span>}
      {status === 'available' && <span className="status-badge info">Update available!</span>}
      {status === 'error' && <span className="status-badge error">Check failed</span>}
    </motion.button>
  );
};

export default UpdateChecker;
