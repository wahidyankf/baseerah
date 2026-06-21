import type { Locale } from "./config";

const translations: Record<Locale, Record<string, string>> = {
  en: {
    readMore: "Read More",
    lastUpdated: "Last updated",
    publishedOn: "Published on",
    author: "Author",
    tags: "Tags",
    categories: "Categories",
    share: "Share",
    relatedContent: "Related Content",
    openSourceProject: "Source-Available Project",
    search: "Search...",
    onThisPage: "On this page",
    previous: "Previous",
    next: "Next",
    noResults: "No results found",
    toggleTheme: "Toggle theme",
    skipToContent: "Skip to content",
    toolsPageTitle: "Tools",
    toolsPageCalcLink: "Cost of Living Calculator",
    toolsPageCalcDesc: "Compare monthly living costs, savings, and the minimum role needed across cities.",
    breadcrumbHome: "Home",
    breadcrumbCalculator: "Calculator",

    // Calculator — page
    calcTitle: "Cost of Living Calculator",
    calcSubtitle: "Compare cost of living and salary savings across cities",
    ariaTabsNav: "Calculator tabs",
    tabCostOfLiving: "Cost of living",
    tabCostDesc: "Compare monthly living costs across cities",
    tabSavings: "Savings",
    tabSavingsDesc: "See how much you'd save",
    tabMinRole: "Minimum role",
    tabMinRoleDesc: "Find the min role you need",
    dataLastUpdated: "Data last updated",
    estimatesOnly: "Estimates only",

    // Calculator — disclaimers
    disclaimerPension: "Savings are before voluntary pension / retirement contributions.",
    disclaimerClothing: "Clothing and personal care are folded into lifestyle expenses.",
    disclaimerFx:
      "A positive USD savings figure does not mean equal purchasing power — USD uses a nominal FX snapshot, not PPP.",
    disclaimerSnapshot: "Data is a snapshot — verify current figures before making relocation decisions.",
    disclaimerTax:
      "Tax is a simplified effective rate (federal + sub-national for US/CA/CH only) — not a full bracket calculation; excludes filing status, deductions, benefits-in-kind, and contribution caps.",
    disclaimerHealthcare: "Healthcare models out-of-pocket costs only; the funding scheme is shown per country.",
    disclaimerRelocation:
      "Relocation sunk costs are a one-time estimate kept out of monthly savings math; the cash cushion is a reserve you keep, not a sunk cost.",
    disclaimerRoleSalary:
      "Role salary is modeled at the national (country) level — cities inherit their country's p25/median/p75 distribution.",
    disclaimerNonSalary:
      "Non-salary comp (RSU/equity + bonus) is informational total-comp context only, not part of the savings math.",

    // Calculator — geo filters
    labelRegion: "Region",
    labelCountry: "Country",
    labelCity: "City",
    optAllRegions: "All regions",
    optAllCountries: "All countries",
    optAllCities: "All cities",
    clearRegion: "Clear",
    regionAutoAdvisory: "Region updated automatically to match the selected country.",

    // Calculator — controls
    labelAdults: "Adults",
    labelPreschoolKids: "Preschool children",
    labelSchoolKids: "School-age children",
    labelSchoolType: "School type",
    optPublic: "Public",
    optPrivate: "Private",
    labelArea: "Area",
    optCenter: "City center",
    optRural: "Rural",

    // Calculator — cost-of-living table
    colCountry: "Country",
    colCity: "City",
    colHealthcareScheme: "Healthcare scheme",
    tooltipHealthcareScheme:
      "How healthcare costs are funded in this country: tax-funded, mandatory payroll insurance, or out-of-pocket.",
    colHousing: "Housing",
    colFood: "Food",
    colTransport: "Transport",
    colUtilities: "Utilities",
    colHealthcareOOP: "Healthcare (OOP)",
    colHealthcareOOPPrefix: "Healthcare",
    colChildcare: "Childcare",
    colSchool: "School",
    colEssentials: "Essentials",
    colTotal: "Total",
    previewMonthlyEstimate: "estimated monthly essentials",
    colRelocationSunk: "Relocation (sunk)",
    colLiquidityReserve: "Liquidity reserve",
    tooltipRelocationSunk:
      "One-time sunk costs: rental deposit, key money, moving, and visa fees. Not a monthly expense.",
    tooltipLiquidityReserve:
      "Cash cushion you keep on hand — not a sunk cost. Covers first months before salary starts.",
    oopLegend: "OOP = out-of-pocket — healthcare you pay yourself, on top of any tax-funded or insurance coverage.",

    // Calculator — healthcare scheme badges
    healthcareTaxFunded: "tax-funded",
    healthcareMandatoryPayroll: "mandatory payroll insurance",
    healthcareOutOfPocket: "out-of-pocket",

    // Calculator — city detail
    sectionMonthlyExpenses: "Monthly expenses",
    sectionRelocationCosts: "Relocation costs",
    labelHousing: "Housing",
    labelFood: "Food",
    labelTransport: "Transport",
    labelUtilities: "Utilities",
    labelHealthcareOOP: "Healthcare (OOP)",
    labelChildcare: "Childcare",
    labelSchool: "School",
    labelEssentialsSubtotal: "Essentials subtotal",
    labelMonthlyTotal: "Monthly total",
    labelRelocationSunkCost: "One-time relocation sunk cost",
    labelLiquidityReserve: "Liquidity reserve (cash cushion — kept, not spent)",
    backToAllCities: "← Back to all cities",

    // Calculator — savings table
    savingsEmptyStateMessage: "Enter your gross monthly salary to see how much you could save in each city.",
    grossMonthlySalaryLabel: "Gross monthly salary (before tax)",
    salaryCurrencyIndicator: "Currency: USD",
    salaryCurrencyExplanation: "Salaries are compared in USD across all cities.",
    annualGrossLabel: "Annual gross",
    nonSalaryCompNote: "Non-salary comp (RSU/equity + bonus) is informational only — not in savings math.",
    colNet: "Net (monthly)",
    colSavingsEssential: "Savings after essentials ↕",
    colSavingsLifestyle: "Savings after lifestyle",
    colNonSalaryComp: "Typical non-salary comp (info, annual)",
    colTotalComp: "Total comp (info, annual)",
    sortBySavings: "Sort by savings",
    subNationalIndicator: "(fed+state)",

    // Calculator — min-role table
    labelBaselineSource: "Baseline source",
    optSavingsTarget: "Monthly savings target",
    optReferenceRole: "Reference role",
    optMySalary: "My salary",
    labelMonthlySavingsTarget: "Monthly savings target",
    labelTargetCurrency: "Target currency",
    labelRefCity: "Reference city",
    labelRefRole: "Reference role",
    labelMyGrossMonthly: "My gross monthly (USD)",
    labelMySalaryCity: "My salary city",
    labelDisplayCurrency: "Display currency",
    rankBasisNote:
      "Ranking key: essential savings (housing + food + transport + utilities + healthcare + school). Lifestyle excluded — personal preference variable.",
    nonSalaryRankNote: "Non-salary comp (RSU / equity / bonus) is informational only — not used in ranking.",
    noQualifierMessage: "No role reaches this savings bar in any city.",
    minRoleEmptyStateMessage: "Enter a monthly savings target to see which roles reach it in each city.",
    seRolesCaption: "Roles: software-engineering (IC + management)",
    qualifyingDivider: "— roles below do not reach the savings bar —",
    colRole: "Role",
    colTrack: "Track",
    colBestCity: "Best city",
    colP25: "P25 (monthly)",
    colMedian: "Median",
    colP75: "P75",
    colEssentialSavings: "Essential savings",
    colNonSalaryCompInfo: "Non-salary comp (info, annual, RSU/equity)",
    minimumMarker: "← min",
  },
  id: {
    readMore: "Baca Selengkapnya",
    lastUpdated: "Terakhir diperbarui",
    publishedOn: "Dipublikasikan pada",
    author: "Penulis",
    tags: "Tag",
    categories: "Kategori",
    share: "Bagikan",
    relatedContent: "Konten Terkait",
    openSourceProject: "Proyek Source-Available",
    search: "Cari...",
    onThisPage: "Di halaman ini",
    previous: "Sebelumnya",
    next: "Selanjutnya",
    noResults: "Tidak ada hasil",
    toggleTheme: "Ubah tema",
    skipToContent: "Langsung ke konten",
    toolsPageTitle: "Alat",
    toolsPageCalcLink: "Kalkulator Biaya Hidup",
    toolsPageCalcDesc:
      "Bandingkan biaya hidup bulanan, tabungan, dan jabatan minimum yang dibutuhkan di berbagai kota.",
    breadcrumbHome: "Beranda",
    breadcrumbCalculator: "Kalkulator",

    // Calculator — page
    calcTitle: "Kalkulator Biaya Hidup",
    calcSubtitle: "Bandingkan biaya hidup dan tabungan gaji di berbagai kota",
    ariaTabsNav: "Tab kalkulator",
    tabCostOfLiving: "Biaya hidup",
    tabCostDesc: "Bandingkan biaya hidup bulanan di berbagai kota",
    tabSavings: "Tabungan",
    tabSavingsDesc: "Lihat seberapa banyak yang bisa Anda hemat",
    tabMinRole: "Jabatan minimum",
    tabMinRoleDesc: "Temukan jabatan minimum yang Anda butuhkan",
    dataLastUpdated: "Data terakhir diperbarui",
    estimatesOnly: "Hanya perkiraan",

    // Calculator — disclaimers
    disclaimerPension: "Tabungan sebelum kontribusi pensiun / dana hari tua sukarela.",
    disclaimerClothing: "Pakaian dan perawatan pribadi termasuk dalam pengeluaran gaya hidup.",
    disclaimerFx:
      "Angka tabungan USD positif tidak berarti daya beli yang sama — USD menggunakan snapshot FX nominal, bukan PPP.",
    disclaimerSnapshot: "Data adalah snapshot — verifikasi angka terkini sebelum membuat keputusan relokasi.",
    disclaimerTax:
      "Pajak menggunakan tarif efektif yang disederhanakan (federal + sub-nasional untuk AS/CA/CH saja) — bukan perhitungan bracket penuh; tidak termasuk status pengisian, potongan, tunjangan natura, dan batas kontribusi.",
    disclaimerHealthcare: "Kesehatan memodelkan biaya out-of-pocket saja; skema pendanaan ditampilkan per negara.",
    disclaimerRelocation:
      "Biaya sunk relokasi adalah perkiraan sekali dan tidak termasuk dalam perhitungan tabungan bulanan; cadangan tunai adalah dana yang Anda simpan, bukan biaya.",
    disclaimerRoleSalary:
      "Gaji jabatan dimodelkan di tingkat nasional (negara) — kota mewarisi distribusi p25/median/p75 negaranya.",
    disclaimerNonSalary:
      "Kompensasi non-gaji (RSU/ekuitas + bonus) hanya sebagai konteks informasi total kompensasi, bukan bagian dari perhitungan tabungan.",

    // Calculator — geo filters
    labelRegion: "Wilayah",
    labelCountry: "Negara",
    labelCity: "Kota",
    optAllRegions: "Semua wilayah",
    optAllCountries: "Semua negara",
    optAllCities: "Semua kota",
    clearRegion: "Hapus",
    regionAutoAdvisory: "Wilayah diperbarui otomatis agar sesuai dengan negara yang dipilih.",

    // Calculator — controls
    labelAdults: "Dewasa",
    labelPreschoolKids: "Anak prasekolah",
    labelSchoolKids: "Anak usia sekolah",
    labelSchoolType: "Jenis sekolah",
    optPublic: "Negeri",
    optPrivate: "Swasta",
    labelArea: "Wilayah",
    optCenter: "Pusat kota",
    optRural: "Pedesaan",

    // Calculator — cost-of-living table
    colCountry: "Negara",
    colCity: "Kota",
    colHealthcareScheme: "Skema kesehatan",
    tooltipHealthcareScheme:
      "Bagaimana biaya kesehatan didanai di negara ini: didanai pajak, asuransi penggajian wajib, atau bayar sendiri.",
    colHousing: "Perumahan",
    colFood: "Makanan",
    colTransport: "Transportasi",
    colUtilities: "Utilitas",
    colHealthcareOOP: "Kesehatan (OOP)",
    colHealthcareOOPPrefix: "Kesehatan",
    colChildcare: "Penitipan anak",
    colSchool: "Sekolah",
    colEssentials: "Kebutuhan pokok",
    colTotal: "Total",
    previewMonthlyEstimate: "perkiraan kebutuhan pokok bulanan",
    colRelocationSunk: "Relokasi (biaya hangus)",
    colLiquidityReserve: "Cadangan likuiditas",
    tooltipRelocationSunk:
      "Biaya hangus sekali: deposit sewa, uang kunci, pindahan, dan biaya visa. Bukan pengeluaran bulanan.",
    tooltipLiquidityReserve:
      "Dana cadangan yang Anda simpan — bukan biaya hangus. Menutup bulan-bulan awal sebelum gaji mulai.",
    oopLegend:
      "OOP = out-of-pocket — biaya kesehatan yang Anda bayar sendiri, di luar jaminan dari pajak atau asuransi.",

    // Calculator — healthcare scheme badges
    healthcareTaxFunded: "didanai pajak",
    healthcareMandatoryPayroll: "asuransi penggajian wajib",
    healthcareOutOfPocket: "bayar sendiri",

    // Calculator — city detail
    sectionMonthlyExpenses: "Pengeluaran bulanan",
    sectionRelocationCosts: "Biaya relokasi",
    labelHousing: "Perumahan",
    labelFood: "Makanan",
    labelTransport: "Transportasi",
    labelUtilities: "Utilitas",
    labelHealthcareOOP: "Kesehatan (OOP)",
    labelChildcare: "Penitipan anak",
    labelSchool: "Sekolah",
    labelEssentialsSubtotal: "Subtotal kebutuhan pokok",
    labelMonthlyTotal: "Total bulanan",
    labelRelocationSunkCost: "Biaya sunk relokasi sekali",
    labelLiquidityReserve: "Cadangan likuiditas (dana cadangan — disimpan, tidak dibelanjakan)",
    backToAllCities: "← Kembali ke semua kota",

    // Calculator — savings table
    savingsEmptyStateMessage:
      "Masukkan gaji kotor bulanan Anda untuk melihat berapa banyak yang bisa Anda hemat di setiap kota.",
    grossMonthlySalaryLabel: "Gaji kotor bulanan (sebelum pajak)",
    salaryCurrencyIndicator: "Mata uang: USD",
    salaryCurrencyExplanation: "Gaji dibandingkan dalam USD di semua kota.",
    annualGrossLabel: "Total gaji tahunan",
    nonSalaryCompNote:
      "Kompensasi non-gaji (RSU/ekuitas + bonus) hanya informasi — tidak termasuk dalam perhitungan tabungan.",
    colNet: "Bersih (bulanan)",
    colSavingsEssential: "Tabungan setelah kebutuhan pokok ↕",
    colSavingsLifestyle: "Tabungan setelah gaya hidup",
    colNonSalaryComp: "Kompensasi non-gaji tipikal (info, tahunan)",
    colTotalComp: "Total kompensasi (info, tahunan)",
    sortBySavings: "Urutkan berdasarkan tabungan",
    subNationalIndicator: "(federal+negara bagian)",

    // Calculator — min-role table
    labelBaselineSource: "Sumber baseline",
    optSavingsTarget: "Target tabungan bulanan",
    optReferenceRole: "Jabatan referensi",
    optMySalary: "Gaji saya",
    labelMonthlySavingsTarget: "Target tabungan bulanan",
    labelTargetCurrency: "Mata uang target",
    labelRefCity: "Kota referensi",
    labelRefRole: "Jabatan referensi",
    labelMyGrossMonthly: "Gaji kotor bulanan saya (USD)",
    labelMySalaryCity: "Kota gaji saya",
    labelDisplayCurrency: "Mata uang tampilan",
    rankBasisNote:
      "Kunci peringkat: tabungan kebutuhan pokok (perumahan + makanan + transportasi + utilitas + kesehatan + sekolah). Gaya hidup dikecualikan — variabel preferensi pribadi.",
    nonSalaryRankNote: "Kompensasi non-gaji (RSU / ekuitas / bonus) hanya informasi — tidak digunakan dalam peringkat.",
    noQualifierMessage: "Tidak ada jabatan yang mencapai target tabungan ini di kota manapun.",
    minRoleEmptyStateMessage:
      "Masukkan target tabungan bulanan untuk melihat jabatan mana yang mencapainya di setiap kota.",
    seRolesCaption: "Jabatan: rekayasa perangkat lunak (IC + manajemen)",
    qualifyingDivider: "— jabatan di bawah tidak mencapai target tabungan —",
    colRole: "Jabatan",
    colTrack: "Jalur",
    colBestCity: "Kota terbaik",
    colP25: "P25 (bulanan)",
    colMedian: "Median",
    colP75: "P75",
    colEssentialSavings: "Tabungan kebutuhan pokok",
    colNonSalaryCompInfo: "Kompensasi non-gaji (info, tahunan, RSU/ekuitas)",
    minimumMarker: "← min",
  },
};

export function t(locale: Locale, key: string): string {
  return translations[locale]?.[key] ?? key;
}
