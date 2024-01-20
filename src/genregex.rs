use std::collections::BTreeSet;

use itertools::Itertools;

/// Generalizes a list of strings into a regex.
/// The main strategy is to find those complete strings, suffixes, or
/// prefixes in the set that seem to be restricted in their distribution
/// and issue a regular expression (Python or foma), that matches a limited
/// set of strings.

/// This is achieved through a number of tests.
/// We first make the assumption that strings in a set are drawn from a
/// uniform distribution with n members.  Then, we
///  (1) ask how likely it is to draw this particular sequence, assuming the
/// set really has n+1 members (i.e. where we never happened to see the
/// n+1th member) which is
///       p = 1-(1/(n+1)) ** num_draws
/// where num draws is the length of the list of strings. If this p < 0.05 (by default)
/// we assume we have seen all members of the set and declare the set to be fixed.

/// If the set of members is not found to be fixed, we further investigate
/// the suffixes and prefixes in the set. We the find the longest
///  (2a) set of suffixes that can be assumed to be fixed
///  (2b) prefix that fulfills the same conditions.
/// We also examine the distribution of string lengths. If, by the same analysis,
/// the lengths of strings can be assumed to be drawn from a fixed set, we
/// limit the set of allowable lengths.

/// # Examples
/// ````
/// use paradigmextract::genregex::Genregex;
/// let words = vec!["ab","ab","ab","ba","ba","ba","ab","ba","a","b"];
/// let r = Genregex::new(&words);
/// assert_eq!(r.pyregex(),
/// "^(?=.*(a|b)$)(?=.{1,2}$)(a|b)");
/// ````
pub struct Genregex {
    stringset: StringSet,
    suffixset: StringSet,
    prefixset: StringSet,
    lenrange: (usize, usize),
}

type StringSet = BTreeSet<String>;

impl Default for Genregex {
    fn default() -> Self {
        Self {
            stringset: Default::default(),
            suffixset: Default::default(),
            prefixset: Default::default(),
            lenrange: Default::default(),
        }
    }
}
impl Genregex {
    pub fn new(strings: &[&str]) -> Genregex {
        let pvalue = 0.05;
        let length = true;

        // Case (1): if the totality of strings seems to have a limited distribution
        let stringset = StringSet::from_iter(strings.iter().map(|s| String::from(*s)));
        if Self::significancetest(strings.len(), stringset.len(), pvalue) {
            return Self {
                stringset,
                ..Default::default()
            };
        }
        let minlen = strings
            .iter()
            .min_by_key(|s| s.len())
            .map(|s| s.len())
            .unwrap_or(0);
        let mut suffixset = StringSet::new();
        let mut prefixset = StringSet::new();
        let mut lenrange = (0, 0);
        // Case (2a): find longest suffix that has limited distribution
        for i in (-(minlen as i32))..0 {
            let suffstrings: Vec<_> = strings
                .iter()
                .map(|s| s[((s.len() as i32 + i) as usize)..].to_string())
                .collect();
            let sufflen = suffstrings.len();
            let tmpset = StringSet::from_iter(suffstrings.into_iter());
            if Self::significancetest(sufflen, tmpset.len(), pvalue) {
                suffixset = tmpset;
                break;
            }
        }
        // Case (2b): find longest prefix that has limited distribution
        for i in (1..=minlen).rev() {
            let prefstrings: Vec<_> = strings.iter().map(|s| s[..i].to_string()).collect();
            let preflen = prefstrings.len();
            let tmpset = StringSet::from_iter(prefstrings.into_iter());
            if Self::significancetest(preflen, tmpset.len(), pvalue) {
                prefixset = tmpset;
                break;
            }
        }
        // Case (2c): find out if stringlengths have limited distribution
        if length {
            let stringlengths = BTreeSet::from_iter(strings.iter().map(|s| s.len()));
            if Self::significancetest(strings.len(), stringlengths.len(), pvalue) {
                let maxlen = strings
                    .iter()
                    .max_by_key(|s| s.len())
                    .map(|s| s.len())
                    .unwrap_or(0);
                lenrange = (minlen, maxlen);
            }
        }
        Self {
            suffixset,
            prefixset,
            lenrange,
            ..Default::default()
        }
    }

    pub fn pyregex(&self) -> String {
        // ^(?=.*suffix$)(?=.{min,max}$)prefix
        if self.stringset.len() > 0 {
            let joined = self.stringset.iter().join("|");
            return format!("^({})$", joined);
        }
        let mut re = String::new();
        if self.suffixset.len() > 0 {
            dbg!(&self.suffixset);
            re = format!("(?=.*({})$)", self.suffixset.iter().join("|"));
        }
        if self.lenrange != (0, 0) {
            re = format!("{}(?=.{{{},{}}}$)", re, self.lenrange.0, self.lenrange.1);
        }
        if self.prefixset.len() > 0 {
            re = format!("{}({})", re, self.prefixset.iter().join("|"));
        }
        if re.is_empty() {
            ".+".to_string()
        } else {
            format!("^{}", re)
        }
    }
    fn significancetest(num: usize, uniq: usize, pvalue: f64) -> bool {
        return (1.0 - (1.0 / (uniq as f64 + 1.0))).powi(num as i32) as f64 <= pvalue;
    }
}
