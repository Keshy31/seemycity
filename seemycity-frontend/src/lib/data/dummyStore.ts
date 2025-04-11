// src/lib/data/dummyStore.ts
import type { FeatureCollection } from 'geojson';

// Interface for individual year's financial data, matching API spec
export interface FinancialYearData {
    year: number;
    revenue: number | null;
    expenditure: number | null;
    capital_expenditure: number | null;
    debt: number | null;
    audit_outcome: 'Clean' | 'Unqualified' | 'Qualified' | 'Adverse' | 'Disclaimer' | 'Unavailable' | null; // Allow null
    score: number | null; // Score for that specific year
}

export interface MunicipalityDetails { // Reflects schema + calculated fields
    id: string; // Corresponds to municipalities.id
    name: string; // municipalities.name
    province: string; // municipalities.province
    population: number; // municipalities.population
    classification: string | null; // Added based on spec/backend
    website: string | null; // Added based on spec/backend

    // Array of financial data per year, matching API spec
    financials: FinancialYearData[];

    // Overall score (e.g., latest year's score, or an average - TBD based on frontend needs)
    latest_score: number | null;

    // Calculated score components (optional but useful for display)
    score_breakdown: { // Based on the LATEST year's data for now
        financial_health: number; // Weighted % contribution
        infrastructure: number;   // Weighted % contribution
        efficiency: number;       // Weighted % contribution
        accountability: number;   // Weighted % contribution
    } | null; // Allow null if no score/financials
}

// Helper function to calculate score (simplified version for dummy data)
// Based on logic from tech.md
const calculateDummyYearScore = (yearData: Omit<FinancialYearData, 'score'>, population: number ): 
    { score: number; breakdown: MunicipalityDetails['score_breakdown'] } => {
    // Destructure needed values from the single year's data
    const { revenue, debt, expenditure, capital_expenditure, audit_outcome } = yearData;

    // Avoid division by zero / handle missing data gracefully
    const safePopulation = population > 0 ? population : 1; 
    // Use nullish coalescing to treat null financial values as 0 for calculation safety
    const safeRevenue = revenue ?? 0;
    const safeDebt = debt ?? 0;
    const safeExpenditure = expenditure ?? 0;
    const safeCapex = capital_expenditure ?? 0;

    // Pillar Scores (0-100, unweighted first)
    // Check for division by zero when calculating ratios
    const rev_per_capita_norm = Math.min(safeRevenue / safePopulation, 15000) / 15000 * 100; 
    const debt_ratio_norm = safeRevenue > 0 ? (100 - (Math.min(safeDebt / safeRevenue, 1.5) / 1.5 * 100)) : 0; 
    const financial_health_raw = (rev_per_capita_norm * 0.6) + (debt_ratio_norm * 0.4); // Example internal weighting

    const capex_ratio_norm = safeExpenditure > 0 ? (Math.min(safeCapex / safeExpenditure, 0.3) / 0.3 * 100) : 0; 

    // Using (Expenditure - Capex) as proxy for Operating Expenditure
    const operating_expenditure = safeExpenditure - safeCapex;
    const efficiency_ratio_norm = safeExpenditure > 0 ? (100 - (Math.min(operating_expenditure / safeExpenditure, 1.0) / 1.0 * 100)) : 0;

    const audit_score_raw = { // Handle potential null audit outcome
        'Clean': 100,
        'Unqualified': 75,
        'Qualified': 50,
        'Adverse': 0,
        'Disclaimer': 0,
        'Unavailable': 0, // Treat unavailable as 0
        'null': 0, // Treat null as 0
    }[audit_outcome ?? 'null']; // Use nullish coalescing

    // Weighted Pillar Contributions
    const financial_health_weighted = financial_health_raw * 0.30;
    const infrastructure_weighted = capex_ratio_norm * 0.25;
    const efficiency_weighted = efficiency_ratio_norm * 0.25;
    const accountability_weighted = audit_score_raw * 0.20;

    const total_score = Math.round(financial_health_weighted + infrastructure_weighted + efficiency_weighted + accountability_weighted);

    return {
        score: Math.max(0, Math.min(100, total_score)), // Clamp score 0-100
        breakdown: {
            financial_health: financial_health_weighted,
            infrastructure: infrastructure_weighted,
            efficiency: efficiency_weighted,
            accountability: accountability_weighted,
        }
    };
};

// -- Dummy Data --
// Note: Financial figures are purely illustrative and not based on real data

