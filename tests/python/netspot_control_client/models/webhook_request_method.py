from enum import Enum


class WebhookRequestMethod(str, Enum):
    GET = "GET"
    POST = "POST"
    PUT = "PUT"

    def __str__(self) -> str:
        return str(self.value)
