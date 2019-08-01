from PyQt5.QtWidgets import QApplication, QErrorMessage
import sys
import json

app = QApplication(sys.argv)
try:
    local_config = open('config.json')
except FileNotFoundError:
    config = {
        "type": "local",
        "remote": {
            "url": "http://example.com",
            "auth": {
                "user": "username",
                "pass": "password"
            }
        },
        "local_file": "~/Downloads/Your_File",
        "encryption_key": None
    }
    f = open('config.json', 'w+')
    f.write(json.dumps(config))
    f.close()
    msg = QErrorMessage()
    msg.showMessage('No config file found. Please edit "config.json" in the "streamline-control" folder to configure.')

app.exec_()
