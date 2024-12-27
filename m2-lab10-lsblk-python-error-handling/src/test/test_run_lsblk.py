from shutil import which

import pytest

from blkpy.util import run_lsblk

pytestmark = pytest.mark.skipif(which("lsblk") is None, reason="lsblk not found")


@pytest.fixture(scope="session")
def lsblk_data():
    data = run_lsblk()
    return data


class TestRunLsblk:
    def test_data_is_not_none(self, lsblk_data):
        assert lsblk_data is not None

    def test_data_is_valid_json(self, lsblk_data):
        assert isinstance(lsblk_data, dict)

    def test_data_key(self, lsblk_data):
        assert "blockdevices" in lsblk_data
