// IsSettled iso20022

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_xml_rs;

use serde_xml_rs::from_reader;


// document 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct document {
	#[serde(rename = "Document")]
	pub document: Document,
}


// AccountIdentification4Choice 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct AccountIdentification4Choice {
	#[serde(rename = "IBAN")]
	pub iban: String,
	#[serde(rename = "Othr")]
	pub othr: GenericAccountIdentification1,
}


// AccountSchemeName1Choice 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct AccountSchemeName1Choice {
	#[serde(rename = "Cd")]
	pub cd: String,
	#[serde(rename = "Prtry")]
	pub prtry: String,
}


// ActiveOrHistoricCurrencyAndAmountSimpleType 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ActiveOrHistoricCurrencyAndAmountSimpleType {
	#[serde(rename = "ActiveOrHistoricCurrencyAndAmount_SimpleType")]
	pub active_or_historic_currency_and_amount_simple_type: f64,
}


// ActiveOrHistoricCurrencyAndAmount 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ActiveOrHistoricCurrencyAndAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}


// ActiveOrHistoricCurrencyCode 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ActiveOrHistoricCurrencyCode {
	#[serde(rename = "ActiveOrHistoricCurrencyCode")]
	pub active_or_historic_currency_code: String,
}


// AddressType2Code 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct AddressType2Code {
	#[serde(rename = "AddressType2Code")]
	pub address_type2_code: String,
}


// AddressType3Choice 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct AddressType3Choice {
	#[serde(rename = "Cd")]
	pub cd: String,
	#[serde(rename = "Prtry")]
	pub prtry: GenericIdentification30,
}


// AmendmentInformationDetails14 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct AmendmentInformationDetails14 {
	#[serde(rename = "OrgnlMndtId")]
	pub orgnl_mndt_id: String,
	#[serde(rename = "OrgnlCdtrSchmeId")]
	pub orgnl_cdtr_schme_id: PartyIdentification135,
	#[serde(rename = "OrgnlCdtrAgt")]
	pub orgnl_cdtr_agt: BranchAndFinancialInstitutionIdentification6,
	#[serde(rename = "OrgnlCdtrAgtAcct")]
	pub orgnl_cdtr_agt_acct: CashAccount40,
	#[serde(rename = "OrgnlDbtr")]
	pub orgnl_dbtr: PartyIdentification135,
	#[serde(rename = "OrgnlDbtrAcct")]
	pub orgnl_dbtr_acct: CashAccount40,
	#[serde(rename = "OrgnlDbtrAgt")]
	pub orgnl_dbtr_agt: BranchAndFinancialInstitutionIdentification6,
	#[serde(rename = "OrgnlDbtrAgtAcct")]
	pub orgnl_dbtr_agt_acct: CashAccount40,
	#[serde(rename = "OrgnlFnlColltnDt")]
	pub orgnl_fnl_colltn_dt: u8,
	#[serde(rename = "OrgnlFrqcy")]
	pub orgnl_frqcy: Frequency36Choice,
	#[serde(rename = "OrgnlRsn")]
	pub orgnl_rsn: MandateSetupReason1Choice,
	#[serde(rename = "OrgnlTrckgDays")]
	pub orgnl_trckg_days: String,
}


// AmountType4Choice 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct AmountType4Choice {
	#[serde(rename = "InstdAmt")]
	pub instd_amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "EqvtAmt")]
	pub eqvt_amt: EquivalentAmount2,
}


// AnyBICDec2014Identifier 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct AnyBICDec2014Identifier {
	#[serde(rename = "AnyBICDec2014Identifier")]
	pub any_bic_dec2014_identifier: String,
}


// BICFIDec2014Identifier 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct BICFIDec2014Identifier {
	#[serde(rename = "BICFIDec2014Identifier")]
	pub bicfi_dec2014_identifier: String,
}


// BranchAndFinancialInstitutionIdentification6 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct BranchAndFinancialInstitutionIdentification6 {
	#[serde(rename = "FinInstnId")]
	pub fin_instn_id: FinancialInstitutionIdentification18,
	#[serde(rename = "BrnchId")]
	pub brnch_id: BranchData3,
}


// BranchData3 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct BranchData3 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "LEI")]
	pub lei: String,
	#[serde(rename = "Nm")]
	pub nm: String,
	#[serde(rename = "PstlAdr")]
	pub pstl_adr: PostalAddress24,
}


// Case5 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Case5 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Cretr")]
	pub cretr: Party40Choice,
	#[serde(rename = "ReopCaseIndctn")]
	pub reop_case_indctn: bool,
}


// CaseAssignment5 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CaseAssignment5 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Assgnr")]
	pub assgnr: Party40Choice,
	#[serde(rename = "Assgne")]
	pub assgne: Party40Choice,
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: u8,
}


// CashAccount40 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CashAccount40 {
	#[serde(rename = "Id")]
	pub id: AccountIdentification4Choice,
	#[serde(rename = "Tp")]
	pub tp: CashAccountType2Choice,
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "Nm")]
	pub nm: String,
	#[serde(rename = "Prxy")]
	pub prxy: ProxyAccountIdentification1,
}


// CashAccountType2Choice 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CashAccountType2Choice {
	#[serde(rename = "Cd")]
	pub cd: String,
	#[serde(rename = "Prtry")]
	pub prtry: String,
}


// CategoryPurpose1Choice 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CategoryPurpose1Choice {
	#[serde(rename = "Cd")]
	pub cd: String,
	#[serde(rename = "Prtry")]
	pub prtry: String,
}


// ChargeBearerType1Code 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ChargeBearerType1Code {
	#[serde(rename = "ChargeBearerType1Code")]
	pub charge_bearer_type1_code: String,
}


// ClearingChannel2Code 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ClearingChannel2Code {
	#[serde(rename = "ClearingChannel2Code")]
	pub clearing_channel2_code: String,
}


// ClearingSystemIdentification2Choice 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ClearingSystemIdentification2Choice {
	#[serde(rename = "Cd")]
	pub cd: String,
	#[serde(rename = "Prtry")]
	pub prtry: String,
}


// ClearingSystemIdentification3Choice 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ClearingSystemIdentification3Choice {
	#[serde(rename = "Cd")]
	pub cd: String,
	#[serde(rename = "Prtry")]
	pub prtry: String,
}


// ClearingSystemMemberIdentification2 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ClearingSystemMemberIdentification2 {
	#[serde(rename = "ClrSysId")]
	pub clr_sys_id: ClearingSystemIdentification2Choice,
	#[serde(rename = "MmbId")]
	pub mmb_id: String,
}


