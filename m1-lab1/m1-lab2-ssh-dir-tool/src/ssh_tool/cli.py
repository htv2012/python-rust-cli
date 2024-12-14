import os
import pathlib

import click


@click.group()
def main():
    """
    Provide tools for managing the ~/.ssh directory.
    """
    return


def check_permission(path: pathlib.Path, root: pathlib.Path):
    expected = {
        ".ssh": "700",
        ".ssh/known_hosts": "644",
        "config": "644",
        "authorized_keys": "644",
    }
    actual_permission = oct(os.stat(path).st_mode)[-3:]
    expected_permission = expected.get(
        path.name, "644" if path.suffix == ".pub" else "600"
    )
    click.echo(f"   {actual_permission} ", nl=False)
    if actual_permission > expected_permission:
        color = "red"
    else:
        color = "green"
    click.echo(click.style(f"     {expected_permission}", fg=color), nl=False)
    click.echo(f" {path.relative_to(root)}")


@main.command()
def check():
    """
    Check permissions in ~/.ssh.
    """
    root = pathlib.Path("~/.ssh").expanduser()
    click.echo("Actual Expected Name")
    click.echo("------ -------- ----")
    for file in [root] + sorted(root.glob("*")):
        check_permission(file, root)
