// src/lib/types.ts

// Type for the financial data of a single year
export interface FinancialYearData {
    year: number;
    // Scores are serialized as numbers (f64) or null from Rust Decimal
    overall_score: number | null;
    financial_health_score: number | null;
    infrastructure_score: number | null;
    efficiency_score: number | null;
    accountability_score: number | null;
    revenue: number | null;
    expenditure: number | null;
    capital_expenditure: number | null;
    debt: number | null;
    audit_outcome: string | null;
}

// Type for the detailed information of a single municipality returned by the API
export interface MunicipalityDetail {
    id: string; // Municipality code e.g., "BUF"
    name: string; // e.g., "Buffalo City Metropolitan Municipality"
    province: string; // e.g., "Eastern Cape"
    population: number | null; // e.g., 834997
    classification: string | null; // e.g., "Metro"
    website: string | null; // URL
    financials: FinancialYearData[]; // Array of financial data
    geometry?: any; // Optional GeoJSON geometry if included
}

// You might also want types for data coming from $page.data in Svelte components
// For the comparison page:
export interface ComparisonPageData {
    municipalities: MunicipalityDetail[];
    requestedIds: string[];
    error?: string; // Optional error message from load function
}

// For the single view page (if not already defined elsewhere):
export interface SinglePageData {
    details: MunicipalityDetail | null; // Can be null if fetch failed
    error?: string; // Optional error message from load function
}

// Type for the basic municipality info used in the map/list
export interface MunicipalityBaseInfo {
    id: string;
    name: string;
    province: string;
    latest_overall_score?: string | number | null; // Optional latest score for map styling
    // Include geometry if it comes with this API endpoint
    geometry?: any;
}
