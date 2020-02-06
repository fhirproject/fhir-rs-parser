#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "resourceType")]
pub enum FHIRResource {
  #[serde(rename = "ClinicalImpression")]
  ParsedClinicalImpression(model::ClinicalImpression::ClinicalImpression),

  #[serde(rename = "InsurancePlan")]
  ParsedInsurancePlan(model::InsurancePlan::InsurancePlan),

  #[serde(rename = "Practitioner")]
  ParsedPractitioner(model::Practitioner::Practitioner),

  #[serde(rename = "SubstanceSpecification")]
  ParsedSubstanceSpecification(model::SubstanceSpecification::SubstanceSpecification),

  #[serde(rename = "ChargeItemDefinition")]
  ParsedChargeItemDefinition(model::ChargeItemDefinition::ChargeItemDefinition),

  #[serde(rename = "CoverageEligibilityResponse")]
  ParsedCoverageEligibilityResponse(model::CoverageEligibilityResponse::CoverageEligibilityResponse),

  #[serde(rename = "Flag")]
  ParsedFlag(model::Flag::Flag),

  #[serde(rename = "OperationDefinition")]
  ParsedOperationDefinition(model::OperationDefinition::OperationDefinition),

  #[serde(rename = "SearchParameter")]
  ParsedSearchParameter(model::SearchParameter::SearchParameter),

  #[serde(rename = "Observation")]
  ParsedObservation(model::Observation::Observation),

  #[serde(rename = "SubstanceProtein")]
  ParsedSubstanceProtein(model::SubstanceProtein::SubstanceProtein),

  #[serde(rename = "OrganizationAffiliation")]
  ParsedOrganizationAffiliation(model::OrganizationAffiliation::OrganizationAffiliation),

  #[serde(rename = "PractitionerRole")]
  ParsedPractitionerRole(model::PractitionerRole::PractitionerRole),

  #[serde(rename = "NutritionOrder")]
  ParsedNutritionOrder(model::NutritionOrder::NutritionOrder),

  #[serde(rename = "RiskAssessment")]
  ParsedRiskAssessment(model::RiskAssessment::RiskAssessment),

  #[serde(rename = "AdverseEvent")]
  ParsedAdverseEvent(model::AdverseEvent::AdverseEvent),

  #[serde(rename = "RiskEvidenceSynthesis")]
  ParsedRiskEvidenceSynthesis(model::RiskEvidenceSynthesis::RiskEvidenceSynthesis),

  #[serde(rename = "ResearchStudy")]
  ParsedResearchStudy(model::ResearchStudy::ResearchStudy),

  #[serde(rename = "Slot")]
  ParsedSlot(model::Slot::Slot),

  #[serde(rename = "MeasureReport")]
  ParsedMeasureReport(model::MeasureReport::MeasureReport),

  #[serde(rename = "PaymentNotice")]
  ParsedPaymentNotice(model::PaymentNotice::PaymentNotice),

  #[serde(rename = "Provenance")]
  ParsedProvenance(model::Provenance::Provenance),

  #[serde(rename = "CareTeam")]
  ParsedCareTeam(model::CareTeam::CareTeam),

  #[serde(rename = "Library")]
  ParsedLibrary(model::Library::Library),

  #[serde(rename = "Procedure")]
  ParsedProcedure(model::Procedure::Procedure),

  #[serde(rename = "Coverage")]
  ParsedCoverage(model::Coverage::Coverage),

  #[serde(rename = "Questionnaire")]
  ParsedQuestionnaire(model::Questionnaire::Questionnaire),

  #[serde(rename = "Organization")]
  ParsedOrganization(model::Organization::Organization),

  #[serde(rename = "TestReport")]
  ParsedTestReport(model::TestReport::TestReport),

  #[serde(rename = "Contract")]
  ParsedContract(model::Contract::Contract),

  #[serde(rename = "MessageDefinition")]
  ParsedMessageDefinition(model::MessageDefinition::MessageDefinition),

  #[serde(rename = "MedicinalProductPharmaceutical")]
  ParsedMedicinalProductPharmaceutical(model::MedicinalProductPharmaceutical::MedicinalProductPharmaceutical),

  #[serde(rename = "EvidenceVariable")]
  ParsedEvidenceVariable(model::EvidenceVariable::EvidenceVariable),

  #[serde(rename = "RelatedPerson")]
  ParsedRelatedPerson(model::RelatedPerson::RelatedPerson),

  #[serde(rename = "SupplyRequest")]
  ParsedSupplyRequest(model::SupplyRequest::SupplyRequest),

  #[serde(rename = "Measure")]
  ParsedMeasure(model::Measure::Measure),

