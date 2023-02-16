from typing import TYPE_CHECKING, Any, Dict, List, Type, TypeVar, Union

import attr

from ..models.webhook_request_method import WebhookRequestMethod
from ..models.webhook_stats_type import WebhookStatsType
from ..types import UNSET, Unset

if TYPE_CHECKING:
    from ..models.webhook_headers import WebhookHeaders


T = TypeVar("T", bound="Webhook")


@attr.s(auto_attribs=True)
class Webhook:
    """
    Attributes:
        name (str):
        address (str):
        method (Union[Unset, WebhookRequestMethod]):  Default: WebhookRequestMethod.POST.
        headers (Union[Unset, WebhookHeaders]):
        type (Union[Unset, WebhookStatsType]):  Default: WebhookStatsType.BOTH.
    """

    name: str
    address: str
    method: Union[Unset, WebhookRequestMethod] = WebhookRequestMethod.POST
    headers: Union[Unset, "WebhookHeaders"] = UNSET
    type: Union[Unset, WebhookStatsType] = WebhookStatsType.BOTH
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        name = self.name
        address = self.address
        method: Union[Unset, str] = UNSET
        if not isinstance(self.method, Unset):
            method = self.method.value

        headers: Union[Unset, Dict[str, Any]] = UNSET
        if not isinstance(self.headers, Unset):
            headers = self.headers.to_dict()

        type: Union[Unset, str] = UNSET
        if not isinstance(self.type, Unset):
            type = self.type.value

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "name": name,
                "address": address,
            }
        )
        if method is not UNSET:
            field_dict["method"] = method
        if headers is not UNSET:
            field_dict["headers"] = headers
        if type is not UNSET:
            field_dict["type"] = type

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        from ..models.webhook_headers import WebhookHeaders

        d = src_dict.copy()
        name = d.pop("name")

        address = d.pop("address")

        _method = d.pop("method", UNSET)
        method: Union[Unset, WebhookRequestMethod]
        if isinstance(_method, Unset):
            method = UNSET
        else:
            method = WebhookRequestMethod(_method)

        _headers = d.pop("headers", UNSET)
        headers: Union[Unset, WebhookHeaders]
        if isinstance(_headers, Unset):
            headers = UNSET
        else:
            headers = WebhookHeaders.from_dict(_headers)

        _type = d.pop("type", UNSET)
        type: Union[Unset, WebhookStatsType]
        if isinstance(_type, Unset):
            type = UNSET
        else:
            type = WebhookStatsType(_type)

        webhook = cls(
            name=name,
            address=address,
            method=method,
            headers=headers,
            type=type,
        )

        webhook.additional_properties = d
        return webhook

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
