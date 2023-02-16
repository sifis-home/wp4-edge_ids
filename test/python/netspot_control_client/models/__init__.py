""" Contains all the data models used in inputs/outputs """

from .alarm_message import AlarmMessage
from .alert_status import AlertStatus
from .data_message import DataMessage
from .influx_db1_config import InfluxDB1Config
from .message_type import MessageType
from .miner_config import MinerConfig
from .netspot_config import NetspotConfig
from .process_status import ProcessStatus
from .result_of_int_32_or_string_type_0 import ResultOfInt32OrStringType0
from .result_of_int_32_or_string_type_1 import ResultOfInt32OrStringType1
from .spot_config import SpotConfig
from .stat import Stat
from .stat_config import StatConfig
from .stats_config import StatsConfig
from .status import Status
from .test_alarm_message import TestAlarmMessage
from .webhook import Webhook
from .webhook_headers import WebhookHeaders
from .webhook_item import WebhookItem
from .webhook_request_method import WebhookRequestMethod
from .webhook_stats_type import WebhookStatsType

__all__ = (
    "AlarmMessage",
    "AlertStatus",
    "DataMessage",
    "InfluxDB1Config",
    "MessageType",
    "MinerConfig",
    "NetspotConfig",
    "ProcessStatus",
    "ResultOfInt32OrStringType0",
    "ResultOfInt32OrStringType1",
    "SpotConfig",
    "Stat",
    "StatConfig",
    "StatsConfig",
    "Status",
    "TestAlarmMessage",
    "Webhook",
    "WebhookHeaders",
    "WebhookItem",
    "WebhookRequestMethod",
    "WebhookStatsType",
)
