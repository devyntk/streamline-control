import json
import time
import websocket
from obswebsocket import obsws, requests


class OBSWebsocket:
    def __init__(self, host, event_key):
        sk_url = f"ws://{host}/api/v2/stream/?code={event_key}"
        self.sk_websocket = websocket.create_connection(sk_url)
        self.obs_websocket = obsws('localhost', 4444, "orangealliance")
        self.obs_websocket.connect()

    def trigger_replay_save(self, name):
        bufsave_call = requests.SaveReplayBuffer()
        self.obs_websocket.call(bufsave_call)
        print("TODO: Rename to", name)

    def scorekeeper_replay_buffer_trigger(self):
        while True:
            data = self.sk_websocket.recv_frame().data.decode("utf-8")
            parsed_data = json.loads(data)
            if parsed_data['updateType'] == "MATCH_START":
                time.sleep(158)
                self.trigger_replay_save(parsed_data['payload']['shortName'])
