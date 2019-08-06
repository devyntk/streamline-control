from gi.repository import Gtk, GLib
import threading
import json
import os
import sys


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

    def append_text_async(self, text):
        buffer = self.textbox.get_buffer()
        end_iter = buffer.get_end_iter()
        buffer.insert(end_iter, text, len(text))

    def show_error_async(self, title, subtitle):
        msg = Gtk.MessageDialog(self, 0, Gtk.MessageType.ERROR, Gtk.ButtonsType.CLOSE, title)
        msg.format_secondary_text(subtitle)
        msg.run()
        msg.destroy()
        # Gtk.main_quit()

    def append_text(self, text, append="\n"):
        GLib.idle_add(self.append_text_async, f"{text}{append}")

    def show_error(self, title, subtitle):
        GLib.idle_add(self.show_error_async, title, subtitle)

    def load_config(self):
        branch = os.popen("git branch | grep \* | cut -d ' ' -f2").read()
        if "master" in branch:
            self.append_text("Checking for updates")
            update = os.popen("git pull").read()
            self.append_text(update, "")
            if 'Already up to date.' not in update:
                Gtk.main_quit()
                os.execl(sys.executable, os.path.abspath(__file__), *sys.argv)
                return
        elif "fatal: not a git repository" in branch:
            self.show_error("Incorrect install method", "The method of install is incorrect such that the app cannot "
                                                        "update. Contact the developers for assistance.")
        else:
            self.append_text("Git output" + branch)
            self.append_text("Developer install detected. No updating.")

        try:
            self.append_text("Reading local config file")
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
                "local_file": "~/Downloads/event.json",
                "encryption_key": None
            }
            f = open('config.json', 'w+')
            json.dump(config, f, indent=1)
            f.close()
            self.show_error( "No config file", 'No config file found. Please edit "config.json" in the '
                                               '"streamline-control" folder to configure.')
            return

        try:
            self.append_text("Decoding local config file")
            local_config = json.load(local_config_file)
            local_config_file.close()
        except json.JSONDecodeError:
            self.show_error("Invalid JSON File", "Please either fix your JSON file or delete it and run this app again "
                                                 "to regenerate a valid file.")
            return

        if local_config['type'] == 'local':
            try:
                self.append_text("Opening local remote config file")
                remote_config_file = open(local_config['local_file'], 'r')
            except FileNotFoundError:
                self.show_error("No local file found", f"No file found at {local_config['local_file']}. Try a "
                                                       f"different path.")
                return
        elif local_config['type'] == 'url':
            raise Exception('TODO')  # TODO: add remote config (need a place to test first)

        try:
            self.append_text("Decoding local config file")
            remote_config = json.load(remote_config_file)
            remote_config_file.close()
        except json.JSONDecodeError:
            self.show_error("Invalid JSON File", "Please either fix your JSON file or delete it and run this app again "
                                                 "to regenerate a valid file.")
            return