const dummyDataRaw = [
    {
        id: 'JHB01',
        name: 'Johannesburg Metro (Mock)',
        province: 'GP',
        population: 5635127,
        classification: 'Metro',
        website: 'https://www.joburg.org.za/',
        financials: [
            {
                year: 2023,
                revenue: 70e9, // 70 billion
                debt: 30e9, // 30 billion
                expenditure: 68e9,
                capital_expenditure: 10e9,
                audit_outcome: 'Qualified' as const,
            },
            {
                year: 2022,
                revenue: 65e9,
                debt: 25e9,
                expenditure: 62e9,
                capital_expenditure: 9e9,
                audit_outcome: 'Unqualified' as const,
            },
        ],
    },
    {
        id: 'CPT01',
        name: 'Cape Town Metro (Mock)',
        province: 'WC',
        population: 4617560,
        classification: 'Metro',
        website: 'https://www.capetown.gov.za/',
        financials: [
            {
                year: 2023,
                revenue: 65e9,
                debt: 20e9,
                expenditure: 60e9,
                capital_expenditure: 12e9,
                audit_outcome: 'Clean' as const,
            },
            {
                year: 2022,
                revenue: 60e9,
                debt: 18e9,
                expenditure: 55e9,
                capital_expenditure: 10e9,
                audit_outcome: 'Clean' as const,
            },
        ],
    },
    {
        id: 'ETH01',
        name: 'eThekwini Metro (Mock) (DBN)',
        province: 'KZN',
        population: 3900000, // Approx
        classification: 'Metro',
        website: 'https://www.durban.gov.za/',
        financials: [
            {
                year: 2023,
                revenue: 50e9,
                debt: 35e9,
                expenditure: 52e9, // Expenditure exceeds revenue
                capital_expenditure: 7e9,
                audit_outcome: 'Adverse' as const,
            },
            {
                year: 2022,
                revenue: 45e9,
                debt: 30e9,
                expenditure: 48e9,
                capital_expenditure: 6e9,
                audit_outcome: 'Adverse' as const,
            },
        ],
    },
    {
        id: 'MAN01', // Example: Mangaung
        name: 'Mangaung Metro (Mock)',
        province: 'FS',
        population: 787803,
        classification: 'Metro',
        website: 'https://www.mangaung.co.za/',
        financials: [
            {
                year: 2023,
                revenue: 10e9,
                debt: 8e9,
                expenditure: 9.5e9,
                capital_expenditure: 0.8e9, // Low capex
                audit_outcome: 'Disclaimer' as const,
            },
            {
                year: 2022,
                revenue: 9e9,
                debt: 7e9,
                expenditure: 8.5e9,
                capital_expenditure: 0.7e9,
                audit_outcome: 'Disclaimer' as const,
            },
        ],
    },
    {
        id: 'TSH01', // Example: Tshwane
        name: 'Tshwane Metro (Mock)',
        province: 'GP',
        population: 3390000, // Approx
        classification: 'Metro',
        website: 'https://www.tshwane.gov.za/',
        financials: [
            {
                year: 2023,
                revenue: 45e9,
                debt: 40e9, // High debt
                expenditure: 46e9, 
                capital_expenditure: 5e9,
                audit_outcome: 'Unqualified' as const,
            },
            {
                year: 2022,
                revenue: 40e9,
                debt: 35e9,
                expenditure: 42e9,
                capital_expenditure: 4e9,
                audit_outcome: 'Unqualified' as const,
            },
        ],
    }
];

// Process raw data to calculate scores and structure for export
export const dummyMunicipalityDetails: Record<string, MunicipalityDetails> =
    dummyDataRaw.reduce((acc, rawMuni) => {
        const latestFinancials = rawMuni.financials[rawMuni.financials.length - 1];
        if (!latestFinancials) {
            acc[rawMuni.id] = {
                ...rawMuni,
                latest_score: 0,
                score_breakdown: null,
            };
        } else {
            const { score, breakdown } = calculateDummyYearScore(latestFinancials, rawMuni.population);
            acc[rawMuni.id] = {
                ...rawMuni,
                latest_score: score,
                score_breakdown: breakdown,
            };
        }
        return acc;
    }, {} as Record<string, MunicipalityDetails>);

// Prepare GeoJSON with updated scores
export const dummyMunicipalitiesGeoJSON: FeatureCollection = {
    type: 'FeatureCollection',
    // Map over the original raw data to get base info + geometry coordinates
    features: dummyDataRaw.map(muni => ({
        type: 'Feature',
        geometry: {
            type: 'Polygon', // Still using placeholder squares
            coordinates: getDummyCoordinates(muni.id) // Helper to get coordinates
        },
        // Pull properties from the raw data and the processed details
        properties: {
            id: muni.id,
            name: muni.name,
            province: muni.province, 
            population: muni.population, 
            classification: muni.classification, 
            // Get the calculated latest_score from the processed details object
            score: dummyMunicipalityDetails[muni.id]?.latest_score ?? 0 // Use latest_score, default to 0
        }
    }))
};

// Helper function to provide dummy coordinates (replace with real GeoJSON later)
function getDummyCoordinates(id: string): number[][][] {
    switch (id) {
        case 'JHB01': return [[[27.5, -25.7], [28.5, -25.7], [28.5, -26.7], [27.5, -26.7], [27.5, -25.7]]];
        case 'CPT01': return [[[18.0, -33.5], [19.0, -33.5], [19.0, -34.5], [18.0, -34.5], [18.0, -33.5]]];
        case 'ETH01': return [[[30.5, -29.5], [31.5, -29.5], [31.5, -30.5], [30.5, -30.5], [30.5, -29.5]]];
        case 'MAN01': return [[[26.0, -29.0], [27.0, -29.0], [27.0, -29.5], [26.0, -29.5], [26.0, -29.0]]]; // Approx Mangaung
        case 'TSH01': return [[[28.0, -25.5], [29.0, -25.5], [29.0, -26.0], [28.0, -26.0], [28.0, -25.5]]]; // Approx Tshwane
        default: return [[[0, 0], [1, 0], [1, 1], [0, 1], [0, 0]]]; // Default square
    }
}