---
name: scaffold-react-singlefile
description: Scaffold a new Vite + React single-file app from the ctracey/template_vite-react-singlefile template. Builds to a single HTML file with inline CSS and JS. Use when the user wants a new React single-file project, or invokes /scaffold-react-singlefile.
allowed-tools: Bash, Read, Glob
---

# Scaffold React Single-File App

Create a new project from the `ctracey/template_vite-react-singlefile` template.

## Prerequisites

- Current directory should be empty (or nearly empty)
- SSH access to github.com configured

## Steps

### 1. Verify the directory

```bash
ls -A
```

If the directory has existing project files (package.json, src/, etc.), warn the user and confirm before proceeding.

### 2. Init and pull template

```bash
git init -b main
git remote add template git@github.com:ctracey/template_vite-react-singlefile.git
git pull template main
git remote remove template
```

This pulls the template contents then removes the template remote so the repo is independent.

### 3. Create GitHub repo (ask first)

Ask the user:
- Repo name (default: current directory name)
- Visibility: public or private (default: private)
- Org: which GitHub org/user (default: ctracey)

Then:

```bash
gh repo create <org>/<name> --<visibility> --source=. --remote=origin --push
```

If the user declines, skip this step — they can add a remote later.

### 4. Install dependencies

```bash
npm install
```

### 5. Verify

```bash
npm test
```

Run the tests to confirm everything works.

### 6. Summary

Report what was created:
- Local git repo with template contents
- GitHub repo (if created) with URL
- Dependencies installed
- Test results

## What the template includes

- **Vite** with `vite-plugin-singlefile` — builds everything into one HTML file
- **React 18** with JSX
- **Vitest** with jsdom and Testing Library
- **GitHub Actions** CI workflow
- **Automated version bumping** on PR merge
