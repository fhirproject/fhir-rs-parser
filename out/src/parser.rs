#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "resourceType")]
pub enum FHIRResource {
  #[serde(rename = "EffectEvidenceSynthesis")]
  ParsedEffectEvidenceSynthesis(model::EffectEvidenceSynthesis::EffectEvidenceSynthesis),

  #[serde(rename = "Slot")]
  ParsedSlot(model::Slot::Slot),

  #[serde(rename = "Communication")]
  ParsedCommunication(model::Communication::Communication),

  #[serde(rename = "AllergyIntolerance")]
  ParsedAllergyIntolerance(model::AllergyIntolerance::AllergyIntolerance),

  #[serde(rename = "Composition")]
  ParsedComposition(model::Composition::Composition),

  #[serde(rename = "Appointment")]
  ParsedAppointment(model::Appointment::Appointment),

  #[serde(rename = "List")]
  ParsedList(model::List::List),

  #[serde(rename = "Consent")]
  ParsedConsent(model::Consent::Consent),

  #[serde(rename = "CoverageEligibilityRequest")]
  ParsedCoverageEligibilityRequest(model::CoverageEligibilityRequest::CoverageEligibilityRequest),

  #[serde(rename = "Media")]
  ParsedMedia(model::Media::Media),

  #[serde(rename = "Medication")]
  ParsedMedication(model::Medication::Medication),

  #[serde(rename = "EventDefinition")]
  ParsedEventDefinition(model::EventDefinition::EventDefinition),

  #[serde(rename = "MedicinalProductManufactured")]
  ParsedMedicinalProductManufactured(model::MedicinalProductManufactured::MedicinalProductManufactured),

  #[serde(rename = "Subscription")]
  ParsedSubscription(model::Subscription::Subscription),

  #[serde(rename = "Task")]
  ParsedTask(model::Task::Task),

  #[serde(rename = "HealthcareService")]
  ParsedHealthcareService(model::HealthcareService::HealthcareService),

  #[serde(rename = "ValueSet")]
  ParsedValueSet(model::ValueSet::ValueSet),

  #[serde(rename = "MedicinalProductIndication")]
  ParsedMedicinalProductIndication(model::MedicinalProductIndication::MedicinalProductIndication),

  #[serde(rename = "ResearchStudy")]
  ParsedResearchStudy(model::ResearchStudy::ResearchStudy),

  #[serde(rename = "EvidenceVariable")]
  ParsedEvidenceVariable(model::EvidenceVariable::EvidenceVariable),

  #[serde(rename = "DeviceRequest")]
  ParsedDeviceRequest(model::DeviceRequest::DeviceRequest),

  #[serde(rename = "ImagingStudy")]
  ParsedImagingStudy(model::ImagingStudy::ImagingStudy),

  #[serde(rename = "MedicationDispense")]
  ParsedMedicationDispense(model::MedicationDispense::MedicationDispense),

  #[serde(rename = "TerminologyCapabilities")]
  ParsedTerminologyCapabilities(model::TerminologyCapabilities::TerminologyCapabilities),

  #[serde(rename = "Organization")]
  ParsedOrganization(model::Organization::Organization),

  #[serde(rename = "SubstanceReferenceInformation")]
  ParsedSubstanceReferenceInformation(model::SubstanceReferenceInformation::SubstanceReferenceInformation),

  #[serde(rename = "MedicinalProductUndesirableEffect")]
  ParsedMedicinalProductUndesirableEffect(model::MedicinalProductUndesirableEffect::MedicinalProductUndesirableEffect),

  #[serde(rename = "AuditEvent")]
  ParsedAuditEvent(model::AuditEvent::AuditEvent),

  #[serde(rename = "CarePlan")]
  ParsedCarePlan(model::CarePlan::CarePlan),

  #[serde(rename = "Linkage")]
  ParsedLinkage(model::Linkage::Linkage),

  #[serde(rename = "MedicinalProductPackaged")]
  ParsedMedicinalProductPackaged(model::MedicinalProductPackaged::MedicinalProductPackaged),

  #[serde(rename = "ResearchElementDefinition")]
  ParsedResearchElementDefinition(model::ResearchElementDefinition::ResearchElementDefinition),

  #[serde(rename = "RequestGroup")]
  ParsedRequestGroup(model::RequestGroup::RequestGroup),

  #[serde(rename = "Bundle")]
  ParsedBundle(model::Bundle::Bundle),

  #[serde(rename = "MolecularSequence")]
  ParsedMolecularSequence(model::MolecularSequence::MolecularSequence),

  #[serde(rename = "ServiceRequest")]
  ParsedServiceRequest(model::ServiceRequest::ServiceRequest),

