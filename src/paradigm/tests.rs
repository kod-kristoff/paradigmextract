// import pytest

// from paradigmextract.paradigm import Paradigm
use super::*;

mod form;

#[test]
fn test_match() {
    let form_msds = &[
        (
            "1+a+2",
            vec![(Some("msd".to_string()), "sg indef nom".to_string())],
        ),
        (
            "1+ä+2+er",
            vec![(Some("msd".to_string()), "pl indef nom".to_string())],
        ),
        (
            "1+a+2+s",
            vec![(Some("msd".to_string()), "sg indef gen".to_string())],
        ),
    ];
    let var_insts = vec![vec![("1".into(), "b".into()), ("2".into(), "d".into())]];
    let p = Paradigm::new(form_msds, var_insts);
    let tag = (Some("msd"), "sg indef nom");
    let matches = p.r#match(
        "apa",
        MatchOptions {
            baseform: true,
            tag: Some(&tag),
            constrained: false,
            ..Default::default()
        },
    );
    assert_eq!(matches, vec![None]);
    let matches = p.r#match(
        "vad",
        MatchOptions {
            baseform: true,
            tag: Some(&tag),
            constrained: false,
            ..Default::default()
        },
    );
    assert_eq!(
        matches,
        vec![Some(vec![(1, vec!("v".to_string(), "d".to_string()))])]
    );
}

#[test]
fn test_2() {
    let form_msds = &[
        (
            "1+a+2",
            vec![(Some("msd".to_string()), "sg indef nom".to_string())],
        ),
        (
            "1+ä+2+er",
            vec![(Some("msd".to_string()), "pl indef nom".to_string())],
        ),
        (
            "1+a+2+s",
            vec![(Some("msd".to_string()), "sg indef gen".to_string())],
        ),
    ];
    let var_insts = vec![vec![("1".into(), "b".into()), ("2".into(), "d".into())]];
    let p = Paradigm::new(form_msds, var_insts);
    //     form_msds = [
    //         ("1+a+2", ("msd", "sg indef nom")),
    //         ("1+ä+2+er", ("msd", "pl indef nom")),
    //         ("1+a+2+s", ("msd", "sg indef gen")),
    //     ]
    //     var_insts = [[("1", "b"), ("2", "d")]]
    //     p = Paradigm(form_msds, var_insts)
    let var_insts = &["st", "d"];
    let table = p.inflect(var_insts);
    //     assert [
    //         ("stad", ("msd", "sg indef nom")),
    //         ("städer", ("msd", "pl indef nom")),
    //         ("stads", ("msd", "sg indef gen")),
    //     ] == table
}
// @pytest.mark.xfail(reason="don't know")
// def test_paradigm_match_vars():
//     form_msds = [
//         ("1+a+2", ("msd", "sg indef nom")),
//         ("1+ä+2+er", ("msd", "pl indef nom")),
//         ("1+a+2+s", ("msd", "sg indef gen")),
//     ]
//     var_insts = [[("1", "b"), ("2", "d")]]
//     p = Paradigm(form_msds, var_insts)
//     p.match_vars
