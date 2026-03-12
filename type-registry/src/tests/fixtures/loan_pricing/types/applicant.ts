// L1 — Raw loan application inputs.
// These are the boundary types where the decision engine meets
// the outside world (core banking, loan origination system).

export interface LoanApplication {
  grossAnnualIncome: number;
  totalMonthlyDebt: number;
  requestedMonthlyRepayment: number;
  loanAmount: number;
  termMonths: number;
  purpose: string;
  propertyValue?: number;
  assets?: number;
}

export interface ApplicantProfile {
  bureauScore: number;
  incomeType: string;
  verificationStatus: string;
  existingCustomer: boolean;
  yearsAtAddress?: number;
}
