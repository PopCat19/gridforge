#!/usr/bin/env bash
#
# check-context.test.sh
#
# Purpose: Unit tests for check-context.sh utility functions

set -Euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Source the required modules
source "${SCRIPT_DIR}/lib.sh"
source "${SCRIPT_DIR}/check-context.sh"

# Create a temporary test directory structure
setup_test_dir() {
    local test_dir
    test_dir=$(mktemp -d)

    # Create a test file with header
    mkdir -p "$test_dir"
    cat > "$test_dir/test-file.sh" <<'EOF'
# test-file.sh
#
# Purpose: Test file for context checking
EOF

    # Create a context.md that matches
    cat > "$test_dir/context.md" <<'EOF'
# Context

- `test-file.sh` — Test file for context checking
EOF

    echo "$test_dir"
}

cleanup_test_dir() {
    local test_dir="$1"
    rm -rf "$test_dir"
}

# Test check_context_drift with matching context
test_context_match() {
    local test_dir
    test_dir=$(setup_test_dir)

    # Suppress output and check return value
    if check_context_drift "$test_dir" >/dev/null 2>&1; then
        echo "PASS: check_context_drift returns 0 for matching context"
        cleanup_test_dir "$test_dir"
        return 0
    else
        echo "FAIL: check_context_drift should return 0 for matching context"
        cleanup_test_dir "$test_dir"
        return 1
    fi
}

# Test check_context_drift with missing file
test_context_missing_file() {
    local test_dir
    test_dir=$(setup_test_dir)

    # Add a file that's not in context.md
    echo "# Another file" > "$test_dir/extra-file.sh"

    # This should detect drift
    if ! check_context_drift "$test_dir" >/dev/null 2>&1; then
        echo "PASS: check_context_drift detects unlisted files"
        cleanup_test_dir "$test_dir"
        return 0
    else
        echo "FAIL: check_context_drift should detect unlisted files"
        cleanup_test_dir "$test_dir"
        return 1
    fi
}

# Test check_context_drift with content drift
test_context_content_drift() {
    local test_dir
    test_dir=$(setup_test_dir)

    # Update the file header to different purpose
    cat > "$test_dir/test-file.sh" <<'EOF'
# test-file.sh
#
# Purpose: Different purpose description
EOF

    # This should detect drift
    if ! check_context_drift "$test_dir" >/dev/null 2>&1; then
        echo "PASS: check_context_drift detects content drift"
        cleanup_test_dir "$test_dir"
        return 0
    else
        echo "FAIL: check_context_drift should detect content drift"
        cleanup_test_dir "$test_dir"
        return 1
    fi
}

# Test check_context_drift with no context files
test_context_no_files() {
    local test_dir
    test_dir=$(mktemp -d)

    # This should return 0 (no context files is OK)
    if check_context_drift "$test_dir" >/dev/null 2>&1; then
        echo "PASS: check_context_drift handles empty directory"
        cleanup_test_dir "$test_dir"
        return 0
    else
        echo "FAIL: check_context_drift should handle empty directory"
        cleanup_test_dir "$test_dir"
        return 1
    fi
}

# Run all tests
run_tests() {
    local passed=0
    local failed=0

    echo "Running check-context.sh tests..."
    echo "==================================="

    for test_func in test_context_match test_context_missing_file \
                     test_context_content_drift test_context_no_files; do
        if "$test_func"; then
            ((passed++))
        else
            ((failed++))
        fi
    done

    echo ""
    echo "==================================="
    echo "Results: $passed passed, $failed failed"

    if [[ $failed -gt 0 ]]; then
        return 1
    fi
    return 0
}

# Run tests
run_tests
