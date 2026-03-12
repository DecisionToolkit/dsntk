// L3 — Affordability sub-score output types.
// These carry the results of DTI calculation, affordability classification,
// and income verification modification through the DRD.

export interface DtiCalculationOutput {
  dtiRatio: number;
  stressedDtiRatio: number;
  stressedRepayment: number;
  calculationBasis: string;
}

export interface AffordabilityClassificationOutput {
  affordabilityBand: string;
  reasonCode: string;
  dtiRatio: number;
  stressedDtiRatio: number;
}

export interface IncomeVerificationOutput {
  adjustedBand: string;
  modifierCode: string;
  originalBand: string;
}

export interface AffordabilityResult {
  finalBand: string;
  reasonCode: string;
  modifierCode: string;
  dtiRatio: number;
  stressedDtiRatio: number;
  calculationBasis: string;
  auditTrail: {
    tableAOutput: string;
    tableBBand: string;
    tableBReason: string;
    tableCModifier: string;
    tableCOriginalBand: string;
  };
}
