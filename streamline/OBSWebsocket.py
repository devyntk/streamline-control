import json
import time
import websocket
import obswsrc


class OBSWebsocket:
    def __init__(self, host, event_key):
        sk_url = f"ws://{host}/api/v2/stream/?code={event_key}"
        #websocket.enableTrace(True)
        self.sk_websocket = websocket.create_connection(sk_url)
        self.obs_websocket = obswsrc.OBSWS('localhost', 4444, "orangealliance")

    def scorekeeper_replay_buffer_trigger(self):
        while True:
            data = self.sk_websocket.recv_frame().data.decode("utf-8")
            parsed_data = json.loads(data)
            if parsed_data['updateType'] == "MATCH_START":
                time.sleep(158)
                print(parsed_data['payload']['shortName'])
        # stream_setup =
