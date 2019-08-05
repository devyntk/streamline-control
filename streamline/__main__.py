import gi
gi.require_version('Gtk', '3.0')
from gi.repository import Gtk
from streamline.ConfigWindow import ConfigWindow

win = ConfigWindow()
win.connect("destroy", Gtk.main_quit)
win.show_all()
Gtk.main()
