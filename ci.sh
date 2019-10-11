#!/bin/bash
brew update && brew upgrade pyenv || echo "Not macOS"
(cd /opt/pyenv/plugins/python-build/../.. && git pull origin master && cd -) || echo "Not Ubuntu"
pyenv install 3.6.8
pyenv global 3.6.8
