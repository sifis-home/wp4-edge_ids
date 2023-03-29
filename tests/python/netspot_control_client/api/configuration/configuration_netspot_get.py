from http import HTTPStatus
from typing import Any, Dict, Optional, Union, cast

import httpx

from ... import errors
from ...client import Client
from ...models.netspot_config import NetspotConfig
from ...models.result_of_int_32_or_string_type_0 import ResultOfInt32OrStringType0
from ...models.result_of_int_32_or_string_type_1 import ResultOfInt32OrStringType1
from ...types import Response


def _get_kwargs(
    id: Union["ResultOfInt32OrStringType0", "ResultOfInt32OrStringType1"],
    *,
    client: Client,
) -> Dict[str, Any]:
    url = "{}/netspot/{id}".format(client.base_url, id=id)

    headers: Dict[str, str] = client.get_headers()
    cookies: Dict[str, Any] = client.get_cookies()

    return {
        "method": "get",
        "url": url,
        "headers": headers,
        "cookies": cookies,
        "timeout": client.get_timeout(),
    }


def _parse_response(*, client: Client, response: httpx.Response) -> Optional[Union[Any, NetspotConfig]]:
    if response.status_code == HTTPStatus.OK:
        response_200 = NetspotConfig.from_dict(response.json())

        return response_200
    if response.status_code == HTTPStatus.NOT_FOUND:
        response_404 = cast(Any, None)
        return response_404
    if client.raise_on_unexpected_status:
        raise errors.UnexpectedStatus(f"Unexpected status code: {response.status_code}")
    else:
        return None


def _build_response(*, client: Client, response: httpx.Response) -> Response[Union[Any, NetspotConfig]]:
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
) -> Response[Union[Any, NetspotConfig]]:
    """Get netspot configuration

     Get netspot configuration by ID

    Args:
        id (Union['ResultOfInt32OrStringType0', 'ResultOfInt32OrStringType1']):

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        Response[Union[Any, NetspotConfig]]
    """

    kwargs = _get_kwargs(
        id=id,
        client=client,
    )

    response = httpx.request(
        verify=client.verify_ssl,
        **kwargs,
    )

    return _build_response(client=client, response=response)


def sync(
    id: Union["ResultOfInt32OrStringType0", "ResultOfInt32OrStringType1"],
    *,
    client: Client,
) -> Optional[Union[Any, NetspotConfig]]:
    """Get netspot configuration

     Get netspot configuration by ID

    Args:
        id (Union['ResultOfInt32OrStringType0', 'ResultOfInt32OrStringType1']):

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        Response[Union[Any, NetspotConfig]]
    """

    return sync_detailed(
        id=id,
        client=client,
    ).parsed


async def asyncio_detailed(
    id: Union["ResultOfInt32OrStringType0", "ResultOfInt32OrStringType1"],
    *,
    client: Client,
) -> Response[Union[Any, NetspotConfig]]:
    """Get netspot configuration

     Get netspot configuration by ID

    Args:
        id (Union['ResultOfInt32OrStringType0', 'ResultOfInt32OrStringType1']):

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        Response[Union[Any, NetspotConfig]]
    """

    kwargs = _get_kwargs(
        id=id,
        client=client,
    )

    async with httpx.AsyncClient(verify=client.verify_ssl) as _client:
        response = await _client.request(**kwargs)

    return _build_response(client=client, response=response)


async def asyncio(
    id: Union["ResultOfInt32OrStringType0", "ResultOfInt32OrStringType1"],
    *,
    client: Client,
) -> Optional[Union[Any, NetspotConfig]]:
    """Get netspot configuration

     Get netspot configuration by ID

    Args:
        id (Union['ResultOfInt32OrStringType0', 'ResultOfInt32OrStringType1']):

    Raises:
        errors.UnexpectedStatus: If the server returns an undocumented status code and Client.raise_on_unexpected_status is True.
        httpx.TimeoutException: If the request takes longer than Client.timeout.

    Returns:
        Response[Union[Any, NetspotConfig]]
    """

    return (
        await asyncio_detailed(
            id=id,
            client=client,
        )
    ).parsed