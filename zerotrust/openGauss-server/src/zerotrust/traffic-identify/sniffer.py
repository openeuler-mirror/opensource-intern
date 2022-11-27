# -*- coding: utf-8 -*-

import os
import sys
from threading import Thread
import time
import pandas as pd
import numpy as np
from scapy.all import *
from apply import identify

# sys.stdout = open(os.devnull, 'w')

pkts = []
pkts_num_max = 100000
pcap_file = '/home/omm/zt_pkts.pcap'
csv_file = '/home/omm/zt_pkts.csv'
logfile = '/home/omm/zerotrust.log'
columns = ['dst_port',
           'bwd_pkt_len_min',
           'subflow_fwd_byts',
           'totlen_fwd_pkts',
           'fwd_pkt_len_mean',
           'bwd_pkt_len_std',
           'flow_duration',
           'flow_iat_std',
           'init_fwd_win_byts',
           'bwd_pkts_s',
           'fwd_psh_flags',
           'bwd_psh_flags',
           'pkt_size_avg']
# iface name is up to system
iface = "eth0"

def log(msg, err=False):
    timestamp = time.strftime('%Y-%m-%d %H:%M:%S', time.localtime())
    log_str = f'{timestamp} [ZerotrustTi] '
    if err == False:
        log_str += f'LOG: {msg}\n'
    else:
        log_str += f'ERROR: "{msg}"\n'
    with open(logfile, 'a') as f:
        f.write(log_str)


def data_pre_process():
    """data cleansing"""
    global csv_file, columns
    df = pd.read_csv(csv_file, index_col=None)
    df = df[columns]
    df['fwd_psh_flags'] = df['fwd_psh_flags'] + df['bwd_psh_flags']
    df.drop('bwd_psh_flags', axis=1, inplace=True)
    df.rename(columns={'fwd_psh_flags': 'psh_flag_count'}, inplace=True)
    df.drop('dst_port', axis=1, inplace=True)
    return df


class Identifier(Thread):
    """
    to handle traffic packet
    """

    def __init__(self, soc):
        """
        :type soc: socket.socket
        """
        super().__init__()
        self.soc = soc

    def send(self, n):
        self.soc.send(n.encode())
        self.soc.close()

    def run(self):
        global pkts, pcap_file, csv_file
        res = '1'
        if len(pkts) == 0:
            log('No traffic to identify')
        else:
            wrpcap(pcap_file, pkts)
            os.system(f"cicflowmeter -f '{pcap_file}' -c '{csv_file}'")
            df = data_pre_process()
            log('Ready to identify traffic')
            data = np.array(df)
            totality = len(data)
            normality = 0
            anomaly = 0
            for traffic in data:
                if identify(traffic):
                    normality += 1
                else:
                    anomaly += 1
            if anomaly > 0:
                res = '0'
            log('Traffic identity is completed, '
                '{} traffic lines  were analyzed, '
                '{} of them is/are nomal, '
                '{} of them is/are abnormal, '
                'and the analysis result is {}'.format(totality, normality, anomaly, res))
        self.send(res)



def handelPacket(p):
    global pkts
    if len(pkts) == pkts_num_max:
        pkts.pop(0)
    pkts.append(p)
    

class Sniffer(Thread):
    def __init__(self):
        super().__init__()

    def run(self):
        log('Start packet capturing...')
        sniff(iface=iface, filter="tcp or udp", prn=handelPacket)
