from typing import Any, Dict, List, Type, TypeVar, Union

import attr

from ..types import UNSET, Unset

T = TypeVar("T", bound="InfluxDB1Config")


@attr.s(auto_attribs=True)
class InfluxDB1Config:
    """
    Attributes:
        data (Union[Unset, bool]):
        alarm (Union[Unset, bool]):
        address (Union[Unset, str]):  Default: 'http://127.0.0.1:8086'.
        database (Union[Unset, str]):  Default: 'netspot'.
        username (Union[Unset, str]):  Default: 'netspot'.
        password (Union[Unset, str]):  Default: 'netspot'.
        batch_size (Union[Unset, int]):  Default: 10.
        agent_name (Union[Unset, str]):  Default: 'local'.
    """

    data: Union[Unset, bool] = False
    alarm: Union[Unset, bool] = False
    address: Union[Unset, str] = "http://127.0.0.1:8086"
    database: Union[Unset, str] = "netspot"
    username: Union[Unset, str] = "netspot"
    password: Union[Unset, str] = "netspot"
    batch_size: Union[Unset, int] = 10
    agent_name: Union[Unset, str] = "local"
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        data = self.data
        alarm = self.alarm
        address = self.address
        database = self.database
        username = self.username
        password = self.password
        batch_size = self.batch_size
        agent_name = self.agent_name

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update({})
        if data is not UNSET:
            field_dict["data"] = data
        if alarm is not UNSET:
            field_dict["alarm"] = alarm
        if address is not UNSET:
            field_dict["address"] = address
        if database is not UNSET:
            field_dict["database"] = database
        if username is not UNSET:
            field_dict["username"] = username
        if password is not UNSET:
            field_dict["password"] = password
        if batch_size is not UNSET:
            field_dict["batch_size"] = batch_size
        if agent_name is not UNSET:
            field_dict["agent_name"] = agent_name

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        data = d.pop("data", UNSET)

        alarm = d.pop("alarm", UNSET)

        address = d.pop("address", UNSET)

        database = d.pop("database", UNSET)

        username = d.pop("username", UNSET)

        password = d.pop("password", UNSET)

        batch_size = d.pop("batch_size", UNSET)

        agent_name = d.pop("agent_name", UNSET)

        influx_db1_config = cls(
            data=data,
            alarm=alarm,
            address=address,
            database=database,
            username=username,
            password=password,
            batch_size=batch_size,
            agent_name=agent_name,
        )

        influx_db1_config.additional_properties = d
        return influx_db1_config

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
