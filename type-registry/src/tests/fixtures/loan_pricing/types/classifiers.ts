// L2 — Input classifier enums for loan pricing decisions.
// These constrain raw application inputs into categorical values
// that drive downstream decision table activation.

export type ProductType = "mortgage" | "unsecured term" | "revolving facility" | "buy-to-let";

export type BorrowerType = "individual" | "sme" | "corporate";

export type IncomeType = "employed" | "self-employed" | "contract" | "benefits";

export type VerificationStatus =
  | "payslip verified"
  | "bank statement only"
  | "unverified"
  | "fraud flag"
  | "1 year accounts"
  | "2+ year accounts"
  | "contract < 6 months"
  | "contract 6-24 months"
  | "contract > 24 months";

export type AffordabilityBand = "Pass" | "Marginal" | "Refer" | "Fail";

export type AffordabilityReasonCode =
  | "AFC-01"
  | "AFC-02"
  | "AFC-03"
  | "AFC-04"
  | "AFC-05"
  | "AFC-06"
  | "AFC-07"
  | "AFC-08"
  | "AFC-09";

export type IncomeModifierCode =
  | "INC-00"
  | "INC-01"
  | "INC-02"
  | "INC-03"
  | "INC-04"
  | "INC-05"
  | "INC-06"
  | "INC-07"
  | "INC-08"
  | "INC-09"
  | "INC-10"
  | "INC-99";
