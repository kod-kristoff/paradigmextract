// import logging
// import re
// from collections import defaultdict
// from typing import Any, Optional, Sequence, Tuple

// from paradigmextract import genregex, regexmatcher

use crate::regexmatcher::MRegex;

// logger = logging.getLogger(__name__)
#[cfg(test)]
mod tests;

// class Paradigm:
//     """A class representing a paradigm.

//     Args:
//        form_msds:list(tuple)
//             Ex: [('1+en',[('tense','pres')]), ...,
//        var_insts:list(tuple)
//             Ex: [[('1','dimm')],[('1','dank')], ...]
//     """
pub struct Paradigm {
    forms: Vec<Form>,
    var_insts: Vec<Vec<(String, String)>>,
}

impl Paradigm {
    pub fn new(
        form_msds: &[(&str, Vec<(Option<String>, String)>)],
        var_insts: Vec<Vec<(String, String)>>,
    ) -> Paradigm {
        //     def __init__(
        //         self,
        //         form_msds: list[Tuple[str, list[Tuple[Optional[str], str]]]],
        //         var_insts: list[list[Tuple[str, str]]],
        //         p_id: str = "",
        //         pos: str = "",
        //         uuid: str = "",
        //     ) -> None:
        //         logger.debug("make paradigm %s %s", p_id, uuid)
        //         self._p_info: dict[str, Any] = {}
        let mut forms = vec![];
        //         self.pos = pos
        //         self.uuid = uuid
        //         self.var_insts = var_insts
        //         self.p_id = p_id

        for (f, msd) in form_msds {
            //         self.forms.extend(Form(f, msd, var_insts) for f, msd in form_msds)
            forms.push(Form::new(f, msd.clone(), &var_insts));
        }
        Self { forms, var_insts }
    }
    fn tmp() -> Paradigm {
        Self {
            forms: Vec::new(),
            var_insts: Vec::new(),
        }
    }
    //     def __getattr__(self, attr):
    //         """
    //         Caches information about paradigm.

    //         NOTE: Stuff gets weird when the paradigm has no members,
    //         TODO: should naming of a paradigm really be done here?
    //         """
    //         if len(self._p_info) > 0:
    //             return self._p_info[attr]
    //         if self.p_id:
    //             self._p_info["name"] = self.p_id
    //         if len(self.var_insts) == 0:
    //             if not self.p_id:
    //                 self._p_info["name"] = f"p_{self.__call__()[0][0]}"
    //             self._p_info["members"] = []
    //             self._p_info["count"] = 0
    //         else:
    //             if not self.p_id:
    //                 self._p_info[
    //                     "name"
    //                 ] = f"p_{self.__call__(*[s for _, s in self.var_insts[0][1:]])[0][0]}"
    //             self._p_info["count"] = len(self.var_insts)
    //             self._p_info["members"] = [var[0][1] for var in self.var_insts]
    //         self._p_info["slots"] = self.__slots()
    //         return self._p_info[attr]

    //     def __slots(self) -> list[Tuple[bool, Any]]:
    //         """Compute the content of the slots."""
    //         slts: list = []
    //         # string slots
    //         fs: list[list[str]] = [f.strs() for f in self.forms]
    //         str_slots: list[Tuple[str, ...]] = list(zip(*fs))
    //         # var slots
    //         vt: dict[str, list[str]] = defaultdict(list)
    //         for vs in self.var_insts:
    //             for v, s in vs:
    //                 vt[v].append(s)
    //         var_slots = list(vt.items())
    //         var_slots.sort(key=lambda x: x[0])
    //         (s_index, v_index) = (0, 0)
    //         for i in range(
    //             len(str_slots) + len(var_slots)
    //         ):  # interleave strings and variables
    //             if i % 2 == 0:
    //                 slts.append((False, str_slots[s_index]))
    //                 s_index += 1
    //             elif var_slots:  # handle empty paradigms
    //                 slts.append((True, var_slots[v_index][1]))
    //                 v_index += 1
    //         return slts

    //     def fits_paradigm(
    //         self, w: str, tag: str = "", constrained: bool = True, baseform: bool = False
    //     ) -> bool:
    //         for f in self.forms:
    //             if f.match(w, tag=tag, constrained=constrained):
    //                 return True
    //             if baseform:
    //                 break
    //         return False

