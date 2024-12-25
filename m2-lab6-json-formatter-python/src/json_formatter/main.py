import fileinput

import click
import rich


@click.command()
@click.option("-f", "--file-name", help="Specify input file")
@click.option("-i", "--indent", type=int, default=2, help="Specify indentation")
@click.option("-s", "--sort-keys", is_flag=True, help="Sort keys alphabetically")
def main(file_name, indent, sort_keys):
    with fileinput.input(file_name) as lines:
        json_text = "".join(lines)
    rich.print_json(json_text, indent=indent, sort_keys=sort_keys)
