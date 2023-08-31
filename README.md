# Rust + Node.js + Docker Project

## Status: Work in Progress

### Table of Contents
1. [Overview](#overview)
2. [Kanban Board](#kanban-board)
3. [Problem Statements](#problem-statements)
4. [Root Cause Analysis](#root-cause-analysis)
5. [4 Whys](#4-whys)
6. [`.gitignore` File](#gitignore-file)
7. [Next Steps](#next-steps)

---

## Overview
This project involves a backend written in Rust, a frontend in Node.js, and containerization through Docker.

---

## Kanban Board
- **To Do**: Debugging, Docker image optimization
- **Doing**: Rust code troubleshooting, `.gitignore` setup
- **Done**: Initial setup, Dependency Installation

---

## Problem Statements
- Various build and runtime errors in the Rust code and Docker setup.
- Need for a `.gitignore` file.

---

## Root Cause Analysis
- Rust: Errors due to mismatched versions and configurations.
- Docker: Errors due to inadequate setup of shared libraries and dependencies.
- Lack of `.gitignore` leads to potential commit of unwanted files.

---

## 4 Whys
1. What languages and tools are in use?
    - Rust, Node.js, Docker
2. Why is a `.gitignore` necessary?
    - To keep the repo clean and secure by ignoring files that shouldn't be versioned.
3. What types of files should be ignored?
    - Compiled binaries, dependency folders, environment variables, and Docker build artifacts.
4. Where should the `.gitignore` file be placed?
    - Root of the project directory.

---

## `.gitignore` File
The `.gitignore` file should contain the following:

\`\`\`gitignore
# Rust
/target
**/*.rs.bk

# Node
/node_modules
/frontend/node_modules
npm-debug.log
yarn-error.log

# Docker
Dockerfile.*.bak
*.dockerignore
.dockerignore
.docker/

# Environment files
.env

# IDE - Visual Studio Code
.vscode/

# Miscellaneous
.DS_Store
\`\`\`

---

## Next Steps
1. Debug the Rust code to resolve build and runtime issues.
2. Optimize the Docker setup to include all necessary shared libraries.
3. Perform comprehensive testing.
4. Implement additional features and optimizations.