  #[serde(rename = "MedicationAdministration")]
  ParsedMedicationAdministration(model::MedicationAdministration::MedicationAdministration),

  #[serde(rename = "DeviceRequest")]
  ParsedDeviceRequest(model::DeviceRequest::DeviceRequest),

  #[serde(rename = "CapabilityStatement")]
  ParsedCapabilityStatement(model::CapabilityStatement::CapabilityStatement),

  #[serde(rename = "HealthcareService")]
  ParsedHealthcareService(model::HealthcareService::HealthcareService),

  #[serde(rename = "CommunicationRequest")]
  ParsedCommunicationRequest(model::CommunicationRequest::CommunicationRequest),

  #[serde(rename = "ExampleScenario")]
  ParsedExampleScenario(model::ExampleScenario::ExampleScenario),

  #[serde(rename = "Subscription")]
  ParsedSubscription(model::Subscription::Subscription),

  #[serde(rename = "MedicinalProduct")]
  ParsedMedicinalProduct(model::MedicinalProduct::MedicinalProduct),

  #[serde(rename = "CompartmentDefinition")]
  ParsedCompartmentDefinition(model::CompartmentDefinition::CompartmentDefinition),

  #[serde(rename = "Binary")]
  ParsedBinary(model::Binary::Binary),

  #[serde(rename = "BodyStructure")]
  ParsedBodyStructure(model::BodyStructure::BodyStructure),

  #[serde(rename = "ValueSet")]
  ParsedValueSet(model::ValueSet::ValueSet),

  #[serde(rename = "Appointment")]
  ParsedAppointment(model::Appointment::Appointment),

  #[serde(rename = "Communication")]
  ParsedCommunication(model::Communication::Communication),

  #[serde(rename = "Goal")]
  ParsedGoal(model::Goal::Goal),

  #[serde(rename = "DocumentReference")]
  ParsedDocumentReference(model::DocumentReference::DocumentReference),

  #[serde(rename = "EnrollmentResponse")]
  ParsedEnrollmentResponse(model::EnrollmentResponse::EnrollmentResponse),

  #[serde(rename = "Consent")]
  ParsedConsent(model::Consent::Consent),

  #[serde(rename = "SubstanceNucleicAcid")]
  ParsedSubstanceNucleicAcid(model::SubstanceNucleicAcid::SubstanceNucleicAcid),

  #[serde(rename = "ClaimResponse")]
  ParsedClaimResponse(model::ClaimResponse::ClaimResponse),

  #[serde(rename = "FamilyMemberHistory")]
  ParsedFamilyMemberHistory(model::FamilyMemberHistory::FamilyMemberHistory),

  #[serde(rename = "MedicinalProductIngredient")]
  ParsedMedicinalProductIngredient(model::MedicinalProductIngredient::MedicinalProductIngredient),

  #[serde(rename = "Task")]
  ParsedTask(model::Task::Task),

  #[serde(rename = "OperationOutcome")]
  ParsedOperationOutcome(model::OperationOutcome::OperationOutcome),

  #[serde(rename = "ObservationDefinition")]
  ParsedObservationDefinition(model::ObservationDefinition::ObservationDefinition),

  #[serde(rename = "GraphDefinition")]
  ParsedGraphDefinition(model::GraphDefinition::GraphDefinition),

  #[serde(rename = "MedicationStatement")]
  ParsedMedicationStatement(model::MedicationStatement::MedicationStatement),

  #[serde(rename = "AppointmentResponse")]
  ParsedAppointmentResponse(model::AppointmentResponse::AppointmentResponse),

  #[serde(rename = "MedicinalProductAuthorization")]
  ParsedMedicinalProductAuthorization(model::MedicinalProductAuthorization::MedicinalProductAuthorization),

  #[serde(rename = "SupplyDelivery")]
  ParsedSupplyDelivery(model::SupplyDelivery::SupplyDelivery),

  #[serde(rename = "EventDefinition")]
  ParsedEventDefinition(model::EventDefinition::EventDefinition),

  #[serde(rename = "TestScript")]
  ParsedTestScript(model::TestScript::TestScript),

  #[serde(rename = "DetectedIssue")]
  ParsedDetectedIssue(model::DetectedIssue::DetectedIssue),

  #[serde(rename = "Claim")]
  ParsedClaim(model::Claim::Claim),

  #[serde(rename = "ResearchDefinition")]
  ParsedResearchDefinition(model::ResearchDefinition::ResearchDefinition),

  #[serde(rename = "StructureMap")]
  ParsedStructureMap(model::StructureMap::StructureMap),

  #[serde(rename = "ChargeItem")]
  ParsedChargeItem(model::ChargeItem::ChargeItem),

  #[serde(rename = "MedicinalProductContraindication")]
  ParsedMedicinalProductContraindication(model::MedicinalProductContraindication::MedicinalProductContraindication),