    pub fn match_word(&self, w: &str) -> Vec<Option<Vec<(usize, Vec<String>)>>> {
        self.r#match(w, MatchOptions::default())
    }
    pub fn r#match(
        &self,
        w: &str,
        MatchOptions {
            selection,
            constrained,
            tag,
            baseform,
        }: MatchOptions,
    ) -> Vec<Option<Vec<(usize, Vec<String>)>>> {
        //     def match(
        //         self,
        //         w: str,
        //         selection: Optional[Sequence[int]] = None,
        //         constrained: bool = True,
        //         tag: str = "",
        //         baseform: bool = False,
        //     ) -> list[Optional[list[Tuple[int, Any]]]]:
        println!(
            "paradigm.Paradigm.match(w={w},selection={selection:?},constrained={constrained},tag=,baseform={baseform})"
        );
        let mut result = vec![];
        let forms =
        //         if selection is not None:
        //             forms = [self.forms[i] for i in selection]
                if baseform {
                &self.forms[..1]
            }
        //             forms = self.forms[:1]
                else {
                &self.forms[..]
            };
        //             forms = self.forms
        //         if tag:
        // let mut forms: dyn Iterator<Item = &Form> = if let Some(tag) = tag {
        //     //             forms = [f for f in forms if f.msd == tag]
        //     forms.iter().filter(|f| f.msd == tag)
        // } else {
        //     forms.iter()
        // };
        for f in forms {
            if let Some(tag) = &tag {
                if f.msd.as_ref() != *tag {
                    continue;
                }
            }
            let xs = f.match_vars(w, constrained);
            println!("paradigm.Paradigm.match: xs = {xs:?}");
            //             if xs and len(self.var_insts) >= 1 and len(self.var_insts[0]) > 1:
            //                 print(f"paradigm.Paradigm.match: sorting, {xs[0][1][1]}")
            //                 result.append(sorted(xs, key=lambda x: len(x[1][1])))
            //             else:
            //                 result.append(xs)
            result.push(xs);
        }
        return result;
    }

    //     def __call__(self, *insts):
    //         table = []
    //         for f in self.forms:
    //             (w, msd) = f(*insts)
    //             table.append(("".join(w), msd))
    //         return table

    //     def __str__(self):
    //         p = "#".join([f.__str__() for f in self.forms])
    //         v = "#".join([",,".join(["%s=%s" % v for v in vs]) for vs in self.var_insts])
    //         return "%s\t%s" % (p, v)
}

#[derive(Debug, Clone)]
pub struct MatchOptions<'a> {
    pub selection: Option<&'a [usize]>,
    pub constrained: bool,
    pub tag: Option<&'a (Option<&'a str>, &'a str)>,
    pub baseform: bool,
}

impl<'a> Default for MatchOptions<'a> {
    fn default() -> Self {
        Self {
            selection: None,
            constrained: true,
            tag: None,
            baseform: false,
        }
    }
}
/// A class representing a paradigmatic wordform and, possibly, its
/// morphosyntactic description.
///
/// Args:
///    form:str
///         Ex: 1+a+2
///    msd:list(tuple)
///         Ex: [('num','sg'),('case':'nom') .. ]
///             [] no msd available
///             [(None,'SGNOM')] no msd type available
pub struct Form {
    regex: String,
    scount: usize,
    form: Vec<String>,
    msd: Vec<(Option<String>, String)>,
}

impl Form {
    pub fn new(
        form: &str,
        msd: Vec<(Option<String>, String)>,
        v_insts: &Vec<Vec<(String, String)>>,
    ) -> Form {
        //     def __init__(
        //         self,
        //         form: str,
        //         msd: list[Tuple[Optional[str], str]] = (),
        //         v_insts: Sequence[list[Tuple[str, Any]]] = (),
        //     ):
        //         self.form: list[str] = form.split("+")
        let form: Vec<_> = form.split('+').map(String::from).collect();
        //         self.msd = msd
        //         self.scount: int = 0
        let mut scount: usize = 0;
        //         # self.identifier = len(msd) > 0 and len(msd[0]) > 1 and msd[0][1] == "identifier"
        //         r = ""
        //         for f in self.form:
        //             if f.isdigit():
        //                 r += "(.+)"
        //             else:
        //                 r += f
        //                 self.scount += len(f)
        let regex: String = form
            .iter()
            .map(|f| {
                if f.bytes().all(|c| c.is_ascii_digit()) {
                    "(.+)"
                } else {
                    scount += f.len();
                    f.as_str()
                }
            })
            .collect();
        //         self.regex = r
        //         self.cregex = re.compile(self.regex)
        //         # vars
        //         collect_vars: dict[str, set[str]] = defaultdict(set)
        //         for vs in v_insts:
        //             for i, v in vs:
        //                 collect_vars[i].add(v)
        //         self.v_regex = []
        //         for ss in collect_vars.values():
        //             try:
        //                 self.v_regex.append(
        //                     re.compile(genregex.Genregex(ss, pvalue=0.05).pyregex())
        //                 )
        //             except:
        //                 logging.error(f"error reading {ss}!")
        //                 raise
        Self {
            regex,
            form,
            msd,
            scount,
        }
    }

