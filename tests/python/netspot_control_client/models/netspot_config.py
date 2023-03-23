from typing import TYPE_CHECKING, Any, Dict, List, Type, TypeVar, Union

import attr

from ..types import UNSET, Unset

if TYPE_CHECKING:
    from ..models.influx_db1_config import InfluxDB1Config
    from ..models.miner_config import MinerConfig
    from ..models.spot_config import SpotConfig
    from ..models.stats_config import StatsConfig


T = TypeVar("T", bound="NetspotConfig")


@attr.s(auto_attribs=True)
class NetspotConfig:
    """
    Attributes:
        configuration (MinerConfig):
        influxdb1 (Union[Unset, None, InfluxDB1Config]):
        spot (Union[Unset, SpotConfig]):
        stats (Union[Unset, StatsConfig]):
    """

    configuration: "MinerConfig"
    influxdb1: Union[Unset, None, "InfluxDB1Config"] = UNSET
    spot: Union[Unset, "SpotConfig"] = UNSET
    stats: Union[Unset, "StatsConfig"] = UNSET
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        configuration = self.configuration.to_dict()

        influxdb1: Union[Unset, None, Dict[str, Any]] = UNSET
        if not isinstance(self.influxdb1, Unset):
            influxdb1 = self.influxdb1.to_dict() if self.influxdb1 else None

        spot: Union[Unset, Dict[str, Any]] = UNSET
        if not isinstance(self.spot, Unset):
            spot = self.spot.to_dict()

        stats: Union[Unset, Dict[str, Any]] = UNSET
        if not isinstance(self.stats, Unset):
            stats = self.stats.to_dict()

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "configuration": configuration,
            }
        )
        if influxdb1 is not UNSET:
            field_dict["influxdb1"] = influxdb1
        if spot is not UNSET:
            field_dict["spot"] = spot
        if stats is not UNSET:
            field_dict["stats"] = stats

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        from ..models.influx_db1_config import InfluxDB1Config
        from ..models.miner_config import MinerConfig
        from ..models.spot_config import SpotConfig
        from ..models.stats_config import StatsConfig

        d = src_dict.copy()
        configuration = MinerConfig.from_dict(d.pop("configuration"))

        _influxdb1 = d.pop("influxdb1", UNSET)
        influxdb1: Union[Unset, None, InfluxDB1Config]
        if _influxdb1 is None:
            influxdb1 = None
        elif isinstance(_influxdb1, Unset):
            influxdb1 = UNSET
        else:
            influxdb1 = InfluxDB1Config.from_dict(_influxdb1)

        _spot = d.pop("spot", UNSET)
        spot: Union[Unset, SpotConfig]
        if isinstance(_spot, Unset):
            spot = UNSET
        else:
            spot = SpotConfig.from_dict(_spot)

        _stats = d.pop("stats", UNSET)
        stats: Union[Unset, StatsConfig]
        if isinstance(_stats, Unset):
            stats = UNSET
        else:
            stats = StatsConfig.from_dict(_stats)

        netspot_config = cls(
            configuration=configuration,
            influxdb1=influxdb1,
            spot=spot,
            stats=stats,
        )

        netspot_config.additional_properties = d
        return netspot_config

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
