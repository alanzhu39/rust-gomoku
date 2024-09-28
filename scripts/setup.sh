#!/usr/bin/env bash

# Copy service spec to system directory
sudo cp scripts/rust-gomoku.service /etc/systemd/system/

sudo systemctl daemon-reload
sudo systemctl enable rust-gomoku.service
sudo systemctl start rust-gomoku.service

