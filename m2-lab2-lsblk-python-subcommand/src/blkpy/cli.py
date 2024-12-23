#!/usr/bin/env python3
import sys

import click
import rich

from .util import find_first, run_lsblk


def print_error(message: str):
    click.echo(click.style(message, fg="red"), file=sys.stderr)


@click.group()
@click.pass_context
@click.option("-v", "--verbose", is_flag=True)
def main(ctx, verbose):
    ctx.ensure_object(dict)
    ctx.obj["verbose"] = verbose


@main.command()
@click.pass_context
@click.argument("device", required=False, default="all")
def info(ctx, device: str):
    try:
        data = run_lsblk()["blockdevices"]
    except FileNotFoundError as error:
        print_error("lsblk is not found, please install")
        if ctx.obj["verbose"]:
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
