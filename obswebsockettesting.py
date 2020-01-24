from streamline.ConfigWindow import ConfigItem, logger
from streamline.OBSWebsocket import OBSWebsocket

config = [ConfigItem("event_code", "ratelimittest"), ConfigItem("scorekeeper_ip", "127.0.0.1:80"), ConfigItem("twitch_key", "blahblah")]

obs_ws = OBSWebsocket(config)

obs_ws.switch_scenes()

logger.debug("Scenes switched!")