  #[serde(rename = "DocumentManifest")]
  ParsedDocumentManifest(model::DocumentManifest::DocumentManifest),

  #[serde(rename = "DiagnosticReport")]
  ParsedDiagnosticReport(model::DiagnosticReport::DiagnosticReport),

  #[serde(rename = "ImmunizationRecommendation")]
  ParsedImmunizationRecommendation(model::ImmunizationRecommendation::ImmunizationRecommendation),

  #[serde(rename = "AuditEvent")]
  ParsedAuditEvent(model::AuditEvent::AuditEvent),

  #[serde(rename = "Linkage")]
  ParsedLinkage(model::Linkage::Linkage),

  #[serde(rename = "QuestionnaireResponse")]
  ParsedQuestionnaireResponse(model::QuestionnaireResponse::QuestionnaireResponse),

  #[serde(rename = "BiologicallyDerivedProduct")]
  ParsedBiologicallyDerivedProduct(model::BiologicallyDerivedProduct::BiologicallyDerivedProduct),

  #[serde(rename = "CarePlan")]
  ParsedCarePlan(model::CarePlan::CarePlan),

  #[serde(rename = "Condition")]
  ParsedCondition(model::Condition::Condition),

  #[serde(rename = "EpisodeOfCare")]
  ParsedEpisodeOfCare(model::EpisodeOfCare::EpisodeOfCare),

  #[serde(rename = "Group")]
  ParsedGroup(model::Group::Group),

  #[serde(rename = "MedicinalProductPackaged")]
  ParsedMedicinalProductPackaged(model::MedicinalProductPackaged::MedicinalProductPackaged),

  #[serde(rename = "ExplanationOfBenefit")]
  ParsedExplanationOfBenefit(model::ExplanationOfBenefit::ExplanationOfBenefit),

  #[serde(rename = "Device")]
  ParsedDevice(model::Device::Device),

  #[serde(rename = "Composition")]
  ParsedComposition(model::Composition::Composition),

  #[serde(rename = "MedicinalProductInteraction")]
  ParsedMedicinalProductInteraction(model::MedicinalProductInteraction::MedicinalProductInteraction),

  #[serde(rename = "Basic")]
  ParsedBasic(model::Basic::Basic),

  #[serde(rename = "Specimen")]
  ParsedSpecimen(model::Specimen::Specimen),

  #[serde(rename = "Substance")]
  ParsedSubstance(model::Substance::Substance),

  #[serde(rename = "MedicinalProductIndication")]
  ParsedMedicinalProductIndication(model::MedicinalProductIndication::MedicinalProductIndication),

  #[serde(rename = "Location")]
  ParsedLocation(model::Location::Location),

  #[serde(rename = "DeviceMetric")]
  ParsedDeviceMetric(model::DeviceMetric::DeviceMetric),

  #[serde(rename = "Account")]
  ParsedAccount(model::Account::Account),

  #[serde(rename = "ConceptMap")]
  ParsedConceptMap(model::ConceptMap::ConceptMap),

  #[serde(rename = "DeviceDefinition")]
  ParsedDeviceDefinition(model::DeviceDefinition::DeviceDefinition),

  #[serde(rename = "Parameters")]
  ParsedParameters(model::Parameters::Parameters),

  #[serde(rename = "SpecimenDefinition")]
  ParsedSpecimenDefinition(model::SpecimenDefinition::SpecimenDefinition),

  #[serde(rename = "MedicinalProductUndesirableEffect")]
  ParsedMedicinalProductUndesirableEffect(model::MedicinalProductUndesirableEffect::MedicinalProductUndesirableEffect),

  #[serde(rename = "StructureDefinition")]
  ParsedStructureDefinition(model::StructureDefinition::StructureDefinition),

  #[serde(rename = "ServiceRequest")]
  ParsedServiceRequest(model::ServiceRequest::ServiceRequest),

  #[serde(rename = "VerificationResult")]
  ParsedVerificationResult(model::VerificationResult::VerificationResult),

  #[serde(rename = "MessageHeader")]
  ParsedMessageHeader(model::MessageHeader::MessageHeader),

  #[serde(rename = "EffectEvidenceSynthesis")]
  ParsedEffectEvidenceSynthesis(model::EffectEvidenceSynthesis::EffectEvidenceSynthesis),

  #[serde(rename = "ImagingStudy")]
  ParsedImagingStudy(model::ImagingStudy::ImagingStudy),

  #[serde(rename = "MedicationDispense")]
  ParsedMedicationDispense(model::MedicationDispense::MedicationDispense),

  #[serde(rename = "Medication")]
  ParsedMedication(model::Medication::Medication),

