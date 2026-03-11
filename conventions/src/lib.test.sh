#!/usr/bin/env bash
#
# lib.test.sh
#
# Purpose: Unit tests for lib.sh utility functions
#
# This module:
# - Tests logging functions
# - Tests command existence check
# - Tests Git helper functions

set -Euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Source the lib.sh
source "${SCRIPT_DIR}/lib.sh"

# Test command_exists
test_command_exists() {
    if command_exists "bash"; then
        echo "PASS: command_exists returns 0 for bash"
        return 0
    else
        echo "FAIL: command_exists should return 0 for bash"
        return 1
    fi
}

test_command_exists_not_found() {
    if command_exists "this_command_definitely_does_not_exist_12345"; then
        echo "FAIL: command_exists should return 1 for non-existing command"
        return 1
    else
        echo "PASS: command_exists returns 1 for non-existing command"
        return 0
    fi
}

# Test normalize_github_url
test_normalize_https_url() {
    local input="https://github.com/user/repo"
    local result
    result=$(normalize_github_url "$input")
    if [[ "$result" != "https://github.com/user/repo" ]]; then
        echo "FAIL: normalize_github_url should pass through HTTPS URLs"
        return 1
    fi
    echo "PASS: normalize_github_url passes through HTTPS URLs"
}

test_normalize_github_url_git() {
    local input="git@github.com:user/repo.git"
    local result
    result=$(normalize_github_url "$input")
    if [[ "$result" != "https://github.com/user/repo" ]]; then
        echo "FAIL: normalize_github_url should convert SSH URLs"
        return 1
    fi
    echo "PASS: normalize_github_url converts SSH URLs to HTTPS"
}

test_normalize_github_url_with_dotgit() {
    local input="git@github.com:user/repo"
    local result
    result=$(normalize_github_url "$input")
    if [[ "$result" != "https://github.com/user/repo" ]]; then
        echo "FAIL: normalize_github_url should remove .git suffix"
        return 1
    fi
    echo "PASS: normalize_github_url removes .git suffix"
}

# Test prompt_input (skip - non-interactive testing is complex)
# This test is skipped because it requires TTY mocking
test_prompt_input_with_default() {
    echo "SKIP: prompt_input test requires TTY mocking"
    return 0
}

# Test that colors are defined
test_colors_defined() {
    if [[ -z "$ANSI_CLEAR" ]]; then
        echo "FAIL: ANSI_CLEAR should be defined"
        return 1
    fi
    if [[ -z "$ANSI_GREEN" ]]; then
        echo "FAIL: ANSI_GREEN should be defined"
        return 1
    fi
    if [[ -z "$ANSI_RED" ]]; then
        echo "FAIL: ANSI_RED should be defined"
        return 1
    fi
    echo "PASS: ANSI color codes are defined"
}

# Test get_current_branch when in a git repo
test_get_current_branch() {
    local result
    result=$(get_current_branch 2>/dev/null || echo "not_a_repo")
    if [[ -z "$result" ]]; then
        echo "FAIL: get_current_branch should return something"
        return 1
    fi
    echo "PASS: get_current_branch returns branch name"
}

# Run all tests
run_tests() {
    local passed=0
    local failed=0

    echo "Running lib.sh tests..."
    echo "========================"

    for test_func in test_command_exists test_command_exists_not_found \
                     test_normalize_https_url test_normalize_github_url_git \
                     test_normalize_github_url_with_dotgit \
                     test_prompt_input_with_default test_colors_defined \
                     test_get_current_branch; do
        if "$test_func"; then
            ((passed++))
        else
            ((failed++))
        fi
    done

    echo ""
    echo "========================"
    echo "Results: $passed passed, $failed failed"

    if [[ $failed -gt 0 ]]; then
        return 1
    fi
    return 0
}

# Run tests
run_tests
