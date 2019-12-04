from streamline.OBSWebsocket import OBSWebsocket

obs_ws = OBSWebsocket("127.0.0.1:80", "ratelimittest")

obs_ws.trigger_replay_save("Debug")

print("Save triggered!")
