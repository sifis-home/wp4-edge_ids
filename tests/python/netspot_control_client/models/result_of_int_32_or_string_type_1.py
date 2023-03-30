from typing import Any, Dict, List, Type, TypeVar

import attr

T = TypeVar("T", bound="ResultOfInt32OrStringType1")


@attr.s(auto_attribs=True)
class ResultOfInt32OrStringType1:
    """
    Attributes:
        err (str):
    """

    err: str
    additional_properties: Dict[str, Any] = attr.ib(init=False, factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        err = self.err

        field_dict: Dict[str, Any] = {}
        field_dict.update(self.additional_properties)
        field_dict.update(
            {
                "Err": err,
            }
        )

        return field_dict

    @classmethod
    def from_dict(cls: Type[T], src_dict: Dict[str, Any]) -> T:
        d = src_dict.copy()
        err = d.pop("Err")

        result_of_int_32_or_string_type_1 = cls(
            err=err,
        )

        result_of_int_32_or_string_type_1.additional_properties = d
        return result_of_int_32_or_string_type_1

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
