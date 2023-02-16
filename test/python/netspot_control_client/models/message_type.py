from enum import Enum


class MessageType(str, Enum):
    ALARM = "alarm"
    DATA = "data"

    def __str__(self) -> str:
        return str(self.value)