// Contact4 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Contact4 {
	#[serde(rename = "NmPrfx")]
	pub nm_prfx: String,
	#[serde(rename = "Nm")]
	pub nm: String,
	#[serde(rename = "PhneNb")]
	pub phne_nb: String,
	#[serde(rename = "MobNb")]
	pub mob_nb: String,
	#[serde(rename = "FaxNb")]
	pub fax_nb: String,
	#[serde(rename = "EmailAdr")]
	pub email_adr: String,
	#[serde(rename = "EmailPurp")]
	pub email_purp: String,
	#[serde(rename = "JobTitl")]
	pub job_titl: String,
	#[serde(rename = "Rspnsblty")]
	pub rspnsblty: String,
	#[serde(rename = "Dept")]
	pub dept: String,
	#[serde(rename = "Othr")]
	pub othr: Vec<OtherContact1>,
	#[serde(rename = "PrefrdMtd")]
	pub prefrd_mtd: String,
}


// CountryCode 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CountryCode {
	#[serde(rename = "CountryCode")]
	pub country_code: String,
}


// CreditDebitCode 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CreditDebitCode {
	#[serde(rename = "CreditDebitCode")]
	pub credit_debit_code: String,
}


// CreditTransferMandateData1 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CreditTransferMandateData1 {
	#[serde(rename = "MndtId")]
	pub mndt_id: String,
	#[serde(rename = "Tp")]
	pub tp: MandateTypeInformation2,
	#[serde(rename = "DtOfSgntr")]
	pub dt_of_sgntr: u8,
	#[serde(rename = "DtOfVrfctn")]
	pub dt_of_vrfctn: u8,
	#[serde(rename = "ElctrncSgntr")]
	pub elctrnc_sgntr: String,
	#[serde(rename = "FrstPmtDt")]
	pub frst_pmt_dt: u8,
	#[serde(rename = "FnlPmtDt")]
	pub fnl_pmt_dt: u8,
	#[serde(rename = "Frqcy")]
	pub frqcy: Frequency36Choice,
	#[serde(rename = "Rsn")]
	pub rsn: MandateSetupReason1Choice,
}


// CreditorReferenceInformation2 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CreditorReferenceInformation2 {
	#[serde(rename = "Tp")]
	pub tp: CreditorReferenceType2,
	#[serde(rename = "Ref")]
	pub ref_attr: String,
}


// CreditorReferenceType1Choice 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CreditorReferenceType1Choice {
	#[serde(rename = "Cd")]
	pub cd: String,
	#[serde(rename = "Prtry")]
	pub prtry: String,
}


// CreditorReferenceType2 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CreditorReferenceType2 {
	#[serde(rename = "CdOrPrtry")]
	pub cd_or_prtry: CreditorReferenceType1Choice,
	#[serde(rename = "Issr")]
	pub issr: String,
}


// DateAndDateTime2Choice 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DateAndDateTime2Choice {
	#[serde(rename = "Dt")]
	pub dt: u8,
	#[serde(rename = "DtTm")]
	pub dt_tm: u8,
}


// DateAndPlaceOfBirth1 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DateAndPlaceOfBirth1 {
	#[serde(rename = "BirthDt")]
	pub birth_dt: u8,
	#[serde(rename = "PrvcOfBirth")]
	pub prvc_of_birth: String,
	#[serde(rename = "CityOfBirth")]
	pub city_of_birth: String,
	#[serde(rename = "CtryOfBirth")]
	pub ctry_of_birth: String,
}


// DatePeriod2 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DatePeriod2 {
	#[serde(rename = "FrDt")]
	pub fr_dt: u8,
	#[serde(rename = "ToDt")]
	pub to_dt: u8,
}


// DecimalNumber 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DecimalNumber {
	#[serde(rename = "DecimalNumber")]
	pub decimal_number: f64,
}


// DiscountAmountAndType1 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DiscountAmountAndType1 {
	#[serde(rename = "Tp")]
	pub tp: DiscountAmountType1Choice,
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
}


// DiscountAmountType1Choice 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DiscountAmountType1Choice {
	#[serde(rename = "Cd")]
	pub cd: String,
	#[serde(rename = "Prtry")]
	pub prtry: String,
}


// DocumentAdjustment1 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DocumentAdjustment1 {
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "CdtDbtInd")]
	pub cdt_dbt_ind: String,
	#[serde(rename = "Rsn")]
	pub rsn: String,
	#[serde(rename = "AddtlInf")]
	pub addtl_inf: String,
}


// DocumentLineIdentification1 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DocumentLineIdentification1 {
	#[serde(rename = "Tp")]
	pub tp: DocumentLineType1,
	#[serde(rename = "Nb")]
	pub nb: String,
	#[serde(rename = "RltdDt")]
	pub rltd_dt: u8,
}


// DocumentLineInformation1 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DocumentLineInformation1 {
	#[serde(rename = "Id")]
	pub id: Vec<DocumentLineIdentification1>,
	#[serde(rename = "Desc")]
	pub desc: String,
	#[serde(rename = "Amt")]
	pub amt: RemittanceAmount3,
}


// DocumentLineType1 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DocumentLineType1 {
	#[serde(rename = "CdOrPrtry")]
	pub cd_or_prtry: DocumentLineType1Choice,
	#[serde(rename = "Issr")]
	pub issr: String,
}


// DocumentLineType1Choice 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DocumentLineType1Choice {
	#[serde(rename = "Cd")]
	pub cd: String,
	#[serde(rename = "Prtry")]
	pub prtry: String,
}


// DocumentType3Code 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DocumentType3Code {
	#[serde(rename = "DocumentType3Code")]
	pub document_type3_code: String,
}


// DocumentType6Code 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DocumentType6Code {
	#[serde(rename = "DocumentType6Code")]
	pub document_type6_code: String,
}


// EquivalentAmount2 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct EquivalentAmount2 {
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "CcyOfTrf")]
	pub ccy_of_trf: String,
}


// Exact2NumericText 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Exact2NumericText {
	#[serde(rename = "Exact2NumericText")]
	pub exact2_numeric_text: String,
}


// Exact4AlphaNumericText 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Exact4AlphaNumericText {
	#[serde(rename = "Exact4AlphaNumericText")]
	pub exact4_alpha_numeric_text: String,
}


// ExternalAccountIdentification1Code 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ExternalAccountIdentification1Code {
	#[serde(rename = "ExternalAccountIdentification1Code")]
	pub external_account_identification1_code: String,
}


// ExternalAgentInstruction1Code 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ExternalAgentInstruction1Code {
	#[serde(rename = "ExternalAgentInstruction1Code")]
	pub external_agent_instruction1_code: String,
}


