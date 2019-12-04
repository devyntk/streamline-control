#!/bin/bash
brew upgrade pyenv || echo "Not macOS"
(cd /opt/pyenv/plugins/python-build/../.. && git pull origin master && cd -) || echo "Not Ubuntu"
export PATH="/opt/pyenv/bin:$PATH"
eval "$(pyenv init -)"
eval "$(pyenv virtualenv-init -)"
env PYTHON_CONFIGURE_OPTS="--enable-shared" pyenv install 3.6.8
pyenv global 3.6.8
pip3 install -Ur requirements.txt
pip3 install pyinstaller
pyinstaller streamline/__main__.py -F
mv dist/* "dist/streamline-control-$BUILDOS"
python3 obswebsockettesting.py
