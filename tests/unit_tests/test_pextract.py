from paradigmextract.pextract import _longest_variable

import pytest


def test_longest_variable_with_empty_string_returns_0():
    assert _longest_variable("") == 0


def test_longest_variable_with_string_wo_variable_returns_0():
    assert _longest_variable("test") == 0


@pytest.mark.parametrize(
    "s,length",
    [
        ("test[a]", 1),
        ("test[ab]", 2),
        ("t[e]st[a]", 1),
        ("t[ea]st[a]", 2),
        ("t[e]st[ab]", 2),
        ("t[e]st[ab]t[dfg]sfy[ty]", 3),
    ],
)
def test_longest_variable_with_string_w_variables_returns_correct_length(
    s: str, length: int
):
    assert _longest_variable(s) == length

