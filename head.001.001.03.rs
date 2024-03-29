// IsSettled iso20022

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_xml_rs;

use serde_xml_rs::from_reader;


// app_hdr 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct app_hdr {
	#[serde(rename = "AppHdr")]
	pub app_hdr: BusinessApplicationHeaderV03,
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


// BusinessApplicationHeader7 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct BusinessApplicationHeader7 {
	#[serde(rename = "CharSet")]
	pub char_set: String,
	#[serde(rename = "Fr")]
	pub fr: Party44Choice,
	#[serde(rename = "To")]
	pub to: Party44Choice,
	#[serde(rename = "BizMsgIdr")]
	pub biz_msg_idr: String,
	#[serde(rename = "MsgDefIdr")]
	pub msg_def_idr: String,
	#[serde(rename = "BizSvc")]
	pub biz_svc: String,
	#[serde(rename = "MktPrctc")]
	pub mkt_prctc: ImplementationSpecification1,
	#[serde(rename = "CreDt")]
	pub cre_dt: u8,
	#[serde(rename = "BizPrcgDt")]
	pub biz_prcg_dt: u8,
	#[serde(rename = "CpyDplct")]
	pub cpy_dplct: String,
	#[serde(rename = "PssblDplct")]
	pub pssbl_dplct: bool,
	#[serde(rename = "Prty")]
	pub prty: String,
	#[serde(rename = "Sgntr")]
	pub sgntr: SignatureEnvelope,
}


// BusinessApplicationHeaderV03 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct BusinessApplicationHeaderV03 {
	#[serde(rename = "CharSet")]
	pub char_set: String,
	#[serde(rename = "Fr")]
	pub fr: Party44Choice,
	#[serde(rename = "To")]
	pub to: Party44Choice,
	#[serde(rename = "BizMsgIdr")]
	pub biz_msg_idr: String,
	#[serde(rename = "MsgDefIdr")]
	pub msg_def_idr: String,
	#[serde(rename = "BizSvc")]
	pub biz_svc: String,
	#[serde(rename = "MktPrctc")]
	pub mkt_prctc: ImplementationSpecification1,
	#[serde(rename = "CreDt")]
	pub cre_dt: u8,
	#[serde(rename = "BizPrcgDt")]
	pub biz_prcg_dt: u8,
	#[serde(rename = "CpyDplct")]
	pub cpy_dplct: String,
	#[serde(rename = "PssblDplct")]
	pub pssbl_dplct: bool,
	#[serde(rename = "Prty")]
	pub prty: String,
	#[serde(rename = "Sgntr")]
	pub sgntr: SignatureEnvelope,
	#[serde(rename = "Rltd")]
	pub rltd: Vec<BusinessApplicationHeader7>,
}


// BusinessMessagePriorityCode 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct BusinessMessagePriorityCode {
	#[serde(rename = "BusinessMessagePriorityCode")]
	pub business_message_priority_code: String,
}


// ClearingSystemIdentification2Choice 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ClearingSystemIdentification2Choice {
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


// CopyDuplicate1Code 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CopyDuplicate1Code {
	#[serde(rename = "CopyDuplicate1Code")]
	pub copy_duplicate1_code: String,
}


// CountryCode 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CountryCode {
	#[serde(rename = "CountryCode")]
	pub country_code: String,
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


// Exact4AlphaNumericText 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Exact4AlphaNumericText {
	#[serde(rename = "Exact4AlphaNumericText")]
	pub exact4_alpha_numeric_text: String,
}


// ExternalClearingSystemIdentification1Code 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ExternalClearingSystemIdentification1Code {
	#[serde(rename = "ExternalClearingSystemIdentification1Code")]
	pub external_clearing_system_identification1_code: String,
}


// ExternalFinancialInstitutionIdentification1Code 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ExternalFinancialInstitutionIdentification1Code {
	#[serde(rename = "ExternalFinancialInstitutionIdentification1Code")]
	pub external_financial_institution_identification1_code: String,
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


// ImplementationSpecification1 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ImplementationSpecification1 {
	#[serde(rename = "Regy")]
	pub regy: String,
	#[serde(rename = "Id")]
	pub id: String,
}


// LEIIdentifier 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct LEIIdentifier {
	#[serde(rename = "LEIIdentifier")]
	pub lei_identifier: String,
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


// NamePrefix2Code 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct NamePrefix2Code {
	#[serde(rename = "NamePrefix2Code")]
	pub name_prefix2_code: String,
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


// Party44Choice 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Party44Choice {
	#[serde(rename = "OrgId")]
	pub org_id: PartyIdentification135,
	#[serde(rename = "FIId")]
	pub fi_id: BranchAndFinancialInstitutionIdentification6,
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


// SignatureEnvelope 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SignatureEnvelope {
}


// UnicodeChartsCode 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct UnicodeChartsCode {
	#[serde(rename = "UnicodeChartsCode")]
	pub unicode_charts_code: String,
}


// YesNoIndicator 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct YesNoIndicator {
	#[serde(rename = "YesNoIndicator")]
	pub yes_no_indicator: bool,
}