// ExternalCashAccountType1Code 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ExternalCashAccountType1Code {
	#[serde(rename = "ExternalCashAccountType1Code")]
	pub external_cash_account_type1_code: String,
}


// ExternalCashClearingSystem1Code 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ExternalCashClearingSystem1Code {
	#[serde(rename = "ExternalCashClearingSystem1Code")]
	pub external_cash_clearing_system1_code: String,
}


// ExternalCategoryPurpose1Code 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ExternalCategoryPurpose1Code {
	#[serde(rename = "ExternalCategoryPurpose1Code")]
	pub external_category_purpose1_code: String,
}


// ExternalClearingSystemIdentification1Code 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ExternalClearingSystemIdentification1Code {
	#[serde(rename = "ExternalClearingSystemIdentification1Code")]
	pub external_clearing_system_identification1_code: String,
}


// ExternalCreditorAgentInstruction1Code 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ExternalCreditorAgentInstruction1Code {
	#[serde(rename = "ExternalCreditorAgentInstruction1Code")]
	pub external_creditor_agent_instruction1_code: String,
}


// ExternalDiscountAmountType1Code 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ExternalDiscountAmountType1Code {
	#[serde(rename = "ExternalDiscountAmountType1Code")]
	pub external_discount_amount_type1_code: String,
}


// ExternalDocumentLineType1Code 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ExternalDocumentLineType1Code {
	#[serde(rename = "ExternalDocumentLineType1Code")]
	pub external_document_line_type1_code: String,
}


// ExternalFinancialInstitutionIdentification1Code 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ExternalFinancialInstitutionIdentification1Code {
	#[serde(rename = "ExternalFinancialInstitutionIdentification1Code")]
	pub external_financial_institution_identification1_code: String,
}


// ExternalGarnishmentType1Code 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ExternalGarnishmentType1Code {
	#[serde(rename = "ExternalGarnishmentType1Code")]
	pub external_garnishment_type1_code: String,
}


// ExternalLocalInstrument1Code 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ExternalLocalInstrument1Code {
	#[serde(rename = "ExternalLocalInstrument1Code")]
	pub external_local_instrument1_code: String,
}


// ExternalMandateSetupReason1Code 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ExternalMandateSetupReason1Code {
	#[serde(rename = "ExternalMandateSetupReason1Code")]
	pub external_mandate_setup_reason1_code: String,
}


// ExternalOrganisationIdentification1Code 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ExternalOrganisationIdentification1Code {
	#[serde(rename = "ExternalOrganisationIdentification1Code")]
	pub external_organisation_identification1_code: String,
}


// ExternalPersonIdentification1Code 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ExternalPersonIdentification1Code {
	#[serde(rename = "ExternalPersonIdentification1Code")]
	pub external_person_identification1_code: String,
}


// ExternalProxyAccountType1Code 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ExternalProxyAccountType1Code {
	#[serde(rename = "ExternalProxyAccountType1Code")]
	pub external_proxy_account_type1_code: String,
}


// ExternalPurpose1Code 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ExternalPurpose1Code {
	#[serde(rename = "ExternalPurpose1Code")]
	pub external_purpose1_code: String,
}


// ExternalServiceLevel1Code 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ExternalServiceLevel1Code {
	#[serde(rename = "ExternalServiceLevel1Code")]
	pub external_service_level1_code: String,
}


// ExternalTaxAmountType1Code 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ExternalTaxAmountType1Code {
	#[serde(rename = "ExternalTaxAmountType1Code")]
	pub external_tax_amount_type1_code: String,
}


// FinancialIdentificationSchemeName1Choice 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct FinancialIdentificationSchemeName1Choice {
	#[serde(rename = "Cd")]
	pub cd: String,
	#[serde(rename = "Prtry")]
	pub prtry: String,
}


// FinancialInstitutionIdentification18 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct FinancialInstitutionIdentification18 {
	#[serde(rename = "BICFI")]
	pub bicfi: String,
	#[serde(rename = "ClrSysMmbId")]
	pub clr_sys_mmb_id: ClearingSystemMemberIdentification2,
	#[serde(rename = "LEI")]
	pub lei: String,
	#[serde(rename = "Nm")]
	pub nm: String,
	#[serde(rename = "PstlAdr")]
	pub pstl_adr: PostalAddress24,
	#[serde(rename = "Othr")]
	pub othr: GenericFinancialIdentification1,
}


// Frequency36Choice 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Frequency36Choice {
	#[serde(rename = "Tp")]
	pub tp: String,
	#[serde(rename = "Prd")]
	pub prd: FrequencyPeriod1,
	#[serde(rename = "PtInTm")]
	pub pt_in_tm: FrequencyAndMoment1,
}


// Frequency6Code 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Frequency6Code {
	#[serde(rename = "Frequency6Code")]
	pub frequency6_code: String,
}


// FrequencyAndMoment1 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct FrequencyAndMoment1 {
	#[serde(rename = "Tp")]
	pub tp: String,
	#[serde(rename = "PtInTm")]
	pub pt_in_tm: String,
}


// FrequencyPeriod1 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct FrequencyPeriod1 {
	#[serde(rename = "Tp")]
	pub tp: String,
	#[serde(rename = "CntPerPrd")]
	pub cnt_per_prd: f64,
}


// Garnishment3 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Garnishment3 {
	#[serde(rename = "Tp")]
	pub tp: GarnishmentType1,
	#[serde(rename = "Grnshee")]
	pub grnshee: PartyIdentification135,
	#[serde(rename = "GrnshmtAdmstr")]
	pub grnshmt_admstr: PartyIdentification135,
	#[serde(rename = "RefNb")]
	pub ref_nb: String,
	#[serde(rename = "Dt")]
	pub dt: u8,
	#[serde(rename = "RmtdAmt")]
	pub rmtd_amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "FmlyMdclInsrncInd")]
	pub fmly_mdcl_insrnc_ind: bool,
	#[serde(rename = "MplyeeTermntnInd")]
	pub mplyee_termntn_ind: bool,
}


// GarnishmentType1 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct GarnishmentType1 {
	#[serde(rename = "CdOrPrtry")]
	pub cd_or_prtry: GarnishmentType1Choice,
	#[serde(rename = "Issr")]
	pub issr: String,
}


// GarnishmentType1Choice 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct GarnishmentType1Choice {
	#[serde(rename = "Cd")]
	pub cd: String,
	#[serde(rename = "Prtry")]
	pub prtry: String,
}


// GenericAccountIdentification1 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct GenericAccountIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: AccountSchemeName1Choice,
	#[serde(rename = "Issr")]
	pub issr: String,
}


