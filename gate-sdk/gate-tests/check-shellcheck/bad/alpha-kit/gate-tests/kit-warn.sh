#!/usr/bin/env bash
# Fixture script: carries a warning-severity ShellCheck finding (SC2164 — cd
# without || exit); the gate must REJECT this case.
cd /nonexistent-fixture-dir
echo "after cd"
