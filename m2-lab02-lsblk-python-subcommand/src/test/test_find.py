import json
import pathlib

import pytest

from blkpy.util import find_all, find_first


@pytest.fixture
def block_devices():
    """
    Return the `blockdevices` key of the data
    """
    here = pathlib.Path(__file__).parent
    data_path = here / "lsblk-J.json"
    with open(data_path, "r", encoding="utf-8") as stream:
        data = json.load(stream)
    return data["blockdevices"]


def test_find_first_leaf(block_devices):
    """
    Find the first device, which is a leaf.
    """
    assert find_first(block_devices, "sdb") == {
        "name": "sdb",
        "maj:min": "8:16",
        "rm": True,
        "size": "0B",
        "ro": False,
        "type": "disk",
        "mountpoints": [None],
    }


def test_find_all(block_devices):
    """
    Find all the devices.
    """
    names = [device["name"] for device in find_all(block_devices)]
    assert len(names) == 8
    assert "sdb" in names
    assert "sda" in names
    assert "sda1" in names
    assert "sda2" in names
    assert "sda3" in names
    assert "lvmlmde" in names
    assert "lvmlmde-root" in names
    assert "lvmlmde-swap" in names
