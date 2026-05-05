use notion_dev_refactor_guard::{classify, score, Signal};
#[test]
fn fixture_decisions() {
    let signal = Signal { demand: 55, capacity: 102, latency: 20, risk: 11, weight: 4 };
    assert_eq!(score(signal), 141);
    assert_eq!(classify(signal), "review");
    let signal = Signal { demand: 87, capacity: 104, latency: 27, risk: 10, weight: 8 };
    assert_eq!(score(signal), 222);
    assert_eq!(classify(signal), "accept");
    let signal = Signal { demand: 97, capacity: 92, latency: 10, risk: 22, weight: 13 };
    assert_eq!(score(signal), 234);
    assert_eq!(classify(signal), "accept");
}
