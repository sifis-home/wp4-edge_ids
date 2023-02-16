from enum import Enum


class Stat(str, Enum):
    AVG_PKT_SIZE = "AVG_PKT_SIZE"
    PERF = "PERF"
    R_ACK = "R_ACK"
    R_ARP = "R_ARP"
    R_DST_SRC = "R_DST_SRC"
    R_DST_SRC_PORT = "R_DST_SRC_PORT"
    R_ICMP = "R_ICMP"
    R_IP = "R_IP"
    R_SYN = "R_SYN"
    TRAFFIC = "TRAFFIC"

    def __str__(self) -> str:
        return str(self.value)
