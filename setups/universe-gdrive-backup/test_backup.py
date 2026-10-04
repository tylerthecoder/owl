import importlib.util
import io
from pathlib import Path
import plistlib
import tempfile
import unittest
from unittest.mock import patch, Mock
import fcntl
import subprocess

spec = importlib.util.spec_from_file_location('universe_backup', Path(__file__).with_name('backup.py'))
backup = importlib.util.module_from_spec(spec)
spec.loader.exec_module(backup)


class BackupTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix='owl-backup-test-')
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name).resolve()
        self.source = self.root / 'Universe'
        self.source.mkdir()
        self.drive = self.root / 'Drive account & spaces' / 'My Drive'
        self.drive.mkdir(parents=True)
        self.state = self.root / 'cache'

    def test_real_rsync_copies_notes_and_propagates_only_destination_deletions(self):
        notes = self.source / 'notes'
        notes.mkdir()
        (notes / 'daily note.md').write_text('precious writing')
        (self.source / '.DS_Store').write_text('ignored')
        (self.source / '.stversions').mkdir()
        (self.source / '.stversions/old.md').write_text('ignored')
        (notes / '.git').mkdir()
        (notes / '.git/HEAD').write_text('ref: refs/heads/main')
        destination = self.drive / 'Universe Backup'
        destination.mkdir()
        (destination / 'obsolete.md').write_text('old mirror content')
        self.assertEqual(backup.mirror(self.source, self.drive, self.state), 0)
        self.assertEqual((destination / 'notes/daily note.md').read_text(), 'precious writing')
        self.assertTrue((destination / 'notes/.git/HEAD').is_file())
        self.assertFalse((destination / 'obsolete.md').exists())
        self.assertFalse((destination / '.DS_Store').exists())
        self.assertFalse((destination / '.stversions').exists())
        self.assertEqual((notes / 'daily note.md').read_text(), 'precious writing')
        (notes / 'daily note.md').unlink()
        self.assertEqual(backup.mirror(self.source, self.drive, self.state), 0)
        self.assertFalse((destination / 'notes/daily note.md').exists())

    def test_dry_run_does_not_create_or_change_destination(self):
        (self.source / 'note.md').write_text('new')
        self.assertEqual(backup.mirror(self.source, self.drive, self.state, True), 0)
        destination = self.drive / 'Universe Backup'
        self.assertFalse(destination.exists())
        destination.mkdir()
        (destination / 'keep.md').write_text('keep')
        self.assertEqual(backup.mirror(self.source, self.drive, self.state, True), 0)
        self.assertEqual((destination / 'keep.md').read_text(), 'keep')
        self.assertFalse((destination / 'note.md').exists())

    def test_missing_source_or_mount_never_touches_backup(self):
        destination = self.drive / 'Universe Backup'
        destination.mkdir()
        (destination / 'keep.md').write_text('keep')
        with self.assertRaises(ValueError):
            backup.mirror(self.root / 'missing', self.drive, self.state)
        self.assertEqual((destination / 'keep.md').read_text(), 'keep')
        with self.assertRaises(ValueError):
            backup.drive_root(self.root, self.root / 'missing-drive')
        with self.assertRaises(ValueError):
            backup.mirror(self.source, self.root / 'missing-drive', self.state)
        self.assertFalse((self.root / 'missing-drive').exists())

    def test_account_discovery_rejects_zero_and_multiple_accounts(self):
        with self.assertRaises(ValueError):
            backup.drive_root(self.root)
        first = self.root / 'Library/CloudStorage/GoogleDrive-first/My Drive'
        first.mkdir(parents=True)
        self.assertEqual(backup.drive_root(self.root), first)
        second = first.parent.parent / 'GoogleDrive-second/My Drive'
        second.mkdir(parents=True)
        with self.assertRaises(ValueError):
            backup.drive_root(self.root)
        self.assertEqual(backup.drive_root(self.root, second), second)

    def test_overlapping_paths_and_destination_symlink_are_refused(self):
        with self.assertRaises(ValueError):
            backup.paths(self.root, self.drive)
        nested = self.drive / 'Universe Backup/nested'
        nested.mkdir(parents=True)
        with self.assertRaises(ValueError):
            backup.paths(nested, self.drive)
        elsewhere = self.root / 'elsewhere'
        elsewhere.mkdir()
        other_drive = self.root / 'other-drive'
        other_drive.mkdir()
        (other_drive / 'Universe Backup').symlink_to(elsewhere, target_is_directory=True)
        with self.assertRaises(ValueError):
            backup.paths(self.source, other_drive)

    def test_concurrent_run_does_not_invoke_rsync(self):
        self.state.mkdir()
        with (self.state / 'backup.lock').open('a') as lock:
            fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
            with patch.object(backup.subprocess, 'run') as run:
                self.assertEqual(backup.mirror(self.source, self.drive, self.state), 0)
                run.assert_not_called()

    def test_rsync_failure_is_logged_and_propagated(self):
        with patch.object(backup.subprocess, 'run', return_value=subprocess.CompletedProcess([], 23)):
            self.assertEqual(backup.mirror(self.source, self.drive, self.state), 23)
        self.assertIn('exit=23', (self.state / 'backup.log').read_text())

    def test_install_retains_previous_plist_and_replaces_same_user_job(self):
        agents = self.root / 'Library/LaunchAgents'
        agents.mkdir(parents=True)
        target = agents / (backup.LABEL + '.plist')
        target.write_bytes(b'previous configuration')
        with patch.object(backup.subprocess, 'run', return_value=subprocess.CompletedProcess([], 0)) as run:
            self.assertEqual(backup.install(self.root, self.source, self.drive), 0)
        plist = plistlib.loads(target.read_bytes())
        self.assertEqual(plist['Label'], backup.LABEL)
        self.assertEqual(plist['StartInterval'], 3600)
        self.assertTrue(plist['RunAtLoad'])
        self.assertEqual(plist['ProgramArguments'][-1], str(self.drive))
        self.assertEqual(plist['ProgramArguments'][4], str(self.source))
        self.assertTrue(any('bootout' in call.args[0] for call in run.call_args_list))
        self.assertTrue(any('bootstrap' in call.args[0] for call in run.call_args_list))
        copies = list((self.root / '.cache/universe-backup/launchagent-backups').iterdir())
        self.assertEqual(len(copies), 1)
        self.assertEqual(copies[0].read_bytes(), b'previous configuration')

    def test_install_preview_does_not_write_files_or_launch_job(self):
        output = Mock(buffer=io.BytesIO())
        with patch.object(backup.sys, 'stdout', output), patch.object(backup.subprocess, 'run') as run:
            self.assertEqual(backup.install(self.root, self.source, self.drive, True), 0)
            run.assert_not_called()
        self.assertEqual(plistlib.loads(output.buffer.getvalue())['StartInterval'], 3600)
        self.assertFalse((self.root / 'Library/LaunchAgents').exists())
        self.assertFalse((self.root / '.cache').exists())


if __name__ == '__main__':
    unittest.main()
