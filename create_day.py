#!/usr/bin/env python3
import argparse
import os
import subprocess
from typing import List
from typing import Optional
from urllib import request


def main(argv: Optional[List[str]] = None) -> int:
    parser = argparse.ArgumentParser(
        'create a new day in rust. '
        'attempts pull token from a file named TOKEN if --token isn\'t passed'
    )
    parser.add_argument('day', help='day for AoC')
    parser.add_argument('--token', help='session token for AoC')
    args = parser.parse_args()
    day = args.day
    session = args.token

    if not os.path.exists(day):
        subprocess.check_call(['cargo', 'new', '--name', f'day{day}', day])

    if session is None:
        with open('TOKEN') as f:
            session = f.read().strip()

    req = request.Request(
        f'https://adventofcode.com/2019/day/{day}/input',
        headers={'Cookie': f'session={session}'},
    )
    with request.urlopen(req) as r, open(f'{day}/input.txt', 'w') as f:
        f.write(r.read().decode('utf-8'))

    return 0


if __name__ == '__main__':
    exit(main())