  #[serde(rename = "SubstanceSpecification")]
  ParsedSubstanceSpecification(model::SubstanceSpecification::SubstanceSpecification),

  #[serde(rename = "CoverageEligibilityResponse")]
  ParsedCoverageEligibilityResponse(model::CoverageEligibilityResponse::CoverageEligibilityResponse),

  #[serde(rename = "Group")]
  ParsedGroup(model::Group::Group),

  #[serde(rename = "Flag")]
  ParsedFlag(model::Flag::Flag),

  #[serde(rename = "ObservationDefinition")]
  ParsedObservationDefinition(model::ObservationDefinition::ObservationDefinition),

  #[serde(rename = "Measure")]
  ParsedMeasure(model::Measure::Measure),

  #[serde(rename = "SubstanceSourceMaterial")]
  ParsedSubstanceSourceMaterial(model::SubstanceSourceMaterial::SubstanceSourceMaterial),

  #[serde(rename = "SubstanceNucleicAcid")]
  ParsedSubstanceNucleicAcid(model::SubstanceNucleicAcid::SubstanceNucleicAcid),

  #[serde(rename = "GraphDefinition")]
  ParsedGraphDefinition(model::GraphDefinition::GraphDefinition),

  #[serde(rename = "ResearchDefinition")]
  ParsedResearchDefinition(model::ResearchDefinition::ResearchDefinition),

  #[serde(rename = "StructureDefinition")]
  ParsedStructureDefinition(model::StructureDefinition::StructureDefinition),

  #[serde(rename = "MessageHeader")]
  ParsedMessageHeader(model::MessageHeader::MessageHeader),

  #[serde(rename = "SubstancePolymer")]
  ParsedSubstancePolymer(model::SubstancePolymer::SubstancePolymer),

  #[serde(rename = "InsurancePlan")]
  ParsedInsurancePlan(model::InsurancePlan::InsurancePlan),

  #[serde(rename = "ChargeItemDefinition")]
  ParsedChargeItemDefinition(model::ChargeItemDefinition::ChargeItemDefinition),

  #[serde(rename = "MedicinalProduct")]
  ParsedMedicinalProduct(model::MedicinalProduct::MedicinalProduct),

  #[serde(rename = "ResearchSubject")]
  ParsedResearchSubject(model::ResearchSubject::ResearchSubject),

  #[serde(rename = "DeviceDefinition")]
  ParsedDeviceDefinition(model::DeviceDefinition::DeviceDefinition),

  #[serde(rename = "Questionnaire")]
  ParsedQuestionnaire(model::Questionnaire::Questionnaire),

  #[serde(rename = "SearchParameter")]
  ParsedSearchParameter(model::SearchParameter::SearchParameter),

  #[serde(rename = "Invoice")]
  ParsedInvoice(model::Invoice::Invoice),

  #[serde(rename = "GuidanceResponse")]
  ParsedGuidanceResponse(model::GuidanceResponse::GuidanceResponse),

  #[serde(rename = "ImplementationGuide")]
  ParsedImplementationGuide(model::ImplementationGuide::ImplementationGuide),

  #[serde(rename = "Encounter")]
  ParsedEncounter(model::Encounter::Encounter),

  #[serde(rename = "CompartmentDefinition")]
  ParsedCompartmentDefinition(model::CompartmentDefinition::CompartmentDefinition),

  #[serde(rename = "MedicinalProductInteraction")]
  ParsedMedicinalProductInteraction(model::MedicinalProductInteraction::MedicinalProductInteraction),

  #[serde(rename = "TestReport")]
  ParsedTestReport(model::TestReport::TestReport),

  #[serde(rename = "Evidence")]
  ParsedEvidence(model::Evidence::Evidence),

  #[serde(rename = "VerificationResult")]
  ParsedVerificationResult(model::VerificationResult::VerificationResult),

  #[serde(rename = "MedicationStatement")]
  ParsedMedicationStatement(model::MedicationStatement::MedicationStatement),

  #[serde(rename = "BodyStructure")]
  ParsedBodyStructure(model::BodyStructure::BodyStructure),

  #[serde(rename = "ClaimResponse")]
  ParsedClaimResponse(model::ClaimResponse::ClaimResponse),

  #[serde(rename = "Location")]
  ParsedLocation(model::Location::Location),

  #[serde(rename = "OperationDefinition")]
  ParsedOperationDefinition(model::OperationDefinition::OperationDefinition),

  #[serde(rename = "ClinicalImpression")]
  ParsedClinicalImpression(model::ClinicalImpression::ClinicalImpression),

  #[serde(rename = "SupplyDelivery")]
  ParsedSupplyDelivery(model::SupplyDelivery::SupplyDelivery),

