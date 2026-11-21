# Ledger (Windows + Android)

Private finance tracker. No accounts, no servers.

## Build the apps (free, about 10 minutes)
1. Create a free account at github.com and make a new PRIVATE repository.
2. Upload everything in this folder to it, including the hidden `.github` folder
   (GitHub Desktop or `git push` is the most reliable way to include it).
3. Open the repository's **Actions** tab > **Build apps** > **Run workflow**.
4. When both jobs finish, open the run and download:
   - **Ledger-Windows-installer**: a zip with the setup .exe
   - **Ledger-Android-apk**: a zip with app-debug.apk

## Install
- Windows: run the setup .exe. If SmartScreen warns, choose More info > Run anyway (the app is unsigned).
- Android: copy app-debug.apk to the phone and open it. Allow "install unknown apps" for your file manager or browser when asked.

## Your data
- Windows: auto-saved to Documents\Ledger\ledger-data.json after every change. If the app is ever reinstalled on a fresh profile, it restores from that file.
- Android: stored inside the app. Use Backup > "Share backup file" to save a copy to Files, Drive, etc.
- Move data between devices: "Share/Download backup file" on one, "Load backup file" on the other. Loading merges, so nothing is duplicated.
- Coming from the web version: download a backup file there and load it in the new app.
