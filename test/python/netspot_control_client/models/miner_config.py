from typing import Any, Dict, List, Type, TypeVar, Union

import attr

from ..types import UNSET, Unset

T = TypeVar("T", bound="MinerConfig")


@attr.s(auto_attribs=True)
class MinerConfig:
    """
    Attributes:
        name (str):
        device (Union[Unset, str]):  Default: 'any'.
        promiscuous (Union[Unset, bool]):  Default: True.
        enabled (Union[Unset, bool]):  Default: True.
    """

    name: str
    device: Union[Unset, str] = "any"
    promiscuous: Union[Unset, bool] = True
    enabled: Union[Unset, bool] = True
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        name = self.name
        device = self.device
        promiscuous = self.promiscuous
        enabled = self.enabled

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "name": name,
            }
        )
        if device is not UNSET:
            field_dict["device"] = device
        if promiscuous is not UNSET:
            field_dict["promiscuous"] = promiscuous
        if enabled is not UNSET:
            field_dict["enabled"] = enabled

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        name = d.pop("name")

        device = d.pop("device", UNSET)

        promiscuous = d.pop("promiscuous", UNSET)

        enabled = d.pop("enabled", UNSET)

        miner_config = cls(
            name=name,
            device=device,
            promiscuous=promiscuous,
            enabled=enabled,
        )

        miner_config.additional_properties = d
        return miner_config

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
