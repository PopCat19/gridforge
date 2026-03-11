# Test Results - 2026-03-11

## Shell Tests

### conventions/src/lib.test.sh
```
Running lib.sh tests...
========================
PASS: command_exists returns 0 for bash
PASS: command_exists returns 1 for non-existing command
PASS: normalize_github_url passes through HTTPS URLs
PASS: normalize_github_url converts SSH URLs to HTTPS
PASS: normalize_github_url removes .git suffix
SKIP: prompt_input test requires TTY mocking
PASS: ANSI color codes are defined
PASS: get_current_branch returns branch name
========================
Results: 8 passed, 0 failed
```

### conventions/src/check-context.test.sh
```
Running check-context.sh tests...
===================================
PASS: check_context_drift returns 0 for matching context
PASS: check_context_drift detects unlisted files
PASS: check_context_drift detects content drift
PASS: check_context_drift handles empty directory
===================================
Results: 4 passed, 0 failed
```

## Biome Lint Check
- 55 errors found (style/formatting)
- 5 warnings
- Issues: non-null assertions, quote style, import sorting, JSON formatting

## Bun Tests
- Unable to run: SIGILL (CPU incompatibility)
- Test file exists at src/utils/music.test.ts with 200+ unit tests
