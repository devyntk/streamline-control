from gi.repository import Gtk, Gio


class MainWindow(Gtk.ApplicationWindow):

    def __init__(self, config, *args, **kwargs):
        Gtk.ApplicationWindow.__init__(self, *args, title="Streamline Control", **kwargs)
        self.set_border_width(10)
        self.set_default_size(600, 400)
        self.config = config

        hb = Gtk.HeaderBar()
        hb.set_show_close_button(True)
        hb.props.title = "Streamline Control"
        self.set_titlebar(hb)

        self.hbox = Gtk.Box(spacing=0,orientation=Gtk.Orientation.HORIZONTAL)

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
        self.present()

       # self.ConfigWindow = ConfigWindow(application=self.get_application())
        #self.ConfigWindow.present()

    def music_stack(self):
        vbox = Gtk.Box(spacing = 5, orientation= Gtk.Orientation.HORIZONTAL)
        return vbox

    def streaming_stack(self):
        vbox = Gtk.Box(spacing=5, orientation=Gtk.Orientation.HORIZONTAL)
        return vbox

    def scorekeeper_stack(self):
        vbox = Gtk.Box(spacing=5, orientation=Gtk.Orientation.HORIZONTAL)
        return vbox
