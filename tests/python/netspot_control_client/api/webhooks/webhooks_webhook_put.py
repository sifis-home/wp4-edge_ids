from http import HTTPStatus
from typing import Any, Dict, Optional, Union

import httpx

from ... import errors
from ...client import Client
from ...models.result_of_int_32_or_string_type_0 import ResultOfInt32OrStringType0
from ...models.result_of_int_32_or_string_type_1 import ResultOfInt32OrStringType1
from ...models.webhook import Webhook
from ...types import Response


def _get_kwargs(
    id: Union["ResultOfInt32OrStringType0", "ResultOfInt32OrStringType1"],
    *,
    client: Client,
    json_body: Webhook,
) -> Dict[str, Any]:
    url = "{}/netspots/webhook/{id}".format(client.base_url, id=id)

    headers: Dict[str, str] = client.get_headers()
    cookies: Dict[str, Any] = client.get_cookies()

    json_json_body = json_body.to_dict()

    return {
        "method": "put",
        "url": url,
        "headers": headers,
        "cookies": cookies,
        "timeout": client.get_timeout(),
        "json": json_json_body,
    }


def _parse_response(*, client: Client, response: httpx.Response) -> Optional[Any]:
    if response.status_code == HTTPStatus.OK:
        return None
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
    id: Union["ResultOfInt32OrStringType0", "ResultOfInt32OrStringType1"],
    *,
    client: Client,
    json_body: Webhook,
) -> Response[Any]:
    """Update webhook configuration

     Update webhook configuration by ID

    Args:
        id (Union['ResultOfInt32OrStringType0', 'ResultOfInt32OrStringType1']):
        json_body (Webhook):

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        Response[Any]
    """

    kwargs = _get_kwargs(
        id=id,
        client=client,
        json_body=json_body,
    )

    response = httpx.request(
        verify=client.verify_ssl,
        **kwargs,
    )

    return _build_response(client=client, response=response)


async def asyncio_detailed(
    id: Union["ResultOfInt32OrStringType0", "ResultOfInt32OrStringType1"],
    *,
    client: Client,
    json_body: Webhook,
) -> Response[Any]:
    """Update webhook configuration

     Update webhook configuration by ID

    Args:
        id (Union['ResultOfInt32OrStringType0', 'ResultOfInt32OrStringType1']):
        json_body (Webhook):

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        Response[Any]
    """

    kwargs = _get_kwargs(
        id=id,
        client=client,
        json_body=json_body,
    )

    async with httpx.AsyncClient(verify=client.verify_ssl) as _client:
        response = await _client.request(**kwargs)

    return _build_response(client=client, response=response)
