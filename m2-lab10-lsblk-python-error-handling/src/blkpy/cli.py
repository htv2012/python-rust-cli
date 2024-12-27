#!/usr/bin/env python3
import sys
import types

import click
import rich

from . import logger
from .util import find_first, run_lsblk


def print_error(message: str):
    click.echo(click.style(message, fg="red"), file=sys.stderr)


@click.group()
@click.pass_context
@click.option("-v", "--verbose", is_flag=True)
@click.option(
    "-l",
    "--log-level",
    envvar="LOGLEVEL",
    default="WARNING",
    type=click.Choice(
        "DEBUG INFO WARNING ERROR CRITICAL".split(), case_sensitive=False
    ),
)
def main(ctx, log_level, verbose):
    ctx.ensure_object(types.SimpleNamespace)
    ctx.obj.verbose = verbose
    ctx.obj.log_level = log_level


@main.command()
@click.pass_context
@click.argument("device", required=False, default="all")
def info(ctx, device: str):
    print(f"Log level = {ctx.obj.log_level!r}")

    try:
        data = run_lsblk()["blockdevices"]
    except FileNotFoundError as error:
        print_error("lsblk is not found, please install")
        logger.exception("lsblk is not found")
        if ctx.obj.verbose:
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
