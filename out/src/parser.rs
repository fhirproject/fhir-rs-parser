#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "resourceType")]
pub enum FHIRResource {
  #[serde(rename = "Appointment")]
  ParsedAppointment(model::Appointment::Appointment),

  #[serde(rename = "InsurancePlan")]
  ParsedInsurancePlan(model::InsurancePlan::InsurancePlan),

  #[serde(rename = "CatalogEntry")]
  ParsedCatalogEntry(model::CatalogEntry::CatalogEntry),

  #[serde(rename = "MedicationKnowledge")]
  ParsedMedicationKnowledge(model::MedicationKnowledge::MedicationKnowledge),

  #[serde(rename = "DeviceMetric")]
  ParsedDeviceMetric(model::DeviceMetric::DeviceMetric),

  #[serde(rename = "BodyStructure")]
  ParsedBodyStructure(model::BodyStructure::BodyStructure),

  #[serde(rename = "OrganizationAffiliation")]
  ParsedOrganizationAffiliation(model::OrganizationAffiliation::OrganizationAffiliation),

  #[serde(rename = "ObservationDefinition")]
  ParsedObservationDefinition(model::ObservationDefinition::ObservationDefinition),

  #[serde(rename = "Specimen")]
  ParsedSpecimen(model::Specimen::Specimen),

  #[serde(rename = "Person")]
  ParsedPerson(model::Person::Person),

  #[serde(rename = "MedicinalProductIndication")]
  ParsedMedicinalProductIndication(model::MedicinalProductIndication::MedicinalProductIndication),

  #[serde(rename = "SubstanceNucleicAcid")]
  ParsedSubstanceNucleicAcid(model::SubstanceNucleicAcid::SubstanceNucleicAcid),

  #[serde(rename = "Bundle")]
  ParsedBundle(model::Bundle::Bundle),

  #[serde(rename = "RiskAssessment")]
  ParsedRiskAssessment(model::RiskAssessment::RiskAssessment),

  #[serde(rename = "SubstanceSourceMaterial")]
  ParsedSubstanceSourceMaterial(model::SubstanceSourceMaterial::SubstanceSourceMaterial),

  #[serde(rename = "AllergyIntolerance")]
  ParsedAllergyIntolerance(model::AllergyIntolerance::AllergyIntolerance),

  #[serde(rename = "ImplementationGuide")]
  ParsedImplementationGuide(model::ImplementationGuide::ImplementationGuide),

  #[serde(rename = "Provenance")]
  ParsedProvenance(model::Provenance::Provenance),

  #[serde(rename = "EnrollmentRequest")]
  ParsedEnrollmentRequest(model::EnrollmentRequest::EnrollmentRequest),

  #[serde(rename = "CodeSystem")]
  ParsedCodeSystem(model::CodeSystem::CodeSystem),

  #[serde(rename = "ResearchElementDefinition")]
  ParsedResearchElementDefinition(model::ResearchElementDefinition::ResearchElementDefinition),

  #[serde(rename = "TestScript")]
  ParsedTestScript(model::TestScript::TestScript),

  #[serde(rename = "CommunicationRequest")]
  ParsedCommunicationRequest(model::CommunicationRequest::CommunicationRequest),

  #[serde(rename = "EventDefinition")]
  ParsedEventDefinition(model::EventDefinition::EventDefinition),

  #[serde(rename = "ClaimResponse")]
  ParsedClaimResponse(model::ClaimResponse::ClaimResponse),

  #[serde(rename = "GraphDefinition")]
  ParsedGraphDefinition(model::GraphDefinition::GraphDefinition),

  #[serde(rename = "Linkage")]
  ParsedLinkage(model::Linkage::Linkage),

  #[serde(rename = "FamilyMemberHistory")]
  ParsedFamilyMemberHistory(model::FamilyMemberHistory::FamilyMemberHistory),

  #[serde(rename = "RelatedPerson")]
  ParsedRelatedPerson(model::RelatedPerson::RelatedPerson),

  #[serde(rename = "Schedule")]
  ParsedSchedule(model::Schedule::Schedule),

  #[serde(rename = "SearchParameter")]
  ParsedSearchParameter(model::SearchParameter::SearchParameter),

  #[serde(rename = "CompartmentDefinition")]
  ParsedCompartmentDefinition(model::CompartmentDefinition::CompartmentDefinition),

  #[serde(rename = "MessageDefinition")]
  ParsedMessageDefinition(model::MessageDefinition::MessageDefinition),

  #[serde(rename = "EvidenceVariable")]
  ParsedEvidenceVariable(model::EvidenceVariable::EvidenceVariable),

  #[serde(rename = "CarePlan")]
  ParsedCarePlan(model::CarePlan::CarePlan),