  #[serde(rename = "SubstanceReferenceInformation")]
  ParsedSubstanceReferenceInformation(model::SubstanceReferenceInformation::SubstanceReferenceInformation),

  #[serde(rename = "SubstanceSourceMaterial")]
  ParsedSubstanceSourceMaterial(model::SubstanceSourceMaterial::SubstanceSourceMaterial),

  #[serde(rename = "CatalogEntry")]
  ParsedCatalogEntry(model::CatalogEntry::CatalogEntry),

  #[serde(rename = "ResearchSubject")]
  ParsedResearchSubject(model::ResearchSubject::ResearchSubject),

  #[serde(rename = "PaymentReconciliation")]
  ParsedPaymentReconciliation(model::PaymentReconciliation::PaymentReconciliation),

  #[serde(rename = "ActivityDefinition")]
  ParsedActivityDefinition(model::ActivityDefinition::ActivityDefinition),

  #[serde(rename = "ImplementationGuide")]
  ParsedImplementationGuide(model::ImplementationGuide::ImplementationGuide),

  #[serde(rename = "List")]
  ParsedList(model::List::List),

  #[serde(rename = "AllergyIntolerance")]
  ParsedAllergyIntolerance(model::AllergyIntolerance::AllergyIntolerance),

  #[serde(rename = "Endpoint")]
  ParsedEndpoint(model::Endpoint::Endpoint),

  #[serde(rename = "NamingSystem")]
  ParsedNamingSystem(model::NamingSystem::NamingSystem),

  #[serde(rename = "TerminologyCapabilities")]
  ParsedTerminologyCapabilities(model::TerminologyCapabilities::TerminologyCapabilities),

  #[serde(rename = "Immunization")]
  ParsedImmunization(model::Immunization::Immunization),

  #[serde(rename = "VisionPrescription")]
  ParsedVisionPrescription(model::VisionPrescription::VisionPrescription),

  #[serde(rename = "PlanDefinition")]
  ParsedPlanDefinition(model::PlanDefinition::PlanDefinition),

  #[serde(rename = "Bundle")]
  ParsedBundle(model::Bundle::Bundle),

  #[serde(rename = "Encounter")]
  ParsedEncounter(model::Encounter::Encounter),

  #[serde(rename = "ResearchElementDefinition")]
  ParsedResearchElementDefinition(model::ResearchElementDefinition::ResearchElementDefinition),

  #[serde(rename = "Schedule")]
  ParsedSchedule(model::Schedule::Schedule),

  #[serde(rename = "Invoice")]
  ParsedInvoice(model::Invoice::Invoice),

  #[serde(rename = "Person")]
  ParsedPerson(model::Person::Person),

  #[serde(rename = "GuidanceResponse")]
  ParsedGuidanceResponse(model::GuidanceResponse::GuidanceResponse),

  #[serde(rename = "ImmunizationEvaluation")]
  ParsedImmunizationEvaluation(model::ImmunizationEvaluation::ImmunizationEvaluation),

  #[serde(rename = "MolecularSequence")]
  ParsedMolecularSequence(model::MolecularSequence::MolecularSequence),

  #[serde(rename = "Patient")]
  ParsedPatient(model::Patient::Patient),

  #[serde(rename = "RequestGroup")]
  ParsedRequestGroup(model::RequestGroup::RequestGroup),

  #[serde(rename = "MedicationKnowledge")]
  ParsedMedicationKnowledge(model::MedicationKnowledge::MedicationKnowledge),

  #[serde(rename = "Evidence")]
  ParsedEvidence(model::Evidence::Evidence),

  #[serde(rename = "CoverageEligibilityRequest")]
  ParsedCoverageEligibilityRequest(model::CoverageEligibilityRequest::CoverageEligibilityRequest),

  #[serde(rename = "DeviceUseStatement")]
  ParsedDeviceUseStatement(model::DeviceUseStatement::DeviceUseStatement),

  #[serde(rename = "EnrollmentRequest")]
  ParsedEnrollmentRequest(model::EnrollmentRequest::EnrollmentRequest),

  #[serde(rename = "CodeSystem")]
  ParsedCodeSystem(model::CodeSystem::CodeSystem),

  #[serde(rename = "MedicationRequest")]
  ParsedMedicationRequest(model::MedicationRequest::MedicationRequest),

  #[serde(rename = "MedicinalProductManufactured")]
  ParsedMedicinalProductManufactured(model::MedicinalProductManufactured::MedicinalProductManufactured),

  #[serde(rename = "Media")]
  ParsedMedia(model::Media::Media),

  #[serde(rename = "SubstancePolymer")]
  ParsedSubstancePolymer(model::SubstancePolymer::SubstancePolymer),

}