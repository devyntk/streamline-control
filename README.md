# Streamline-Control

This is the main app developed for the StreamlineOS project, a project centered around automatic running of 
[FIRST Tech Challenge](https://www.firstinspires.org/robotics/ftc) events. Written using GTK and PyGObject, it's designed to
work on Ubuntu, however will most likely work on other Linux distributions and Windows & macOS.

# Running Streamline-Control

Streamline-Control is a python package, meaning that you can launch it with the following syntax:

    python -m streamline
    
Make sure you've installed the requirements with `pip -Ur requirements.txt` before running the software. Everything else
should either auto install itself on the client PC or will tell you need it installed (like OBS).

# Remote Config Files

The majority of the way Streamline works is to pull from a online config file for that week's event and automatically
configure everything in the system based off of that online config file. The local config file (config.json) specifies 
the online URL for this remote config file, as well as any authentication for it. This config file contains several
important keys that need to be kept secret, such as your twitch streaming key, your TOA DataSync key, and potentially more.
As such, it's recommended you configure your webserver to require a user and pass to access this file, which is supported
in config.json. During testing, you can also point it to a local file.

An example remote config file is included in the repository as `remote_config.json`.

# License

Every part of this project is licensed under the MIT license, included in this repository.