  #[serde(rename = "Immunization")]
  ParsedImmunization(model::Immunization::Immunization),

  #[serde(rename = "OperationDefinition")]
  ParsedOperationDefinition(model::OperationDefinition::OperationDefinition),

  #[serde(rename = "EpisodeOfCare")]
  ParsedEpisodeOfCare(model::EpisodeOfCare::EpisodeOfCare),

  #[serde(rename = "Basic")]
  ParsedBasic(model::Basic::Basic),

  #[serde(rename = "CapabilityStatement")]
  ParsedCapabilityStatement(model::CapabilityStatement::CapabilityStatement),

  #[serde(rename = "BiologicallyDerivedProduct")]
  ParsedBiologicallyDerivedProduct(model::BiologicallyDerivedProduct::BiologicallyDerivedProduct),

  #[serde(rename = "MedicinalProductContraindication")]
  ParsedMedicinalProductContraindication(model::MedicinalProductContraindication::MedicinalProductContraindication),

  #[serde(rename = "MedicinalProductIngredient")]
  ParsedMedicinalProductIngredient(model::MedicinalProductIngredient::MedicinalProductIngredient),

  #[serde(rename = "SpecimenDefinition")]
  ParsedSpecimenDefinition(model::SpecimenDefinition::SpecimenDefinition),

  #[serde(rename = "Medication")]
  ParsedMedication(model::Medication::Medication),

  #[serde(rename = "Flag")]
  ParsedFlag(model::Flag::Flag),

  #[serde(rename = "MedicinalProduct")]
  ParsedMedicinalProduct(model::MedicinalProduct::MedicinalProduct),

  #[serde(rename = "Questionnaire")]
  ParsedQuestionnaire(model::Questionnaire::Questionnaire),

  #[serde(rename = "ExplanationOfBenefit")]
  ParsedExplanationOfBenefit(model::ExplanationOfBenefit::ExplanationOfBenefit),

  #[serde(rename = "Procedure")]
  ParsedProcedure(model::Procedure::Procedure),

  #[serde(rename = "StructureDefinition")]
  ParsedStructureDefinition(model::StructureDefinition::StructureDefinition),

  #[serde(rename = "SubstancePolymer")]
  ParsedSubstancePolymer(model::SubstancePolymer::SubstancePolymer),

  #[serde(rename = "PractitionerRole")]
  ParsedPractitionerRole(model::PractitionerRole::PractitionerRole),

  #[serde(rename = "Endpoint")]
  ParsedEndpoint(model::Endpoint::Endpoint),

  #[serde(rename = "MedicinalProductUndesirableEffect")]
  ParsedMedicinalProductUndesirableEffect(model::MedicinalProductUndesirableEffect::MedicinalProductUndesirableEffect),

  #[serde(rename = "TestReport")]
  ParsedTestReport(model::TestReport::TestReport),

  #[serde(rename = "SupplyDelivery")]
  ParsedSupplyDelivery(model::SupplyDelivery::SupplyDelivery),

  #[serde(rename = "MeasureReport")]
  ParsedMeasureReport(model::MeasureReport::MeasureReport),

  #[serde(rename = "AppointmentResponse")]
  ParsedAppointmentResponse(model::AppointmentResponse::AppointmentResponse),

  #[serde(rename = "DiagnosticReport")]
  ParsedDiagnosticReport(model::DiagnosticReport::DiagnosticReport),

  #[serde(rename = "ExampleScenario")]
  ParsedExampleScenario(model::ExampleScenario::ExampleScenario),

  #[serde(rename = "Parameters")]
  ParsedParameters(model::Parameters::Parameters),

  #[serde(rename = "ResearchDefinition")]
  ParsedResearchDefinition(model::ResearchDefinition::ResearchDefinition),

  #[serde(rename = "ServiceRequest")]
  ParsedServiceRequest(model::ServiceRequest::ServiceRequest),

  #[serde(rename = "Communication")]
  ParsedCommunication(model::Communication::Communication),

  #[serde(rename = "TerminologyCapabilities")]
  ParsedTerminologyCapabilities(model::TerminologyCapabilities::TerminologyCapabilities),

  #[serde(rename = "List")]
  ParsedList(model::List::List),

  #[serde(rename = "Goal")]
  ParsedGoal(model::Goal::Goal),

  #[serde(rename = "SubstanceProtein")]
  ParsedSubstanceProtein(model::SubstanceProtein::SubstanceProtein),

  #[serde(rename = "MedicationStatement")]
  ParsedMedicationStatement(model::MedicationStatement::MedicationStatement),

  #[serde(rename = "DocumentReference")]
  ParsedDocumentReference(model::DocumentReference::DocumentReference),

