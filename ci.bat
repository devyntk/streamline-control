@echo off
choco install python3
export PATH="/c/Python37:/c/Python37/Scripts:$PATH"
pip3 install pycairo
pip3 install -Ur requirements.txt
pip3 install pyinstaller
pyinstaller streamline/__main__.py -F
mv dist/* "dist/streamline-control-windows.exe"
