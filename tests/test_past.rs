#[test]
#[should_panic]
fn should_return_empty_list_when_no_date() {
    //given
    let date = "";

    //when
    funniversaries::find_anniversaries_past(date);

    //then
    //panic
}

#[test]
#[should_panic]
fn should_return_empty_list_when_incorrect_date() {
    //given
    let date = "2020:01:02T06:05:04.333Z";

    //when
    funniversaries::find_anniversaries_past(date);

    //then
    //panic
}

#[test]
fn should_return_list_when_recent_date() {
    //given
    let date = "2020-01-02T06:05:04.333Z";

    //when
    let anniversaries = funniversaries::find_anniversaries_past(date);

    //then
    assert_eq!(anniversaries.len() > 0, true);
}

#[test]
fn should_return_list_when_old_date() {
    //given
    let date = "1605-11-05T23:59:58.666Z";

    //when
    let anniversaries = funniversaries::find_anniversaries_past(date);

    //then
    assert_eq!(anniversaries.len() > 0, true);
}

#[test]
fn should_return_list_when_future_date() {
    //given
    let date = "2222-11-22T11:22:11.222Z";

    //when
    let anniversaries = funniversaries::find_anniversaries_past(date);

    //then
    assert_eq!(anniversaries.len() > 0, false);
}
