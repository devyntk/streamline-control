import datetime
import json
import os
import time
import websocket
from obswebsocket import obsws, requests


class OBSWebsocket:
    def __init__(self, host, event_key):
        sk_url = f"ws://{host}/api/v2/stream/?code={event_key}"
        self.sk_websocket = websocket.create_connection(sk_url)
        self.obs_websocket = obsws('localhost', 4444, "orangealliance")
        self.obs_websocket.connect()
        self.event_key = event_key

    def two_digit_date(self, number):
        if number >= 10:
            return str(number)
        else:
            return f"0{number}"

    def trigger_replay_save(self, name):
        bufsave_call = requests.SaveReplayBuffer()
        self.obs_websocket.call(bufsave_call)
        now = datetime.datetime.now()
        # Filename: Replay 2019-12-04 13-56-27.mkv
        formed_string = f"Replay {now.year}-{self.two_digit_date(now.month)}-{self.two_digit_date(now.day)} " \
                        f"{self.two_digit_date(now.hour)}-{self.two_digit_date(now.minute)}-{self.two_digit_date(now.second + 1)}.mkv"
        print(formed_string)
        os.system(f'mv "{formed_string}" "{self.event_key}/{self.event_key}-{name}.mkv"')

    def scorekeeper_replay_buffer_trigger(self):
        while True:
            data = self.sk_websocket.recv_frame().data.decode("utf-8")
            parsed_data = json.loads(data)
            if parsed_data['updateType'] == "MATCH_START":
                time.sleep(158)
                self.trigger_replay_save(parsed_data['payload']['shortName'])