  #[serde(rename = "NutritionOrder")]
  ParsedNutritionOrder(model::NutritionOrder::NutritionOrder),

  #[serde(rename = "VisionPrescription")]
  ParsedVisionPrescription(model::VisionPrescription::VisionPrescription),

  #[serde(rename = "Group")]
  ParsedGroup(model::Group::Group),

  #[serde(rename = "CoverageEligibilityResponse")]
  ParsedCoverageEligibilityResponse(model::CoverageEligibilityResponse::CoverageEligibilityResponse),

  #[serde(rename = "Coverage")]
  ParsedCoverage(model::Coverage::Coverage),

  #[serde(rename = "RequestGroup")]
  ParsedRequestGroup(model::RequestGroup::RequestGroup),

  #[serde(rename = "ChargeItem")]
  ParsedChargeItem(model::ChargeItem::ChargeItem),

  #[serde(rename = "Evidence")]
  ParsedEvidence(model::Evidence::Evidence),

  #[serde(rename = "ImmunizationRecommendation")]
  ParsedImmunizationRecommendation(model::ImmunizationRecommendation::ImmunizationRecommendation),

  #[serde(rename = "NamingSystem")]
  ParsedNamingSystem(model::NamingSystem::NamingSystem),

  #[serde(rename = "Patient")]
  ParsedPatient(model::Patient::Patient),

  #[serde(rename = "ResearchStudy")]
  ParsedResearchStudy(model::ResearchStudy::ResearchStudy),

  #[serde(rename = "Location")]
  ParsedLocation(model::Location::Location),

  #[serde(rename = "Task")]
  ParsedTask(model::Task::Task),

  #[serde(rename = "CareTeam")]
  ParsedCareTeam(model::CareTeam::CareTeam),

  #[serde(rename = "ResearchSubject")]
  ParsedResearchSubject(model::ResearchSubject::ResearchSubject),

  #[serde(rename = "ConceptMap")]
  ParsedConceptMap(model::ConceptMap::ConceptMap),

  #[serde(rename = "Device")]
  ParsedDevice(model::Device::Device),

  #[serde(rename = "QuestionnaireResponse")]
  ParsedQuestionnaireResponse(model::QuestionnaireResponse::QuestionnaireResponse),

  #[serde(rename = "ActivityDefinition")]
  ParsedActivityDefinition(model::ActivityDefinition::ActivityDefinition),

  #[serde(rename = "DocumentManifest")]
  ParsedDocumentManifest(model::DocumentManifest::DocumentManifest),

  #[serde(rename = "Invoice")]
  ParsedInvoice(model::Invoice::Invoice),

  #[serde(rename = "OperationOutcome")]
  ParsedOperationOutcome(model::OperationOutcome::OperationOutcome),

  #[serde(rename = "Slot")]
  ParsedSlot(model::Slot::Slot),

  #[serde(rename = "DeviceDefinition")]
  ParsedDeviceDefinition(model::DeviceDefinition::DeviceDefinition),

  #[serde(rename = "VerificationResult")]
  ParsedVerificationResult(model::VerificationResult::VerificationResult),

  #[serde(rename = "ImagingStudy")]
  ParsedImagingStudy(model::ImagingStudy::ImagingStudy),

  #[serde(rename = "Account")]
  ParsedAccount(model::Account::Account),

  #[serde(rename = "HealthcareService")]
  ParsedHealthcareService(model::HealthcareService::HealthcareService),

  #[serde(rename = "SupplyRequest")]
  ParsedSupplyRequest(model::SupplyRequest::SupplyRequest),

  #[serde(rename = "DeviceRequest")]
  ParsedDeviceRequest(model::DeviceRequest::DeviceRequest),

  #[serde(rename = "AuditEvent")]
  ParsedAuditEvent(model::AuditEvent::AuditEvent),

  #[serde(rename = "MedicinalProductPackaged")]
  ParsedMedicinalProductPackaged(model::MedicinalProductPackaged::MedicinalProductPackaged),

  #[serde(rename = "Substance")]
  ParsedSubstance(model::Substance::Substance),

  #[serde(rename = "Consent")]
  ParsedConsent(model::Consent::Consent),

  #[serde(rename = "ImmunizationEvaluation")]
  ParsedImmunizationEvaluation(model::ImmunizationEvaluation::ImmunizationEvaluation),

  #[serde(rename = "PlanDefinition")]
  ParsedPlanDefinition(model::PlanDefinition::PlanDefinition),

  #[serde(rename = "PaymentNotice")]
  ParsedPaymentNotice(model::PaymentNotice::PaymentNotice),

  #[serde(rename = "Claim")]
  ParsedClaim(model::Claim::Claim),

  #[serde(rename = "Contract")]
  ParsedContract(model::Contract::Contract),

