from typing import TYPE_CHECKING, Any, Dict, List, Type, TypeVar, Union

import attr

from ..types import UNSET, Unset

if TYPE_CHECKING:
    from ..models.stat_config import StatConfig


T = TypeVar("T", bound="StatsConfig")


@attr.s(auto_attribs=True)
class StatsConfig:
    """
    Attributes:
        avg_pkt_size (Union[Unset, None, StatConfig]):
        perf (Union[Unset, None, StatConfig]):
        r_ack (Union[Unset, None, StatConfig]):
        r_arp (Union[Unset, None, StatConfig]):
        r_dst_src (Union[Unset, None, StatConfig]):
        r_dst_src_port (Union[Unset, None, StatConfig]):
        r_icmp (Union[Unset, None, StatConfig]):
        r_ip (Union[Unset, None, StatConfig]):
        r_syn (Union[Unset, None, StatConfig]):
        traffic (Union[Unset, None, StatConfig]):
    """

    avg_pkt_size: Union[Unset, None, "StatConfig"] = UNSET
    perf: Union[Unset, None, "StatConfig"] = UNSET
    r_ack: Union[Unset, None, "StatConfig"] = UNSET
    r_arp: Union[Unset, None, "StatConfig"] = UNSET
    r_dst_src: Union[Unset, None, "StatConfig"] = UNSET
    r_dst_src_port: Union[Unset, None, "StatConfig"] = UNSET
    r_icmp: Union[Unset, None, "StatConfig"] = UNSET
    r_ip: Union[Unset, None, "StatConfig"] = UNSET
    r_syn: Union[Unset, None, "StatConfig"] = UNSET
    traffic: Union[Unset, None, "StatConfig"] = UNSET
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        avg_pkt_size: Union[Unset, None, Dict[str, Any]] = UNSET
        if not isinstance(self.avg_pkt_size, Unset):
            avg_pkt_size = self.avg_pkt_size.to_dict() if self.avg_pkt_size else None

        perf: Union[Unset, None, Dict[str, Any]] = UNSET
        if not isinstance(self.perf, Unset):
            perf = self.perf.to_dict() if self.perf else None

        r_ack: Union[Unset, None, Dict[str, Any]] = UNSET
        if not isinstance(self.r_ack, Unset):
            r_ack = self.r_ack.to_dict() if self.r_ack else None

        r_arp: Union[Unset, None, Dict[str, Any]] = UNSET
        if not isinstance(self.r_arp, Unset):
            r_arp = self.r_arp.to_dict() if self.r_arp else None

        r_dst_src: Union[Unset, None, Dict[str, Any]] = UNSET
        if not isinstance(self.r_dst_src, Unset):
            r_dst_src = self.r_dst_src.to_dict() if self.r_dst_src else None

        r_dst_src_port: Union[Unset, None, Dict[str, Any]] = UNSET
        if not isinstance(self.r_dst_src_port, Unset):
            r_dst_src_port = self.r_dst_src_port.to_dict() if self.r_dst_src_port else None

        r_icmp: Union[Unset, None, Dict[str, Any]] = UNSET
        if not isinstance(self.r_icmp, Unset):
            r_icmp = self.r_icmp.to_dict() if self.r_icmp else None

        r_ip: Union[Unset, None, Dict[str, Any]] = UNSET
        if not isinstance(self.r_ip, Unset):
            r_ip = self.r_ip.to_dict() if self.r_ip else None

        r_syn: Union[Unset, None, Dict[str, Any]] = UNSET
        if not isinstance(self.r_syn, Unset):
            r_syn = self.r_syn.to_dict() if self.r_syn else None

        traffic: Union[Unset, None, Dict[str, Any]] = UNSET
        if not isinstance(self.traffic, Unset):
            traffic = self.traffic.to_dict() if self.traffic else None

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update({})
        if avg_pkt_size is not UNSET:
            field_dict["avg_pkt_size"] = avg_pkt_size
        if perf is not UNSET:
            field_dict["perf"] = perf
        if r_ack is not UNSET:
            field_dict["r_ack"] = r_ack
        if r_arp is not UNSET:
            field_dict["r_arp"] = r_arp
        if r_dst_src is not UNSET:
            field_dict["r_dst_src"] = r_dst_src
        if r_dst_src_port is not UNSET:
            field_dict["r_dst_src_port"] = r_dst_src_port
        if r_icmp is not UNSET:
            field_dict["r_icmp"] = r_icmp
        if r_ip is not UNSET:
            field_dict["r_ip"] = r_ip
        if r_syn is not UNSET:
            field_dict["r_syn"] = r_syn
        if traffic is not UNSET:
            field_dict["traffic"] = traffic

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        from ..models.stat_config import StatConfig

        d = src_dict.copy()
        _avg_pkt_size = d.pop("avg_pkt_size", UNSET)
        avg_pkt_size: Union[Unset, None, StatConfig]
        if _avg_pkt_size is None:
            avg_pkt_size = None
        elif isinstance(_avg_pkt_size, Unset):
            avg_pkt_size = UNSET
        else:
            avg_pkt_size = StatConfig.from_dict(_avg_pkt_size)

        _perf = d.pop("perf", UNSET)
        perf: Union[Unset, None, StatConfig]
        if _perf is None:
            perf = None
        elif isinstance(_perf, Unset):
            perf = UNSET
        else:
            perf = StatConfig.from_dict(_perf)

        _r_ack = d.pop("r_ack", UNSET)
        r_ack: Union[Unset, None, StatConfig]
        if _r_ack is None:
            r_ack = None
        elif isinstance(_r_ack, Unset):
            r_ack = UNSET
        else:
            r_ack = StatConfig.from_dict(_r_ack)

        _r_arp = d.pop("r_arp", UNSET)
        r_arp: Union[Unset, None, StatConfig]
        if _r_arp is None:
            r_arp = None
        elif isinstance(_r_arp, Unset):
            r_arp = UNSET
        else:
            r_arp = StatConfig.from_dict(_r_arp)

        _r_dst_src = d.pop("r_dst_src", UNSET)
        r_dst_src: Union[Unset, None, StatConfig]
        if _r_dst_src is None:
            r_dst_src = None
        elif isinstance(_r_dst_src, Unset):
            r_dst_src = UNSET
        else:
            r_dst_src = StatConfig.from_dict(_r_dst_src)

        _r_dst_src_port = d.pop("r_dst_src_port", UNSET)
        r_dst_src_port: Union[Unset, None, StatConfig]
        if _r_dst_src_port is None:
            r_dst_src_port = None
        elif isinstance(_r_dst_src_port, Unset):
            r_dst_src_port = UNSET
        else:
            r_dst_src_port = StatConfig.from_dict(_r_dst_src_port)

        _r_icmp = d.pop("r_icmp", UNSET)
        r_icmp: Union[Unset, None, StatConfig]
        if _r_icmp is None:
            r_icmp = None
        elif isinstance(_r_icmp, Unset):
            r_icmp = UNSET
        else:
            r_icmp = StatConfig.from_dict(_r_icmp)

        _r_ip = d.pop("r_ip", UNSET)
        r_ip: Union[Unset, None, StatConfig]
        if _r_ip is None:
            r_ip = None
        elif isinstance(_r_ip, Unset):
            r_ip = UNSET
        else:
            r_ip = StatConfig.from_dict(_r_ip)

        _r_syn = d.pop("r_syn", UNSET)
        r_syn: Union[Unset, None, StatConfig]
        if _r_syn is None:
            r_syn = None
        elif isinstance(_r_syn, Unset):
            r_syn = UNSET
        else:
            r_syn = StatConfig.from_dict(_r_syn)

        _traffic = d.pop("traffic", UNSET)
        traffic: Union[Unset, None, StatConfig]
        if _traffic is None:
            traffic = None
        elif isinstance(_traffic, Unset):
            traffic = UNSET
        else:
            traffic = StatConfig.from_dict(_traffic)

        stats_config = cls(
            avg_pkt_size=avg_pkt_size,
            perf=perf,
            r_ack=r_ack,
            r_arp=r_arp,
            r_dst_src=r_dst_src,
            r_dst_src_port=r_dst_src_port,
            r_icmp=r_icmp,
            r_ip=r_ip,
            r_syn=r_syn,
            traffic=traffic,
        )

        stats_config.additional_properties = d
        return stats_config

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
