from typing import Any, Dict, List, Type, TypeVar, Union

import attr

from ..types import UNSET, Unset

T = TypeVar("T", bound="StatConfig")


@attr.s(auto_attribs=True)
class StatConfig:
    """
    Attributes:
        enabled (Union[Unset, bool]):
        depth (Union[Unset, None, int]):
        q (Union[Unset, None, float]):
        n_init (Union[Unset, None, int]):
        level (Union[Unset, None, float]):
        up (Union[Unset, None, bool]):
        down (Union[Unset, None, bool]):
        alert (Union[Unset, None, bool]):
        bounded (Union[Unset, None, bool]):
        max_excess (Union[Unset, None, int]):
    """

    enabled: Union[Unset, bool] = False
    depth: Union[Unset, None, int] = UNSET
    q: Union[Unset, None, float] = UNSET
    n_init: Union[Unset, None, int] = UNSET
    level: Union[Unset, None, float] = UNSET
    up: Union[Unset, None, bool] = UNSET
    down: Union[Unset, None, bool] = UNSET
    alert: Union[Unset, None, bool] = UNSET
    bounded: Union[Unset, None, bool] = UNSET
    max_excess: Union[Unset, None, int] = UNSET
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        enabled = self.enabled
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
        if enabled is not UNSET:
            field_dict["enabled"] = enabled
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
        enabled = d.pop("enabled", UNSET)

        depth = d.pop("depth", UNSET)

        q = d.pop("q", UNSET)

        n_init = d.pop("n_init", UNSET)

        level = d.pop("level", UNSET)

        up = d.pop("up", UNSET)

        down = d.pop("down", UNSET)

        alert = d.pop("alert", UNSET)

        bounded = d.pop("bounded", UNSET)

        max_excess = d.pop("max_excess", UNSET)

        stat_config = cls(
            enabled=enabled,
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

        stat_config.additional_properties = d
        return stat_config

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
