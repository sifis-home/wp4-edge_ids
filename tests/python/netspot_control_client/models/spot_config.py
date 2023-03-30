from typing import Any, Dict, List, Type, TypeVar, Union

import attr

from ..types import UNSET, Unset

T = TypeVar("T", bound="SpotConfig")


@attr.s(auto_attribs=True)
class SpotConfig:
    """
    Attributes:
        depth (Union[Unset, int]):  Default: 50.
        q (Union[Unset, float]):  Default: 0.0001.
        n_init (Union[Unset, int]):  Default: 1000.
        level (Union[Unset, float]):  Default: 0.8.
        up (Union[Unset, bool]):  Default: True.
        down (Union[Unset, bool]):
        alert (Union[Unset, bool]):  Default: True.
        bounded (Union[Unset, bool]):  Default: True.
        max_excess (Union[Unset, int]):  Default: 200.
    """

    depth: Union[Unset, int] = 50
    q: Union[Unset, float] = 0.0001
    n_init: Union[Unset, int] = 1000
    level: Union[Unset, float] = 0.8
    up: Union[Unset, bool] = True
    down: Union[Unset, bool] = False
    alert: Union[Unset, bool] = True
    bounded: Union[Unset, bool] = True
    max_excess: Union[Unset, int] = 200
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        depth = self.depth
        q = self.q
        n_init = self.n_init
        level = self.level
        up = self.up
        down = self.down
        alert = self.alert
        bounded = self.bounded
        max_excess = self.max_excess

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update({})
        if depth is not UNSET:
            field_dict["depth"] = depth
        if q is not UNSET:
            field_dict["q"] = q
        if n_init is not UNSET:
            field_dict["n_init"] = n_init
        if level is not UNSET:
            field_dict["level"] = level
        if up is not UNSET:
            field_dict["up"] = up
        if down is not UNSET:
            field_dict["down"] = down
        if alert is not UNSET:
            field_dict["alert"] = alert
        if bounded is not UNSET:
            field_dict["bounded"] = bounded
        if max_excess is not UNSET:
            field_dict["max_excess"] = max_excess

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        depth = d.pop("depth", UNSET)

        q = d.pop("q", UNSET)

        n_init = d.pop("n_init", UNSET)

        level = d.pop("level", UNSET)

        up = d.pop("up", UNSET)

        down = d.pop("down", UNSET)

        alert = d.pop("alert", UNSET)

        bounded = d.pop("bounded", UNSET)

        max_excess = d.pop("max_excess", UNSET)

        spot_config = cls(
            depth=depth,
            q=q,
            n_init=n_init,
            level=level,
            up=up,
            down=down,
            alert=alert,
            bounded=bounded,
            max_excess=max_excess,
        )

        spot_config.additional_properties = d
        return spot_config

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
