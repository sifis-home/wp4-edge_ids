from http import HTTPStatus
from typing import Any, Dict, List, Optional, Union

import httpx

from ... import errors
from ...client import Client
from ...models.alarm_message import AlarmMessage
from ...types import UNSET, Response, Unset


def _get_kwargs(
    *,
    client: Client,
    time: Union[Unset, None, int] = UNSET,
    last: Union[Unset, None, int] = UNSET,
) -> Dict[str, Any]:
    url = "{}/netspots/alarms".format(client.base_url)

    headers: Dict[str, str] = client.get_headers()
    cookies: Dict[str, Any] = client.get_cookies()

    params: Dict[str, Any] = {}
    params["time"] = time

    params["last"] = last

    params = {k: v for k, v in params.items() if v is not UNSET and v is not None}

    return {
        "method": "get",
        "url": url,
        "headers": headers,
        "cookies": cookies,
        "timeout": client.get_timeout(),
        "params": params,
    }


def _parse_response(*, client: Client, response: httpx.Response) -> Optional[List["AlarmMessage"]]:
    if response.status_code == HTTPStatus.OK:
        response_200 = []
        _response_200 = response.json()
        for response_200_item_data in _response_200:
            response_200_item = AlarmMessage.from_dict(response_200_item_data)

            response_200.append(response_200_item)

        return response_200
    if client.raise_on_unexpected_status:
        raise errors.UnexpectedStatus(f"Unexpected status code: {response.status_code}")
    else:
        return None


def _build_response(*, client: Client, response: httpx.Response) -> Response[List["AlarmMessage"]]:
    return Response(
        status_code=HTTPStatus(response.status_code),
        content=response.content,
        headers=response.headers,
        parsed=_parse_response(client=client, response=response),
    )


def sync_detailed(
    *,
    client: Client,
    time: Union[Unset, None, int] = UNSET,
    last: Union[Unset, None, int] = UNSET,
) -> Response[List["AlarmMessage"]]:
    """Read alarms from netspot statistics

     Reads recorded alarms from netspot statistics.

    We can use parameters to limit which results are returned. Without parameters, only 100 last items
    are returned.

    Args:
        time (Union[Unset, None, int]):
        last (Union[Unset, None, int]):

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        Response[List['AlarmMessage']]
    """

    kwargs = _get_kwargs(
        client=client,
        time=time,
        last=last,
    )

    response = httpx.request(
        verify=client.verify_ssl,
        **kwargs,
    )

    return _build_response(client=client, response=response)


def sync(
    *,
    client: Client,
    time: Union[Unset, None, int] = UNSET,
    last: Union[Unset, None, int] = UNSET,
) -> Optional[List["AlarmMessage"]]:
    """Read alarms from netspot statistics

     Reads recorded alarms from netspot statistics.

    We can use parameters to limit which results are returned. Without parameters, only 100 last items
    are returned.

    Args:
        time (Union[Unset, None, int]):
        last (Union[Unset, None, int]):

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        Response[List['AlarmMessage']]
    """

    return sync_detailed(
        client=client,
        time=time,
        last=last,
    ).parsed


async def asyncio_detailed(
    *,
    client: Client,
    time: Union[Unset, None, int] = UNSET,
    last: Union[Unset, None, int] = UNSET,
) -> Response[List["AlarmMessage"]]:
    """Read alarms from netspot statistics

     Reads recorded alarms from netspot statistics.

    We can use parameters to limit which results are returned. Without parameters, only 100 last items
    are returned.

    Args:
        time (Union[Unset, None, int]):
        last (Union[Unset, None, int]):

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        Response[List['AlarmMessage']]
    """

    kwargs = _get_kwargs(
        client=client,
        time=time,
        last=last,
    )

    async with httpx.AsyncClient(verify=client.verify_ssl) as _client:
        response = await _client.request(**kwargs)

    return _build_response(client=client, response=response)


async def asyncio(
    *,
    client: Client,
    time: Union[Unset, None, int] = UNSET,
    last: Union[Unset, None, int] = UNSET,
) -> Optional[List["AlarmMessage"]]:
    """Read alarms from netspot statistics

     Reads recorded alarms from netspot statistics.

    We can use parameters to limit which results are returned. Without parameters, only 100 last items
    are returned.

    Args:
        time (Union[Unset, None, int]):
        last (Union[Unset, None, int]):

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        Response[List['AlarmMessage']]
    """

    return (
        await asyncio_detailed(
            client=client,
            time=time,
            last=last,
        )
    ).parsed
