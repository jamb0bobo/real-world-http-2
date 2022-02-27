#!/usr/bin/env bash

rustc --version

# git config
git config --global user.email '100462573+jamb0bobo@users.noreply.github.com'
git config --global user.name 'jamb0bobo'
git config --global core.editor 'vim'
git config --global color.ui 'auto'
git config --global grep.linenumber 'true'
git config --global push.default 'simple'
git config --global rebase.autosquash 'true'
git config --global alias.co 'checkout'
git config --global alias.ci 'commit'
git config --global alias.sb 'status -sb'
git config --global alias.pushf 'push --force-with-lease'

# ssh-agent
mkdir -p ~/.ssh/
curl -s -o ~/.ssh/id_ed25519 "https://github.com/$(git config --global --get user.name).keys"

# cargo tools
cargo install --quiet cargo-edit
