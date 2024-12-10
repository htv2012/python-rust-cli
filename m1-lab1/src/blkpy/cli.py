#!/usr/bin/env python3
import json

import click
import rich

from .util import find_first, run_lsblk


@click.command()
@click.argument("device")
def main(device):
    data = run_lsblk()["blockdevices"]
    if device == "all":
        device_info = data
    else:
        try:
            device_info = find_first(data, device)
        except ValueError as error:
            raise SystemExit(str(error))
    rich.print_json(json.dumps(device_info, indent=4))


if __name__ == "__main__":
    main()
