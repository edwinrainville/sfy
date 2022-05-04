import pytest
from datetime import datetime, timezone

from sfy import hub
from sfy.axl import AxlCollection


@pytest.fixture
def sfy(tmpdir):
    h = hub.Hub.from_env()
    h.cache = tmpdir
    return h


def test_collect(sfy):
    b = sfy.buoy("dev864475044204278")
    pcks = b.axl_packages_range(datetime(2022, 4, 26, 11, 34 , tzinfo=timezone.utc),
                                datetime(2022, 4, 26, 11, 35, tzinfo=timezone.utc))
    assert len(pcks) > 2

    c = AxlCollection(pcks)

