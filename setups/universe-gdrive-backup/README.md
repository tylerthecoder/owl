# Universe backup to Google Drive

Mirrors `~/Universe/` (including `notes/`) into `My Drive/Universe Backup/`
every hour and when the user LaunchAgent loads. Google Drive for desktop then
uploads that directory. A successful rsync run confirms the local mirror, not
completion of the cloud upload. **Deletions propagate to the mirror**; retain
Git history or another versioned backup for recovery.

## Setup

Requires macOS, Python 3 (`/usr/bin/python3`, supplied with Apple developer
tools), `~/Universe`, and Google Drive for desktop signed in with `My Drive`
available under `~/Library/CloudStorage/GoogleDrive-*/`. The `mac` nest includes
this setup. To install it independently:

```sh
owl setup universe-gdrive-backup all
```

The installer selects a single mounted Drive account and writes its explicit
path into the LaunchAgent. With multiple accounts, select one yourself:

```sh
owl setup universe-gdrive-backup link
~/owl/setups/universe-gdrive-backup/install.sh --drive-root "$HOME/Library/CloudStorage/GoogleDrive-YOUR_ACCOUNT/My Drive"
```

Preview the generated plist without installing or starting anything:

```sh
~/owl/setups/universe-gdrive-backup/install.sh --dry-run
```

The setup reuses `com.tylertracy.universe-gdrive-backup`, replacing the previous
job rather than adding a second schedule. Existing plists are retained under
`~/.cache/universe-backup/launchagent-backups/`. The plist points at this checkout;
keep it available and rerun installation if you move it. No administrator access,
Drive credentials, Syncthing configuration, or note-sync configuration is needed.

## Running and troubleshooting

```sh
universe-gdrive-backup --dry-run  # changes are listed in backup.log
universe-gdrive-backup           # mirror now
launchctl list com.tylertracy.universe-gdrive-backup
tail -n 20 ~/.cache/universe-backup/backup.log
```

With multiple mounted accounts, also pass `--drive-root` for manual runs.
The generated schedule retains the account selected during installation.
Configuration lives in this directory; the generated local plist is
`~/Library/LaunchAgents/com.tylertracy.universe-gdrive-backup.plist`.
Launchd output and errors are in `~/.cache/universe-backup/launchd.{out,err}.log`.
Missing source/mount directories fail without creating a substitute mount.
Concurrent runs skip while another run holds the backup lock.

The original exclusions are retained: `.DS_Store`, `.stfolder`, `.stversions`,
and filenames beginning `.sync-conflict-`. Other files, including notes and
Git history, are copied. No files are read from Drive into Universe.

To disable the job without removing either folder:

```sh
launchctl disable "gui/$(id -u)/com.tylertracy.universe-gdrive-backup"
launchctl bootout "gui/$(id -u)/com.tylertracy.universe-gdrive-backup"
```

Run the installer again to enable it. Checks for this setup:

```sh
python3 -m unittest discover -s setups/universe-gdrive-backup -p 'test_*.py'
```
