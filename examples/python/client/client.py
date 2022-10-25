#!/usr/bin/env python3
from argparse import ArgumentParser
from time import sleep

import requests

SERVER_API = 'v1/'
DEFAULT_ADDRESS = 'http://127.0.0.1:80/'
DEFAULT_LIMIT = 5


# Alarm and data messages are similar. This function handles both.
# Message type is selected with type_name, which should be either:
# alarms or data
def get_messages(server_address, time, limit, type_name):
    # Making GET request to '/netspots/(alarms|type)' endpoint
    request_url = server_address + SERVER_API + 'netspots/' + type_name

    # We add params to enable filtering
    params = {'last': limit}
    if time is not None:
        params['time'] = time

    # Making GET request
    reply = requests.get(request_url, params)

    # Status code should be 200
    if reply.status_code == 200:
        messages = reply.json()
        for message in messages:
            print(message)
    else:
        print(f'Something went wrong, we got status code {reply.status_code}')
        print(reply.text)


# Alarm and data messages are similar. This function handles both.
# Message type is selected with type_name, which should be either:
# alarms or data
def follow_messages(server_address, type_name):
    previous_message = {}
    time = 0
    limit = 1
    request_url = server_address + SERVER_API + 'netspots/' + type_name
    params = {'last': limit}
    while True:
        # We keep making request using the time code for filtering from the previous message (0 for the first request)
        params['time'] = time
        reply = requests.get(request_url, params)
        if reply.status_code == 200:
            messages = reply.json()
            if len(messages) == 1:
                message = messages[0]
                time = message['time']
                # Only print message if it is different from previous
                if previous_message != message:
                    previous_message = message
                    print(message)
        else:
            print(f'Something went wrong, we got status code {reply.status_code}')
            print(reply.text)
            return
        # Server updates statistics in one second interval. Therefore, we make request at the same interval.
        sleep(1)


def show_status(server_address):
    # Making GET request to '/netspots' endpoint
    request_url = server_address + SERVER_API + 'netspots'
    reply = requests.get(request_url)

    # Status code should be 200
    if reply.status_code == 200:
        netspots = reply.json()
        for netspot in netspots:
            name = netspot['name']
            status = netspot['status']
            print(f'{name} is {status}')
    else:
        print(f'Something went wrong, we got status code {reply.status_code}')
        print(reply.text)


def main():
    parser = ArgumentParser(description='Netspot Control Client')
    parser.add_argument('-a', '--address', nargs='?', metavar='ADDRESS', dest='server_address', default=DEFAULT_ADDRESS,
                        help=f'Netspot Control server address. By default, the address is: {DEFAULT_ADDRESS}')

    action_group = parser.add_argument_group('Actions', 'Available actions that can be performed')
    action_group.add_argument('-s', '--status', action='store_true', help='Show netspot configuration statuses')
    action_group.add_argument('-ga', '--get-alarms', action='store_true', help='Get alarm messages')
    action_group.add_argument('-gd', '--get-data', action='store_true', help='Get data messages')
    action_group.add_argument('-fa', '--follow-alarms', action='store_true', help='Keeps printing alarms messages')
    action_group.add_argument('-fd', '--follow-data', action='store_true', help='Keeps printing data messages')

    message_options = parser.add_argument_group('Filtering', 'Message filtering options')
    message_options.add_argument('-l', '--limit', nargs='?', metavar='COUNT', dest='limit', default=DEFAULT_LIMIT,
                                 help=f'Limits how many results are displayed. The default is: {DEFAULT_ADDRESS}.')
    message_options.add_argument('-t', '--time', nargs='?', metavar='TIME',
                                 help='Only show results that are newer than this.'
                                      'Time is nanoseconds since the Unix epoch.')
    args = parser.parse_args()

    if args.status:
        print('Show netspot configuration statuses')
        print('-----------------------------------')
        show_status(args.server_address)
        print()

    if args.get_alarms:
        print('Alarm messages')
        print('--------------')
        get_messages(args.server_address, args.time, args.limit, 'alarms')
        print()

    if args.get_data:
        print('Data messages')
        print('-------------')
        get_messages(args.server_address, args.time, args.limit, 'data')
        print()

    if args.follow_alarms:
        print('Alarm messages')
        print('--------------')
        follow_messages(args.server_address, 'alarms')
    elif args.follow_data:
        print('Data messages')
        print('-------------')
        follow_messages(args.server_address, 'data')

    if not args.status and not args.get_alarms and not args.get_data\
            and not args.follow_alarms and not args.follow_data:
        parser.print_help()


if __name__ == '__main__':
    main()