    pub fn wo_msd(form: &str) -> Form {
        Self::new(form, Vec::new(), &vec![])
    }

    //     def __call__(self, *insts):
    //         """Instantiate the variables of the wordform.
    //         Args:
    //          insts: fun args
    //                 Ex: f('schr','i','b')
    //         """
    //         (w, vindex) = ([], 0)
    //         for p in self.form:
    //             if p.isdigit():  # is a variable
    //                 w.append(insts[vindex])
    //                 vindex += 1
    //             else:
    //                 w.append(p)
    //         return w, self.msd

    //     def match(self, w: str, tag: str = "", constrained: bool = True) -> bool:
    //         if tag and self.msd != tag:
    //             return False
    //         return self.match_vars(w, constrained) is not None

    pub fn match_vars(&self, w: &str, constrained: bool) -> Option<Vec<(usize, Vec<String>)>> {
        //     ) -> Optional[list[Tuple[int, Any]]]:
        println!("paradigm.Form.match_vars(w={w},constrained={constrained})");
        println!("paradigm.Form.match_vars: self.regex = {}", self.regex);
        //         matcher = regexmatcher.MRegex(self.regex)
        let mut matcher = MRegex::new(&self.regex);
        let ms = matcher.findall(w);
        //         ms = matcher.findall(w)
        if let Some(ms) = ms {
            dbg!(&ms);
            if ms.is_empty() {
                return Some(vec![]);
            }
            if !constrained {
                return Some(
                    ms.into_iter()
                        .map(|m| (self.scount, m.iter().map(|s| s.to_string()).collect()))
                        .collect(),
                );
                //[(self.scount, m) for m in ms];
            }
            todo!("handle match");
        //         result = []
        //         for vs in ms:
        //             var_and_reg = (
        //                 [(vs, self.v_regex[0])]
        //                 if isinstance(vs, str)
        //                 else zip(vs, self.v_regex)
        //             )
        //             vcount = 0
        //             m_all = True
        //             for s, r in var_and_reg:
        //                 m = r.match(s)
        //                 if m is None:
        //                     return None
        //                 xs = m.groups()  # .+-matches have no grouping
        //                 if len(xs) > 0 or r.pattern == ".+":
        //                     if r.pattern != ".+":
        //                         vcount += len("".join(xs))  # select the variable specificity
        //                 else:
        //                     m_all = False
        //                     break
        //             if m_all:
        //                 result.append((self.scount + vcount, vs))
        //         return result or None
        } else {
            return None;
        }
    }
    //     def strs(self) -> list[str]:
    //         """Collects the strings in a wordform.
    //         A variable is assumed to be surrounded by (possibly empty) strings.
    //         """
    //         ss = []
    //         if self.form[0].isdigit():
    //             ss.append("_")
    //         for i in range(len(self.form)):
    //             if not (self.form[i].isdigit()):
    //                 ss.append(self.form[i])
    //             elif i < len(self.form) - 1 and self.form[i + 1].isdigit():
    //                 ss.append("_")
    //         if self.form[-1].isdigit():
    //             ss.append("_")
    //         return ss

    //     def __str__(self) -> str:
    //         ms = []
    //         for t, v in self.msd:
    //             if t is None:
    //                 if v is not None:
    //                     ms.append(v)
    //             elif v is not None:
    //                 ms.append(f"{t}={v}")
    //             else:
    //                 ms.append(t)
    //         return f'{"+".join(self.form)}::{",,".join(ms)}' if ms else "+".join(self.form)
}