// GenericFinancialIdentification1 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct GenericFinancialIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: FinancialIdentificationSchemeName1Choice,
	#[serde(rename = "Issr")]
	pub issr: String,
}


// GenericIdentification30 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct GenericIdentification30 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Issr")]
	pub issr: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: String,
}


// GenericOrganisationIdentification1 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct GenericOrganisationIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: OrganisationIdentificationSchemeName1Choice,
	#[serde(rename = "Issr")]
	pub issr: String,
}


// GenericPersonIdentification1 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct GenericPersonIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: PersonIdentificationSchemeName1Choice,
	#[serde(rename = "Issr")]
	pub issr: String,
}


// IBAN2007Identifier 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct IBAN2007Identifier {
	#[serde(rename = "IBAN2007Identifier")]
	pub iban2007_identifier: String,
}


// ISODate 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ISODate {
	#[serde(rename = "ISODate")]
	pub iso_date: u8,
}


// ISODateTime 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ISODateTime {
	#[serde(rename = "ISODateTime")]
	pub iso_date_time: u8,
}


// ISOYear 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ISOYear {
	#[serde(rename = "ISOYear")]
	pub iso_year: String,
}


// Instruction4Code 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Instruction4Code {
	#[serde(rename = "Instruction4Code")]
	pub instruction4_code: String,
}


// InstructionForAssignee1 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct InstructionForAssignee1 {
	#[serde(rename = "Cd")]
	pub cd: String,
	#[serde(rename = "InstrInf")]
	pub instr_inf: String,
}


// InstructionForCreditorAgent3 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct InstructionForCreditorAgent3 {
	#[serde(rename = "Cd")]
	pub cd: String,
	#[serde(rename = "InstrInf")]
	pub instr_inf: String,
}


// InstructionForNextAgent1 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct InstructionForNextAgent1 {
	#[serde(rename = "Cd")]
	pub cd: String,
	#[serde(rename = "InstrInf")]
	pub instr_inf: String,
}


// LEIIdentifier 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct LEIIdentifier {
	#[serde(rename = "LEIIdentifier")]
	pub lei_identifier: String,
}


// LocalInstrument2Choice 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct LocalInstrument2Choice {
	#[serde(rename = "Cd")]
	pub cd: String,
	#[serde(rename = "Prtry")]
	pub prtry: String,
}


// MandateClassification1Choice 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct MandateClassification1Choice {
	#[serde(rename = "Cd")]
	pub cd: String,
	#[serde(rename = "Prtry")]
	pub prtry: String,
}


// MandateClassification1Code 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct MandateClassification1Code {
	#[serde(rename = "MandateClassification1Code")]
	pub mandate_classification1_code: String,
}


// MandateRelatedData2Choice 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct MandateRelatedData2Choice {
	#[serde(rename = "DrctDbtMndt")]
	pub drct_dbt_mndt: MandateRelatedInformation15,
	#[serde(rename = "CdtTrfMndt")]
	pub cdt_trf_mndt: CreditTransferMandateData1,
}


// MandateRelatedInformation15 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct MandateRelatedInformation15 {
	#[serde(rename = "MndtId")]
	pub mndt_id: String,
	#[serde(rename = "DtOfSgntr")]
	pub dt_of_sgntr: u8,
	#[serde(rename = "AmdmntInd")]
	pub amdmnt_ind: bool,
	#[serde(rename = "AmdmntInfDtls")]
	pub amdmnt_inf_dtls: AmendmentInformationDetails14,
	#[serde(rename = "ElctrncSgntr")]
	pub elctrnc_sgntr: String,
	#[serde(rename = "FrstColltnDt")]
	pub frst_colltn_dt: u8,
	#[serde(rename = "FnlColltnDt")]
	pub fnl_colltn_dt: u8,
	#[serde(rename = "Frqcy")]
	pub frqcy: Frequency36Choice,
	#[serde(rename = "Rsn")]
	pub rsn: MandateSetupReason1Choice,
	#[serde(rename = "TrckgDays")]
	pub trckg_days: String,
}


// MandateSetupReason1Choice 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct MandateSetupReason1Choice {
	#[serde(rename = "Cd")]
	pub cd: String,
	#[serde(rename = "Prtry")]
	pub prtry: String,
}


// MandateTypeInformation2 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct MandateTypeInformation2 {
	#[serde(rename = "SvcLvl")]
	pub svc_lvl: ServiceLevel8Choice,
	#[serde(rename = "LclInstrm")]
	pub lcl_instrm: LocalInstrument2Choice,
	#[serde(rename = "CtgyPurp")]
	pub ctgy_purp: CategoryPurpose1Choice,
	#[serde(rename = "Clssfctn")]
	pub clssfctn: MandateClassification1Choice,
}


// Max1025Text 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Max1025Text {
	#[serde(rename = "Max1025Text")]
	pub max1025_text: String,
}


// Max10KBinary 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Max10KBinary {
	#[serde(rename = "Max10KBinary")]
	pub max10_k_binary: String,
}


// Max128Text 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Max128Text {
	#[serde(rename = "Max128Text")]
	pub max128_text: String,
}


// Max140Text 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Max140Text {
	#[serde(rename = "Max140Text")]
	pub max140_text: String,
}


// Max16Text 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Max16Text {
	#[serde(rename = "Max16Text")]
	pub max16_text: String,
}


// Max2048Text 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Max2048Text {
	#[serde(rename = "Max2048Text")]
	pub max2048_text: String,
}


// Max34Text 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Max34Text {
	#[serde(rename = "Max34Text")]
	pub max34_text: String,
}


// Max350Text 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Max350Text {
	#[serde(rename = "Max350Text")]
	pub max350_text: String,
}


// Max35Text 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Max35Text {
	#[serde(rename = "Max35Text")]
	pub max35_text: String,
}


// Max4Text 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Max4Text {
	#[serde(rename = "Max4Text")]
	pub max4_text: String,
}


// Max70Text 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Max70Text {
	#[serde(rename = "Max70Text")]
	pub max70_text: String,
}


// NameAndAddress16 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct NameAndAddress16 {
	#[serde(rename = "Nm")]
	pub nm: String,
	#[serde(rename = "Adr")]
	pub adr: PostalAddress24,
}


// NamePrefix2Code 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct NamePrefix2Code {
	#[serde(rename = "NamePrefix2Code")]
	pub name_prefix2_code: String,
}


// Number 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Number {
	#[serde(rename = "Number")]
	pub number: f64,
}


// OrganisationIdentification29 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct OrganisationIdentification29 {
	#[serde(rename = "AnyBIC")]
	pub any_bic: String,
	#[serde(rename = "LEI")]
	pub lei: String,
	#[serde(rename = "Othr")]
	pub othr: Vec<GenericOrganisationIdentification1>,
}


