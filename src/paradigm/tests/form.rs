use super::Form;

#[test]
fn constructor_minimum() {
    let form = Form::wo_msd("");

    assert_eq!(form.form, [""]);
    //     # assert isinstance(form.msd, list)
}

#[test]
fn constructor_single_msd() {
    let msd = vec![("vb")];
    // let form = Form::new("", msd);

    // assert_eq!(form.msd, [("vb",)]);
}
