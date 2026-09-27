# Implementation Plan: Release notes export

**Branch**: `001-release-notes` | **Date**: 2026-09-27 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/001-release-notes/spec.md`

## Summary

A command reads the merged changes between two tags and writes them as one markdown file, per the [feature specification](spec.md#requirements-mandatory).

## Technical Context

**Language/Version**: POSIX sh

**Primary Dependencies**: git

**Testing**: a scratch repository with two tags

## Constitution Check

The plan meets both principles of the [constitution](../../.specify/memory/constitution.md): every entry comes from a merged change, and the export is one command. The tag rule it relies on is specs/001-release-notes/spec.md §Assumptions.

## Project Structure

### Documentation (this feature)

- `spec.md`, this plan, and [tasks.md](tasks.md).
