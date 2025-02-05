import pytest

from paradigmextract.paradigm import Paradigm


def test_1():
    form_msds = [
        ("1+a+2", ("msd", "sg indef nom")),
        ("1+ä+2+er", ("msd", "pl indef nom")),
        ("1+a+2+s", ("msd", "sg indef gen")),
    ]
    var_insts = [[("1", "b"), ("2", "d")]]
    p = Paradigm(form_msds, var_insts)
    tag = ("msd", "sg indef nom")
    matches = p.match("apa", constrained=False, tag=tag, baseform=True)
    assert matches == [None]
    matches = p.match("vad", constrained=False, tag=tag, baseform=True)
    assert matches == [[(1, ("v", "d"))]]


def test_2():
    form_msds = [
        ("1+a+2", ("msd", "sg indef nom")),
        ("1+ä+2+er", ("msd", "pl indef nom")),
        ("1+a+2+s", ("msd", "sg indef gen")),
    ]
    var_insts = [[("1", "b"), ("2", "d")]]
    p = Paradigm(form_msds, var_insts)
    var_insts = ["st", "d"]
    table = p(*var_insts)
    assert table == [
        ("stad", ("msd", "sg indef nom")),
        ("städer", ("msd", "pl indef nom")),
        ("stads", ("msd", "sg indef gen")),
    ]


@pytest.mark.xfail(reason="don't know")
def test_paradigm_match_vars():
    form_msds = [
        ("1+a+2", ("msd", "sg indef nom")),
        ("1+ä+2+er", ("msd", "pl indef nom")),
        ("1+a+2+s", ("msd", "sg indef gen")),
    ]
    var_insts = [[("1", "b"), ("2", "d")]]
    p = Paradigm(form_msds, var_insts)
    _match_vars = p.match_vars
