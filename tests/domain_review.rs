use notion_dev_refactor_guard::domain_review::{review_lane, review_score, DomainCase};

#[test]
fn domain_review_case_is_stable() {
    let case = DomainCase { signal: 40, slack: 29, drag: 15, confidence: 84 };
    assert_eq!(review_score(case), 148);
    assert_eq!(review_lane(case), "ship");
}
