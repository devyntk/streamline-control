from gi.repository import Gtk, Gio
from streamline.ConfigWindow import ConfigWindow


class MainWindow(Gtk.ApplicationWindow):

    def __init__(self, *args, **kwargs):
        Gtk.ApplicationWindow.__init__(self, *args, title="Streamline Control", **kwargs)
        self.set_border_width(10)
        self.set_default_size(600, 400)

        hb = Gtk.HeaderBar()
        hb.set_show_close_button(True)
        hb.props.title = "Streamline Control"
        self.set_titlebar(hb)

        self.add(Gtk.TextView())
        self.present()

        self.ConfigWindow = ConfigWindow(application=self.get_application())
        self.ConfigWindow.present()
