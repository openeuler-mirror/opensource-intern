# -*- coding: utf-8 -*-

import socket
import signal
import sys
import os
from sniffer import Sniffer, Identifier, log
import warnings

def quit(signum, frame):
    sys.exit(0)


if __name__ == '__main__':
    signal.signal(signal.SIGINT, quit)
    warnings.filterwarnings('ignore')
    sys.stdout = open(os.devnull, 'w')
    
    sn = Sniffer()
    sn.start()

    try:
        server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        server_socket.bind(('127.0.0.1', 8000))
        server_socket.listen(5)  # max number of connection
        log("Main loop listener start")
        while True:
            soc, addr = server_socket.accept()
            client = Identifier(soc)
            client.start()
    except OSError:
        log("Socket address already in use", True)