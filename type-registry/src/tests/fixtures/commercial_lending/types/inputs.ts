// Commercial lending input types.
// Field names match the FEEL names used in decision table columns.

export interface ApplicantData {
  "Business Name": string;
  "Business Type": string;
  "Years Trading": number;
  "Sector": string;
  "Annual Turnover": number;
  "Director Count": number;
  "Has Guarantee": boolean;
}

export interface BureauReport {
  "Bureau Score": number;
  "CCJ Count": number;
  "CCJ Total Value": number;
  "Payment Days Average": number;
  "Filing Status": string;
  "Director Defaults": number;
}

export interface FinancialStatements {
  "Net Profit": number;
  "Depreciation": number;
  "Interest Expense": number;
  "Total Debt": number;
  "Total Assets": number;
  "Accounts Type": string;
  "Accounts Age": number;
  "Leverage Ratio": number;
  "Annual Debt Service": number;
}

export interface LoanRequest {
  "Requested Amount": number;
  "Term Months": number;
  "Purpose": string;
  "Repayment Type": string;
}

export interface PropertyData {
  "Property Type": string;
  "Valuation Amount": number;
  "Location": string;
  "Tenure": string;
  "Lease Remaining"?: number;
  "Environmental Risk": string;
  "Flood Zone": string;
}