// OrganisationIdentificationSchemeName1Choice 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct OrganisationIdentificationSchemeName1Choice {
	#[serde(rename = "Cd")]
	pub cd: String,
	#[serde(rename = "Prtry")]
	pub prtry: String,
}


// OriginalGroupInformation29 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct OriginalGroupInformation29 {
	#[serde(rename = "OrgnlMsgId")]
	pub orgnl_msg_id: String,
	#[serde(rename = "OrgnlMsgNmId")]
	pub orgnl_msg_nm_id: String,
	#[serde(rename = "OrgnlCreDtTm")]
	pub orgnl_cre_dt_tm: u8,
}


// OriginalTransactionReference35 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct OriginalTransactionReference35 {
	#[serde(rename = "IntrBkSttlmAmt")]
	pub intr_bk_sttlm_amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "Amt")]
	pub amt: AmountType4Choice,
	#[serde(rename = "IntrBkSttlmDt")]
	pub intr_bk_sttlm_dt: u8,
	#[serde(rename = "ReqdColltnDt")]
	pub reqd_colltn_dt: u8,
	#[serde(rename = "ReqdExctnDt")]
	pub reqd_exctn_dt: DateAndDateTime2Choice,
	#[serde(rename = "CdtrSchmeId")]
	pub cdtr_schme_id: PartyIdentification135,
	#[serde(rename = "SttlmInf")]
	pub sttlm_inf: SettlementInstruction11,
	#[serde(rename = "PmtTpInf")]
	pub pmt_tp_inf: PaymentTypeInformation27,
	#[serde(rename = "PmtMtd")]
	pub pmt_mtd: String,
	#[serde(rename = "MndtRltdInf")]
	pub mndt_rltd_inf: MandateRelatedData2Choice,
	#[serde(rename = "RmtInf")]
	pub rmt_inf: RemittanceInformation21,
	#[serde(rename = "UltmtDbtr")]
	pub ultmt_dbtr: Party40Choice,
	#[serde(rename = "Dbtr")]
	pub dbtr: Party40Choice,
	#[serde(rename = "DbtrAcct")]
	pub dbtr_acct: CashAccount40,
	#[serde(rename = "DbtrAgt")]
	pub dbtr_agt: BranchAndFinancialInstitutionIdentification6,
	#[serde(rename = "DbtrAgtAcct")]
	pub dbtr_agt_acct: CashAccount40,
	#[serde(rename = "CdtrAgt")]
	pub cdtr_agt: BranchAndFinancialInstitutionIdentification6,
	#[serde(rename = "CdtrAgtAcct")]
	pub cdtr_agt_acct: CashAccount40,
	#[serde(rename = "Cdtr")]
	pub cdtr: Party40Choice,
	#[serde(rename = "CdtrAcct")]
	pub cdtr_acct: CashAccount40,
	#[serde(rename = "UltmtCdtr")]
	pub ultmt_cdtr: Party40Choice,
	#[serde(rename = "Purp")]
	pub purp: Purpose2Choice,
}


// OtherContact1 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct OtherContact1 {
	#[serde(rename = "ChanlTp")]
	pub chanl_tp: String,
	#[serde(rename = "Id")]
	pub id: String,
}


// Party38Choice 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Party38Choice {
	#[serde(rename = "OrgId")]
	pub org_id: OrganisationIdentification29,
	#[serde(rename = "PrvtId")]
	pub prvt_id: PersonIdentification13,
}


// Party40Choice 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Party40Choice {
	#[serde(rename = "Pty")]
	pub pty: PartyIdentification135,
	#[serde(rename = "Agt")]
	pub agt: BranchAndFinancialInstitutionIdentification6,
}


// PartyIdentification135 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct PartyIdentification135 {
	#[serde(rename = "Nm")]
	pub nm: String,
	#[serde(rename = "PstlAdr")]
	pub pstl_adr: PostalAddress24,
	#[serde(rename = "Id")]
	pub id: Party38Choice,
	#[serde(rename = "CtryOfRes")]
	pub ctry_of_res: String,
	#[serde(rename = "CtctDtls")]
	pub ctct_dtls: Contact4,
}


// PaymentMethod4Code 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct PaymentMethod4Code {
	#[serde(rename = "PaymentMethod4Code")]
	pub payment_method4_code: String,
}


// PaymentTypeInformation27 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct PaymentTypeInformation27 {
	#[serde(rename = "InstrPrty")]
	pub instr_prty: String,
	#[serde(rename = "ClrChanl")]
	pub clr_chanl: String,
	#[serde(rename = "SvcLvl")]
	pub svc_lvl: Vec<ServiceLevel8Choice>,
	#[serde(rename = "LclInstrm")]
	pub lcl_instrm: LocalInstrument2Choice,
	#[serde(rename = "SeqTp")]
	pub seq_tp: String,
	#[serde(rename = "CtgyPurp")]
	pub ctgy_purp: CategoryPurpose1Choice,
}


// PercentageRate 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct PercentageRate {
	#[serde(rename = "PercentageRate")]
	pub percentage_rate: f64,
}


// PersonIdentification13 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct PersonIdentification13 {
	#[serde(rename = "DtAndPlcOfBirth")]
	pub dt_and_plc_of_birth: DateAndPlaceOfBirth1,
	#[serde(rename = "Othr")]
	pub othr: Vec<GenericPersonIdentification1>,
}


// PersonIdentificationSchemeName1Choice 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct PersonIdentificationSchemeName1Choice {
	#[serde(rename = "Cd")]
	pub cd: String,
	#[serde(rename = "Prtry")]
	pub prtry: String,
}


// PhoneNumber 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct PhoneNumber {
	#[serde(rename = "PhoneNumber")]
	pub phone_number: String,
}


// PostalAddress24 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct PostalAddress24 {
	#[serde(rename = "AdrTp")]
	pub adr_tp: AddressType3Choice,
	#[serde(rename = "Dept")]
	pub dept: String,
	#[serde(rename = "SubDept")]
	pub sub_dept: String,
	#[serde(rename = "StrtNm")]
	pub strt_nm: String,
	#[serde(rename = "BldgNb")]
	pub bldg_nb: String,
	#[serde(rename = "BldgNm")]
	pub bldg_nm: String,
	#[serde(rename = "Flr")]
	pub flr: String,
	#[serde(rename = "PstBx")]
	pub pst_bx: String,
	#[serde(rename = "Room")]
	pub room: String,
	#[serde(rename = "PstCd")]
	pub pst_cd: String,
	#[serde(rename = "TwnNm")]
	pub twn_nm: String,
	#[serde(rename = "TwnLctnNm")]
	pub twn_lctn_nm: String,
	#[serde(rename = "DstrctNm")]
	pub dstrct_nm: String,
	#[serde(rename = "CtrySubDvsn")]
	pub ctry_sub_dvsn: String,
	#[serde(rename = "Ctry")]
	pub ctry: String,
	#[serde(rename = "AdrLine")]
	pub adr_line: Vec<String>,
}