  #[serde(rename = "ChargeItemDefinition")]
  ParsedChargeItemDefinition(model::ChargeItemDefinition::ChargeItemDefinition),

  #[serde(rename = "MedicationAdministration")]
  ParsedMedicationAdministration(model::MedicationAdministration::MedicationAdministration),

  #[serde(rename = "PaymentReconciliation")]
  ParsedPaymentReconciliation(model::PaymentReconciliation::PaymentReconciliation),

  #[serde(rename = "StructureMap")]
  ParsedStructureMap(model::StructureMap::StructureMap),

  #[serde(rename = "RiskEvidenceSynthesis")]
  ParsedRiskEvidenceSynthesis(model::RiskEvidenceSynthesis::RiskEvidenceSynthesis),

  #[serde(rename = "EnrollmentResponse")]
  ParsedEnrollmentResponse(model::EnrollmentResponse::EnrollmentResponse),

  #[serde(rename = "MedicationRequest")]
  ParsedMedicationRequest(model::MedicationRequest::MedicationRequest),

  #[serde(rename = "Subscription")]
  ParsedSubscription(model::Subscription::Subscription),

  #[serde(rename = "Observation")]
  ParsedObservation(model::Observation::Observation),

  #[serde(rename = "ValueSet")]
  ParsedValueSet(model::ValueSet::ValueSet),

  #[serde(rename = "MedicationDispense")]
  ParsedMedicationDispense(model::MedicationDispense::MedicationDispense),

  #[serde(rename = "MedicinalProductInteraction")]
  ParsedMedicinalProductInteraction(model::MedicinalProductInteraction::MedicinalProductInteraction),

  #[serde(rename = "MedicinalProductManufactured")]
  ParsedMedicinalProductManufactured(model::MedicinalProductManufactured::MedicinalProductManufactured),

  #[serde(rename = "MolecularSequence")]
  ParsedMolecularSequence(model::MolecularSequence::MolecularSequence),

  #[serde(rename = "AdverseEvent")]
  ParsedAdverseEvent(model::AdverseEvent::AdverseEvent),

  #[serde(rename = "Binary")]
  ParsedBinary(model::Binary::Binary),

  #[serde(rename = "ClinicalImpression")]
  ParsedClinicalImpression(model::ClinicalImpression::ClinicalImpression),

  #[serde(rename = "Encounter")]
  ParsedEncounter(model::Encounter::Encounter),

  #[serde(rename = "SubstanceReferenceInformation")]
  ParsedSubstanceReferenceInformation(model::SubstanceReferenceInformation::SubstanceReferenceInformation),

  #[serde(rename = "DetectedIssue")]
  ParsedDetectedIssue(model::DetectedIssue::DetectedIssue),

  #[serde(rename = "CoverageEligibilityRequest")]
  ParsedCoverageEligibilityRequest(model::CoverageEligibilityRequest::CoverageEligibilityRequest),

  #[serde(rename = "EffectEvidenceSynthesis")]
  ParsedEffectEvidenceSynthesis(model::EffectEvidenceSynthesis::EffectEvidenceSynthesis),

  #[serde(rename = "Organization")]
  ParsedOrganization(model::Organization::Organization),

  #[serde(rename = "GuidanceResponse")]
  ParsedGuidanceResponse(model::GuidanceResponse::GuidanceResponse),

  #[serde(rename = "MedicinalProductAuthorization")]
  ParsedMedicinalProductAuthorization(model::MedicinalProductAuthorization::MedicinalProductAuthorization),

  #[serde(rename = "Practitioner")]
  ParsedPractitioner(model::Practitioner::Practitioner),

  #[serde(rename = "Library")]
  ParsedLibrary(model::Library::Library),

  #[serde(rename = "DeviceUseStatement")]
  ParsedDeviceUseStatement(model::DeviceUseStatement::DeviceUseStatement),

  #[serde(rename = "Composition")]
  ParsedComposition(model::Composition::Composition),

  #[serde(rename = "Measure")]
  ParsedMeasure(model::Measure::Measure),

  #[serde(rename = "MedicinalProductPharmaceutical")]
  ParsedMedicinalProductPharmaceutical(model::MedicinalProductPharmaceutical::MedicinalProductPharmaceutical),

  #[serde(rename = "Media")]
  ParsedMedia(model::Media::Media),

  #[serde(rename = "Condition")]
  ParsedCondition(model::Condition::Condition),

  #[serde(rename = "MessageHeader")]
  ParsedMessageHeader(model::MessageHeader::MessageHeader),

  #[serde(rename = "SubstanceSpecification")]
  ParsedSubstanceSpecification(model::SubstanceSpecification::SubstanceSpecification),

}