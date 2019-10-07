import gi
gi.require_version("Gtk", "3.0")
import sys
from streamline.Application import Application


app = Application()
app.run(sys.argv)