// PreferredContactMethod1Code 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct PreferredContactMethod1Code {
	#[serde(rename = "PreferredContactMethod1Code")]
	pub preferred_contact_method1_code: String,
}


// Priority2Code 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Priority2Code {
	#[serde(rename = "Priority2Code")]
	pub priority2_code: String,
}


// ProxyAccountIdentification1 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ProxyAccountIdentification1 {
	#[serde(rename = "Tp")]
	pub tp: ProxyAccountType1Choice,
	#[serde(rename = "Id")]
	pub id: String,
}


// ProxyAccountType1Choice 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ProxyAccountType1Choice {
	#[serde(rename = "Cd")]
	pub cd: String,
	#[serde(rename = "Prtry")]
	pub prtry: String,
}


// Purpose2Choice 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Purpose2Choice {
	#[serde(rename = "Cd")]
	pub cd: String,
	#[serde(rename = "Prtry")]
	pub prtry: String,
}


// ReferredDocumentInformation7 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ReferredDocumentInformation7 {
	#[serde(rename = "Tp")]
	pub tp: ReferredDocumentType4,
	#[serde(rename = "Nb")]
	pub nb: String,
	#[serde(rename = "RltdDt")]
	pub rltd_dt: u8,
	#[serde(rename = "LineDtls")]
	pub line_dtls: Vec<DocumentLineInformation1>,
}


// ReferredDocumentType3Choice 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ReferredDocumentType3Choice {
	#[serde(rename = "Cd")]
	pub cd: String,
	#[serde(rename = "Prtry")]
	pub prtry: String,
}


// ReferredDocumentType4 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ReferredDocumentType4 {
	#[serde(rename = "CdOrPrtry")]
	pub cd_or_prtry: ReferredDocumentType3Choice,
	#[serde(rename = "Issr")]
	pub issr: String,
}


// RemittanceAmount2 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct RemittanceAmount2 {
	#[serde(rename = "DuePyblAmt")]
	pub due_pybl_amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "DscntApldAmt")]
	pub dscnt_apld_amt: Vec<DiscountAmountAndType1>,
	#[serde(rename = "CdtNoteAmt")]
	pub cdt_note_amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "TaxAmt")]
	pub tax_amt: Vec<TaxAmountAndType1>,
	#[serde(rename = "AdjstmntAmtAndRsn")]
	pub adjstmnt_amt_and_rsn: Vec<DocumentAdjustment1>,
	#[serde(rename = "RmtdAmt")]
	pub rmtd_amt: ActiveOrHistoricCurrencyAndAmount,
}


// RemittanceAmount3 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct RemittanceAmount3 {
	#[serde(rename = "DuePyblAmt")]
	pub due_pybl_amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "DscntApldAmt")]
	pub dscnt_apld_amt: Vec<DiscountAmountAndType1>,
	#[serde(rename = "CdtNoteAmt")]
	pub cdt_note_amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "TaxAmt")]
	pub tax_amt: Vec<TaxAmountAndType1>,
	#[serde(rename = "AdjstmntAmtAndRsn")]
	pub adjstmnt_amt_and_rsn: Vec<DocumentAdjustment1>,
	#[serde(rename = "RmtdAmt")]
	pub rmtd_amt: ActiveOrHistoricCurrencyAndAmount,
}


// RemittanceInformation21 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct RemittanceInformation21 {
	#[serde(rename = "Ustrd")]
	pub ustrd: Vec<String>,
	#[serde(rename = "Strd")]
	pub strd: Vec<StructuredRemittanceInformation17>,
}


// RemittanceLocation7 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct RemittanceLocation7 {
	#[serde(rename = "RmtId")]
	pub rmt_id: String,
	#[serde(rename = "RmtLctnDtls")]
	pub rmt_lctn_dtls: Vec<RemittanceLocationData1>,
}


// RemittanceLocationData1 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct RemittanceLocationData1 {
	#[serde(rename = "Mtd")]
	pub mtd: String,
	#[serde(rename = "ElctrncAdr")]
	pub elctrnc_adr: String,
	#[serde(rename = "PstlAdr")]
	pub pstl_adr: NameAndAddress16,
}


// RemittanceLocationMethod2Code 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct RemittanceLocationMethod2Code {
	#[serde(rename = "RemittanceLocationMethod2Code")]
	pub remittance_location_method2_code: String,
}


// RequestToModifyPaymentV08 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct RequestToModifyPaymentV08 {
	#[serde(rename = "Assgnmt")]
	pub assgnmt: CaseAssignment5,
	#[serde(rename = "Case")]
	pub case: Case5,
	#[serde(rename = "Undrlyg")]
	pub undrlyg: UnderlyingTransaction7Choice,
	#[serde(rename = "Mod")]
	pub mod_attr: RequestedModification10,
	#[serde(rename = "InstrForAssgne")]
	pub instr_for_assgne: InstructionForAssignee1,
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Vec<SupplementaryData1>,
}


