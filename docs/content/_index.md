---
date: 2021-11-06
title: ":house: Home"
---

[![build](https://github.com/tamada/sibling/actions/workflows/build.yaml/badge.svg)](https://github.com/tamada/sibling/actions/workflows/build.yaml)
[![Coverage Status](https://coveralls.io/repos/github/tamada/sibling/badge.svg?branch=main)](https://coveralls.io/github/tamada/sibling?branch=main)

[![codebeat badge](https://codebeat.co/badges/aef821a8-27ef-45ec-af37-9bf67a427837)](https://codebeat.co/projects/github-com-tamada-sibling-main)
[![Go Report Card](https://goreportcard.com/badge/github.com/tamada/sibling)](https://goreportcard.com/report/github.com/tamada/sibling)

[![License](https://img.shields.io/badge/License-WTFPL-green.svg)](https://github.com/tamada/sibling/blob/master/LICENSE)
[![Version](https://img.shields.io/badge/Version-1.1.0--beta2-green.svg)](https://github.com/tamada/sibling/releases/tag/v1.1.0-beta2)

## :speaking_head: Description

When a directory has too may sub directories, we are tiresome to traverse whole of sub directories.
Because, sometimes we lose where we are.
Ideally, we move directory by specifying ‘next’ or ‘previous,' not directory name.

The command like following makes us tired :-1:.

    cd ../next_directory_name

We should type command like below :+1:.

    cdnext

For this, I implemented `sibling`.

## Table of Contents

- [:house: Home](#)
  - [:speaking_head: Description](#-description)
- [:runner: Usage](usage)
  - [:cool: Utilities (bash)](usage/#-utilities-bash)
- [:anchor: Installation](install)
  - [:beer: Homebrew](install/#-homebrew)
  - [Go lang](install/#go-lang)
  - [:hammer_and_wrench: Install from source](install/#-install-from-source)
  - [:briefcase: Requirements](install/#-requirements)
- [:smile: About](about)
  - [:scroll: License](about/#-license)
  - [:man_office_worker: Developers :woman_office_worker:](about/#-developers-)
