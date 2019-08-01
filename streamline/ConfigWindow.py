# from PyQt5.QtCore import pyqtSignal, pyqtSlot
# from PyQt5.QtWidgets import QWidget, QVBoxLayout, QTextEdit, QMainWindow, QErrorMessage
# import json
# from streamline import app
# import time
# import sys
#
# class ConfigWindow(QMainWindow):
#
#     def __init__(self):
#         super().__init__()
#         self.setWindowTitle('Streamline-Config Initial Setup')
#
#         layout = QVBoxLayout()
#
#         self.textbox = QTextEdit()
#         self.textbox.setReadOnly(True)
#         layout.addWidget(self.textbox)
#
#         widget = QWidget()
#         widget.setLayout(layout)
#         self.setCentralWidget(widget)
#         self.show()
#         self.read_config()
#
#     def read_config(self):
#         app.processEvents()
#         try:
#             self.textbox.append("Opening local config file")
#             local_config_file = open('config.json')
#
#         except FileNotFoundError:
#             config = {
#                 "type": "local",
#                 "remote": {
#                     "url": "http://example.com",
#                     "auth": {
#                         "user": "username",
#                         "pass": "password"
#                     }
#                 },
#                 "local_file": "~/Downloads/Your_File",
#                 "encryption_key": None
#             }
#             f = open('config.json', 'w+')
#             json.dump(config, f, indent=1)
#             f.close()
#             msg = QErrorMessage(self)
#             msg.showMessage(
#                 'No config file found. Please edit "config.json" in the "streamline-control" folder to configure.')
#             msg.accepted.connect(sys.exit)
#
#         try:
#             self.textbox.append("Decoding local config file")
#             local_config = json.load(local_config_file)
#         except json.JSONDecodeError:
#             msg = QErrorMessage()
#             msg.showMessage(
#                 "Invalid JSON file. Please either fix your JSON file or delete it and run this app again to " +
#                 "regenerate a valid file.")
#             app.quit()
#         app.processEvents()
#         time.sleep(5)
#         self.textbox.append("Test")