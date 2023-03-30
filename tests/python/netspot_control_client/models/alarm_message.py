from typing import Any, Dict, List, Type, TypeVar

import attr

from ..models.alert_status import AlertStatus
from ..models.message_type import MessageType
from ..models.stat import Stat

T = TypeVar("T", bound="AlarmMessage")


@attr.s(auto_attribs=True)
class AlarmMessage:
    """
    Attributes:
        time (int):
        name (str):
        series (str):
        stat (Stat):
        status (AlertStatus):
        value (float):
        probability (float):
        code (int):
        type (MessageType):
    """

    time: int
    name: str
    series: str
    stat: Stat
    status: AlertStatus
    value: float
    probability: float
    code: int
    type: MessageType
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        time = self.time
        name = self.name
        series = self.series
        stat = self.stat.value

        status = self.status.value

        value = self.value
        probability = self.probability
        code = self.code
        type = self.type.value

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "time": time,
                "name": name,
                "series": series,
                "stat": stat,
                "status": status,
                "value": value,
                "probability": probability,
                "code": code,
                "type": type,
            }
        )

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        time = d.pop("time")

        name = d.pop("name")

        series = d.pop("series")

        stat = Stat(d.pop("stat"))

        status = AlertStatus(d.pop("status"))

        value = d.pop("value")

        probability = d.pop("probability")

        code = d.pop("code")

        type = MessageType(d.pop("type"))

        alarm_message = cls(
            time=time,
            name=name,
            series=series,
            stat=stat,
            status=status,
            value=value,
            probability=probability,
            code=code,
            type=type,
        )

        alarm_message.additional_properties = d
        return alarm_message

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
