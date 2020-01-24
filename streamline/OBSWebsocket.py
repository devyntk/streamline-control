import datetime
import json
import os
import time
import websocket
from obswebsocket import obsws, requests


class OBSWebsocket:
    def __init__(self, config):
        event_key = ""
        host = ""
        twitch_key = ""
        obs_port = 4444

        for config_item in config:
            if config_item.name == "event_code":
                event_key = config_item.value
            elif config_item.name == "scorekeeper_ip":
                host = config_item.value
            elif config_item.name == "twitch_key":
                twitch_key = config_item.value
            elif config_item.name == "obs_port":
                obs_port = config_item.value
        sk_url = f"ws://{host}/api/v2/stream/?code={event_key}"
        self.sk_websocket = websocket.create_connection(sk_url)
        self.twitch_key = twitch_key
        self.obs_websocket = obsws('localhost', obs_port, "orangealliance")
        self.obs_websocket.connect()
        self.event_key = event_key

    def two_digit_date(self, number):
        if number >= 10:
            return str(number)
        else:
            return f"0{number}"

    def switch_scenes(self, second_arg_so_gtk_is_happy=None):
        self.obs_websocket.call(requests.TransitionToProgram("cut"))

    def go_live(self, second_arg_so_gtk_is_happy):
        calls = [requests.SetRecordingFolder(os.getcwd()),
                 requests.StartReplayBuffer(),
                 requests.StartRecording(),
                 requests.StartStreaming(stream_settings_key=self.twitch_key)]
        for call in calls:
            self.obs_websocket.call(call)

    def trigger_replay_save(self, name):
        bufsave_call = requests.SaveReplayBuffer()
        self.obs_websocket.call(bufsave_call)
        now = datetime.datetime.now()
        # Filename: Replay 2019-12-04 13-56-27.mkv
        formed_string = f"Replay {now.year}-{self.two_digit_date(now.month)}-{self.two_digit_date(now.day)} " \
                        f"{self.two_digit_date(now.hour)}-{self.two_digit_date(now.minute)}-{self.two_digit_date(now.second + 1)}.mkv"
        os.system(f'mv "{formed_string}" "{self.event_key}/{self.event_key}-{name}.mkv"')

    def scorekeeper_replay_buffer_trigger(self):
        while True:
            data = self.sk_websocket.recv_frame().data.decode("utf-8")
            parsed_data = json.loads(data)
            if parsed_data['updateType'] == "MATCH_START":
                time.sleep(158)
                self.trigger_replay_save(parsed_data['payload']['shortName'])
