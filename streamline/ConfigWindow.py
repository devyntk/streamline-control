from gi.repository import Gtk, GLib
import threading
import json
import os
import sys
import requests


class ConfigWindow(Gtk.ApplicationWindow):

    def __init__(self, *args, **kwargs):
        Gtk.ApplicationWindow.__init__(self, *args, title="Streamline Config Setup", **kwargs)
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
        self.get_application().quit()

    def append_text(self, text, append="\n"):
        GLib.idle_add(self.append_text_async, f"{text}{append}")

    def show_error(self, title, subtitle=""):
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
        elif len(branch.split()) > 1:
            self.append_text("Unknown git output, please contact the developers:\n"+branch)
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
            self.show_error("No config file", 'No config file found. Please edit "config.json" in the '
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
                remote_config_file = open(os.path.expanduser(local_config['local_file']), 'r')
            except FileNotFoundError:
                self.show_error("No local file found", f"No file found at {local_config['local_file']}. Try a "
                                                       f"different path.")
                return

            try:
                self.append_text("Decoding local config file")
                remote_config = json.load(remote_config_file)
                remote_config_file.close()
            except json.JSONDecodeError:
                self.show_error("Invalid JSON File",
                                "Please either fix your JSON file or delete it and run this app again "
                                "to regenerate a valid file.")
                return

        elif local_config['type'] == 'url':
            remote_details = local_config['remote']
            r = requests.get(remote_details['url'], auth=(remote_details['auth']['user'],
                                                          remote_details['auth']['pass']))
            # TODO: make this more applicable to different auth kinds (or no auth)
            try:
                remote_config = r.json()
            except ValueError:
                self.show_error("Invalid JSON in remote file.")
                return
        else:
            self.show_error("Unknown remote config destination type.")
            return

        if remote_config["type"] == "list":
            pass
            # TODO: Handle event lists
        elif remote_config["type"] == "event":
            self.load_event(remote_config)
        else:
            self.show_error("Unknown remote config type.")

    def load_event(self, remote_config):
        global config
        cwd = os.getcwd()
        os.mkdir(cwd+"/"+remote_config['event_code'])
        for app, url in remote_config['downloads'].items():
            os.mkdir(cwd + "/" + remote_config['event_code']+"/"+app)
            r = requests.get(url)
            with open(f"{cwd}/{remote_config['event_code']}/{app}/{app}.zip", 'wb') as f:
                f.write(r.content)