// RequestedModification10 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct RequestedModification10 {
	#[serde(rename = "InstrId")]
	pub instr_id: String,
	#[serde(rename = "EndToEndId")]
	pub end_to_end_id: String,
	#[serde(rename = "TxId")]
	pub tx_id: String,
	#[serde(rename = "PmtTpInf")]
	pub pmt_tp_inf: PaymentTypeInformation27,
	#[serde(rename = "ReqdExctnDt")]
	pub reqd_exctn_dt: DateAndDateTime2Choice,
	#[serde(rename = "ReqdColltnDt")]
	pub reqd_colltn_dt: u8,
	#[serde(rename = "IntrBkSttlmDt")]
	pub intr_bk_sttlm_dt: u8,
	#[serde(rename = "SttlmTmIndctn")]
	pub sttlm_tm_indctn: SettlementDateTimeIndication1,
	#[serde(rename = "Amt")]
	pub amt: AmountType4Choice,
	#[serde(rename = "IntrBkSttlmAmt")]
	pub intr_bk_sttlm_amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "ChrgBr")]
	pub chrg_br: String,
	#[serde(rename = "UltmtDbtr")]
	pub ultmt_dbtr: PartyIdentification135,
	#[serde(rename = "Dbtr")]
	pub dbtr: PartyIdentification135,
	#[serde(rename = "DbtrAcct")]
	pub dbtr_acct: CashAccount40,
	#[serde(rename = "DbtrAgtAcct")]
	pub dbtr_agt_acct: CashAccount40,
	#[serde(rename = "SttlmInf")]
	pub sttlm_inf: SettlementInstruction13,
	#[serde(rename = "CdtrAgtAcct")]
	pub cdtr_agt_acct: CashAccount40,
	#[serde(rename = "Cdtr")]
	pub cdtr: PartyIdentification135,
	#[serde(rename = "CdtrAcct")]
	pub cdtr_acct: CashAccount40,
	#[serde(rename = "UltmtCdtr")]
	pub ultmt_cdtr: PartyIdentification135,
	#[serde(rename = "Purp")]
	pub purp: Purpose2Choice,
	#[serde(rename = "InstrForDbtrAgt")]
	pub instr_for_dbtr_agt: String,
	#[serde(rename = "InstrForNxtAgt")]
	pub instr_for_nxt_agt: Vec<InstructionForNextAgent1>,
	#[serde(rename = "InstrForCdtrAgt")]
	pub instr_for_cdtr_agt: Vec<InstructionForCreditorAgent3>,
	#[serde(rename = "RltdRmtInf")]
	pub rltd_rmt_inf: Vec<RemittanceLocation7>,
	#[serde(rename = "RmtInf")]
	pub rmt_inf: RemittanceInformation21,
}


// SequenceType3Code 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SequenceType3Code {
	#[serde(rename = "SequenceType3Code")]
	pub sequence_type3_code: String,
}


// ServiceLevel8Choice 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ServiceLevel8Choice {
	#[serde(rename = "Cd")]
	pub cd: String,
	#[serde(rename = "Prtry")]
	pub prtry: String,
}


// SettlementDateTimeIndication1 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SettlementDateTimeIndication1 {
	#[serde(rename = "DbtDtTm")]
	pub dbt_dt_tm: u8,
	#[serde(rename = "CdtDtTm")]
	pub cdt_dt_tm: u8,
}


// SettlementInstruction11 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SettlementInstruction11 {
	#[serde(rename = "SttlmMtd")]
	pub sttlm_mtd: String,
	#[serde(rename = "SttlmAcct")]
	pub sttlm_acct: CashAccount40,
	#[serde(rename = "ClrSys")]
	pub clr_sys: ClearingSystemIdentification3Choice,
	#[serde(rename = "InstgRmbrsmntAgt")]
	pub instg_rmbrsmnt_agt: BranchAndFinancialInstitutionIdentification6,
	#[serde(rename = "InstgRmbrsmntAgtAcct")]
	pub instg_rmbrsmnt_agt_acct: CashAccount40,
	#[serde(rename = "InstdRmbrsmntAgt")]
	pub instd_rmbrsmnt_agt: BranchAndFinancialInstitutionIdentification6,
	#[serde(rename = "InstdRmbrsmntAgtAcct")]
	pub instd_rmbrsmnt_agt_acct: CashAccount40,
	#[serde(rename = "ThrdRmbrsmntAgt")]
	pub thrd_rmbrsmnt_agt: BranchAndFinancialInstitutionIdentification6,
	#[serde(rename = "ThrdRmbrsmntAgtAcct")]
	pub thrd_rmbrsmnt_agt_acct: CashAccount40,
}


// SettlementInstruction13 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SettlementInstruction13 {
	#[serde(rename = "InstgRmbrsmntAgt")]
	pub instg_rmbrsmnt_agt: BranchAndFinancialInstitutionIdentification6,
	#[serde(rename = "InstgRmbrsmntAgtAcct")]
	pub instg_rmbrsmnt_agt_acct: CashAccount40,
	#[serde(rename = "InstdRmbrsmntAgt")]
	pub instd_rmbrsmnt_agt: BranchAndFinancialInstitutionIdentification6,
	#[serde(rename = "InstdRmbrsmntAgtAcct")]
	pub instd_rmbrsmnt_agt_acct: CashAccount40,
}


// SettlementMethod1Code 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SettlementMethod1Code {
	#[serde(rename = "SettlementMethod1Code")]
	pub settlement_method1_code: String,
}


// StructuredRemittanceInformation17 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct StructuredRemittanceInformation17 {
	#[serde(rename = "RfrdDocInf")]
	pub rfrd_doc_inf: Vec<ReferredDocumentInformation7>,
	#[serde(rename = "RfrdDocAmt")]
	pub rfrd_doc_amt: RemittanceAmount2,
	#[serde(rename = "CdtrRefInf")]
	pub cdtr_ref_inf: CreditorReferenceInformation2,
	#[serde(rename = "Invcr")]
	pub invcr: PartyIdentification135,
	#[serde(rename = "Invcee")]
	pub invcee: PartyIdentification135,
	#[serde(rename = "TaxRmt")]
	pub tax_rmt: TaxData1,
	#[serde(rename = "GrnshmtRmt")]
	pub grnshmt_rmt: Garnishment3,
	#[serde(rename = "AddtlRmtInf")]
	pub addtl_rmt_inf: Vec<String>,
}


// SupplementaryData1 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SupplementaryData1 {
	#[serde(rename = "PlcAndNm")]
	pub plc_and_nm: String,
	#[serde(rename = "Envlp")]
	pub envlp: SupplementaryDataEnvelope1,
}


// SupplementaryDataEnvelope1 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SupplementaryDataEnvelope1 {
}


// TaxAmount3 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct TaxAmount3 {
	#[serde(rename = "Rate")]
	pub rate: f64,
	#[serde(rename = "TaxblBaseAmt")]
	pub taxbl_base_amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "TtlAmt")]
	pub ttl_amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "Dtls")]
	pub dtls: Vec<TaxRecordDetails3>,
}


// TaxAmountAndType1 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct TaxAmountAndType1 {
	#[serde(rename = "Tp")]
	pub tp: TaxAmountType1Choice,
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
}


// TaxAmountType1Choice 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct TaxAmountType1Choice {
	#[serde(rename = "Cd")]
	pub cd: String,
	#[serde(rename = "Prtry")]
	pub prtry: String,
}


// TaxAuthorisation1 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct TaxAuthorisation1 {
	#[serde(rename = "Titl")]
	pub titl: String,
	#[serde(rename = "Nm")]
	pub nm: String,
}


