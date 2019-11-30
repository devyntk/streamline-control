import gi
gi.require_version('Gtk', '3.0')
from gi.repository import Gtk, Gio
from streamline.ConfigWindow import ConfigWindow
import logging
import sys

logger = logging.getLogger()


class Application(Gtk.Application):

    def __init__(self, *args, **kwargs):
        super().__init__(*args, application_id="org.theorangealliance.streamline", **kwargs)
        self.main_window = None

        self.config = None

    def do_startup(self):
        Gtk.Application.do_startup(self)

        logger.setLevel(logging.DEBUG)
        ch = logging.StreamHandler(sys.stdout)
        ch.setLevel(logging.DEBUG)
        logger.addHandler(ch)

        # action = Gio.SimpleAction.new("quit", None)
        # action.connect("activate", self.on_quit)
        # self.add_action(action)

        # builder = Gtk.Builder.new_from_file("streamline/menu.xml")
        # self.set_menubar(builder.get_object('menubar'))

    def do_activate(self):

        self.config_window = ConfigWindow(application=self)
        print("Config window inited")
        self.config_window.show_all()
        print("Showing all")

    def on_quit(self):
        self.quit()
