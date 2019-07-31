from PyQt5.QtWidgets import QMainWindow, QPushButton


class MainWindow(QMainWindow):

    def __init__(self):
        super().__init__()

        self.statusBar().showMessage('Ready')

        self.setGeometry(300, 300, 300, 220)
        self.setWindowTitle('Streamline Control')
        self.show()
