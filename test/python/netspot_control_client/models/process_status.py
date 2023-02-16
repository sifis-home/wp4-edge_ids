from enum import Enum


class ProcessStatus(str, Enum):
    RUNNING = "running"
    STOPPED = "stopped"
    DISABLED = "disabled"

    def __str__(self) -> str:
        return str(self.value)
