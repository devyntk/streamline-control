from PyQt5.QtWidgets import QApplication
from streamline.MainWindow import MainWindow
import sys

app = QApplication(sys.argv)
main = MainWindow()
sys.exit(app.exec_())
