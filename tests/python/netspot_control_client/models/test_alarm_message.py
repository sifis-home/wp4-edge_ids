from typing import Any, Dict, List, Type, TypeVar, Union

import attr

from ..models.alert_status import AlertStatus
from ..models.stat import Stat
from ..types import UNSET, Unset

T = TypeVar("T", bound="TestAlarmMessage")


@attr.s(auto_attribs=True)
class TestAlarmMessage:
    """
    Attributes:
        name (Union[Unset, str]):  Default: 'Test alarm'.
        stat (Union[Unset, Stat]):  Default: Stat.R_SYN.
        status (Union[Unset, AlertStatus]):  Default: AlertStatus.UP_ALERT.
        value (Union[Unset, float]):  Default: 1000.0.
        probability (Union[Unset, float]):  Default: 0.75.
    """

    name: Union[Unset, str] = "Test alarm"
    stat: Union[Unset, Stat] = Stat.R_SYN
    status: Union[Unset, AlertStatus] = AlertStatus.UP_ALERT
    value: Union[Unset, float] = 1000.0
    probability: Union[Unset, float] = 0.75
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        name = self.name
        stat: Union[Unset, str] = UNSET
        if not isinstance(self.stat, Unset):
            stat = self.stat.value

        status: Union[Unset, str] = UNSET
        if not isinstance(self.status, Unset):
            status = self.status.value

        value = self.value
        probability = self.probability

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update({})
        if name is not UNSET:
            field_dict["name"] = name
        if stat is not UNSET:
            field_dict["stat"] = stat
        if status is not UNSET:
            field_dict["status"] = status
        if value is not UNSET:
            field_dict["value"] = value
        if probability is not UNSET:
            field_dict["probability"] = probability

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        name = d.pop("name", UNSET)

        _stat = d.pop("stat", UNSET)
        stat: Union[Unset, Stat]
        if isinstance(_stat, Unset):
            stat = UNSET
        else:
            stat = Stat(_stat)

        _status = d.pop("status", UNSET)
        status: Union[Unset, AlertStatus]
        if isinstance(_status, Unset):
            status = UNSET
        else:
            status = AlertStatus(_status)

        value = d.pop("value", UNSET)

        probability = d.pop("probability", UNSET)

        test_alarm_message = cls(
            name=name,
            stat=stat,
            status=status,
            value=value,
            probability=probability,
        )

        test_alarm_message.additional_properties = d
        return test_alarm_message

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
