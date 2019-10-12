#!/bin/bash
brew update && brew upgrade pyenv || echo "Not macOS"
(cd /opt/pyenv/plugins/python-build/../.. && git pull origin master && cd -) || echo "Not Ubuntu"
pyenv install 3.6.8
pyenv global 3.6.8
pip install pycairo
pip install -Ur requirements.txt
pip install pyinstaller
pyinstaller streamline/__main__.py -F
mv dist/* "dist/streamline-control-$TRAVIS_OS_NAME"
