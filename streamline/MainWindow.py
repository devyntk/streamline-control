import _thread
from gi.repository import Gtk, Gio
from .OBSWebsocket import OBSWebsocket


class MainWindow(Gtk.ApplicationWindow):

    def __init__(self, config, *args, **kwargs):
        Gtk.ApplicationWindow.__init__(self, *args, title="Streamline Control", **kwargs)
        self.set_border_width(10)
        self.set_default_size(600, 400)
        self.config = config

        self.obs_ws = OBSWebsocket(config=config)

        hb = Gtk.HeaderBar()
        hb.set_show_close_button(True)
        hb.props.title = "Streamline Control"
        self.set_titlebar(hb)

        self.hbox = Gtk.Box(spacing=0, orientation=Gtk.Orientation.HORIZONTAL)

        self.stack = Gtk.Stack()
        self.stack.set_transition_type(Gtk.StackTransitionType.SLIDE_UP_DOWN)
        self.stack.set_transition_duration(1000)
        self.hbox.add(self.stack)

        self.music = self.music_stack()
        self.stack.add_titled(self.music, "music", "Music")

        self.streaming = self.streaming_stack()
        self.stack.add_titled(self.streaming, "streaming", "Streaming")

        self.sk = self.scorekeeper_stack()
        self.stack.add_titled(self.sk, "sk", "Scorekeeping")

        self.sidebar = Gtk.StackSidebar()
        self.sidebar.set_stack(self.stack)
        self.hbox.add(self.sidebar)

        self.add(self.hbox)

        _thread.start_new_thread(self.obs_ws.scorekeeper_replay_buffer_trigger, ())
        self.present()

    def music_stack(self):
        vbox = Gtk.Box(spacing=5, orientation=Gtk.Orientation.HORIZONTAL)
        vbox.add(Gtk.Label("Spotify controls coming soon"))
        return vbox

    def streaming_stack(self):
        vbox = Gtk.Box(spacing=5, orientation=Gtk.Orientation.VERTICAL)

        vbox.add(Gtk.Label("Configure your scenes in OBS, then press the start button"))
        go_live_button = Gtk.Button("Start")
        go_live_button.connect("clicked", self.obs_ws.go_live)
        vbox.add(go_live_button)
        switch_button = Gtk.Button("Switch Scenes")
        switch_button.connect("clicked", self.obs_ws.switch_scenes)
        vbox.add(switch_button)
        vbox.add(Gtk.Label("Upload to YouTube in real time (TODO)"))
        vbox.add(Gtk.Switch())
        return vbox

    def scorekeeper_stack(self):
        vbox = Gtk.Box(spacing=5, orientation=Gtk.Orientation.HORIZONTAL)
        vbox.add(Gtk.Label("Not really sure what goes here"))
        return vbox
