from enum import Enum


class AlertStatus(str, Enum):
    DOWN_ALERT = "DOWN_ALERT"
    UP_ALERT = "UP_ALERT"

    def __str__(self) -> str:
        return str(self.value)
