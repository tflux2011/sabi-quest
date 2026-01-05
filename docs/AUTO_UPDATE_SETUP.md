# Sabi Quest - Auto-Update Setup Guide

## Overview
Sabi Quest uses Tauri's built-in updater to deliver updates to users automatically. When you release a new version, users will see an update notification and can install it with one click.

## Quick Setup (5 minutes)

### 1. Generate Signing Keys

Run this command in your terminal:
```bash
npx @tauri-apps/cli signer generate -w ~/.tauri/sabi-quest.key
```

This will output:
- **Private Key**: Save this securely, it signs your updates
- **Public Key**: Goes in `tauri.conf.json`

### 2. Add GitHub Secrets

Go to your GitHub repo → Settings → Secrets and variables → Actions → New repository secret

Add these secrets:
- `TAURI_SIGNING_PRIVATE_KEY` - Your private key (the entire key including headers)
- `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` - Password you set when generating

### 3. Update tauri.conf.json

Replace the placeholder pubkey in `src-tauri/tauri.conf.json`:
```json
"plugins": {
  "updater": {
    "pubkey": "YOUR_PUBLIC_KEY_HERE",
    "endpoints": [
      "https://github.com/YOUR_USERNAME/sabi-quest/releases/latest/download/latest.json"
    ]
  }
}
```

### 4. Push and Create a Release

```bash
# Update version in tauri.conf.json and package.json
# For example: "version": "0.2.0"

# Commit and tag
git add .
git commit -m "Release v0.2.0"
git tag v0.2.0
git push origin main --tags
```

The GitHub Action will automatically:
1. Build for macOS (Intel + Apple Silicon), Windows, and Linux
2. Sign the updates
3. Upload artifacts to a GitHub Release
4. Generate `latest.json` for the updater

## How It Works

1. **On App Launch**: App checks the endpoint for `latest.json`
2. **If Update Available**: Shows a banner with "Update Available"
3. **User Clicks Update**: Downloads and installs in background
4. **App Restarts**: User is on the new version!

## Version Bumping

Always update version in TWO places:
- `src-tauri/tauri.conf.json` → `version`
- `package.json` → `version`

Use semantic versioning: MAJOR.MINOR.PATCH
- Patch (0.1.1): Bug fixes
- Minor (0.2.0): New features (backwards compatible)
- Major (1.0.0): Breaking changes

## Testing Updates Locally

During development, the updater won't find updates (no endpoint configured yet). 
To test the UI:
1. Set up a local server with a mock `latest.json`
2. Or use a staging GitHub repo

## Troubleshooting

### "Could not check for updates"
- Network issue or endpoint not configured
- Check if GitHub releases are public

### Update downloads but doesn't install
- Signing key mismatch
- Corrupted download

### macOS Gatekeeper blocks app
- Need to sign with Apple Developer ID (paid $99/year)
- Or users run: `xattr -cr /Applications/Sabi\ Quest.app`

## Security Notes

⚠️ **Keep your private key SECRET!**
- Never commit it to git
- Store in GitHub Secrets only
- If compromised, generate new keys

The public key is safe to share - it can only verify, not sign.

## File Structure

```
.github/
  workflows/
    release.yml          # GitHub Actions workflow
src-tauri/
  tauri.conf.json        # Updater config (pubkey, endpoints)
src/
  components/
    UpdateChecker.tsx    # Update UI component
    UpdateChecker.css    # Styles
```

## Support

For issues with the updater:
1. Check browser console for errors
2. Check Tauri logs: `~/Library/Logs/com.projectnigeria.app/`
3. Test endpoint manually: `curl <your-endpoint>/latest.json`
