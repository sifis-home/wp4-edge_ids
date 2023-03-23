from http import HTTPStatus
from typing import Any, Dict, Optional

import httpx

from ... import errors
from ...client import Client
from ...models.test_alarm_message import TestAlarmMessage
from ...types import Response


def _get_kwargs(
    *,
    client: Client,
    json_body: TestAlarmMessage,
) -> Dict[str, Any]:
    url = "{}/netspots/test/alarm".format(client.base_url)

    headers: Dict[str, str] = client.get_headers()
    cookies: Dict[str, Any] = client.get_cookies()

    json_json_body = json_body.to_dict()

    return {
        "method": "post",
        "url": url,
        "headers": headers,
        "cookies": cookies,
        "timeout": client.get_timeout(),
        "json": json_json_body,
    }


def _parse_response(*, client: Client, response: httpx.Response) -> Optional[Any]:
    if client.raise_on_unexpected_status:
        raise errors.UnexpectedStatus(f"Unexpected status code: {response.status_code}")
    else:
        return None


def _build_response(*, client: Client, response: httpx.Response) -> Response[Any]:
    return Response(
        status_code=HTTPStatus(response.status_code),
        content=response.content,
        headers=response.headers,
        parsed=_parse_response(client=client, response=response),
    )


def sync_detailed(
    *,
    client: Client,
    json_body: TestAlarmMessage,
) -> Response[Any]:
    """Send test alarm

     This endpoint allows developers to send test alarm messages.

    Test alarm messages have automatically the following parameters set:

    * `time` as nanoseconds since Unix Epoch

    * `series` \"TEST ALARM\"

    * `code`  1

    * `type` \"alarm\"

    Other alarm parameters can be defined in request body, but this is optional.

    Args:
        json_body (TestAlarmMessage):

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        Response[Any]
    """

    kwargs = _get_kwargs(
        client=client,
        json_body=json_body,
    )

    response = httpx.request(
        verify=client.verify_ssl,
        **kwargs,
    )

    return _build_response(client=client, response=response)


async def asyncio_detailed(
    *,
    client: Client,
    json_body: TestAlarmMessage,
) -> Response[Any]:
    """Send test alarm

     This endpoint allows developers to send test alarm messages.

    Test alarm messages have automatically the following parameters set:

    * `time` as nanoseconds since Unix Epoch

    * `series` \"TEST ALARM\"

    * `code`  1

    * `type` \"alarm\"

    Other alarm parameters can be defined in request body, but this is optional.

    Args:
        json_body (TestAlarmMessage):

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        Response[Any]
    """

    kwargs = _get_kwargs(
        client=client,
        json_body=json_body,
    )

    async with httpx.AsyncClient(verify=client.verify_ssl) as _client:
        response = await _client.request(**kwargs)

    return _build_response(client=client, response=response)
