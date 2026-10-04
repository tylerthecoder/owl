#!/usr/bin/env python3
"""Mirror Universe into Google Drive, or install its per-user launchd job."""
import argparse
import datetime
import fcntl
import os
from pathlib import Path
import plistlib
import shutil
import subprocess
import sys

LABEL = 'com.tylertracy.universe-gdrive-backup'
EXCLUDES = ['.DS_Store', '.stfolder', '.stversions', '.sync-conflict-*']


def drive_root(home, explicit=None):
    if explicit is not None:
        root = explicit.expanduser()
        if not root.is_dir():
            raise ValueError('Google Drive My Drive directory is not available: ' + str(root))
        return root.resolve()
    roots = sorted((home / 'Library/CloudStorage').glob('GoogleDrive-*/My Drive'))
    roots = [root for root in roots if root.is_dir()]
    if len(roots) != 1:
        raise ValueError('Expected one mounted Google Drive account; pass --drive-root "/path/to/My Drive"')
    return roots[0].resolve()


def paths(source, root):
    if not source.is_dir():
        raise ValueError('Universe source directory is not available: ' + str(source))
    source = source.resolve()
    destination = root / 'Universe Backup'
    if destination.is_symlink():
        raise ValueError('Refusing a symlink at the backup destination')
    destination = destination.resolve()
    if source == destination or source in destination.parents or destination in source.parents:
        raise ValueError('Source and backup destination must not overlap')
    return source, destination


def mirror(source, root, state, dry_run=False):
    source, destination = paths(source, root)
    state.mkdir(parents=True, exist_ok=True)
    with (state / 'backup.lock').open('a') as lock:
        try:
            fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
        except BlockingIOError:
            print('Universe backup already running; skipped')
            return 0
        # Never create the Drive mount itself: an unavailable mount must fail.
        if not root.is_dir():
            raise ValueError('Google Drive mount disappeared before backup')
        if not dry_run:
            destination.mkdir(exist_ok=True)
        command = ['/usr/bin/rsync', '-a', '--delete']
        if dry_run:
            command += ['--dry-run', '--itemize-changes']
        for pattern in EXCLUDES:
            command += ['--exclude', pattern]
        command += [str(source) + '/', str(destination) + '/']
        with (state / 'backup.log').open('a') as log:
            log.write('===== ' + datetime.datetime.now().isoformat() + ' =====\n')
            log.flush()
            result = subprocess.run(command, stdout=log, stderr=subprocess.STDOUT, pass_fds=(lock.fileno(),))
            log.write('exit={} {}\n'.format(result.returncode, datetime.datetime.now().isoformat()))
        print('Universe backup {} (details: {})'.format('completed' if result.returncode == 0 else 'failed', state / 'backup.log'))
        return result.returncode


def launch_agent(home, source, root, script, interpreter):
    state = home / '.cache/universe-backup'
    return {
        'Label': LABEL,
        'ProgramArguments': [str(interpreter), str(script), 'run', '--source', str(source), '--drive-root', str(root)],
        'StartInterval': 3600,
        'RunAtLoad': True,
        'ProcessType': 'Background',
        'StandardOutPath': str(state / 'launchd.out.log'),
        'StandardErrorPath': str(state / 'launchd.err.log'),
    }


def install(home, source, root, dry_run=False):
    source, _ = paths(source, root)
    data = plistlib.dumps(launch_agent(home, source, root, Path(__file__).resolve(), Path(sys.executable)))
    if dry_run:
        sys.stdout.buffer.write(data)
        return 0
    agents = home / 'Library/LaunchAgents'
    state = home / '.cache/universe-backup'
    agents.mkdir(parents=True, exist_ok=True)
    state.mkdir(parents=True, exist_ok=True)
    target = agents / (LABEL + '.plist')
    if target.exists():
        backups = state / 'launchagent-backups'
        backups.mkdir(exist_ok=True)
        stamp = datetime.datetime.now().strftime('%Y%m%d-%H%M%S-%f')
        shutil.copy2(target, backups / (stamp + '.plist'))
    domain = 'gui/' + str(os.getuid())
    service = domain + '/' + LABEL
    present = subprocess.run(['/bin/launchctl', 'print', service], capture_output=True)
    if present.returncode == 0:
        subprocess.run(['/bin/launchctl', 'bootout', service], check=True)
    temporary = target.with_suffix('.plist.tmp')
    temporary.write_bytes(data)
    temporary.replace(target)
    subprocess.run(['/bin/launchctl', 'enable', service], check=True)
    subprocess.run(['/bin/launchctl', 'bootstrap', domain, str(target)], check=True)
    print('Installed hourly Universe backup; launchd has started the first run.')
    return 0


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('mode', choices=['run', 'install'], nargs='?', default='run')
    parser.add_argument('--source', type=Path, default=Path.home() / 'Universe')
    parser.add_argument('--drive-root', type=Path, help='Mounted Google Drive My Drive directory; required if multiple accounts exist')
    parser.add_argument('--dry-run', action='store_true', help='Preview rsync changes or print the launchd plist without installing it')
    args = parser.parse_args()
    if sys.platform != 'darwin':
        parser.error('This setup requires macOS and Google Drive for desktop')
    try:
        root = drive_root(Path.home(), args.drive_root)
        if args.mode == 'install':
            return install(Path.home(), args.source.expanduser(), root, args.dry_run)
        return mirror(args.source.expanduser(), root, Path.home() / '.cache/universe-backup', args.dry_run)
    except (ValueError, OSError, subprocess.CalledProcessError) as error:
        print('Universe backup error: ' + str(error), file=sys.stderr)
        return 1


if __name__ == '__main__':
    sys.exit(main())
