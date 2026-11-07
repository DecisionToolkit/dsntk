use super::*;

from_examples!(DMN_3_1130);

static_context!(CTX, "{}");

fn eq(invocable: &str, ctx: &FeelContext, expected: &str) {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable,
        ctx,
        expected,
    );
}

#[test]
fn _0001() {
    eq(
        "during",
        &CTX,
        r#"{duringPRF: false, duringPRF2: false, duringPRF3: false, duringPRT: true, duringPRT2: true, duringPRT3: true, duringRRT: true, duringRRT2: true, duringRRT3: true, duringRRT4: true, duringRRT5: true, duringRRT6: true, duringRRT7: true, duringRRT8: true}"#,
    );
}

#[test]
fn _0002() {
    eq(
        "after",
        &CTX,
        r#"{afterPPF: false, afterPPT: true, afterPRF: false, afterPRT: true, afterPRT2: true, afterRPF: false, afterRPF2: false, afterRPT: true, afterRPT2: true, afterRRF: false, afterRRT: true, afterRRT2: true, afterRRT3: true}"#,
    );
}

#[test]
fn _0003() {
    eq(
        "started by",
        &CTX,
        r#"{startedbyRPF: false, startedbyRPF2: false, startedbyRPT: true, startedbyRRF: false, startedbyRRF2: false, startedbyRRT: true, startedbyRRT2: true, startedbyRRT3: true, startedbyRRT4: true, startedbyRRT5: true}"#,
    );
}

#[test]
fn _0004() {
    eq(
        "includes",
        &CTX,
        r#"{includesRPF: false, includesRPF2: false, includesRPF3: false, includesRPT: true, includesRPT2: true, includesRPT3: true, includesRRT: true, includesRRT2: true, includesRRT3: true, includesRRT4: true, includesRRT5: true, includesRRT6: true, includesRRT7: true, includesRRT8: true}"#,
    );
}

#[test]
fn _0005() {
    eq(
        "met by",
        &CTX,
        r#"{metbyRRF: false, metbyRRF2: false, metbyRRF3: false, metbyRRT: true}"#,
    );
}

#[test]
fn _0006() {
    eq(
        "before",
        &CTX,
        r#"{beforPPF: false, beforePPT: true, beforePRF: false, beforePRT: true, beforePRT2: true, beforeRPF: false, beforeRPT: true, beforeRPT2: true, beforeRRF: false, beforeRRT: true, beforeRRT2: true, beforeRRT3: true}"#,
    );
}

#[test]
fn _0007() {
    eq(
        "overlaps",
        &CTX,
        r#"{overlapsRRF: false, overlapsRRF2: false, overlapsRRF3: false, overlapsRRF4: false, overlapsRRF5: false, overlapsRRF6: false, overlapsRRF7: false, overlapsRRF8: false, overlapsRRT: true, overlapsRRT2: true, overlapsRRT3: true, overlapsRRT4: true, overlapsRRT5: true, overlapsRRT6: true}"#,
    );
}

#[test]
fn _0008() {
    eq(
        "overlaps before",
        &CTX,
        r#"{overlapsbeforeRRF: false, overlapsbeforeRRF2: false, overlapsbeforeRRF3: false, overlapsbeforeRRF4: false, overlapsbeforeRRF5: false, overlapsbeforeRRT: true, overlapsbeforeRRT2: true, overlapsbeforeRRT3: true, overlapsbeforeRRT4: true}"#,
    );
}

#[test]
fn _0009() {
    eq(
        "finishes",
        &CTX,
        r#"{finishesPRF: false, finishesPRT: true, finishesRRF: false, finishesRRF2: true, finishesRRT: true, finishesRRT2: true, finishesRRT3: true}"#,
    );
}

#[test]
fn _0010() {
    eq(
        "finished by",
        &CTX,
        r#"{finishedbyRPF: false, finishedbyRPT: true, finishedbyRRF: false, finishedbyRRT: true, finishedbyRRT2: true, finishedbyRRT3: true, finishedbyRRT4: true}"#,
    );
}

#[test]
fn _0011() {
    eq(
        "starts",
        &CTX,
        r#"{startsPRF: false, startsPRF2: false, startsPRT: true, startsRRF: false, startsRRF2: false, startsRRT: true, startsRRT2: true, startsRRT3: true, startsRRT4: true, startsRRT5: true}"#,
    );
}

#[test]
fn _0012() {
    eq(
        "coincides",
        &CTX,
        r#"{coincidesPPF: false, coincidesPPT: true, coincidesRRF: false, coincidesRRF2: false, coincidesRRT: true}"#,
    );
}

#[test]
fn _0013() {
    eq(
        "overlaps after",
        &CTX,
        r#"{overlapsafterRRF: false, overlapsafterRRF2: false, overlapsafterRRF3: false, overlapsafterRRF4: false, overlapsafterRRF5: false, overlapsafterRRT: true, overlapsafterRRT2: true, overlapsafterRRT3: true, overlapsafterRRT4: true}"#,
    );
}

#[test]
fn _0014() {
    eq(
        "meets",
        &CTX,
        r#"{meetsRRF: false, meetsRRF2: false, meetsRRF3: false, meetsRRT: true}"#,
    );
}
