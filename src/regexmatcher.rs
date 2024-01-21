/// Simple re.findall replacement that returns all possible matches -
/// not just the leftmost-longest match.
/// Only handles (.+) and c constructs (c being an arbitrary character).
/// Expressions are automatically anchored in the head and the tail,
/// i.e. the regexes behave as if they began with ^ and ended with $.
///
/// # Examples
/// ````
/// use paradigmextract::regexmatcher::MRegex;
/// let mut m = MRegex::new("(.+)a(.+)as");
/// let word = "bananas";
/// let result = m.findall(word);
/// assert_eq!(result, Some(vec![vec!("b", "nan"), vec!("ban", "n")]));
/// ````
pub struct MRegex {
    regex: String,
    state: MatchState,
}

struct MatchState {
    textlen: usize,
    matched: bool,
    results: Vec<Vec<(usize, usize)>>,
}

impl Default for MatchState {
    fn default() -> Self {
        Self {
            textlen: 0,
            matched: false,
            results: Vec::default(),
        }
    }
}

impl MatchState {
    fn new(text: &str) -> Self {
        Self {
            textlen: text.len(),
            matched: false,
            results: Vec::default(),
        }
    }
}
impl MRegex {
    pub fn new<S: Into<String>>(regex: S) -> MRegex {
        Self {
            regex: regex.into(),
            state: MatchState::default(),
        }
    }

    pub fn findall<'a>(&mut self, text: &'a str) -> Option<Vec<Vec<&'a str>>> {
        let strindex = 0;
        let regindex = 0;
        self.state = MatchState::new(text);
        self.r#match(text, strindex, regindex, vec![]);
        if self.state.matched {
            let result = self
                .state
                .results
                .iter()
                .map(|r| r.into_iter().map(|(i, j)| &text[*i..*j]).collect())
                .collect();
            return Some(result);
        }
        return None;
    }

    fn r#match(
        &mut self,
        text: &str,
        strindex: usize,
        regindex: usize,
        groups: Vec<(usize, usize)>,
    ) {
        println!("strindex={strindex},regindex={regindex}");
        // Are we at end of regex _1and_ text?
        if strindex == self.state.textlen && regindex == self.regex.len() {
            self.state.matched = true;
            if groups.len() > 0 {
                self.state.results.push(groups);
            }
            return;
        }
        // Jump out if only regex or text is consumed
        if strindex == self.state.textlen || regindex == self.regex.len() {
            return;
        }
        // Match (.+)-construct
        if (regindex + 4) <= self.regex.len() && &self.regex[regindex..regindex + 4] == "(.+)" {
            for i in (strindex + 1)..(self.state.textlen + 1) {
                let mut groups_clone = groups.clone();
                groups_clone.push((strindex, i));
                self.r#match(text, i, regindex + 4, groups_clone);
            }
        }
        // Normal match (one character)
        else if &text.as_bytes()[strindex..(strindex + 1)]
            == &self.regex.as_bytes()[regindex..(regindex + 1)]
        {
            self.r#match(text, strindex + 1, regindex + 1, groups);
        }
    }
}
