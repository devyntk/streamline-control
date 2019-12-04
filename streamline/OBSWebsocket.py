import json
import time
import websocket
import obswsrc
from obswsrc.requests import SaveReplayBuffer


class OBSWebsocket:
    def __init__(self, host, event_key):
        sk_url = f"ws://{host}/api/v2/stream/?code={event_key}"
        self.sk_websocket = websocket.create_connection(sk_url)
        self.obs_websocket = obswsrc.OBSWS('localhost', 4444, "orangealliance")

    def trigger_replay_save(self, name):
        bufsave_call = obswsrc.requests
        print("TODO: Call replay buffer save on OBS websocket and save with", name)

    def scorekeeper_replay_buffer_trigger(self):
        while True:
            data = self.sk_websocket.recv_frame().data.decode("utf-8")
            parsed_data = json.loads(data)
            if parsed_data['updateType'] == "MATCH_START":
                time.sleep(158)
                self.trigger_replay_save(parsed_data['payload']['shortName'])