// TaxData1 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct TaxData1 {
	#[serde(rename = "Cdtr")]
	pub cdtr: TaxParty1,
	#[serde(rename = "Dbtr")]
	pub dbtr: TaxParty2,
	#[serde(rename = "UltmtDbtr")]
	pub ultmt_dbtr: TaxParty2,
	#[serde(rename = "AdmstnZone")]
	pub admstn_zone: String,
	#[serde(rename = "RefNb")]
	pub ref_nb: String,
	#[serde(rename = "Mtd")]
	pub mtd: String,
	#[serde(rename = "TtlTaxblBaseAmt")]
	pub ttl_taxbl_base_amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "TtlTaxAmt")]
	pub ttl_tax_amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "Dt")]
	pub dt: u8,
	#[serde(rename = "SeqNb")]
	pub seq_nb: f64,
	#[serde(rename = "Rcrd")]
	pub rcrd: Vec<TaxRecord3>,
}


// TaxParty1 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct TaxParty1 {
	#[serde(rename = "TaxId")]
	pub tax_id: String,
	#[serde(rename = "RegnId")]
	pub regn_id: String,
	#[serde(rename = "TaxTp")]
	pub tax_tp: String,
}


// TaxParty2 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct TaxParty2 {
	#[serde(rename = "TaxId")]
	pub tax_id: String,
	#[serde(rename = "RegnId")]
	pub regn_id: String,
	#[serde(rename = "TaxTp")]
	pub tax_tp: String,
	#[serde(rename = "Authstn")]
	pub authstn: TaxAuthorisation1,
}


// TaxPeriod3 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct TaxPeriod3 {
	#[serde(rename = "Yr")]
	pub yr: String,
	#[serde(rename = "Tp")]
	pub tp: String,
	#[serde(rename = "FrToDt")]
	pub fr_to_dt: DatePeriod2,
}


// TaxRecord3 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct TaxRecord3 {
	#[serde(rename = "Tp")]
	pub tp: String,
	#[serde(rename = "Ctgy")]
	pub ctgy: String,
	#[serde(rename = "CtgyDtls")]
	pub ctgy_dtls: String,
	#[serde(rename = "DbtrSts")]
	pub dbtr_sts: String,
	#[serde(rename = "CertId")]
	pub cert_id: String,
	#[serde(rename = "FrmsCd")]
	pub frms_cd: String,
	#[serde(rename = "Prd")]
	pub prd: TaxPeriod3,
	#[serde(rename = "TaxAmt")]
	pub tax_amt: TaxAmount3,
	#[serde(rename = "AddtlInf")]
	pub addtl_inf: String,
}


// TaxRecordDetails3 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct TaxRecordDetails3 {
	#[serde(rename = "Prd")]
	pub prd: TaxPeriod3,
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
}


// TaxRecordPeriod1Code 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct TaxRecordPeriod1Code {
	#[serde(rename = "TaxRecordPeriod1Code")]
	pub tax_record_period1_code: String,
}


// TrueFalseIndicator 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct TrueFalseIndicator {
	#[serde(rename = "TrueFalseIndicator")]
	pub true_false_indicator: bool,
}


// UUIDv4Identifier 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct UUIDv4Identifier {
	#[serde(rename = "UUIDv4Identifier")]
	pub uui_dv4_identifier: String,
}


// UnderlyingGroupInformation1 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct UnderlyingGroupInformation1 {
	#[serde(rename = "OrgnlMsgId")]
	pub orgnl_msg_id: String,
	#[serde(rename = "OrgnlMsgNmId")]
	pub orgnl_msg_nm_id: String,
	#[serde(rename = "OrgnlCreDtTm")]
	pub orgnl_cre_dt_tm: u8,
	#[serde(rename = "OrgnlMsgDlvryChanl")]
	pub orgnl_msg_dlvry_chanl: String,
}


// UnderlyingPaymentInstruction7 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct UnderlyingPaymentInstruction7 {
	#[serde(rename = "OrgnlGrpInf")]
	pub orgnl_grp_inf: UnderlyingGroupInformation1,
	#[serde(rename = "OrgnlPmtInfId")]
	pub orgnl_pmt_inf_id: String,
	#[serde(rename = "OrgnlInstrId")]
	pub orgnl_instr_id: String,
	#[serde(rename = "OrgnlEndToEndId")]
	pub orgnl_end_to_end_id: String,
	#[serde(rename = "OrgnlUETR")]
	pub orgnl_uetr: String,
	#[serde(rename = "OrgnlInstdAmt")]
	pub orgnl_instd_amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "ReqdExctnDt")]
	pub reqd_exctn_dt: DateAndDateTime2Choice,
	#[serde(rename = "ReqdColltnDt")]
	pub reqd_colltn_dt: u8,
	#[serde(rename = "OrgnlTxRef")]
	pub orgnl_tx_ref: OriginalTransactionReference35,
}


// UnderlyingPaymentTransaction6 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct UnderlyingPaymentTransaction6 {
	#[serde(rename = "OrgnlGrpInf")]
	pub orgnl_grp_inf: UnderlyingGroupInformation1,
	#[serde(rename = "OrgnlInstrId")]
	pub orgnl_instr_id: String,
	#[serde(rename = "OrgnlEndToEndId")]
	pub orgnl_end_to_end_id: String,
	#[serde(rename = "OrgnlTxId")]
	pub orgnl_tx_id: String,
	#[serde(rename = "OrgnlUETR")]
	pub orgnl_uetr: String,
	#[serde(rename = "OrgnlIntrBkSttlmAmt")]
	pub orgnl_intr_bk_sttlm_amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "OrgnlIntrBkSttlmDt")]
	pub orgnl_intr_bk_sttlm_dt: u8,
	#[serde(rename = "OrgnlTxRef")]
	pub orgnl_tx_ref: OriginalTransactionReference35,
}


// UnderlyingStatementEntry3 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct UnderlyingStatementEntry3 {
	#[serde(rename = "OrgnlGrpInf")]
	pub orgnl_grp_inf: OriginalGroupInformation29,
	#[serde(rename = "OrgnlStmtId")]
	pub orgnl_stmt_id: String,
	#[serde(rename = "OrgnlNtryId")]
	pub orgnl_ntry_id: String,
	#[serde(rename = "OrgnlUETR")]
	pub orgnl_uetr: String,
}


// UnderlyingTransaction7Choice 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct UnderlyingTransaction7Choice {
	#[serde(rename = "Initn")]
	pub initn: UnderlyingPaymentInstruction7,
	#[serde(rename = "IntrBk")]
	pub intr_bk: UnderlyingPaymentTransaction6,
	#[serde(rename = "StmtNtry")]
	pub stmt_ntry: UnderlyingStatementEntry3,
}


// YesNoIndicator 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct YesNoIndicator {
	#[serde(rename = "YesNoIndicator")]
	pub yes_no_indicator: bool,
}
