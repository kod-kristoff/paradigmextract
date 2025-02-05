"""Genregex."""


class Genregex:
    """Generalizes a list of strings into a regex.

    The main strategy is to find those complete strings, suffixes, or
    prefixes in the set that seem to be restricted in their distribution
    and issue a regular expression (Python or foma), that matches a limited
    set of strings.

    This is achieved through a number of tests.
    We first make the assumption that strings in a set are drawn from a
    uniform distribution with n members.  Then, we
     (1) ask how likely it is to draw this particular sequence, assuming the
    set really has n+1 members (i.e. where we never happened to see the
    n+1th member) which is
          p = 1-(1/(n+1)) ** num_draws
    where num draws is the length of the list of strings. If this p < 0.05 (by default)
    we assume we have seen all members of the set and declare the set to be fixed.

    If the set of members is not found to be fixed, we further investigate
    the suffixes and prefixes in the set. We the find the longest
     (2a) set of suffixes that can be assumed to be fixed
     (2b) prefix that fulfills the same conditions.
    We also examine the distribution of string lengths. If, by the same analysis,
    the lengths of strings can be assumed to be drawn from a fixed set, we
    limit the set of allowable lengths.

    Example:
    >>> words = ['ab','ab','ab','ba','ba','ba','ab','ba','a','b']
    >>> r = Genregex(words)
    >>> pyregex = r.pyregex()
    >>> pyregex_parts = pyregex.split('(')
    >>> assert pyregex_parts[0] == '^'
    >>> assert pyregex_parts[1] == '?=.*'
    >>> assert (pyregex_parts[2] == 'a|b)$)') or (pyregex_parts[2] == 'b|a)$)')
    >>> assert pyregex_parts[3] == '?=.{1,2}$)'
    >>> assert (pyregex_parts[4] == 'a|b)') or (pyregex_parts[4] == 'b|a)')
    """

    def __init__(self, strings: list[str], pvalue: float = 0.05, length: bool = True) -> None:  # noqa: D107
        self.strings = strings
        self.numstrings = len(self.strings)
        self.pvalue = pvalue
        self.minlen = len(min(self.strings, key=len))
        self.maxlen = len(max(self.strings, key=len))
        self.length = length
        self.stringset = set()
        self.prefixset = set()
        self.suffixset = set()
        self.lenrange = ()

        # Case (1): if the totality of strings seems to have a limited distribution
        if self._significancetest(self.numstrings, len(set(self.strings))):
            self.stringset = set(self.strings)
            return
        # Case (2a): find longest suffix that has limited distribution
        for i in range(-self.minlen, 0):
            suffstrings = [x[i:] for x in self.strings]
            if self._significancetest(len(suffstrings), len(set(suffstrings))):
                self.suffixset = set(suffstrings)
                break
        # Case (2b): find longest prefix that has limited distribution
        for i in range(self.minlen, 0, -1):
            prefstrings = [x[:i] for x in self.strings]
            if self._significancetest(len(prefstrings), len(set(prefstrings))):
                self.prefixset = set(prefstrings)
                break
        # Case (2c): find out if stringlengths have limited distribution
        if self.length:
            stringlengths = {len(x) for x in self.strings}
            if self._significancetest(self.numstrings, len(stringlengths)):
                self.lenrange = (self.minlen, self.maxlen)
        return

    def pyregex(self) -> str:  # noqa: D102
        # ^(?=.*suffix$)(?=.{min,max}$)prefix
        re = ""
        if len(self.stringset) > 0:
            return "^(" + "|".join(self.stringset) + ")$"
        if len(self.suffixset) > 0:
            re += "(?=.*(" + "|".join(self.suffixset) + ")$)"
        if len(self.lenrange) > 0:
            re += "(?=.{" + str(self.lenrange[0]) + "," + str(self.lenrange[1]) + "}$)"
        if len(self.prefixset) > 0:
            re += "(" + "|".join(self.prefixset) + ")"
        return f"^{re}" if re else ".+"

    def _significancetest(self, num: int, uniq: int):  # noqa: ANN202
        return (1.0 - (1.0 / (uniq + 1.0))) ** num <= self.pvalue
