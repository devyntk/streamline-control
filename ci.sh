#!/bin/bash
if [ "$TRAVIS_OSNAME" == 'osx' ];
then
  curl pyenv.run | bash;
else
  cd /opt/pyenv/plugins/python-build/../.. && git pull origin master && git pull && cd -;
fi
pyenv install 3.6.8
pyenv global 3.6.8
