//! # Various test cases

use super::*;

#[test]
fn _0001() {
    let scope = &te_scope(
        r#"
      {
        offer: [
          { tenor:   7, rate: 3.8300 },
          { tenor:  14, rate: 4.0100 },
          { tenor:  30, rate: 4.2100 },
          { tenor:  90, rate: 4.8400 },
          { tenor: 180, rate: 5.1000 },
          { tenor: 360, rate: 5.2000 }
        ],
        bid: [
          { tenor:   7, rate: 3.6300 },
          { tenor:  14, rate: 3.8100 },
          { tenor:  30, rate: 4.0100 },
          { tenor:  90, rate: 4.6400 },
          { tenor: 180, rate: 4.9000 },
          { tenor: 360, rate: 5.0000 }
        ]
      }
  "#,
    );
    let expression = r#"
      for i in 1..6 return ((log(1 + (offer[i].rate / 100) * (offer[i].tenor / 365))) / (offer[i].tenor / 365) + (log(1 + (bid[i].rate / 100) * (bid[i].tenor / 365))) / (bid[i].tenor / 365)) / 2
  "#;
    let expected = r#"[
    0.03728665564658653288015998590256039,
    0.03907069051683237804063421928246624,
    0.04103069553301539980418540264587278,
    0.04712502129847047437093656355027444,
    0.04939327625296464963140082299537544,
    0.0497583218296869197946039145374107
  ]"#;
    te_be_value(false, scope, expression, expected);
}

#[test]
fn _0002() {
    let scope = &te_scope(
        r#"
        {
          Bounds: {
            Min: { Tenor: 7,  Rate: 0.03728665564658653288015998590256039 },
            Max: { Tenor: 14, Rate: 0.03907069051683237804063421928246624 }
          },
          Days: 12
        }
    "#,
    );
    let expression = r#" ((Bounds.Max.Rate - Bounds.Min.Rate) * (Days - Bounds.Min.Tenor) / (Bounds.Max.Tenor - Bounds.Min.Tenor)) + Bounds.Min.Rate "#;
    te_be_value(
        false,
        scope,
        expression,
        r#"0.03856096626819070799478443831677885"#,
    );
}
