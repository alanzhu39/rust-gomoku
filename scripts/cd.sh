#!/usr/bin/env bash

# Install dependencies
cargo install --path .

# Restart service
sudo systemctl restart rust-gomoku.service

