# Changelog
This file will be used to track changes from Ocelot version `0.1.1` onward.

## 0.1.4
This is a patch finishing the fixes introduced in `0.1.3`.

### Changes
- Fix castling changing piece colors
- Prevent player from taking their own pieces

## 0.1.3
This is a patch bringing plenty of fixes, and overall release number 4.

### Changes
- Add proper stalemate evaluation
- Add `eval` UCI command
- Fix inaccuracies in README
- Format entire codebase
- Add time cutoff for search, so if allotted time is surpassed search ends early
- Fix `position` command parsing to accept FEN strings
- Add check for illegal moves when parsing user move in TUI
- Start work on proper `go` command parsing. This exists as dead code for now, but it will be fully implemented next release.
- Make coordinate generation safer to reduce crashes

## 0.1.2
This patch is one of version `0.1`, and overall release number 3.

Another minor patch, simply adding a help menu.

### Changes
- Added `--help` option to display usage information for the CLI.
- Set `license` field in manifest, so the license shows up as MIT now.

## 0.1.1
This is patch one of version `0.1`, and overall release number 2.

This one's pretty minor. All I did was add more state messages and auto-exiting in the TUI. This means it actually e2xits instead of letting you keep playing after the game should end.

### Changes
- Added automatic match ending and associated messages in TUI
- Added display of previous engine move in TUI
- Remove `install.sh` from crate; it was just for development purposes.
