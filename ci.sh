#!/bin/bash
brew upgrade pyenv || echo "Not macOS"
(cd /opt/pyenv/plugins/python-build/../.. && git pull origin master && cd -) || echo "Not Ubuntu"
export PATH="/opt/pyenv/bin:$PATH"
echo $PATH
pyenv install 3.6.8
pyenv global 3.6.8
pip3 install pycairo
pip3 install -Ur requirements.txt
pip3 install pyinstaller
pyinstaller streamline/__main__.py -F
mv dist/* "dist/streamline-control-$TRAVIS_OS_NAME"