  #[serde(rename = "DeviceUseStatement")]
  ParsedDeviceUseStatement(model::DeviceUseStatement::DeviceUseStatement),

  #[serde(rename = "Condition")]
  ParsedCondition(model::Condition::Condition),

  #[serde(rename = "CatalogEntry")]
  ParsedCatalogEntry(model::CatalogEntry::CatalogEntry),

  #[serde(rename = "Immunization")]
  ParsedImmunization(model::Immunization::Immunization),

  #[serde(rename = "DocumentManifest")]
  ParsedDocumentManifest(model::DocumentManifest::DocumentManifest),

  #[serde(rename = "MedicinalProductPharmaceutical")]
  ParsedMedicinalProductPharmaceutical(model::MedicinalProductPharmaceutical::MedicinalProductPharmaceutical),

  #[serde(rename = "DocumentReference")]
  ParsedDocumentReference(model::DocumentReference::DocumentReference),

  #[serde(rename = "Account")]
  ParsedAccount(model::Account::Account),

  #[serde(rename = "SpecimenDefinition")]
  ParsedSpecimenDefinition(model::SpecimenDefinition::SpecimenDefinition),

  #[serde(rename = "MeasureReport")]
  ParsedMeasureReport(model::MeasureReport::MeasureReport),

  #[serde(rename = "RiskEvidenceSynthesis")]
  ParsedRiskEvidenceSynthesis(model::RiskEvidenceSynthesis::RiskEvidenceSynthesis),

  #[serde(rename = "Specimen")]
  ParsedSpecimen(model::Specimen::Specimen),

  #[serde(rename = "OrganizationAffiliation")]
  ParsedOrganizationAffiliation(model::OrganizationAffiliation::OrganizationAffiliation),

  #[serde(rename = "NamingSystem")]
  ParsedNamingSystem(model::NamingSystem::NamingSystem),

  #[serde(rename = "AppointmentResponse")]
  ParsedAppointmentResponse(model::AppointmentResponse::AppointmentResponse),

  #[serde(rename = "MedicinalProductAuthorization")]
  ParsedMedicinalProductAuthorization(model::MedicinalProductAuthorization::MedicinalProductAuthorization),

  #[serde(rename = "MedicinalProductIngredient")]
  ParsedMedicinalProductIngredient(model::MedicinalProductIngredient::MedicinalProductIngredient),

  #[serde(rename = "QuestionnaireResponse")]
  ParsedQuestionnaireResponse(model::QuestionnaireResponse::QuestionnaireResponse),

  #[serde(rename = "Coverage")]
  ParsedCoverage(model::Coverage::Coverage),

  #[serde(rename = "MedicationKnowledge")]
  ParsedMedicationKnowledge(model::MedicationKnowledge::MedicationKnowledge),

  #[serde(rename = "OperationOutcome")]
  ParsedOperationOutcome(model::OperationOutcome::OperationOutcome),

  #[serde(rename = "Provenance")]
  ParsedProvenance(model::Provenance::Provenance),

  #[serde(rename = "ChargeItem")]
  ParsedChargeItem(model::ChargeItem::ChargeItem),

  #[serde(rename = "DetectedIssue")]
  ParsedDetectedIssue(model::DetectedIssue::DetectedIssue),

  #[serde(rename = "Substance")]
  ParsedSubstance(model::Substance::Substance),

  #[serde(rename = "SupplyRequest")]
  ParsedSupplyRequest(model::SupplyRequest::SupplyRequest),

  #[serde(rename = "Goal")]
  ParsedGoal(model::Goal::Goal),

  #[serde(rename = "ImmunizationRecommendation")]
  ParsedImmunizationRecommendation(model::ImmunizationRecommendation::ImmunizationRecommendation),

  #[serde(rename = "ConceptMap")]
  ParsedConceptMap(model::ConceptMap::ConceptMap),

  #[serde(rename = "Observation")]
  ParsedObservation(model::Observation::Observation),

  #[serde(rename = "BiologicallyDerivedProduct")]
  ParsedBiologicallyDerivedProduct(model::BiologicallyDerivedProduct::BiologicallyDerivedProduct),

  #[serde(rename = "Patient")]
  ParsedPatient(model::Patient::Patient),

  #[serde(rename = "Practitioner")]
  ParsedPractitioner(model::Practitioner::Practitioner),

  #[serde(rename = "EnrollmentResponse")]
  ParsedEnrollmentResponse(model::EnrollmentResponse::EnrollmentResponse),

  #[serde(rename = "NutritionOrder")]
  ParsedNutritionOrder(model::NutritionOrder::NutritionOrder),

  #[serde(rename = "DiagnosticReport")]
  ParsedDiagnosticReport(model::DiagnosticReport::DiagnosticReport),

