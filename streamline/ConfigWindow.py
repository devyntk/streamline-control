from gi.repository import Gtk, GLib
import threading
import json
import os
import sys
import requests
import logging

logger = logging.getLogger()


class LogHandler(logging.Handler):

    def __init__(self, buffer, window):
        super().__init__()
        self.setLevel(logging.DEBUG)
        self.buffer = buffer
        self.window = window

    def emit(self, record):
        GLib.idle_add(self.window.append_text_async, record.getMessage())


class ConfigWindow(Gtk.ApplicationWindow):

    def __init__(self, *args, **kwargs):
        Gtk.ApplicationWindow.__init__(self, *args, title="Streamline Config Setup", **kwargs)
        self.set_border_width(10)

        self.vbox = Gtk.Box(orientation=Gtk.Orientation.VERTICAL, spacing=6)
        self.add(self.vbox)

        self.textbox = Gtk.TextView()
        self.textbox.set_editable(False)
        self.textbox.set_cursor_visible(False)
        self.vbox.pack_start(self.textbox, True, True, 0)

        self.spinner = Gtk.Spinner()
        self.spinner.start()
        self.vbox.pack_end(self.spinner, True, True, 0)
        self.show_all()

        self.loghandler = LogHandler(self.textbox.get_buffer(), self)
        logger.addHandler(self.loghandler)

        thread = threading.Thread(target=self.load_config)
        thread.daemon = True  # Make sure program exits even if only this thread is still running
        thread.start()

    def append_text_async(self, text):
        buffer = self.textbox.get_buffer()
        buffer.set_text(text)
    #
    # def show_error_async(self, title, subtitle):
    #     msg = Gtk.MessageDialog(self, 0, Gtk.MessageType.ERROR, Gtk.ButtonsType.CLOSE, title)
    #     msg.format_secondary_text(subtitle)
    #     msg.run()
    #     msg.destroy()
    #     self.get_application().quit()

    def load_config(self):
        branch = os.popen("git branch | grep \* | cut -d ' ' -f2").read()
        if "master" in branch:
            logger.info("Checking for updates")
            update = os.popen("git pull").read()
            logger.debug(update)
            if 'Already up to date.' not in update:
                logger.info("Update found, updating and restarting.")
                Gtk.main_quit()
                os.execl(sys.executable, os.path.abspath(__file__), *sys.argv)
                return
        elif "fatal: not a git repository" in branch:
            logger.critical("The method of install is incorrect such that the app cannot update. Contact the developers "
                            "for assistance.")
        elif len(branch.split()) > 1:
            logger.critical("Unknown git output, please contact the developers:\n"+branch)
        else:
            logger.warning("Git output" + branch)
            logger.warning("Developer install detected. No updating.")

        try:
            logger.info("Reading local config file")
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
            logger.error('No config file found. Please edit "config.json" in the "streamline-control" folder '
                         'to configure.')
            return

        try:
            logger.info("Decoding local config file")
            local_config = json.load(local_config_file)
            local_config_file.close()
        except json.JSONDecodeError:
            logger.error("Invalid JSON File Please either fix your JSON file or delete it and run this app again "
                         "to regenerate a valid file.")
            return

        if local_config['type'] == 'local':
            try:
                logger.info("Opening local remote config file")
                remote_config_file = open(os.path.expanduser(local_config['local_file']), 'r')
            except FileNotFoundError:
                logger.error(f"No file found at {local_config['local_file']}. Try a different path.")
                return

            try:
                logger.info("Decoding local config file")
                remote_config = json.load(remote_config_file)
                remote_config_file.close()
            except json.JSONDecodeError:
                logger.info("No JSON file found. Please either fix your JSON file or delete it and run this app again "
                            "to regenerate a valid file.")
                return

        elif local_config['type'] == 'url':
            logger.info("Getting remote local config")
            remote_details = local_config['remote']
            r = requests.get(remote_details['url'], auth=(remote_details['auth']['user'],
                                                          remote_details['auth']['pass']))
            # TODO: make this more applicable to different auth kinds (or no auth)
            try:
                remote_config = r.json()
            except ValueError:
                logger.error("Invalid JSON in remote file.")
                return
        else:
            logger.error("Unknown remote config destination type.")
            return

        if remote_config["type"] == "list":
            pass
            # TODO: Handle event lists
        elif remote_config["type"] == "event":
            self.load_event(remote_config)
        else:
            logger.error("Unknown remote config type.")

    def load_event(self, remote_config):
        global config
        cwd = os.getcwd()
        try:
            os.mkdir(cwd+"/"+remote_config['event_code'])
        except FileExistsError:
            logger.error("Event folder already exists!")
            return  # TODO: make this actually prompt the user somehow rather than fail silently
        for app, url in remote_config['downloads'].items():
            logger.debug("downloading {} from {}".format(app, url))
            os.mkdir(cwd + "/" + remote_config['event_code']+"/"+app)
            r = requests.get(url)
            with open(f"{cwd}/{remote_config['event_code']}/{app}/{app}.zip", 'wb') as f:
                f.write(r.content)
        logger.debug("Done downloading files.")
