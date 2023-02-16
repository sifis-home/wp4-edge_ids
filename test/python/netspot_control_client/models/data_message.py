from typing import Any, Dict, List, Type, TypeVar, Union

import attr

from ..models.message_type import MessageType
from ..types import UNSET, Unset

T = TypeVar("T", bound="DataMessage")


@attr.s(auto_attribs=True)
class DataMessage:
    """
    Attributes:
        time (int):
        name (str):
        series (str):
        type (MessageType):
        avg_pkt_size (Union[Unset, None, float]):
        avg_pkt_size_down (Union[Unset, None, float]):
        avg_pkt_size_up (Union[Unset, None, float]):
        perf (Union[Unset, None, float]):
        perf_down (Union[Unset, None, float]):
        perf_up (Union[Unset, None, float]):
        r_ack (Union[Unset, None, float]):
        r_ack_down (Union[Unset, None, float]):
        r_ack_up (Union[Unset, None, float]):
        r_arp (Union[Unset, None, float]):
        r_arp_down (Union[Unset, None, float]):
        r_arp_up (Union[Unset, None, float]):
        r_dst_src (Union[Unset, None, float]):
        r_dst_src_down (Union[Unset, None, float]):
        r_dst_src_up (Union[Unset, None, float]):
        r_dst_src_port (Union[Unset, None, float]):
        r_dst_src_port_down (Union[Unset, None, float]):
        r_dst_src_port_up (Union[Unset, None, float]):
        r_icmp (Union[Unset, None, float]):
        r_icmp_down (Union[Unset, None, float]):
        r_icmp_up (Union[Unset, None, float]):
        r_ip (Union[Unset, None, float]):
        r_ip_down (Union[Unset, None, float]):
        r_ip_up (Union[Unset, None, float]):
        r_syn (Union[Unset, None, float]):
        r_syn_down (Union[Unset, None, float]):
        r_syn_up (Union[Unset, None, float]):
        traffic (Union[Unset, None, float]):
        traffic_down (Union[Unset, None, float]):
        traffic_up (Union[Unset, None, float]):
    """

    time: int
    name: str
    series: str
    type: MessageType
    avg_pkt_size: Union[Unset, None, float] = UNSET
    avg_pkt_size_down: Union[Unset, None, float] = UNSET
    avg_pkt_size_up: Union[Unset, None, float] = UNSET
    perf: Union[Unset, None, float] = UNSET
    perf_down: Union[Unset, None, float] = UNSET
    perf_up: Union[Unset, None, float] = UNSET
    r_ack: Union[Unset, None, float] = UNSET
    r_ack_down: Union[Unset, None, float] = UNSET
    r_ack_up: Union[Unset, None, float] = UNSET
    r_arp: Union[Unset, None, float] = UNSET
    r_arp_down: Union[Unset, None, float] = UNSET
    r_arp_up: Union[Unset, None, float] = UNSET
    r_dst_src: Union[Unset, None, float] = UNSET
    r_dst_src_down: Union[Unset, None, float] = UNSET
    r_dst_src_up: Union[Unset, None, float] = UNSET
    r_dst_src_port: Union[Unset, None, float] = UNSET
    r_dst_src_port_down: Union[Unset, None, float] = UNSET
    r_dst_src_port_up: Union[Unset, None, float] = UNSET
    r_icmp: Union[Unset, None, float] = UNSET
    r_icmp_down: Union[Unset, None, float] = UNSET
    r_icmp_up: Union[Unset, None, float] = UNSET
    r_ip: Union[Unset, None, float] = UNSET
    r_ip_down: Union[Unset, None, float] = UNSET
    r_ip_up: Union[Unset, None, float] = UNSET
    r_syn: Union[Unset, None, float] = UNSET
    r_syn_down: Union[Unset, None, float] = UNSET
    r_syn_up: Union[Unset, None, float] = UNSET
    traffic: Union[Unset, None, float] = UNSET
    traffic_down: Union[Unset, None, float] = UNSET
    traffic_up: Union[Unset, None, float] = UNSET
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        time = self.time
        name = self.name
        series = self.series
        type = self.type.value

        avg_pkt_size = self.avg_pkt_size
        avg_pkt_size_down = self.avg_pkt_size_down
        avg_pkt_size_up = self.avg_pkt_size_up
        perf = self.perf
        perf_down = self.perf_down
        perf_up = self.perf_up
        r_ack = self.r_ack
        r_ack_down = self.r_ack_down
        r_ack_up = self.r_ack_up
        r_arp = self.r_arp
        r_arp_down = self.r_arp_down
        r_arp_up = self.r_arp_up
        r_dst_src = self.r_dst_src
        r_dst_src_down = self.r_dst_src_down
        r_dst_src_up = self.r_dst_src_up
        r_dst_src_port = self.r_dst_src_port
        r_dst_src_port_down = self.r_dst_src_port_down
        r_dst_src_port_up = self.r_dst_src_port_up
        r_icmp = self.r_icmp
        r_icmp_down = self.r_icmp_down
        r_icmp_up = self.r_icmp_up
        r_ip = self.r_ip
        r_ip_down = self.r_ip_down
        r_ip_up = self.r_ip_up
        r_syn = self.r_syn
        r_syn_down = self.r_syn_down
        r_syn_up = self.r_syn_up
        traffic = self.traffic
        traffic_down = self.traffic_down
        traffic_up = self.traffic_up

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "time": time,
                "name": name,
                "series": series,
                "type": type,
            }
        )
        if avg_pkt_size is not UNSET:
            field_dict["AVG_PKT_SIZE"] = avg_pkt_size
        if avg_pkt_size_down is not UNSET:
            field_dict["AVG_PKT_SIZE_DOWN"] = avg_pkt_size_down
        if avg_pkt_size_up is not UNSET:
            field_dict["AVG_PKT_SIZE_UP"] = avg_pkt_size_up
        if perf is not UNSET:
            field_dict["PERF"] = perf
        if perf_down is not UNSET:
            field_dict["PERF_DOWN"] = perf_down
        if perf_up is not UNSET:
            field_dict["PERF_UP"] = perf_up
        if r_ack is not UNSET:
            field_dict["R_ACK"] = r_ack
        if r_ack_down is not UNSET:
            field_dict["R_ACK_DOWN"] = r_ack_down
        if r_ack_up is not UNSET:
            field_dict["R_ACK_UP"] = r_ack_up
        if r_arp is not UNSET:
            field_dict["R_ARP"] = r_arp
        if r_arp_down is not UNSET:
            field_dict["R_ARP_DOWN"] = r_arp_down
        if r_arp_up is not UNSET:
            field_dict["R_ARP_UP"] = r_arp_up
        if r_dst_src is not UNSET:
            field_dict["R_DST_SRC"] = r_dst_src
        if r_dst_src_down is not UNSET:
            field_dict["R_DST_SRC_DOWN"] = r_dst_src_down
        if r_dst_src_up is not UNSET:
            field_dict["R_DST_SRC_UP"] = r_dst_src_up
        if r_dst_src_port is not UNSET:
            field_dict["R_DST_SRC_PORT"] = r_dst_src_port
        if r_dst_src_port_down is not UNSET:
            field_dict["R_DST_SRC_PORT_DOWN"] = r_dst_src_port_down
        if r_dst_src_port_up is not UNSET:
            field_dict["R_DST_SRC_PORT_UP"] = r_dst_src_port_up
        if r_icmp is not UNSET:
            field_dict["R_ICMP"] = r_icmp
        if r_icmp_down is not UNSET:
            field_dict["R_ICMP_DOWN"] = r_icmp_down
        if r_icmp_up is not UNSET:
            field_dict["R_ICMP_UP"] = r_icmp_up
        if r_ip is not UNSET:
            field_dict["R_IP"] = r_ip
        if r_ip_down is not UNSET:
            field_dict["R_IP_DOWN"] = r_ip_down
        if r_ip_up is not UNSET:
            field_dict["R_IP_UP"] = r_ip_up
        if r_syn is not UNSET:
            field_dict["R_SYN"] = r_syn
        if r_syn_down is not UNSET:
            field_dict["R_SYN_DOWN"] = r_syn_down
        if r_syn_up is not UNSET:
            field_dict["R_SYN_UP"] = r_syn_up
        if traffic is not UNSET:
            field_dict["TRAFFIC"] = traffic
        if traffic_down is not UNSET:
            field_dict["TRAFFIC_DOWN"] = traffic_down
        if traffic_up is not UNSET:
            field_dict["TRAFFIC_UP"] = traffic_up

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        time = d.pop("time")

        name = d.pop("name")

        series = d.pop("series")

        type = MessageType(d.pop("type"))

        avg_pkt_size = d.pop("AVG_PKT_SIZE", UNSET)

        avg_pkt_size_down = d.pop("AVG_PKT_SIZE_DOWN", UNSET)

        avg_pkt_size_up = d.pop("AVG_PKT_SIZE_UP", UNSET)

        perf = d.pop("PERF", UNSET)

        perf_down = d.pop("PERF_DOWN", UNSET)

        perf_up = d.pop("PERF_UP", UNSET)

        r_ack = d.pop("R_ACK", UNSET)

        r_ack_down = d.pop("R_ACK_DOWN", UNSET)

        r_ack_up = d.pop("R_ACK_UP", UNSET)

        r_arp = d.pop("R_ARP", UNSET)

        r_arp_down = d.pop("R_ARP_DOWN", UNSET)

        r_arp_up = d.pop("R_ARP_UP", UNSET)

        r_dst_src = d.pop("R_DST_SRC", UNSET)

        r_dst_src_down = d.pop("R_DST_SRC_DOWN", UNSET)

        r_dst_src_up = d.pop("R_DST_SRC_UP", UNSET)

        r_dst_src_port = d.pop("R_DST_SRC_PORT", UNSET)

        r_dst_src_port_down = d.pop("R_DST_SRC_PORT_DOWN", UNSET)

        r_dst_src_port_up = d.pop("R_DST_SRC_PORT_UP", UNSET)

        r_icmp = d.pop("R_ICMP", UNSET)

        r_icmp_down = d.pop("R_ICMP_DOWN", UNSET)

        r_icmp_up = d.pop("R_ICMP_UP", UNSET)

        r_ip = d.pop("R_IP", UNSET)

        r_ip_down = d.pop("R_IP_DOWN", UNSET)

        r_ip_up = d.pop("R_IP_UP", UNSET)

        r_syn = d.pop("R_SYN", UNSET)

        r_syn_down = d.pop("R_SYN_DOWN", UNSET)

        r_syn_up = d.pop("R_SYN_UP", UNSET)

        traffic = d.pop("TRAFFIC", UNSET)

        traffic_down = d.pop("TRAFFIC_DOWN", UNSET)

        traffic_up = d.pop("TRAFFIC_UP", UNSET)

        data_message = cls(
            time=time,
            name=name,
            series=series,
            type=type,
            avg_pkt_size=avg_pkt_size,
            avg_pkt_size_down=avg_pkt_size_down,
            avg_pkt_size_up=avg_pkt_size_up,
            perf=perf,
            perf_down=perf_down,
            perf_up=perf_up,
            r_ack=r_ack,
            r_ack_down=r_ack_down,
            r_ack_up=r_ack_up,
            r_arp=r_arp,
            r_arp_down=r_arp_down,
            r_arp_up=r_arp_up,
            r_dst_src=r_dst_src,
            r_dst_src_down=r_dst_src_down,
            r_dst_src_up=r_dst_src_up,
            r_dst_src_port=r_dst_src_port,
            r_dst_src_port_down=r_dst_src_port_down,
            r_dst_src_port_up=r_dst_src_port_up,
            r_icmp=r_icmp,
            r_icmp_down=r_icmp_down,
            r_icmp_up=r_icmp_up,
            r_ip=r_ip,
            r_ip_down=r_ip_down,
            r_ip_up=r_ip_up,
            r_syn=r_syn,
            r_syn_down=r_syn_down,
            r_syn_up=r_syn_up,
            traffic=traffic,
            traffic_down=traffic_down,
            traffic_up=traffic_up,
        )

        data_message.additional_properties = d
        return data_message

    @property
    def additional_keys(self) -> List[str]:
        return list(self.additional_properties.keys())

    def __getitem__(self, key: str) -> Any:
        return self.additional_properties[key]

    def __setitem__(self, key: str, value: Any) -> None:
        self.additional_properties[key] = value

    def __delitem__(self, key: str) -> None:
        del self.additional_properties[key]

    def __contains__(self, key: str) -> bool:
        return key in self.additional_properties
