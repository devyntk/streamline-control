from gi.repository import Gtk, GLib
import threading
import time
import json
import os
import sys
import requests
import logging
import zipfile
from streamline.MainWindow import MainWindow


logger = logging.getLogger()


class LogHandler(logging.Handler):

    def __init__(self, buffer, window):
        super().__init__()
        self.setLevel(logging.DEBUG)
        self.buffer = buffer
        self.window = window

    def emit(self, record):
        GLib.idle_add(self.window.append_text_async, record.getMessage()[:120])


class AlreadyExistsDialog(Gtk.Dialog):

    def __init__(self, parent):
        Gtk.Dialog.__init__(self, "Confirm Event Duplication", parent, 0,
                            (Gtk.STOCK_CANCEL, Gtk.ResponseType.CANCEL,
                             Gtk.STOCK_OK, Gtk.ResponseType.OK))

        self.set_default_size(200, 100)
        self.set_resizable(False)

        self.set_modal(True)

        label = Gtk.Label("""An event with this code already exists in a streamline folder. By default, streamline will \
rename this old folder and create a new one, starting the event from scratch. If you would \
like for this not to happen, and for streamline to continue with this folder as is, with \
its scorekeeper, datasync and other applications already setup, press cancel. If you would \
like to rename this folder and re-download all of the required files, press 'OK'.""")
        label.set_line_wrap(True)

        box = self.get_content_area()
        box.add(label)
        self.show_all()


class ConfirmCloseDialog(Gtk.Dialog):

    def __init__(self, parent):
        Gtk.Dialog.__init__(self, "Confirm Setup Cancellation", parent, 0,
            (Gtk.STOCK_CANCEL, Gtk.ResponseType.CANCEL,
             Gtk.STOCK_OK, Gtk.ResponseType.OK))

        self.set_default_size(200, 100)
        self.set_resizable(False)
        self.set_modal(True)

        label = Gtk.Label("""The setup process for this event is still ongoing. If you cancel now, Streamline will \
continue with a partially setup event. Press 'Cancel' to continue event setup.""")
        label.set_line_wrap(True)

        box = self.get_content_area()
        box.add(label)
        self.show_all()


