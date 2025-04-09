// src/lib/data/dummyStore.ts
import type { FeatureCollection } from 'geojson';

export interface MunicipalityDetails { // Reflects schema + calculated fields
    id: string; // Corresponds to municipalities.id
    name: string; // municipalities.name
    province: string; // municipalities.province
    population: number; // municipalities.population
    year: number; // financial_data.year
    score: number; // financial_data.score (calculated)
    // Raw metrics from financial_data
    revenue: number; 
    debt: number;
    expenditure: number;
    capital_expenditure: number;
    audit_outcome: 'Clean' | 'Unqualified' | 'Qualified' | 'Adverse' | 'Disclaimer' | 'Unavailable'; 
    // Calculated score components (optional but useful for display)
    score_breakdown: {
        financial_health: number; // Weighted % contribution
        infrastructure: number;   // Weighted % contribution
        efficiency: number;       // Weighted % contribution
        accountability: number;   // Weighted % contribution
    };
}

// Helper function to calculate score (simplified version for dummy data)
// Based on logic from tech.md
const calculateDummyScore = (m: Omit<MunicipalityDetails, 'score' | 'score_breakdown'>): { score: number; breakdown: MunicipalityDetails['score_breakdown'] } => {
    const { revenue, population, debt, expenditure, capital_expenditure, audit_outcome } = m;

    // Avoid division by zero / handle missing data gracefully
    const safePopulation = population > 0 ? population : 1;
    const safeRevenue = revenue > 0 ? revenue : 1; // Avoid division by zero for ratios
    const safeExpenditure = expenditure > 0 ? expenditure : 1; 

    // Pillar Scores (0-100, unweighted first)
    const rev_per_capita_norm = Math.min(revenue / safePopulation, 15000) / 15000 * 100; // Max R15k per capita = 100
    const debt_ratio_norm = 100 - (Math.min(debt / safeRevenue, 1.5) / 1.5 * 100); // Max 150% debt-to-revenue -> 0 score
    const financial_health_raw = (rev_per_capita_norm * 0.6) + (debt_ratio_norm * 0.4); // Example internal weighting

    const capex_ratio_norm = Math.min(capital_expenditure / safeExpenditure, 0.3) / 0.3 * 100; // Max 30% capex ratio = 100

    // Using (Expenditure - Capex) as proxy for Operating Expenditure
    const operating_expenditure = expenditure - capital_expenditure;
    const efficiency_ratio_norm = 100 - (Math.min(operating_expenditure / safeExpenditure, 1.0) / 1.0 * 100); // Max 100% operating ratio -> 0

    const audit_score_raw = {
        'Clean': 100,
        'Unqualified': 75,
        'Qualified': 50,
        'Adverse': 0,
        'Disclaimer': 0,
        'Unavailable': 0, // Treat unavailable as 0 for score
    }[audit_outcome];

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
        year: 2023,
        // Raw metrics
        revenue: 70e9, // 70 billion
        debt: 30e9, // 30 billion
        expenditure: 68e9,
        capital_expenditure: 10e9,
        audit_outcome: 'Qualified' as const,
    },
    {
        id: 'CPT01',
        name: 'Cape Town Metro (Mock)',
        province: 'WC',
        population: 4617560,
        year: 2023,
        // Raw metrics
        revenue: 65e9,
        debt: 20e9,
        expenditure: 60e9,
        capital_expenditure: 12e9,
        audit_outcome: 'Clean' as const,
    },
    {
        id: 'ETH01',
        name: 'eThekwini Metro (Mock) (DBN)',
        province: 'KZN',
        population: 3900000, // Approx
        year: 2023,
        // Raw metrics
        revenue: 50e9,
        debt: 35e9,
        expenditure: 52e9, // Expenditure exceeds revenue
        capital_expenditure: 7e9,
        audit_outcome: 'Adverse' as const,
    },
    {
        id: 'MAN01', // Example: Mangaung
        name: 'Mangaung Metro (Mock)',
        province: 'FS',
        population: 787803,
        year: 2023,
        revenue: 10e9,
        debt: 8e9,
        expenditure: 9.5e9,
        capital_expenditure: 0.8e9, // Low capex
        audit_outcome: 'Disclaimer' as const,
    },
    {
        id: 'TSH01', // Example: Tshwane
        name: 'Tshwane Metro (Mock)',
        province: 'GP',
        population: 3390000, // Approx
        year: 2023,
        revenue: 45e9,
        debt: 40e9, // High debt
        expenditure: 46e9, 
        capital_expenditure: 5e9,
        audit_outcome: 'Unqualified' as const,
    }
];

// Process raw data to calculate scores and structure for export
export const dummyMunicipalityDetails: Record<string, MunicipalityDetails> =
    dummyDataRaw.reduce((acc, rawMuni) => {
        const { score, breakdown } = calculateDummyScore(rawMuni);
        acc[rawMuni.id] = {
            ...rawMuni,
            score,
            score_breakdown: breakdown,
        };
        return acc;
    }, {} as Record<string, MunicipalityDetails>);

// Prepare GeoJSON with updated scores
export const dummyMunicipalitiesGeoJSON: FeatureCollection = {
    type: 'FeatureCollection',
    features: dummyDataRaw.map(muni => ({
        type: 'Feature',
        geometry: {
            type: 'Polygon', // Still using placeholder squares
            coordinates: getDummyCoordinates(muni.id) // Helper to get coordinates
        },
        properties: {
             id: muni.id,
             name: muni.name,
             score: dummyMunicipalityDetails[muni.id]?.score ?? 0 // Get calculated score
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