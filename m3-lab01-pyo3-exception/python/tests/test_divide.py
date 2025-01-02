import cowkulator
import pytest


def test_happy_path():
    assert cowkulator.divide(10.0, 2.0) == 5.0


def test_exception():
    with pytest.raises(ZeroDivisionError):
        cowkulator.divide(10.0, 0.0)
