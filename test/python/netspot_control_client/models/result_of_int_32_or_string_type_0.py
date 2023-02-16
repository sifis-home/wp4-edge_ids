from typing import Any, Dict, List, Type, TypeVar

import attr

T = TypeVar("T", bound="ResultOfInt32OrStringType0")


@attr.s(auto_attribs=True)
class ResultOfInt32OrStringType0:
    """
    Attributes:
        ok (int):
    """

    ok: int
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        ok = self.ok

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "Ok": ok,
            }
        )

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        ok = d.pop("Ok")

        result_of_int_32_or_string_type_0 = cls(
            ok=ok,
        )

        result_of_int_32_or_string_type_0.additional_properties = d
        return result_of_int_32_or_string_type_0

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
