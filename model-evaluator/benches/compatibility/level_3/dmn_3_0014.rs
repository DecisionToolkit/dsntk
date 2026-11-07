use super::*;

from_examples!(DMN_3_0014);

#[bench]
fn _0001(b: &mut Bencher) {
    let ctx = context(r#"{p: 1, r: 1, n: 1, pmt: 1}"#);
    let invocable_name = "equity36Mo";
    assert_business_knowledge_model(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"1"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0002(b: &mut Bencher) {
    let ctx = context(r#"{p: 2, r: 1, n: 1, pmt: 1}"#);
    let invocable_name = "equity36Mo";
    assert_business_knowledge_model(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"2.083333333333333333333333333333333"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0003(b: &mut Bencher) {
    let ctx = context(r#"{p: 1, r: 1, n: 1}"#);
    let invocable_name = "monthlyPayment";
    assert_business_knowledge_model(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"1.083333333333333333333333333333338"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0004(b: &mut Bencher) {
    let ctx = context(r#"{p: 2, r: 1, n: 1}"#);
    let invocable_name = "monthlyPayment";
    assert_business_knowledge_model(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"2.166666666666666666666666666666676"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0005(b: &mut Bencher) {
    let ctx = context(
        r#"{requestedAmt: 330000, product: {fee: 0, lenderName: "Oceans Capital", points: 0, rate: 0.03500}}"#,
    );
    let invocable_name = "FinancialMetrics";
    assert_business_knowledge_model(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"{downPmtAmt: 66000, equity36moPct: 0.1229130806675864888391782030891128, fee: 0, lenderName: "Oceans Capital", loanAmt: 330000, paymentAmt: 1481.84746976912090291141532541092, points: 0, rate: 0.03500}"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0006(b: &mut Bencher) {
    let ctx = context(r#"{}"#);
    let invocable_name = "Bankrates";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"[{fee: 0, lenderName: "Oceans Capital", points: 0, rate: 0.03500}, {fee: 2700, lenderName: "eClick Lending", points: 1.1, rate: 0.03200}, {fee: 1200, lenderName: "eClickLending", points: 0.1, rate: 0.03375}, {fee: 3966, lenderName: "AimLoan", points: 1.1, rate: 0.03000}, {fee: 285, lenderName: "Home Loans Today", points: 1.1, rate: 0.03125}, {fee: 4028, lenderName: "Sebonic", points: 0.1, rate: 0.03125}, {fee: 4317, lenderName: "AimLoan", points: 0.1, rate: 0.03125}, {fee: 2518, lenderName: "eRates Mortgage", points: 1.1, rate: 0.03125}, {fee: 822, lenderName: "Home Loans Today", points: 0.1, rate: 0.03250}, {fee: 1995, lenderName: "AimLoan", points: 0, rate: 0.03250}]"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0007(b: &mut Bencher) {
    let ctx = context(r#"{RequestedAmt: 330000}"#);
    let invocable_name = "RankedProducts";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#"{metricsTable: [{downPmtAmt: 66000, equity36moPct: 0.1229130806675864888391782030891128, fee: 0, lenderName: "Oceans Capital", loanAmt: 330000, paymentAmt: 1481.84746976912090291141532541092, points: 0, rate: 0.03500}, {downPmtAmt: 67266, equity36moPct: 0.113702973187458793444684616269924, fee: 2700, lenderName: "eClick Lending", loanAmt: 336330, paymentAmt: 1454.515807764692215596585144120557, points: 1.1, rate: 0.03200}, {downPmtAmt: 66306, equity36moPct: 0.1219807467513805938540322263850632, fee: 1200, lenderName: "eClickLending", loanAmt: 331530, paymentAmt: 1465.681565899863510819264705552203, points: 0.1, rate: 0.03375}, {downPmtAmt: 67519.2, equity36moPct: 0.1154298007315239099518616327317256, fee: 3966, lenderName: "AimLoan", loanAmt: 337596, paymentAmt: 1423.31835370927550454489017290607, points: 1.1, rate: 0.03000}, {downPmtAmt: 66783, equity36moPct: 0.1219478687825970483711604225076944, fee: 285, lenderName: "Home Loans Today", loanAmt: 333915, paymentAmt: 1430.409890005738806923767229688187, points: 1.1, rate: 0.03125}, {downPmtAmt: 66871.6, equity36moPct: 0.1207829702481517269343529237944616, fee: 4028, lenderName: "Sebonic", loanAmt: 334358, paymentAmt: 1432.3075932573823158750489297698, points: 0.1, rate: 0.03125}, {downPmtAmt: 66929.4, equity36moPct: 0.1200230251545745307825755713607728, fee: 4317, lenderName: "AimLoan", loanAmt: 334647, paymentAmt: 1433.545598313194898464034056911078, points: 0.1, rate: 0.03125}, {downPmtAmt: 67229.6, equity36moPct: 0.116076043890009231738223301454312, fee: 2518, lenderName: "eRates Mortgage", loanAmt: 336148, paymentAmt: 1439.975513845287239177067537323045, points: 1.1, rate: 0.03125}, {downPmtAmt: 66230.4, equity36moPct: 0.126102527036108194734017086504168, fee: 822, lenderName: "Home Loans Today", loanAmt: 331152, paymentAmt: 1441.194429734569798816940611550537, points: 0.1, rate: 0.03250}, {downPmtAmt: 66399, equity36moPct: 0.1238778822515121156167560595555856, fee: 1995, lenderName: "AimLoan", loanAmt: 331995, paymentAmt: 1444.863219004349967260442933552328, points: 0, rate: 0.03250}], rankByDownPmt: [{downPmtAmt: 66000, equity36moPct: 0.1229130806675864888391782030891128, fee: 0, lenderName: "Oceans Capital", loanAmt: 330000, paymentAmt: 1481.84746976912090291141532541092, points: 0, rate: 0.03500}, {downPmtAmt: 66230.4, equity36moPct: 0.126102527036108194734017086504168, fee: 822, lenderName: "Home Loans Today", loanAmt: 331152, paymentAmt: 1441.194429734569798816940611550537, points: 0.1, rate: 0.03250}, {downPmtAmt: 66306, equity36moPct: 0.1219807467513805938540322263850632, fee: 1200, lenderName: "eClickLending", loanAmt: 331530, paymentAmt: 1465.681565899863510819264705552203, points: 0.1, rate: 0.03375}, {downPmtAmt: 66399, equity36moPct: 0.1238778822515121156167560595555856, fee: 1995, lenderName: "AimLoan", loanAmt: 331995, paymentAmt: 1444.863219004349967260442933552328, points: 0, rate: 0.03250}, {downPmtAmt: 66783, equity36moPct: 0.1219478687825970483711604225076944, fee: 285, lenderName: "Home Loans Today", loanAmt: 333915, paymentAmt: 1430.409890005738806923767229688187, points: 1.1, rate: 0.03125}, {downPmtAmt: 66871.6, equity36moPct: 0.1207829702481517269343529237944616, fee: 4028, lenderName: "Sebonic", loanAmt: 334358, paymentAmt: 1432.3075932573823158750489297698, points: 0.1, rate: 0.03125}, {downPmtAmt: 66929.4, equity36moPct: 0.1200230251545745307825755713607728, fee: 4317, lenderName: "AimLoan", loanAmt: 334647, paymentAmt: 1433.545598313194898464034056911078, points: 0.1, rate: 0.03125}, {downPmtAmt: 67229.6, equity36moPct: 0.116076043890009231738223301454312, fee: 2518, lenderName: "eRates Mortgage", loanAmt: 336148, paymentAmt: 1439.975513845287239177067537323045, points: 1.1, rate: 0.03125}, {downPmtAmt: 67266, equity36moPct: 0.113702973187458793444684616269924, fee: 2700, lenderName: "eClick Lending", loanAmt: 336330, paymentAmt: 1454.515807764692215596585144120557, points: 1.1, rate: 0.03200}, {downPmtAmt: 67519.2, equity36moPct: 0.1154298007315239099518616327317256, fee: 3966, lenderName: "AimLoan", loanAmt: 337596, paymentAmt: 1423.31835370927550454489017290607, points: 1.1, rate: 0.03000}], rankByEquityPct: [{downPmtAmt: 66230.4, equity36moPct: 0.126102527036108194734017086504168, fee: 822, lenderName: "Home Loans Today", loanAmt: 331152, paymentAmt: 1441.194429734569798816940611550537, points: 0.1, rate: 0.03250}, {downPmtAmt: 66399, equity36moPct: 0.1238778822515121156167560595555856, fee: 1995, lenderName: "AimLoan", loanAmt: 331995, paymentAmt: 1444.863219004349967260442933552328, points: 0, rate: 0.03250}, {downPmtAmt: 66000, equity36moPct: 0.1229130806675864888391782030891128, fee: 0, lenderName: "Oceans Capital", loanAmt: 330000, paymentAmt: 1481.84746976912090291141532541092, points: 0, rate: 0.03500}, {downPmtAmt: 66306, equity36moPct: 0.1219807467513805938540322263850632, fee: 1200, lenderName: "eClickLending", loanAmt: 331530, paymentAmt: 1465.681565899863510819264705552203, points: 0.1, rate: 0.03375}, {downPmtAmt: 66783, equity36moPct: 0.1219478687825970483711604225076944, fee: 285, lenderName: "Home Loans Today", loanAmt: 333915, paymentAmt: 1430.409890005738806923767229688187, points: 1.1, rate: 0.03125}, {downPmtAmt: 66871.6, equity36moPct: 0.1207829702481517269343529237944616, fee: 4028, lenderName: "Sebonic", loanAmt: 334358, paymentAmt: 1432.3075932573823158750489297698, points: 0.1, rate: 0.03125}, {downPmtAmt: 66929.4, equity36moPct: 0.1200230251545745307825755713607728, fee: 4317, lenderName: "AimLoan", loanAmt: 334647, paymentAmt: 1433.545598313194898464034056911078, points: 0.1, rate: 0.03125}, {downPmtAmt: 67229.6, equity36moPct: 0.116076043890009231738223301454312, fee: 2518, lenderName: "eRates Mortgage", loanAmt: 336148, paymentAmt: 1439.975513845287239177067537323045, points: 1.1, rate: 0.03125}, {downPmtAmt: 67519.2, equity36moPct: 0.1154298007315239099518616327317256, fee: 3966, lenderName: "AimLoan", loanAmt: 337596, paymentAmt: 1423.31835370927550454489017290607, points: 1.1, rate: 0.03000}, {downPmtAmt: 67266, equity36moPct: 0.113702973187458793444684616269924, fee: 2700, lenderName: "eClick Lending", loanAmt: 336330, paymentAmt: 1454.515807764692215596585144120557, points: 1.1, rate: 0.03200}], rankByMonthlyPmt: [{downPmtAmt: 67519.2, equity36moPct: 0.1154298007315239099518616327317256, fee: 3966, lenderName: "AimLoan", loanAmt: 337596, paymentAmt: 1423.31835370927550454489017290607, points: 1.1, rate: 0.03000}, {downPmtAmt: 66783, equity36moPct: 0.1219478687825970483711604225076944, fee: 285, lenderName: "Home Loans Today", loanAmt: 333915, paymentAmt: 1430.409890005738806923767229688187, points: 1.1, rate: 0.03125}, {downPmtAmt: 66871.6, equity36moPct: 0.1207829702481517269343529237944616, fee: 4028, lenderName: "Sebonic", loanAmt: 334358, paymentAmt: 1432.3075932573823158750489297698, points: 0.1, rate: 0.03125}, {downPmtAmt: 66929.4, equity36moPct: 0.1200230251545745307825755713607728, fee: 4317, lenderName: "AimLoan", loanAmt: 334647, paymentAmt: 1433.545598313194898464034056911078, points: 0.1, rate: 0.03125}, {downPmtAmt: 67229.6, equity36moPct: 0.116076043890009231738223301454312, fee: 2518, lenderName: "eRates Mortgage", loanAmt: 336148, paymentAmt: 1439.975513845287239177067537323045, points: 1.1, rate: 0.03125}, {downPmtAmt: 66230.4, equity36moPct: 0.126102527036108194734017086504168, fee: 822, lenderName: "Home Loans Today", loanAmt: 331152, paymentAmt: 1441.194429734569798816940611550537, points: 0.1, rate: 0.03250}, {downPmtAmt: 66399, equity36moPct: 0.1238778822515121156167560595555856, fee: 1995, lenderName: "AimLoan", loanAmt: 331995, paymentAmt: 1444.863219004349967260442933552328, points: 0, rate: 0.03250}, {downPmtAmt: 67266, equity36moPct: 0.113702973187458793444684616269924, fee: 2700, lenderName: "eClick Lending", loanAmt: 336330, paymentAmt: 1454.515807764692215596585144120557, points: 1.1, rate: 0.03200}, {downPmtAmt: 66306, equity36moPct: 0.1219807467513805938540322263850632, fee: 1200, lenderName: "eClickLending", loanAmt: 331530, paymentAmt: 1465.681565899863510819264705552203, points: 0.1, rate: 0.03375}, {downPmtAmt: 66000, equity36moPct: 0.1229130806675864888391782030891128, fee: 0, lenderName: "Oceans Capital", loanAmt: 330000, paymentAmt: 1481.84746976912090291141532541092, points: 0, rate: 0.03500}], rankByRate: [{downPmtAmt: 67519.2, equity36moPct: 0.1154298007315239099518616327317256, fee: 3966, lenderName: "AimLoan", loanAmt: 337596, paymentAmt: 1423.31835370927550454489017290607, points: 1.1, rate: 0.03000}, {downPmtAmt: 66783, equity36moPct: 0.1219478687825970483711604225076944, fee: 285, lenderName: "Home Loans Today", loanAmt: 333915, paymentAmt: 1430.409890005738806923767229688187, points: 1.1, rate: 0.03125}, {downPmtAmt: 66871.6, equity36moPct: 0.1207829702481517269343529237944616, fee: 4028, lenderName: "Sebonic", loanAmt: 334358, paymentAmt: 1432.3075932573823158750489297698, points: 0.1, rate: 0.03125}, {downPmtAmt: 66929.4, equity36moPct: 0.1200230251545745307825755713607728, fee: 4317, lenderName: "AimLoan", loanAmt: 334647, paymentAmt: 1433.545598313194898464034056911078, points: 0.1, rate: 0.03125}, {downPmtAmt: 67229.6, equity36moPct: 0.116076043890009231738223301454312, fee: 2518, lenderName: "eRates Mortgage", loanAmt: 336148, paymentAmt: 1439.975513845287239177067537323045, points: 1.1, rate: 0.03125}, {downPmtAmt: 67266, equity36moPct: 0.113702973187458793444684616269924, fee: 2700, lenderName: "eClick Lending", loanAmt: 336330, paymentAmt: 1454.515807764692215596585144120557, points: 1.1, rate: 0.03200}, {downPmtAmt: 66230.4, equity36moPct: 0.126102527036108194734017086504168, fee: 822, lenderName: "Home Loans Today", loanAmt: 331152, paymentAmt: 1441.194429734569798816940611550537, points: 0.1, rate: 0.03250}, {downPmtAmt: 66399, equity36moPct: 0.1238778822515121156167560595555856, fee: 1995, lenderName: "AimLoan", loanAmt: 331995, paymentAmt: 1444.863219004349967260442933552328, points: 0, rate: 0.03250}, {downPmtAmt: 66306, equity36moPct: 0.1219807467513805938540322263850632, fee: 1200, lenderName: "eClickLending", loanAmt: 331530, paymentAmt: 1465.681565899863510819264705552203, points: 0.1, rate: 0.03375}, {downPmtAmt: 66000, equity36moPct: 0.1229130806675864888391782030891128, fee: 0, lenderName: "Oceans Capital", loanAmt: 330000, paymentAmt: 1481.84746976912090291141532541092, points: 0, rate: 0.03500}]}"#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}