  #[serde(rename = "StructureMap")]
  ParsedStructureMap(model::StructureMap::StructureMap),

  #[serde(rename = "EpisodeOfCare")]
  ParsedEpisodeOfCare(model::EpisodeOfCare::EpisodeOfCare),

  #[serde(rename = "RelatedPerson")]
  ParsedRelatedPerson(model::RelatedPerson::RelatedPerson),

  #[serde(rename = "MedicationRequest")]
  ParsedMedicationRequest(model::MedicationRequest::MedicationRequest),

  #[serde(rename = "SubstanceProtein")]
  ParsedSubstanceProtein(model::SubstanceProtein::SubstanceProtein),

  #[serde(rename = "ImmunizationEvaluation")]
  ParsedImmunizationEvaluation(model::ImmunizationEvaluation::ImmunizationEvaluation),

  #[serde(rename = "Basic")]
  ParsedBasic(model::Basic::Basic),

  #[serde(rename = "Contract")]
  ParsedContract(model::Contract::Contract),

  #[serde(rename = "Claim")]
  ParsedClaim(model::Claim::Claim),

  #[serde(rename = "Binary")]
  ParsedBinary(model::Binary::Binary),

  #[serde(rename = "MedicationAdministration")]
  ParsedMedicationAdministration(model::MedicationAdministration::MedicationAdministration),

  #[serde(rename = "Library")]
  ParsedLibrary(model::Library::Library),

  #[serde(rename = "RiskAssessment")]
  ParsedRiskAssessment(model::RiskAssessment::RiskAssessment),

  #[serde(rename = "Schedule")]
  ParsedSchedule(model::Schedule::Schedule),

  #[serde(rename = "ExampleScenario")]
  ParsedExampleScenario(model::ExampleScenario::ExampleScenario),

  #[serde(rename = "PractitionerRole")]
  ParsedPractitionerRole(model::PractitionerRole::PractitionerRole),

  #[serde(rename = "VisionPrescription")]
  ParsedVisionPrescription(model::VisionPrescription::VisionPrescription),

  #[serde(rename = "Parameters")]
  ParsedParameters(model::Parameters::Parameters),

  #[serde(rename = "Person")]
  ParsedPerson(model::Person::Person),

  #[serde(rename = "ActivityDefinition")]
  ParsedActivityDefinition(model::ActivityDefinition::ActivityDefinition),

  #[serde(rename = "Device")]
  ParsedDevice(model::Device::Device),

  #[serde(rename = "MedicinalProductContraindication")]
  ParsedMedicinalProductContraindication(model::MedicinalProductContraindication::MedicinalProductContraindication),

  #[serde(rename = "CommunicationRequest")]
  ParsedCommunicationRequest(model::CommunicationRequest::CommunicationRequest),

  #[serde(rename = "CareTeam")]
  ParsedCareTeam(model::CareTeam::CareTeam),

  #[serde(rename = "PaymentReconciliation")]
  ParsedPaymentReconciliation(model::PaymentReconciliation::PaymentReconciliation),

  #[serde(rename = "PlanDefinition")]
  ParsedPlanDefinition(model::PlanDefinition::PlanDefinition),

  #[serde(rename = "TestScript")]
  ParsedTestScript(model::TestScript::TestScript),

  #[serde(rename = "PaymentNotice")]
  ParsedPaymentNotice(model::PaymentNotice::PaymentNotice),

  #[serde(rename = "EnrollmentRequest")]
  ParsedEnrollmentRequest(model::EnrollmentRequest::EnrollmentRequest),

  #[serde(rename = "Procedure")]
  ParsedProcedure(model::Procedure::Procedure),

  #[serde(rename = "ExplanationOfBenefit")]
  ParsedExplanationOfBenefit(model::ExplanationOfBenefit::ExplanationOfBenefit),

  #[serde(rename = "FamilyMemberHistory")]
  ParsedFamilyMemberHistory(model::FamilyMemberHistory::FamilyMemberHistory),

  #[serde(rename = "Endpoint")]
  ParsedEndpoint(model::Endpoint::Endpoint),

  #[serde(rename = "MessageDefinition")]
  ParsedMessageDefinition(model::MessageDefinition::MessageDefinition),

  #[serde(rename = "CodeSystem")]
  ParsedCodeSystem(model::CodeSystem::CodeSystem),

  #[serde(rename = "AdverseEvent")]
  ParsedAdverseEvent(model::AdverseEvent::AdverseEvent),

  #[serde(rename = "DeviceMetric")]
  ParsedDeviceMetric(model::DeviceMetric::DeviceMetric),

  #[serde(rename = "CapabilityStatement")]
  ParsedCapabilityStatement(model::CapabilityStatement::CapabilityStatement),

}