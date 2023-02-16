from enum import Enum


class WebhookStatsType(str, Enum):
    ALARMS = "alarms"
    BOTH = "both"
    DATA = "data"

    def __str__(self) -> str:
        return str(self.value)