class ConfigWindow(Gtk.ApplicationWindow):

    def __init__(self, application, *args, **kwargs):
        Gtk.ApplicationWindow.__init__(self, *args, title="Streamline Config Setup", **kwargs)
        self.application = application
        self.set_border_width(10)
        self.set_default_size(300, 100)

        self.vbox = Gtk.Box(orientation=Gtk.Orientation.VERTICAL, spacing=6)
        self.add(self.vbox)
        self.pause_for_debug = True
        self.textbox = Gtk.TextView()
        self.textbox.set_editable(False)
        self.textbox.set_cursor_visible(False)
        self.textbox.set_wrap_mode(Gtk.WrapMode.WORD)
        self.vbox.pack_start(self.textbox, True, True, 0)

        self.spinner = Gtk.Spinner()
        self.spinner.start()
        self.vbox.pack_end(self.spinner, True, True, 0)
        self.show_all()

        self.initial_config = None
        self.config_finalized = False
        self.final_config = []

        self.loghandler = LogHandler(self.textbox.get_buffer(), self)
        logger.addHandler(self.loghandler)

        self.thread = threading.Thread(target=self.load_config)
        self.thread.daemon = True  # Make sure program exits even if only this thread is still running
        self.thread.start()

        self.response = None

        self.connect('delete-event', self.delete_attempt)

    def delete_attempt(self, *args):
        dialog = ConfirmCloseDialog(self)
        response = dialog.run()
        dialog.destroy()
        if response == Gtk.ResponseType.CANCEL:
            self.get_application().main_window.destroy()
            return True
        elif response == Gtk.ResponseType.OK:
            return False

    def append_text_async(self, text):
        buffer = self.textbox.get_buffer()
        buffer.set_text(text)

    def show_already_exists(self):
        dialog = AlreadyExistsDialog(self)
        self.response = dialog.run()
        dialog.destroy()

    def load_config(self):
        branch = os.popen("git branch | grep \* | cut -d ' ' -f2").read()
        print("beyond popen")
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
        print("beyond git")
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
            print("Loading event")
            self.load_event(remote_config)
        else:
            logger.error("Unknown remote config type.")

    def finalize_config(self, *args):
        elements = [element for element in self.vbox]
        formed_elements = []
        use_external_sk = False
        count = 0
        while count < len(elements):
            if type(elements[count]) == Gtk.Label:
                label_text = elements[count].get_text()
                if ("scorekeeper" in label_text) and use_external_sk:
                    count += 1
                    continue
                formed_elements.append(ConfigItem(label_text, elements[count + 1]))
                count += 1
            elif type(elements[count]) == Gtk.CheckButton:
                use_external_sk = elements[count].get_active()
            count += 1
        main_window = MainWindow(application=self.application, config=formed_elements)
        main_window.show_all()
        self.destroy()

    def load_event(self, input_config):
        remote_config = GLib.idle_add(self.get_config, input_config)
        # remote_config = self.get_config(input_config)
        while not self.config_finalized:
            time.sleep(1)
            pass
        self.get_application().config = remote_config
        cwd = os.getcwd()
        try:
            os.mkdir(cwd+"/"+remote_config['event_code'])
        except FileExistsError:
            logger.error("Event folder already exists!")
            GLib.idle_add(self.show_already_exists)
            while not self.response:
                pass

            if self.response == Gtk.ResponseType.OK:
                # rename folder, create anew
                current_name = cwd+"/"+remote_config['event_code']
                new_name = f"{current_name}-old"
                logger.debug("renaming folder {} to {}".format(current_name, new_name))
                while True:
                    try:
                        os.rename(current_name, new_name)
                        break
                    except OSError:
                        new_name += "-old"
                        logging.info("-old folder already exists, renaming to {}".format(new_name))

                os.mkdir(cwd + "/" + remote_config['event_code'])

            elif self.response == Gtk.ResponseType.CANCEL:
                # keep folder as is
                logger.debug("Kept folder as is, ignore")

            self.response = None

        for app, url in remote_config['downloads'].items():
            logger.debug("downloading {} from {}".format(app, url))
            try:
                os.mkdir(cwd + "/" + remote_config['event_code']+"/"+app)
            except FileExistsError:
                logger.info("{} folder already exists, ignoring download".format(app))
                continue
            r = requests.get(url)
            with open(f"{cwd}/{remote_config['event_code']}/{app}/{app}.zip", 'wb') as f:
                f.write(r.content)
            with zipfile.ZipFile(f"{cwd}/{remote_config['event_code']}/{app}/{app}.zip", 'r') as zip_ref:
                zip_ref.extractall(f"{cwd}/{remote_config['event_code']}/{app}/")
        logger.debug("Done downloading files.")

    def get_config(self, config):
        self.initial_config = config
        processed_config = []
        for item in config:
            if type(config[item]) == dict:
                for subitem in config[item]:
                    processed_config.append(ConfigItem(f"{item}_{subitem}", config[item][subitem])) # TODO figure out how to actually add
            else:
                processed_config.append(ConfigItem(item, config[item]))
        for config_item in processed_config:
            buf = Gtk.TextBuffer()
            if str(config_item) is not None:
                buf.set_text(str(config_item.value))
                label = Gtk.Label(str(config_item.name))
                text_box = Gtk.TextView.new()
                text_box.set_buffer(buf)
                self.vbox.pack_end(text_box, expand=False, fill=False, padding=5)
                self.vbox.pack_end(label, expand=False, fill=False, padding=5)

        continue_button = Gtk.Button.new_with_label("Continue")
        continue_button.connect("clicked", self.finalize_config)

        use_external_sk = Gtk.CheckButton.new_with_label("Use external Scorekeeper")
        self.vbox.pack_end(use_external_sk, False, False, 5)

        self.vbox.pack_end(continue_button, expand=False, fill=False, padding=5)
        self.vbox.show_all()
        return self.final_config


class ConfigItem:
    def __init__(self, name, value):
        self.name = name
        self.value = value
