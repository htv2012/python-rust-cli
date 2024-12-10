#!/usr/bin/env python3
import click
import rich

from .util import find_first, run_lsblk


@click.command()
@click.argument("device", required=False, default="all")
def main(device):
    data = run_lsblk()["blockdevices"]
    if device != "all":
        try:
            data = find_first(data, device)
        except ValueError as error:
            raise SystemExit(str(error))
    rich.print_json(data=data, indent=4)


if __name__ == "__main__":
    main()
