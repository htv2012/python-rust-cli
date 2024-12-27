#!/usr/bin/env python3
import sys

import click
import rich

from .util import find_first, run_lsblk


def print_error(message: str):
    click.echo(click.style(message, fg="red"), file=sys.stderr)


@click.command()
@click.option("-v", "--verbose", is_flag=True)
@click.argument("device", required=False, default="all")
def main(device: str, verbose: bool):
    try:
        data = run_lsblk()["blockdevices"]
    except FileNotFoundError as error:
        print_error("lsblk is not found, please install")
        if verbose:
            print(error)
        sys.exit(1)

    if device != "all":
        try:
            data = find_first(data, device)
        except ValueError as error:
            print_error(error)
            sys.exit(1)
    rich.print_json(data=data, indent=4)


if __name__ == "__main__":
    main()
