from gi.repository import Gtk, GLib
import threading
import json


class ConfigWindow(Gtk.Window):

    def __init__(self):
        Gtk.Window.__init__(self, title="Streamline Config Setup")
        self.set_border_width(10)

        vbox = Gtk.Box(orientation=Gtk.Orientation.VERTICAL, spacing=6)
        self.add(vbox)

        self.textbox = Gtk.TextView()
        self.textbox.set_editable(False)
        self.textbox.set_cursor_visible(False)
        vbox.pack_start(self.textbox, True, True, 0)

        self.progressbar = Gtk.ProgressBar()
        vbox.pack_start(self.progressbar, True, True, 0)

        thread = threading.Thread(target=self.load_config)
        thread.daemon = True  # Make sure program exits even if only this thread is still running
        thread.start()

    def append_text(self, text):
        buffer = self.textbox.get_buffer()
        end_iter = buffer.get_end_iter()
        buffer.insert(end_iter, text, len(text))

    def show_error(self, title, subtitle):
        msg = Gtk.MessageDialog(self, 0, Gtk.MessageType.ERROR, Gtk.ButtonsType.CLOSE, title)
        msg.format_secondary_text(subtitle)
        msg.run()
        Gtk.main_quit()

    def load_config(self):
        try:
            GLib.idle_add(self.append_text, "Reading local config file\n")
            local_config_file = open('config.json')
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
            json.dump(config, f, indent=1)
            f.close()
            GLib.idle_add(self.show_error, "No config file", 'No config file found. Please edit "config.json" in the '
                                                             '"streamline-control" folder to configure.')
            return

        try:
            GLib.idle_add(self.append_text, "Decoding local config file\n")
            local_config = json.load(local_config_file)
        except json.JSONDecodeError:
            GLib.idle_add(self.show_error, "Invalid JSON File", "Please either fix your JSON file or delete it and run "
                                                                "this app again to regenerate a valid file.")
            return
