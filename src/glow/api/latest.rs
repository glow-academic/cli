// Auto-generated from GLOW API OpenAPI spec v2.15.38
// Do not edit manually — regenerated on each API release.
// Schemas: 1377

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityHistoryResponse {
    #[serde(default)]
    pub items: Option<Vec<SessionListItem>>,
    #[serde(default)]
    pub total_count: Option<i64>,
    #[serde(default)]
    pub page: Option<i64>,
    #[serde(default)]
    pub page_size: Option<i64>,
    #[serde(default)]
    pub total_pages: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityProblemItem {
    pub problem_id: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub message: String,
    pub created_at: String,
    #[serde(default)]
    pub profile_id: Option<String>,
    #[serde(default)]
    pub profile_name: Option<String>,
    #[serde(default)]
    pub session_id: Option<String>,
    #[serde(default)]
    pub resolved: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityRequest {
    #[serde(default)]
    pub date_from: Option<String>,
    #[serde(default)]
    pub date_to: Option<String>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub role_ids: Option<Vec<String>>,
    #[serde(default)]
    pub page_limit: Option<i64>,
    #[serde(default)]
    pub page_offset: Option<i64>,
    #[serde(default)]
    pub summary_profile_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityResources {
    #[serde(default)]
    pub profiles: Option<std::collections::HashMap<String, std::collections::HashMap<String, serde_json::Value>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityResponse {
    #[serde(default)]
    pub sessions_count: Option<i64>,
    #[serde(default)]
    pub active_profiles_count: Option<i64>,
    #[serde(default)]
    pub logins_count: Option<i64>,
    #[serde(default)]
    pub emulations_count: Option<i64>,
    #[serde(default)]
    pub profile_summary: Option<Vec<ProfileSummaryItem>>,
    #[serde(default)]
    pub problems: Option<Vec<ActivityProblemItem>>,
    #[serde(default)]
    pub resources: Option<ActivityResources>,
    #[serde(default)]
    pub analytics: Option<AnalyticsFacets>,
    #[serde(default)]
    pub history: Option<ActivityHistoryResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentDepartmentResource {
    #[serde(default)]
    pub department_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentDescriptionResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentFieldError {
    pub field: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentFlagResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub value: Option<bool>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub icon_id: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentInstructionResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub template: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentModelResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub value: Option<String>,
    #[serde(default)]
    pub provider_id: Option<String>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub temperature_level_ids: Option<Vec<String>>,
    #[serde(default)]
    pub reasoning_level_ids: Option<Vec<String>>,
    #[serde(default)]
    pub quality_ids: Option<Vec<String>>,
    #[serde(default)]
    pub voice_ids: Option<Vec<String>>,
    #[serde(default)]
    pub modality_ids: Option<Vec<String>>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentNameResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentPromptResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub system_prompt: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentQualityResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub quality: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentReasoningLevelResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub reasoning_level: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentResultItem {
    pub success: bool,
    pub message: String,
    #[serde(default)]
    pub agent_id: Option<String>,
    #[serde(default)]
    pub errors: Option<Vec<AgentFieldError>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentRubricResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub total_points: Option<i64>,
    #[serde(default)]
    pub pass_points: Option<i64>,
    #[serde(default)]
    pub simulation_rubric: Option<bool>,
    #[serde(default)]
    pub video_rubric: Option<bool>,
    #[serde(default)]
    pub standard_group_ids: Option<Vec<String>>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentTemperatureLevelResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub temperature: Option<f64>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentToolResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub permission_ids: Option<Vec<String>>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub args_ids: Option<Vec<String>>,
    #[serde(default)]
    pub args_output_ids: Option<Vec<String>>,
    #[serde(default)]
    pub instruction_id: Option<String>,
    #[serde(default)]
    pub agent_id: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentVoiceResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub voice: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatedResults {
    #[serde(default)]
    pub total_score: Option<f64>,
    #[serde(default)]
    pub total_possible_points: Option<f64>,
    #[serde(default)]
    pub percentage: Option<f64>,
    #[serde(default)]
    pub passed: Option<bool>,
    #[serde(default)]
    pub chats_completed: Option<i64>,
    #[serde(default)]
    pub total_chats: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisEntry {
    #[serde(default)]
    pub content: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsFacets {
    pub fields: AnalyticsFilterFields,
    #[serde(default)]
    pub department_options: Option<Vec<AnalyticsFilterOption>>,
    #[serde(default)]
    pub cohort_options: Option<Vec<AnalyticsFilterOption>>,
    #[serde(default)]
    pub role_options: Option<Vec<AnalyticsRoleOption>>,
    #[serde(default)]
    pub attempt_options: Option<Vec<String>>,
    #[serde(default)]
    pub date_range_earliest: Option<String>,
    #[serde(default)]
    pub date_range_latest: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsFilterField {
    #[serde(default)]
    pub visible: Option<bool>,
    #[serde(default)]
    pub disabled: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsFilterFields {
    #[serde(default)]
    pub date_range: Option<AnalyticsFilterField>,
    #[serde(default)]
    pub departments: Option<AnalyticsFilterField>,
    #[serde(default)]
    pub cohorts: Option<AnalyticsFilterField>,
    #[serde(default)]
    pub roles: Option<AnalyticsFilterField>,
    #[serde(default)]
    pub attempts: Option<AnalyticsFilterField>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsFilterOption {
    pub value: String,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsRoleOption {
    pub value: String,
    pub label: String,
    pub id: String,
    pub name: String,
    pub level: i64,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub icon_id: Option<String>,
    #[serde(default)]
    pub color_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchiveAttemptsRequest {
    pub archived: bool,
    #[serde(default)]
    pub attempt_ids: Option<Vec<String>>,
    #[serde(default)]
    pub start_date: Option<String>,
    #[serde(default)]
    pub end_date: Option<String>,
    #[serde(default)]
    pub cohort_ids: Option<Vec<String>>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub simulation_ids: Option<Vec<String>>,
    #[serde(default)]
    pub scenario_ids: Option<Vec<String>>,
    #[serde(default)]
    pub profile_ids_filter: Option<Vec<String>>,
    #[serde(default)]
    pub infinite_mode: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchiveAttemptsResponse {
    #[serde(default)]
    pub updated_count: Option<i64>,
    #[serde(default)]
    pub profile_ids_to_invalidate: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchiveTestsRequest {
    pub test_ids: Vec<String>,
    #[serde(default)]
    pub archived: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchiveTestsResponse {
    #[serde(default)]
    pub updated_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactGenerateRequest {
    #[serde(default)]
    pub instructions: Option<Vec<String>>,
    #[serde(default)]
    pub config: Option<GenerateConfig>,
    #[serde(default)]
    pub modalities: Option<Vec<String>>,
    #[serde(default)]
    pub audios_id: Option<String>,
    #[serde(default)]
    pub conversation_id: Option<String>,
    #[serde(default)]
    pub trace_id: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
    #[serde(default)]
    pub wait_for_complete: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactGenerateResponse {
    pub group_id: String,
    #[serde(default)]
    pub run_id: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub eval: Option<EvalSetup>,
    #[serde(default)]
    pub produced_media: Option<Vec<ProducedMedia>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactSessionGroup {
    pub group_id: String,
    #[serde(default)]
    pub group_name: Option<String>,
    #[serde(default)]
    pub first_run_at: Option<String>,
    #[serde(default)]
    pub last_run_at: Option<String>,
    #[serde(default)]
    pub run_count: Option<i64>,
    #[serde(default)]
    pub total_tokens: Option<i64>,
    #[serde(default)]
    pub total_cost: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptCompleteRequest {
    pub attempt_id: String,
    #[serde(default)]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptCompleteResponse {
    pub success: bool,
    pub completion_id: String,
    pub attempt_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptData {
    pub id: String,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub infinite_mode: Option<bool>,
    #[serde(default)]
    pub profile_id: Option<String>,
    #[serde(default)]
    pub profile_name: Option<String>,
    #[serde(default)]
    pub user_persona_id: Option<String>,
    #[serde(default)]
    pub department_id: Option<String>,
    #[serde(default)]
    pub cohort_id: Option<String>,
    #[serde(default)]
    pub is_archived: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptEntries {
    #[serde(default)]
    pub attempt: Option<Vec<GetAttemptResponse>>,
    #[serde(default)]
    pub attempt_chat: Option<Vec<ChatData>>,
    #[serde(default)]
    pub attempt_message: Option<Vec<MessageData>>,
    #[serde(default)]
    pub runs: Option<GetRunListViewResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptOptionValue {
    pub option_text: String,
    #[serde(default)]
    pub is_correct: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptQuestionValue {
    pub question_text: String,
    #[serde(default)]
    pub allow_multiple: Option<bool>,
    #[serde(default)]
    pub options: Option<Vec<AttemptOptionValue>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptResources {
    #[serde(default)]
    pub scenarios: Option<std::collections::HashMap<String, ScenarioEntry>>,
    #[serde(default)]
    pub personas: Option<std::collections::HashMap<String, PersonaEntry>>,
    #[serde(default)]
    pub documents: Option<std::collections::HashMap<String, DocumentEntry>>,
    #[serde(default)]
    pub images: Option<std::collections::HashMap<String, ImageEntry>>,
    #[serde(default)]
    pub videos: Option<std::collections::HashMap<String, VideoEntry>>,
    #[serde(default)]
    pub objectives: Option<std::collections::HashMap<String, ObjectiveEntry>>,
    #[serde(default)]
    pub questions: Option<std::collections::HashMap<String, QuestionEntry>>,
    #[serde(default)]
    pub options: Option<std::collections::HashMap<String, OptionEntry>>,
    #[serde(default)]
    pub problem_statements: Option<std::collections::HashMap<String, ProblemStatementEntry>>,
    #[serde(default)]
    pub rubrics: Option<std::collections::HashMap<String, RubricEntry>>,
    #[serde(default)]
    pub standard_groups: Option<std::collections::HashMap<String, StandardGroupEntry>>,
    #[serde(default)]
    pub standards: Option<std::collections::HashMap<String, StandardEntry>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptResponsePayload {
    pub chat_id: String,
    pub question_id: String,
    pub option_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptStartRequest {
    #[serde(default)]
    pub home_id: Option<String>,
    #[serde(default)]
    pub practice_id: Option<String>,
    #[serde(default)]
    pub infinite_mode: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptStartResponse {
    pub attempt_id: String,
    pub chat_id: String,
    #[serde(default)]
    pub department_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptStopRequest {
    pub group_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptStopResponse {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioDownloadAttemptApiRequest {
    pub audio_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioDownloadGroupApiRequest {
    pub audio_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioStartInternalResult {
    pub chat_id: String,
    pub attempt_id: String,
    pub conversation_id: String,
    pub group_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioStopInternalResult {
    pub chat_id: String,
    #[serde(default)]
    pub stopped: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioUploadAttemptApiResponse {
    pub audio_id: String,
    pub audios_id: String,
    pub upload_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthDepartmentResource {
    #[serde(default)]
    pub department_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthDescriptionResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthFieldError {
    pub field: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthFlagResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub value: Option<bool>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub icon_id: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthItemResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub position: Option<i64>,
    #[serde(default)]
    pub encrypted: Option<bool>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthNameResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthProtocolResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub value: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResultItem {
    pub success: bool,
    pub message: String,
    #[serde(default)]
    pub auth_id: Option<String>,
    #[serde(default)]
    pub errors: Option<Vec<AuthFieldError>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthSlugResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub value: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailableContinuationOptions {
    pub options: Vec<ContinuationOption>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkDepartmentItem {
    pub department_id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkEvalOperational {
    pub eval_id: String,
    #[serde(default)]
    pub eval_name: Option<String>,
    #[serde(default)]
    pub eval_description: Option<String>,
    #[serde(default)]
    pub model_ids: Option<Vec<String>>,
    #[serde(default)]
    pub total_tests: Option<i64>,
    #[serde(default)]
    pub archived_tests: Option<i64>,
    #[serde(default)]
    pub total_invocations: Option<i64>,
    #[serde(default)]
    pub completed_invocations: Option<i64>,
    #[serde(default)]
    pub highest_score: Option<f64>,
    #[serde(default)]
    pub has_passed: Option<bool>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub infinite_mode: Option<bool>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub rubric_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkHistoryItem {
    pub test_id: String,
    #[serde(default)]
    pub date: Option<String>,
    #[serde(default)]
    pub profile_id: Option<String>,
    #[serde(default)]
    pub profile_name: Option<String>,
    #[serde(default)]
    pub eval_id: Option<String>,
    #[serde(default)]
    pub eval_name: Option<String>,
    #[serde(default)]
    pub rubric_id: Option<String>,
    #[serde(default)]
    pub rubric_name: Option<String>,
    #[serde(default)]
    pub num_models: Option<i64>,
    #[serde(default)]
    pub num_models_completed: Option<i64>,
    #[serde(default)]
    pub model_ids: Option<Vec<String>>,
    #[serde(default)]
    pub model_names: Option<Vec<String>>,
    #[serde(default)]
    pub score: Option<i64>,
    #[serde(default)]
    pub score_status: Option<String>,
    #[serde(default)]
    pub pass_pct: Option<i64>,
    #[serde(default)]
    pub show_view: Option<bool>,
    #[serde(default)]
    pub show_continue: Option<bool>,
    #[serde(default)]
    pub is_archived: Option<bool>,
    #[serde(default)]
    pub infinite_mode: Option<bool>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkHistoryResponse {
    #[serde(default)]
    pub data: Option<Vec<BenchmarkHistoryItem>>,
    #[serde(default)]
    pub total_count: Option<i64>,
    #[serde(default)]
    pub page: Option<i64>,
    #[serde(default)]
    pub page_size: Option<i64>,
    #[serde(default)]
    pub eval_options: Option<Vec<FilterOption>>,
    #[serde(default)]
    pub model_options: Option<Vec<FilterOption>>,
    #[serde(default)]
    pub profile_options: Option<Vec<FilterOption>>,
    #[serde(default)]
    pub rubric_options: Option<Vec<FilterOption>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkRequest {
    #[serde(default)]
    pub start_date: Option<String>,
    #[serde(default)]
    pub end_date: Option<String>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub history_page: Option<i64>,
    #[serde(default)]
    pub history_page_size: Option<i64>,
    #[serde(default)]
    pub history_eval_ids: Option<Vec<String>>,
    #[serde(default)]
    pub history_search: Option<String>,
    #[serde(default)]
    pub history_archived: Option<bool>,
    #[serde(default)]
    pub history_sort_by: Option<String>,
    #[serde(default)]
    pub history_sort_order: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResponse {
    #[serde(default)]
    pub evals: Option<Vec<BenchmarkEvalOperational>>,
    #[serde(default)]
    pub departments: Option<Vec<BenchmarkDepartmentItem>>,
    #[serde(default)]
    pub department_options: Option<Vec<FilterOption>>,
    #[serde(default)]
    pub date_range_earliest: Option<String>,
    #[serde(default)]
    pub date_range_latest: Option<String>,
    #[serde(default)]
    pub history: Option<BenchmarkHistoryResponse>,
    #[serde(default)]
    pub analytics: Option<AnalyticsFacets>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body_parse_agent_csv_agent_csv_post {
    pub file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body_parse_auth_csv_auth_csv_post {
    pub file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body_parse_cohort_csv_cohort_csv_post {
    pub file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body_parse_department_csv_department_csv_post {
    pub file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body_parse_document_csv_document_csv_post {
    pub file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body_parse_eval_csv_eval_csv_post {
    pub file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body_parse_field_csv_field_csv_post {
    pub file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body_parse_model_csv_model_csv_post {
    pub file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body_parse_parameter_csv_parameter_csv_post {
    pub file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body_parse_persona_csv_persona_csv_post {
    pub file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body_parse_profile_csv_profile_csv_post {
    pub file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body_parse_provider_csv_provider_csv_post {
    pub file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body_parse_rubric_csv_rubric_csv_post {
    pub file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body_parse_scenario_csv_scenario_csv_post {
    pub file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body_parse_setting_csv_setting_csv_post {
    pub file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body_parse_simulation_csv_simulation_csv_post {
    pub file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body_parse_tool_csv_tool_csv_post {
    pub file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body_upload_audio_attempt_audio_upload_post {
    #[serde(default)]
    pub file: Option<String>,
    #[serde(default)]
    pub length_seconds: Option<i64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body_upload_file_document_file_upload_post {
    pub file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body_upload_image_scenario_image_upload_post {
    pub file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body_upload_text_document_text_upload_post {
    pub file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body_upload_video_scenario_video_upload_post {
    pub file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallDownloadAgentApiRequest {
    pub call_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallDownloadAttemptApiRequest {
    pub call_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallDownloadAuthApiRequest {
    pub call_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallDownloadCohortApiRequest {
    pub call_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallDownloadDepartmentApiRequest {
    pub call_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallDownloadDocumentApiRequest {
    pub call_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallDownloadEvalApiRequest {
    pub call_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallDownloadFieldApiRequest {
    pub call_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallDownloadGroupApiRequest {
    pub call_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallDownloadModelApiRequest {
    pub call_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallDownloadParameterApiRequest {
    pub call_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallDownloadPersonaApiRequest {
    pub call_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallDownloadProfileApiRequest {
    pub call_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallDownloadProviderApiRequest {
    pub call_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallDownloadRubricApiRequest {
    pub call_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallDownloadScenarioApiRequest {
    pub call_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallDownloadSettingApiRequest {
    pub call_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallDownloadSimulationApiRequest {
    pub call_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallDownloadTestApiRequest {
    pub call_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallDownloadToolApiRequest {
    pub call_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallerPermissions {
    pub can_create: bool,
    pub can_draft: bool,
    pub can_duplicate: bool,
    #[serde(default)]
    pub has_access: Option<bool>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub can_delete: Option<bool>,
    #[serde(default)]
    pub disabled_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatAnalysesRequest {
    pub chat_id: String,
    pub analyses: Vec<ChatAnalysisItem>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatAnalysesResponse {
    pub success: bool,
    pub analysis_ids: Vec<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatAnalysisItem {
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatAudioRequest {
    pub chat_id: String,
    pub message_id: String,
    pub audios_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatAudioResponse {
    pub success: bool,
    pub attempt_audio_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompleteRequest {
    pub chat_id: String,
    #[serde(default)]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompleteResponse {
    pub success: bool,
    pub completion_id: String,
    pub chat_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatData {
    pub id: String,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub completed: Option<bool>,
    #[serde(default)]
    pub is_current: Option<bool>,
    #[serde(default)]
    pub position: Option<i64>,
    #[serde(default)]
    pub grade: Option<GradeData>,
    #[serde(default)]
    pub feedbacks: Option<Vec<FeedbackEntry>>,
    #[serde(default)]
    pub analyses: Option<Vec<AnalysisEntry>>,
    #[serde(default)]
    pub show_problem_statement: Option<bool>,
    #[serde(default)]
    pub show_objectives: Option<bool>,
    #[serde(default)]
    pub copy_paste_allowed: Option<bool>,
    #[serde(default)]
    pub text_enabled: Option<bool>,
    #[serde(default)]
    pub audio_enabled: Option<bool>,
    #[serde(default)]
    pub hints_enabled: Option<bool>,
    #[serde(default)]
    pub analyses_enabled: Option<bool>,
    #[serde(default)]
    pub strengths_enabled: Option<bool>,
    #[serde(default)]
    pub improvements_enabled: Option<bool>,
    #[serde(default)]
    pub problem_statement_enabled: Option<bool>,
    #[serde(default)]
    pub objectives_enabled: Option<bool>,
    #[serde(default)]
    pub video_enabled: Option<bool>,
    #[serde(default)]
    pub images_enabled: Option<bool>,
    #[serde(default)]
    pub questions_enabled: Option<bool>,
    #[serde(default)]
    pub grading_state: Option<GradingStateData>,
    #[serde(default)]
    pub dynamic_rubric: Option<DynamicRubricData>,
    #[serde(default)]
    pub scenario_id: Option<String>,
    #[serde(default)]
    pub problem_statement_id: Option<String>,
    #[serde(default)]
    pub objective_ids: Option<Vec<String>>,
    #[serde(default)]
    pub persona_ids: Option<Vec<String>>,
    #[serde(default)]
    pub image_ids: Option<Vec<String>>,
    #[serde(default)]
    pub video_ids: Option<Vec<String>>,
    #[serde(default)]
    pub question_ids: Option<Vec<String>>,
    #[serde(default)]
    pub option_ids: Option<Vec<String>>,
    #[serde(default)]
    pub responses: Option<Vec<QuizResponse>>,
    #[serde(default)]
    pub document_ids: Option<Vec<String>>,
    #[serde(default)]
    pub rubric_id: Option<String>,
    #[serde(default)]
    pub standard_group_ids: Option<Vec<String>>,
    #[serde(default)]
    pub standard_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatDepartmentResource {
    #[serde(default)]
    pub department_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatDescriptionResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatDocumentResource {
    #[serde(default)]
    pub document_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub file_id: Option<String>,
    #[serde(default)]
    pub text_id: Option<String>,
    #[serde(default)]
    pub image_ids: Option<Vec<String>>,
    #[serde(default)]
    pub template: Option<bool>,
    #[serde(default)]
    pub parameter_field_ids: Option<Vec<String>>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatDraftFormState {
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub problem_statement_id: Option<String>,
    #[serde(default)]
    pub problem_statement: Option<String>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub document_ids: Option<Vec<String>>,
    #[serde(default)]
    pub field_ids: Option<Vec<String>>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub image_ids: Option<Vec<String>>,
    #[serde(default)]
    pub objective_ids: Option<Vec<String>>,
    #[serde(default)]
    pub option_ids: Option<Vec<String>>,
    #[serde(default)]
    pub parameter_field_ids: Option<Vec<String>>,
    #[serde(default)]
    pub parameter_ids: Option<Vec<String>>,
    #[serde(default)]
    pub persona_ids: Option<Vec<String>>,
    #[serde(default)]
    pub question_ids: Option<Vec<String>>,
    #[serde(default)]
    pub scenario_ids: Option<Vec<String>>,
    #[serde(default)]
    pub video_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatFeedbackItem {
    pub feedback: String,
    #[serde(default)]
    pub total: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatFeedbackRequest {
    pub chat_id: String,
    pub feedbacks: Vec<ChatFeedbackItem>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatFeedbackResponse {
    pub success: bool,
    pub feedback_ids: Vec<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatFieldResource {
    #[serde(default)]
    pub field_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub value: Option<String>,
    #[serde(default)]
    pub conditional_parameter_ids: Option<Vec<String>>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatFlagResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub value: Option<bool>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatHighlightItem {
    pub section: String,
    #[serde(default)]
    pub idx: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatHintItem {
    pub hint: String,
    #[serde(default)]
    pub message_id: Option<String>,
    #[serde(default)]
    pub idx: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatHintsRequest {
    pub chat_id: String,
    pub hints: Vec<ChatHintItem>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatHintsResponse {
    pub success: bool,
    pub hint_ids: Vec<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatImageResource {
    #[serde(default)]
    pub image_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatImprovementItem {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub message_id: Option<String>,
    #[serde(default)]
    pub replacements: Option<Vec<ChatReplacementItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatImprovementsRequest {
    pub chat_id: String,
    pub improvements: Vec<ChatImprovementItem>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatImprovementsResponse {
    pub success: bool,
    pub improvement_ids: Vec<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessageRequest {
    pub chat_id: String,
    pub text: String,
    #[serde(default)]
    pub persona_id: Option<String>,
    #[serde(default)]
    pub parent_message_id: Option<String>,
    #[serde(default)]
    pub auto_link_parent: Option<bool>,
    #[serde(default)]
    pub audios_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessageResponse {
    pub success: bool,
    pub message_id: String,
    pub content_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatNameResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatObjectiveResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub objective: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatOptionResource {
    #[serde(default)]
    pub option_id: Option<String>,
    #[serde(default)]
    pub option_text: Option<String>,
    #[serde(default)]
    pub question_id: Option<String>,
    #[serde(default)]
    pub is_correct: Option<bool>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatParameterFieldResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub field_id: Option<String>,
    #[serde(default)]
    pub parameter_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub parameter_name: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatPersonaResource {
    #[serde(default)]
    pub persona_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatProblemStatementResource {
    #[serde(default)]
    pub problem_statement_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub problem_statement: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatQuestionResource {
    #[serde(default)]
    pub question_id: Option<String>,
    #[serde(default)]
    pub question_text: Option<String>,
    #[serde(default)]
    pub allow_multiple: Option<bool>,
    #[serde(default)]
    pub time: Option<i64>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatReplacementItem {
    pub section: String,
    pub replace: String,
    #[serde(default)]
    pub idx: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatScenarioResource {
    #[serde(default)]
    pub scenario_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatSilenceRequest {
    pub chat_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatSimulationOperational {
    pub simulation_id: String,
    #[serde(default)]
    pub simulation_name: Option<String>,
    #[serde(default)]
    pub simulation_description: Option<String>,
    #[serde(default)]
    pub time_limit: Option<i64>,
    #[serde(default)]
    pub chat_entry_id: Option<String>,
    #[serde(default)]
    pub home_id: Option<String>,
    #[serde(default)]
    pub practice_id: Option<String>,
    #[serde(default)]
    pub scenario_ids: Option<Vec<String>>,
    #[serde(default)]
    pub cohort_ids: Option<Vec<String>>,
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub view_mode: Option<String>,
    #[serde(default)]
    pub num_sessions: Option<i64>,
    #[serde(default)]
    pub highest_score: Option<i64>,
    #[serde(default)]
    pub has_passed: Option<bool>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub pass_pct: Option<i64>,
    #[serde(default)]
    pub cohort_names_junction: Option<String>,
    #[serde(default)]
    pub standard_groups: Option<Vec<String>>,
    #[serde(default)]
    pub practice_simulation: Option<bool>,
    #[serde(default)]
    pub completion_pct: Option<i64>,
    #[serde(default)]
    pub passed_count: Option<i64>,
    #[serde(default)]
    pub in_progress_count: Option<i64>,
    #[serde(default)]
    pub not_started_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatSpeakRequest {
    pub audio: String,
    #[serde(default)]
    pub conversation_id: Option<String>,
    #[serde(default)]
    pub chat_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatSpeakResponse {
    pub accepted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatStrengthItem {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub message_id: Option<String>,
    #[serde(default)]
    pub highlights: Option<Vec<ChatHighlightItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatStrengthsRequest {
    pub chat_id: String,
    pub strengths: Vec<ChatStrengthItem>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatStrengthsResponse {
    pub success: bool,
    pub strength_ids: Vec<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatVideoResource {
    #[serde(default)]
    pub video_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub length_seconds: Option<i64>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatVoiceRequest {
    pub chat_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CohortDepartment {
    #[serde(default)]
    pub department_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CohortDescriptionResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CohortFieldError {
    pub field: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CohortFlagResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub value: Option<bool>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub icon_id: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CohortNameResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CohortPersonaResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub instructions: Option<String>,
    #[serde(default)]
    pub examples: Option<Vec<String>>,
    #[serde(default)]
    pub parameter_field_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub mcp: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CohortProfile {
    #[serde(default)]
    pub profile_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub mcp: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CohortProfilePersona {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub profile_id: Option<String>,
    #[serde(default)]
    pub persona_id: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub mcp: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CohortResultItem {
    pub success: bool,
    pub message: String,
    #[serde(default)]
    pub cohort_id: Option<String>,
    #[serde(default)]
    pub errors: Option<Vec<CohortFieldError>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CohortSimulation {
    #[serde(default)]
    pub simulation_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CohortSimulationAvailability {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub simulation_id: Option<String>,
    #[serde(default)]
    pub time: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub mcp: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CohortSimulationPosition {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub simulation_id: Option<String>,
    #[serde(default)]
    pub value: Option<i64>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub mcp: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnInfo {
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub nullable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComposedContextResponse {
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub description: String,
    pub entries: Vec<DocsResponse>,
    pub resources: Vec<DocsResponse>,
    pub permission_docs: Vec<OperationInfo>,
    pub api_operations: Vec<OperationInfo>,
    pub profile: ProfileSummary,
    pub caller_permissions: CallerPermissions,
    #[serde(default)]
    pub artifact: Option<DocsResponse>,
    #[serde(default)]
    pub page_metadata: Option<DocsApiResponse>,
    #[serde(default)]
    pub prompts: Option<OperationPrompts>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentEntry {
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContinuationOption {
    pub scenarios: Vec<PreviousChatOption>,
    pub total_score: f64,
    pub total_time: f64,
    #[serde(default)]
    pub total_percentage: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAgentApiRequest {
    pub agents: Vec<CreateAgentItem>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAgentApiResponse {
    pub results: Vec<AgentResultItem>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAgentItem {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub resource_id: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub model_id: Option<String>,
    #[serde(default)]
    pub reasoning_level_ids: Option<Vec<String>>,
    #[serde(default)]
    pub temperature_level_ids: Option<Vec<String>>,
    #[serde(default)]
    pub tool_ids: Option<Vec<String>>,
    #[serde(default)]
    pub voice_ids: Option<Vec<String>>,
    #[serde(default)]
    pub agent_ids: Option<Vec<String>>,
    #[serde(default)]
    pub rubric_ids: Option<Vec<String>>,
    #[serde(default)]
    pub prompt_id: Option<String>,
    #[serde(default)]
    pub instruction_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateArgInput {
    pub name: String,
    pub field_type: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub default_value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateArgPositionInput {
    pub args_id: String,
    pub value: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateArgsOutputInput {
    pub args_id: String,
    pub name: String,
    #[serde(default)]
    pub template: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAttemptChatApiRequest {
    pub attempt_id: String,
    pub chat_id: String,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub problem_statement_id: Option<String>,
    #[serde(default)]
    pub problem_statement: Option<String>,
    #[serde(default)]
    pub video_id: Option<String>,
    #[serde(default)]
    pub image_id: Option<String>,
    #[serde(default)]
    pub personas_ids: Option<Vec<String>>,
    #[serde(default)]
    pub personas: Option<Vec<String>>,
    #[serde(default)]
    pub documents_ids: Option<Vec<String>>,
    #[serde(default)]
    pub documents: Option<Vec<String>>,
    #[serde(default)]
    pub parameter_fields_ids: Option<Vec<String>>,
    #[serde(default)]
    pub parameter_fields: Option<Vec<String>>,
    #[serde(default)]
    pub objectives_ids: Option<Vec<String>>,
    #[serde(default)]
    pub objectives: Option<Vec<String>>,
    #[serde(default)]
    pub questions_ids: Option<Vec<String>>,
    #[serde(default)]
    pub questions: Option<Vec<AttemptQuestionValue>>,
    #[serde(default)]
    pub options_ids: Option<Vec<String>>,
    #[serde(default)]
    pub previous_attempt_chat_id: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAttemptChatApiResponse {
    pub attempt_chat_id: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAuthApiRequest {
    #[serde(default)]
    pub auths: Option<Vec<CreateAuthItem>>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAuthApiResponse {
    pub results: Vec<AuthResultItem>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub auths: Option<Vec<ListAuthApiAuth>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAuthItem {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub resource_id: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub slug_id: Option<String>,
    #[serde(default)]
    pub slug: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub protocol_ids: Option<Vec<String>>,
    #[serde(default)]
    pub protocol: Option<String>,
    #[serde(default)]
    pub item_ids: Option<Vec<String>>,
    #[serde(default)]
    pub auth_resource_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCohortApiRequest {
    pub cohorts: Vec<CreateCohortItem>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCohortApiResponse {
    pub results: Vec<CohortResultItem>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCohortItem {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub resource_id: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub simulation_ids: Option<Vec<String>>,
    #[serde(default)]
    pub simulation_position_ids: Option<Vec<String>>,
    #[serde(default)]
    pub simulation_availability_ids: Option<Vec<String>>,
    #[serde(default)]
    pub profile_ids: Option<Vec<String>>,
    #[serde(default)]
    pub profile_persona_ids: Option<Vec<String>>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub simulations: Option<Vec<String>>,
    #[serde(default)]
    pub profiles: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDepartmentApiRequest {
    pub departments: Vec<CreateDepartmentItem>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDepartmentApiResponse {
    pub results: Vec<DepartmentResultItem>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub departments: Option<Vec<ListDepartmentApiDepartment>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDepartmentItem {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub resource_id: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub settings_ids: Option<Vec<String>>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDocumentApiRequest {
    pub documents: Vec<CreateDocumentItem>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDocumentApiResponse {
    pub results: Vec<DocumentResultItem>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDocumentItem {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub resource_id: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub parameter_field_ids: Option<Vec<String>>,
    #[serde(default)]
    pub upload_ids: Option<Vec<String>>,
    #[serde(default)]
    pub image_ids: Option<Vec<String>>,
    #[serde(default)]
    pub text_ids: Option<Vec<String>>,
    #[serde(default)]
    pub file_id: Option<String>,
    #[serde(default)]
    pub text_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEvalApiRequest {
    pub evals: Vec<CreateEvalItem>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEvalApiResponse {
    pub results: Vec<EvalResultItem>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub evals: Option<Vec<ListEvalApiEval>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEvalItem {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub resource_id: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub model_ids: Option<Vec<String>>,
    #[serde(default)]
    pub model_flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub model_rubric_ids: Option<Vec<String>>,
    #[serde(default)]
    pub model_position_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFeedbackApiRequest {
    pub grade_id: String,
    pub tool_call_id: String,
    pub standard_group_id: String,
    #[serde(default)]
    pub score: Option<i64>,
    #[serde(default)]
    pub feedback: Option<String>,
    #[serde(default)]
    pub run_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFieldApiRequest {
    pub fields: Vec<CreateFieldItem>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFieldApiResponse {
    pub results: Vec<FieldResultItem>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFieldItem {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub resource_id: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub conditional_parameter_ids: Option<Vec<String>>,
    #[serde(default)]
    pub field_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateGradeApiRequest {
    pub invocation_id: String,
    #[serde(default)]
    pub run_id: Option<String>,
    #[serde(default)]
    pub score: Option<i64>,
    #[serde(default)]
    pub full: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInvocationApiRequest {
    pub test_id: String,
    #[serde(default)]
    pub invocation_id: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub use_custom: Option<bool>,
    #[serde(default)]
    pub position: Option<i64>,
    #[serde(default)]
    pub agent_ids: Option<Vec<String>>,
    #[serde(default)]
    pub rubric_ids: Option<Vec<String>>,
    #[serde(default)]
    pub quality_ids: Option<Vec<String>>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub voice_ids: Option<Vec<String>>,
    #[serde(default)]
    pub reasoning_level_ids: Option<Vec<String>>,
    #[serde(default)]
    pub temperature_level_ids: Option<Vec<String>>,
    #[serde(default)]
    pub modality_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInvocationApiResponse {
    pub invocation_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateModelApiRequest {
    #[serde(default)]
    pub models: Option<Vec<CreateModelItem>>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateModelApiResponse {
    pub results: Vec<ModelResultItem>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub models: Option<Vec<ListModelApiModel>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateModelItem {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub resource_id: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub value_id: Option<String>,
    #[serde(default)]
    pub value: Option<String>,
    #[serde(default)]
    pub provider_id: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub modality_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pricing_ids: Option<Vec<String>>,
    #[serde(default)]
    pub quality_ids: Option<Vec<String>>,
    #[serde(default)]
    pub reasoning_level_ids: Option<Vec<String>>,
    #[serde(default)]
    pub temperature_level_ids: Option<Vec<String>>,
    #[serde(default)]
    pub voice_ids: Option<Vec<String>>,
    #[serde(default)]
    pub model_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateParameterApiRequest {
    pub parameters: Vec<CreateParameterItem>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateParameterApiResponse {
    pub results: Vec<ParameterResultItem>,
    #[serde(default)]
    pub parameters: Option<Vec<ListParameterApiParameter>>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateParameterItem {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub resource_id: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub field_ids: Option<Vec<String>>,
    #[serde(default)]
    pub persona_parameter: Option<bool>,
    #[serde(default)]
    pub document_parameter: Option<bool>,
    #[serde(default)]
    pub scenario_parameter: Option<bool>,
    #[serde(default)]
    pub video_parameter: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePersonaApiRequest {
    #[serde(default)]
    pub personas: Option<Vec<CreatePersonaItem>>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePersonaApiResponse {
    pub results: Vec<PersonaResultItem>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub personas: Option<Vec<ListPersonaApiPersona>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePersonaItem {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub resource_id: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub color_id: Option<String>,
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub icon_id: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub instructions_id: Option<String>,
    #[serde(default)]
    pub instructions: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub parameter_field_ids: Option<Vec<String>>,
    #[serde(default)]
    pub parameter_fields: Option<Vec<String>>,
    #[serde(default)]
    pub example_ids: Option<Vec<String>>,
    #[serde(default)]
    pub examples: Option<Vec<String>>,
    #[serde(default)]
    pub voice_ids: Option<Vec<String>>,
    #[serde(default)]
    pub voices: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProfileApiRequest {
    pub profiles: Vec<CreateProfileItem>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProfileApiResponse {
    pub results: Vec<ProfileResultItem>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub profiles: Option<Vec<ListProfilesApiProfile>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProfileItem {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub resource_id: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub email_ids: Option<Vec<String>>,
    #[serde(default)]
    pub role_id: Option<String>,
    #[serde(default)]
    pub primary_department_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePromptInput {
    pub system_prompt: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProviderApiRequest {
    pub providers: Vec<CreateProviderItem>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProviderApiResponse {
    pub results: Vec<ProviderResultItem>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub providers: Option<Vec<ListProviderApiProvider>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProviderItem {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub resource_id: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub endpoint_ids: Option<Vec<String>>,
    #[serde(default)]
    pub key_ids: Option<Vec<String>>,
    #[serde(default)]
    pub value_id: Option<String>,
    #[serde(default)]
    pub endpoint: Option<String>,
    #[serde(default)]
    pub key: Option<String>,
    #[serde(default)]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRubricApiRequest {
    pub rubrics: Vec<CreateRubricItem>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRubricApiResponse {
    pub results: Vec<RubricResultItem>,
    #[serde(default)]
    pub rubrics: Option<Vec<ListRubricApiRubric>>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRubricItem {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub resource_id: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub simulation_rubric: Option<bool>,
    #[serde(default)]
    pub video_rubric: Option<bool>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub pass_points_id: Option<String>,
    #[serde(default)]
    pub pass_points: Option<i64>,
    #[serde(default)]
    pub standard_group_ids: Option<Vec<String>>,
    #[serde(default)]
    pub standard_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateScenarioApiRequest {
    pub scenarios: Vec<CreateScenarioItem>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateScenarioApiResponse {
    pub results: Vec<ScenarioResultItem>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub scenarios: Option<Vec<ListScenarioApiScenario>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateScenarioItem {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub resource_id: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub problem_statement_id: Option<String>,
    #[serde(default)]
    pub problem_statement: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub persona_ids: Option<Vec<String>>,
    #[serde(default)]
    pub document_ids: Option<Vec<String>>,
    #[serde(default)]
    pub parameter_ids: Option<Vec<String>>,
    #[serde(default)]
    pub parameter_field_ids: Option<Vec<String>>,
    #[serde(default)]
    pub image_ids: Option<Vec<String>>,
    #[serde(default)]
    pub objective_ids: Option<Vec<String>>,
    #[serde(default)]
    pub video_ids: Option<Vec<String>>,
    #[serde(default)]
    pub question_ids: Option<Vec<String>>,
    #[serde(default)]
    pub option_ids: Option<Vec<String>>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub personas: Option<Vec<String>>,
    #[serde(default)]
    pub documents: Option<Vec<String>>,
    #[serde(default)]
    pub parameter_fields: Option<Vec<String>>,
    #[serde(default)]
    pub objectives: Option<Vec<String>>,
    #[serde(default)]
    pub images: Option<Vec<String>>,
    #[serde(default)]
    pub videos: Option<Vec<String>>,
    #[serde(default)]
    pub questions: Option<Vec<String>>,
    #[serde(default)]
    pub options: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSettingApiRequest {
    pub settings: Vec<CreateSettingItem>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSettingApiResponse {
    pub results: Vec<SettingResultItem>,
    #[serde(default)]
    pub settings: Option<Vec<ListSettingApiSetting>>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSettingItem {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub resource_id: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active_flag_id: Option<String>,
    #[serde(default)]
    pub active_flag: Option<bool>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub color_ids: Option<Vec<String>>,
    #[serde(default)]
    pub logins_ids: Option<Vec<String>>,
    #[serde(default)]
    pub system_ids: Option<Vec<String>>,
    #[serde(default)]
    pub mcp_id: Option<String>,
    #[serde(default)]
    pub threshold_ids: Option<Vec<String>>,
    #[serde(default)]
    pub provider_key_ids: Option<Vec<String>>,
    #[serde(default)]
    pub auth_item_key_ids: Option<Vec<String>>,
    #[serde(default)]
    pub auth_item_value_ids: Option<Vec<String>>,
    #[serde(default)]
    pub auth_ids: Option<Vec<String>>,
    #[serde(default)]
    pub provider_ids: Option<Vec<String>>,
    #[serde(default)]
    pub setting_resource_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSimulationApiRequest {
    pub simulations: Vec<CreateSimulationItem>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSimulationApiResponse {
    pub results: Vec<SimulationResultItem>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSimulationItem {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub resource_id: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub scenario_ids: Option<Vec<String>>,
    #[serde(default)]
    pub scenario_flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub scenario_position_ids: Option<Vec<String>>,
    #[serde(default)]
    pub scenario_rubric_ids: Option<Vec<String>>,
    #[serde(default)]
    pub scenario_time_limit_ids: Option<Vec<String>>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub scenarios: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateToolApiRequest {
    pub tools: Vec<CreateToolItem>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateToolApiResponse {
    pub results: Vec<ToolResultItem>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub tools: Option<Vec<ListToolApiTool>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateToolItem {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub resource_id: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub arg_positions_ids: Option<Vec<String>>,
    #[serde(default)]
    pub args_ids: Option<Vec<String>>,
    #[serde(default)]
    pub args_outputs_ids: Option<Vec<String>>,
    #[serde(default)]
    pub permission_ids: Option<Vec<String>>,
    #[serde(default)]
    pub instruction_id: Option<String>,
    #[serde(default)]
    pub tool_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardBundleResponse {
    #[serde(default)]
    pub header_metrics: Option<DashboardHeaderMetrics>,
    #[serde(default)]
    pub primary_metrics: Option<DashboardPrimaryMetrics>,
    #[serde(default)]
    pub secondary_metrics: Option<DashboardSecondaryMetrics>,
    #[serde(default)]
    pub footer_metrics: Option<DashboardFooterMetrics>,
    #[serde(default)]
    pub simulations: Option<Vec<DashboardSimulationMeta>>,
    #[serde(default)]
    pub scenarios: Option<Vec<DashboardScenarioMeta>>,
    #[serde(default)]
    pub rubrics: Option<Vec<DashboardRubricMeta>>,
    #[serde(default)]
    pub parameters: Option<Vec<DashboardParameterMeta>>,
    #[serde(default)]
    pub fields: Option<Vec<DashboardFieldMeta>>,
    #[serde(default)]
    pub thresholds: Option<DashboardThresholds>,
    #[serde(default)]
    pub insights: Option<DashboardInsights>,
    #[serde(default)]
    pub simulation_options: Option<Vec<FilterOption>>,
    #[serde(default)]
    pub profile_name: Option<String>,
    #[serde(default)]
    pub profile_emails: Option<Vec<String>>,
    #[serde(default)]
    pub profile_primary_email: Option<String>,
    #[serde(default)]
    pub profile_role: Option<String>,
    #[serde(default)]
    pub analytics: Option<AnalyticsFacets>,
    #[serde(default)]
    pub history: Option<HistoryResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardFieldMeta {
    #[serde(default)]
    pub field_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub parameter_id: Option<String>,
    #[serde(default)]
    pub parameter_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardFooterMetrics {
    #[serde(default)]
    pub scenario_performance: Option<FooterScenarioPerformance>,
    #[serde(default)]
    pub scenario_stats: Option<FooterScenarioStats>,
    #[serde(default)]
    pub scenario_simulation_performance: Option<FooterScenarioSimulationPerformance>,
    #[serde(default)]
    pub scenario_composition: Option<FooterScenarioComposition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardHeaderMetric {
    #[serde(default)]
    pub current_value: Option<serde_json::Value>,
    #[serde(default)]
    pub trend_data: Option<Vec<DashboardTrendPoint>>,
    #[serde(default)]
    pub has_data: Option<bool>,
    #[serde(default)]
    pub trend_analysis: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardHeaderMetrics {
    #[serde(default)]
    pub average_score: Option<DashboardHeaderMetric>,
    #[serde(default)]
    pub completion_percentage: Option<DashboardHeaderMetric>,
    #[serde(default)]
    pub first_attempt_pass_rate: Option<DashboardHeaderMetric>,
    #[serde(default)]
    pub highest_score: Option<DashboardHeaderMetric>,
    #[serde(default)]
    pub messages_per_session: Option<DashboardHeaderMetric>,
    #[serde(default)]
    pub persona_response_times: Option<DashboardHeaderMetric>,
    #[serde(default)]
    pub session_efficiency: Option<DashboardHeaderMetric>,
    #[serde(default)]
    pub stagnation_rate: Option<DashboardHeaderMetric>,
    #[serde(default)]
    pub time_spent: Option<DashboardHeaderMetric>,
    #[serde(default)]
    pub total_attempts: Option<DashboardHeaderMetric>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardInsightObject {
    #[serde(default)]
    pub insight: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardInsights {
    #[serde(default)]
    pub rubric_trend: Option<String>,
    #[serde(default)]
    pub rubric_heatmap: Option<String>,
    #[serde(default)]
    pub attempt_improvement: Option<String>,
    #[serde(default)]
    pub skill_performance: Option<String>,
    #[serde(default)]
    pub scenario_performance: Option<String>,
    #[serde(default)]
    pub scenario_stats: Option<String>,
    #[serde(default)]
    pub scenario_simulation_performance: Option<String>,
    #[serde(default)]
    pub scenario_composition: Option<String>,
    #[serde(default)]
    pub persona: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(default)]
    pub cohort: Option<std::collections::HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardParameterMeta {
    #[serde(default)]
    pub parameter_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub numerical: Option<bool>,
    #[serde(default)]
    pub document_parameter: Option<bool>,
    #[serde(default)]
    pub persona_parameter: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardPrimaryMetrics {
    #[serde(default)]
    pub rubric_heatmap: Option<PrimaryRubricHeatmap>,
    #[serde(default)]
    pub rubric_trend: Option<PrimaryRubricTrend>,
    #[serde(default)]
    pub skill_performance: Option<SecondarySkillPerformance>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardRequest {
    #[serde(default)]
    pub start_date: Option<String>,
    #[serde(default)]
    pub end_date: Option<String>,
    #[serde(default)]
    pub cohort_ids: Option<Vec<String>>,
    #[serde(default)]
    pub simulation_ids: Option<Vec<String>>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub role_ids: Option<Vec<String>>,
    #[serde(default)]
    pub simulation_filters: Option<Vec<String>>,
    #[serde(default)]
    pub actor_profile_id: Option<String>,
    #[serde(default)]
    pub target_profile_id: Option<String>,
    #[serde(default)]
    pub page_limit: Option<i64>,
    #[serde(default)]
    pub page_offset: Option<i64>,
    #[serde(default)]
    pub rubric_ids: Option<Vec<String>>,
    #[serde(default)]
    pub rubric_search: Option<String>,
    #[serde(default)]
    pub simulation_picker_ids: Option<Vec<String>>,
    #[serde(default)]
    pub simulation_picker_search: Option<String>,
    #[serde(default)]
    pub parameter_ids: Option<Vec<String>>,
    #[serde(default)]
    pub parameter_search: Option<String>,
    #[serde(default)]
    pub scenario_ids: Option<Vec<String>>,
    #[serde(default)]
    pub scenario_search: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardRubricMeta {
    #[serde(default)]
    pub rubric_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardScenarioMeta {
    #[serde(default)]
    pub scenario_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardSecondaryMetrics {
    #[serde(default)]
    pub persona_performance: Option<PrimaryPersonaPerformance>,
    #[serde(default)]
    pub cohort_performance: Option<SecondaryCohortPerformance>,
    #[serde(default)]
    pub attempt_improvement: Option<SecondaryAttemptImprovement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardSimulationMeta {
    #[serde(default)]
    pub simulation_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub time_limit: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardThresholds {
    #[serde(default)]
    pub success: Option<f64>,
    #[serde(default)]
    pub warning: Option<f64>,
    #[serde(default)]
    pub danger: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardTrendPoint {
    #[serde(default)]
    pub date: Option<serde_json::Value>,
    #[serde(default)]
    pub value: Option<f64>,
    #[serde(default)]
    pub count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecryptInvocationKeyApiRequest {
    pub invocation_id: String,
    pub key_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecryptInvocationKeyApiResponse {
    #[serde(default)]
    pub key: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub actor_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecryptProviderKeyApiRequest {
    pub provider_id: String,
    pub key_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecryptProviderKeyApiResponse {
    #[serde(default)]
    pub key: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub actor_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecryptSettingKeyApiRequest {
    pub setting_id: String,
    pub key_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecryptSettingKeyApiResponse {
    #[serde(default)]
    pub key: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub actor_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteAgentApiRequest {
    #[serde(default)]
    pub agent_ids: Option<Vec<String>>,
    #[serde(default)]
    pub all: Option<bool>,
    #[serde(default)]
    pub excluded_ids: Option<Vec<String>>,
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub filter_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_model_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_tool_ids: Option<Vec<String>>,
    #[serde(default)]
    pub department_search: Option<String>,
    #[serde(default)]
    pub model_search: Option<String>,
    #[serde(default)]
    pub tool_search: Option<String>,
    #[serde(default)]
    pub flag_search: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteAgentApiResponse {
    pub results: Vec<DeleteAgentResult>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteAgentResult {
    pub success: bool,
    pub agent_id: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteAuthApiRequest {
    #[serde(default)]
    pub auth_ids: Option<Vec<String>>,
    #[serde(default)]
    pub all: Option<bool>,
    #[serde(default)]
    pub excluded_ids: Option<Vec<String>>,
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub filter_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub department_search: Option<String>,
    #[serde(default)]
    pub flag_search: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteAuthApiResponse {
    pub results: Vec<DeleteAuthResult>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteAuthResult {
    pub success: bool,
    pub message: String,
    #[serde(default)]
    pub auth_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteCohortApiRequest {
    #[serde(default)]
    pub cohort_ids: Option<Vec<String>>,
    #[serde(default)]
    pub all: Option<bool>,
    #[serde(default)]
    pub excluded_ids: Option<Vec<String>>,
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub filter_profile_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_simulation_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub profile_search: Option<String>,
    #[serde(default)]
    pub simulation_search: Option<String>,
    #[serde(default)]
    pub department_search: Option<String>,
    #[serde(default)]
    pub flag_search: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteCohortApiResponse {
    pub results: Vec<DeleteCohortResult>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteCohortResult {
    pub success: bool,
    pub cohort_id: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteDepartmentApiRequest {
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub all: Option<bool>,
    #[serde(default)]
    pub excluded_ids: Option<Vec<String>>,
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub flag_search: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteDepartmentApiResponse {
    pub results: Vec<DeleteDepartmentResult>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteDepartmentResult {
    pub success: bool,
    pub message: String,
    #[serde(default)]
    pub department_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteDocumentApiRequest {
    #[serde(default)]
    pub document_ids: Option<Vec<String>>,
    #[serde(default)]
    pub all: Option<bool>,
    #[serde(default)]
    pub excluded_ids: Option<Vec<String>>,
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub scenario_ids: Option<Vec<String>>,
    #[serde(default)]
    pub field_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub scenario_search: Option<String>,
    #[serde(default)]
    pub field_search: Option<String>,
    #[serde(default)]
    pub department_search: Option<String>,
    #[serde(default)]
    pub flag_search: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteDocumentApiResponse {
    pub results: Vec<DeleteDocumentResult>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteDocumentResult {
    pub success: bool,
    pub message: String,
    #[serde(default)]
    pub document_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteEvalApiRequest {
    #[serde(default)]
    pub eval_ids: Option<Vec<String>>,
    #[serde(default)]
    pub all: Option<bool>,
    #[serde(default)]
    pub excluded_ids: Option<Vec<String>>,
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub filter_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub department_search: Option<String>,
    #[serde(default)]
    pub flag_search: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteEvalApiResponse {
    pub results: Vec<DeleteEvalResult>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteEvalResult {
    pub success: bool,
    pub message: String,
    #[serde(default)]
    pub eval_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteFieldApiRequest {
    #[serde(default)]
    pub field_ids: Option<Vec<String>>,
    #[serde(default)]
    pub all: Option<bool>,
    #[serde(default)]
    pub excluded_ids: Option<Vec<String>>,
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub parameter_ids: Option<Vec<String>>,
    #[serde(default)]
    pub persona_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub parameter_search: Option<String>,
    #[serde(default)]
    pub persona_search: Option<String>,
    #[serde(default)]
    pub department_search: Option<String>,
    #[serde(default)]
    pub flag_search: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteFieldApiResponse {
    pub results: Vec<DeleteFieldResult>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteFieldResult {
    pub success: bool,
    pub message: String,
    #[serde(default)]
    pub field_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteModelApiRequest {
    #[serde(default)]
    pub model_ids: Option<Vec<String>>,
    #[serde(default)]
    pub all: Option<bool>,
    #[serde(default)]
    pub excluded_ids: Option<Vec<String>>,
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub filter_provider_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_agent_ids: Option<Vec<String>>,
    #[serde(default)]
    pub provider_search: Option<String>,
    #[serde(default)]
    pub department_search: Option<String>,
    #[serde(default)]
    pub agent_search: Option<String>,
    #[serde(default)]
    pub flag_search: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteModelApiResponse {
    pub results: Vec<DeleteModelResult>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteModelResult {
    pub success: bool,
    pub message: String,
    #[serde(default)]
    pub model_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteParameterApiRequest {
    #[serde(default)]
    pub parameter_ids: Option<Vec<String>>,
    #[serde(default)]
    pub all: Option<bool>,
    #[serde(default)]
    pub excluded_ids: Option<Vec<String>>,
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub scenario_ids: Option<Vec<String>>,
    #[serde(default)]
    pub field_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub scenario_search: Option<String>,
    #[serde(default)]
    pub field_search: Option<String>,
    #[serde(default)]
    pub department_search: Option<String>,
    #[serde(default)]
    pub flag_search: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteParameterApiResponse {
    pub results: Vec<DeleteParameterResult>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteParameterResult {
    pub success: bool,
    pub message: String,
    #[serde(default)]
    pub parameter_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletePersonaApiRequest {
    #[serde(default)]
    pub ids: Option<Vec<String>>,
    #[serde(default)]
    pub all: Option<bool>,
    #[serde(default)]
    pub excluded_ids: Option<Vec<String>>,
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub scenario_ids: Option<Vec<String>>,
    #[serde(default)]
    pub field_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub scenario_search: Option<String>,
    #[serde(default)]
    pub field_search: Option<String>,
    #[serde(default)]
    pub department_search: Option<String>,
    #[serde(default)]
    pub color_search: Option<String>,
    #[serde(default)]
    pub icon_search: Option<String>,
    #[serde(default)]
    pub voice_search: Option<String>,
    #[serde(default)]
    pub instruction_search: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletePersonaApiResponse {
    pub results: Vec<DeletePersonaResult>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletePersonaResult {
    pub success: bool,
    pub id: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteProfileApiRequest {
    #[serde(default)]
    pub profile_ids: Option<Vec<String>>,
    #[serde(default)]
    pub all: Option<bool>,
    #[serde(default)]
    pub excluded_ids: Option<Vec<String>>,
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub cohort_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub role_filter: Option<String>,
    #[serde(default)]
    pub cohort_search: Option<String>,
    #[serde(default)]
    pub department_search: Option<String>,
    #[serde(default)]
    pub role_search: Option<String>,
    #[serde(default)]
    pub flag_search: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteProfileApiResponse {
    pub results: Vec<DeleteProfileResult>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteProfileResult {
    pub success: bool,
    pub message: String,
    #[serde(default)]
    pub profile_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteProviderApiRequest {
    #[serde(default)]
    pub provider_ids: Option<Vec<String>>,
    #[serde(default)]
    pub all: Option<bool>,
    #[serde(default)]
    pub excluded_ids: Option<Vec<String>>,
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub filter_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_model_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_status: Option<Vec<String>>,
    #[serde(default)]
    pub department_search: Option<String>,
    #[serde(default)]
    pub model_search: Option<String>,
    #[serde(default)]
    pub flag_search: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteProviderApiResponse {
    pub results: Vec<DeleteProviderResult>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteProviderResult {
    pub success: bool,
    pub message: String,
    #[serde(default)]
    pub provider_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteRubricApiRequest {
    #[serde(default)]
    pub rubric_ids: Option<Vec<String>>,
    #[serde(default)]
    pub all: Option<bool>,
    #[serde(default)]
    pub excluded_ids: Option<Vec<String>>,
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub filter_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_simulation_ids: Option<Vec<String>>,
    #[serde(default)]
    pub department_search: Option<String>,
    #[serde(default)]
    pub simulation_search: Option<String>,
    #[serde(default)]
    pub flag_search: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteRubricApiResponse {
    pub results: Vec<DeleteRubricResult>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteRubricResult {
    pub success: bool,
    pub message: String,
    #[serde(default)]
    pub rubric_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteScenarioApiRequest {
    #[serde(default)]
    pub scenario_ids: Option<Vec<String>>,
    #[serde(default)]
    pub all: Option<bool>,
    #[serde(default)]
    pub excluded_ids: Option<Vec<String>>,
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub persona_ids: Option<Vec<String>>,
    #[serde(default)]
    pub simulation_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub persona_search: Option<String>,
    #[serde(default)]
    pub simulation_search: Option<String>,
    #[serde(default)]
    pub department_search: Option<String>,
    #[serde(default)]
    pub flag_search: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteScenarioApiResponse {
    pub results: Vec<DeleteScenarioResult>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteScenarioResult {
    #[serde(default)]
    pub success: Option<bool>,
    #[serde(default)]
    pub scenario_id: Option<String>,
    #[serde(default)]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSettingApiRequest {
    #[serde(default)]
    pub setting_ids: Option<Vec<String>>,
    #[serde(default)]
    pub all: Option<bool>,
    #[serde(default)]
    pub excluded_ids: Option<Vec<String>>,
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub provider_ids: Option<Vec<String>>,
    #[serde(default)]
    pub auth_ids: Option<Vec<String>>,
    #[serde(default)]
    pub system_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub flag_search: Option<String>,
    #[serde(default)]
    pub provider_search: Option<String>,
    #[serde(default)]
    pub auth_search: Option<String>,
    #[serde(default)]
    pub system_search: Option<String>,
    #[serde(default)]
    pub department_search: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSettingApiResponse {
    pub results: Vec<DeleteSettingResult>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSettingResult {
    pub success: bool,
    pub message: String,
    #[serde(default)]
    pub setting_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSimulationApiRequest {
    #[serde(default)]
    pub simulation_ids: Option<Vec<String>>,
    #[serde(default)]
    pub all: Option<bool>,
    #[serde(default)]
    pub excluded_ids: Option<Vec<String>>,
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub filter_scenario_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_cohort_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub scenario_search: Option<String>,
    #[serde(default)]
    pub cohort_search: Option<String>,
    #[serde(default)]
    pub department_search: Option<String>,
    #[serde(default)]
    pub flag_search: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSimulationApiResponse {
    pub results: Vec<DeleteSimulationResult>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSimulationResult {
    pub success: bool,
    pub simulation_id: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteToolApiRequest {
    #[serde(default)]
    pub tool_ids: Option<Vec<String>>,
    #[serde(default)]
    pub all: Option<bool>,
    #[serde(default)]
    pub excluded_ids: Option<Vec<String>>,
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub filter_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_creatable: Option<Vec<String>>,
    #[serde(default)]
    pub filter_agent_ids: Option<Vec<String>>,
    #[serde(default)]
    pub department_search: Option<String>,
    #[serde(default)]
    pub flag_search: Option<String>,
    #[serde(default)]
    pub agent_search: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteToolApiResponse {
    pub results: Vec<DeleteToolResult>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteToolResult {
    pub success: bool,
    pub message: String,
    #[serde(default)]
    pub tool_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepartmentDescriptionResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepartmentFieldError {
    pub field: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepartmentFlagResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub value: Option<bool>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub icon_id: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepartmentNameResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepartmentResultItem {
    pub success: bool,
    pub message: String,
    #[serde(default)]
    pub department_id: Option<String>,
    #[serde(default)]
    pub errors: Option<Vec<DepartmentFieldError>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepartmentSettingResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub provider_key_ids: Option<Vec<String>>,
    #[serde(default)]
    pub auth_ids: Option<Vec<String>>,
    #[serde(default)]
    pub system_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub mcp: Option<bool>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocsApiRequest {
    #[serde(default)]
    pub entity_id: Option<String>,
    #[serde(default)]
    pub snapshot_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocsApiResponse {
    pub list: PageMetaItem,
    pub detail: PageMetaItem,
    pub new: PageMetaItem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocsResponse {
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub description: String,
    pub tables: Vec<TableInfo>,
    pub operations: Vec<OperationInfo>,
    #[serde(default)]
    pub materialized_view: Option<MvInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentDepartmentResource {
    #[serde(default)]
    pub department_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentDescriptionResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentEntry {
    #[serde(default)]
    pub document_id: Option<String>,
    #[serde(default)]
    pub text_id: Option<String>,
    #[serde(default)]
    pub file_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub template: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentFieldError {
    pub field: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentFileResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub files_id: Option<String>,
    #[serde(default)]
    pub file_path: Option<String>,
    #[serde(default)]
    pub mime_type: Option<String>,
    #[serde(default)]
    pub size: Option<i64>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentFlagResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub value: Option<bool>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub icon_id: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentImageResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub image_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub file_path: Option<String>,
    #[serde(default)]
    pub mime_type: Option<String>,
    #[serde(default)]
    pub size: Option<i64>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentNameResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentParameterFieldResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub field_id: Option<String>,
    #[serde(default)]
    pub parameter_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub conditional_parameter_id: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentParameterResource {
    #[serde(default)]
    pub parameter_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub value: Option<String>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub persona_parameter: Option<bool>,
    #[serde(default)]
    pub document_parameter: Option<bool>,
    #[serde(default)]
    pub scenario_parameter: Option<bool>,
    #[serde(default)]
    pub video_parameter: Option<bool>,
    #[serde(default)]
    pub field_ids: Option<Vec<String>>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentResultItem {
    pub success: bool,
    pub message: String,
    #[serde(default)]
    pub document_id: Option<String>,
    #[serde(default)]
    pub errors: Option<Vec<DocumentFieldError>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentTextResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub texts_id: Option<String>,
    #[serde(default)]
    pub file_path: Option<String>,
    #[serde(default)]
    pub mime_type: Option<String>,
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DraftFileValue {
    pub upload_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DraftProfilePersonaValue {
    pub profile_id: String,
    pub persona_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DraftScenarioFlagDenormValue {
    pub scenario_id: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub value: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DraftScenarioFlagValue {
    pub scenario_id: String,
    pub flag_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DraftScenarioPositionValue {
    pub scenario_id: String,
    pub value: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DraftScenarioRubricValue {
    pub scenario_id: String,
    pub rubric_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DraftScenarioTimeLimitValue {
    pub scenario_id: String,
    pub time_limit_seconds: i64,
    #[serde(default)]
    pub negative: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DraftSimulationAvailabilityValue {
    pub simulation_id: String,
    pub time: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DraftSimulationPositionValue {
    pub simulation_id: String,
    pub value: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DraftTextValue {
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateAgentApiRequest {
    pub agent_id: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateAgentApiResponse {
    pub success: bool,
    pub agent_id: String,
    pub message: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateAuthApiRequest {
    pub auth_id: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateAuthApiResponse {
    pub success: bool,
    pub auth_id: String,
    pub message: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub auths: Option<Vec<ListAuthApiAuth>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateCohortApiRequest {
    pub cohort_id: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateCohortApiResponse {
    pub success: bool,
    pub cohort_id: String,
    pub message: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateDepartmentApiRequest {
    pub department_id: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateDepartmentApiResponse {
    pub success: bool,
    pub department_id: String,
    pub message: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub departments: Option<Vec<ListDepartmentApiDepartment>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateDocumentApiRequest {
    pub document_id: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateDocumentApiResponse {
    pub success: bool,
    pub document_id: String,
    pub message: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateEvalApiRequest {
    pub eval_id: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateEvalApiResponse {
    pub success: bool,
    pub eval_id: String,
    pub message: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub evals: Option<Vec<ListEvalApiEval>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateFieldApiRequest {
    pub field_id: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateFieldApiResponse {
    pub success: bool,
    pub field_id: String,
    pub message: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateModelApiRequest {
    pub model_id: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateModelApiResponse {
    pub success: bool,
    pub model_id: String,
    pub message: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub models: Option<Vec<ListModelApiModel>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateParameterApiRequest {
    pub parameter_id: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateParameterApiResponse {
    pub success: bool,
    pub parameter_id: String,
    pub message: String,
    #[serde(default)]
    pub parameters: Option<Vec<ListParameterApiParameter>>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicatePersonaApiRequest {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicatePersonaApiResponse {
    pub success: bool,
    pub id: String,
    pub message: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub personas: Option<Vec<ListPersonaApiPersona>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateProfileApiRequest {
    pub target_profile_id: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateProfileApiResponse {
    pub success: bool,
    pub profile_id: String,
    pub message: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub profiles: Option<Vec<ListProfilesApiProfile>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateProviderApiRequest {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub provider_id: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateProviderApiResponse {
    pub success: bool,
    pub provider_id: String,
    pub message: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub providers: Option<Vec<ListProviderApiProvider>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateRubricApiRequest {
    pub rubric_id: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateRubricApiResponse {
    pub success: bool,
    pub rubric_id: String,
    pub message: String,
    #[serde(default)]
    pub rubrics: Option<Vec<ListRubricApiRubric>>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateScenarioApiRequest {
    pub scenario_id: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateScenarioApiResponse {
    pub success: bool,
    pub scenario_id: String,
    pub message: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub scenarios: Option<Vec<ListScenarioApiScenario>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateSettingApiRequest {
    pub setting_id: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateSettingApiResponse {
    pub success: bool,
    pub setting_id: String,
    pub message: String,
    #[serde(default)]
    pub settings: Option<Vec<ListSettingApiSetting>>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateSimulationApiRequest {
    pub simulation_id: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateSimulationApiResponse {
    pub success: bool,
    pub simulation_id: String,
    pub message: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateToolApiRequest {
    pub tool_id: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateToolApiResponse {
    pub success: bool,
    pub tool_id: String,
    pub message: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub tools: Option<Vec<ListToolApiTool>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicRubricData {
    #[serde(default)]
    pub chat_id: Option<String>,
    #[serde(default)]
    pub score: Option<f64>,
    #[serde(default)]
    pub passed: Option<bool>,
    #[serde(default)]
    pub time_taken: Option<f64>,
    #[serde(default)]
    pub skill_scores: Option<Vec<SkillScore>>,
    #[serde(default)]
    pub skill_feedbacks: Option<Vec<SkillFeedback>>,
    #[serde(default)]
    pub total_possible_points: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmulateProfileApiRequest {
    pub target_profile_id: String,
    #[serde(default)]
    pub ttl_minutes: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmulateProfileApiResponse {
    pub allowed: bool,
    #[serde(default)]
    pub reason: Option<String>,
    #[serde(default)]
    pub grant_id: Option<String>,
    #[serde(default)]
    pub expires_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalDepartmentResource {
    #[serde(default)]
    pub department_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalDescriptionResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalFieldError {
    pub field: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalFlagResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub value: Option<bool>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub icon_id: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalModelFlagOptionResource {
    #[serde(default)]
    pub model_id: Option<String>,
    #[serde(default)]
    pub flag_id: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub value: Option<bool>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalModelFlagResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub model_id: Option<String>,
    #[serde(default)]
    pub flag_id: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub value: Option<bool>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalModelFlagValue {
    pub model_id: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub value: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalModelPositionResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub model_id: Option<String>,
    #[serde(default)]
    pub value: Option<serde_json::Value>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalModelResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub modality_ids: Option<Vec<String>>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalModelRubricResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub model_id: Option<String>,
    #[serde(default)]
    pub rubric_id: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalNameResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalResultItem {
    pub success: bool,
    pub message: String,
    #[serde(default)]
    pub eval_id: Option<String>,
    #[serde(default)]
    pub errors: Option<Vec<EvalFieldError>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalRubricResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalSetup {
    pub test_id: String,
    pub invocations: Vec<InvocationSlot>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportAgentApiRequest {
    #[serde(default)]
    pub agent_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportAgentApiResponse {
    pub file_id: String,
    pub file_name: String,
    pub row_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportAttemptApiRequest {
    #[serde(default)]
    pub view: Option<String>,
    #[serde(default)]
    pub attempt_id: Option<String>,
    #[serde(default)]
    pub record_id: Option<String>,
    #[serde(default)]
    pub date_from: Option<String>,
    #[serde(default)]
    pub date_to: Option<String>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub simulation_ids: Option<Vec<String>>,
    #[serde(default)]
    pub mode: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportAttemptApiResponse {
    pub file_id: String,
    pub file_name: String,
    pub row_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportAuthApiRequest {
    #[serde(default)]
    pub auth_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportAuthApiResponse {
    pub content: String,
    pub file_name: String,
    pub mime_type: String,
    pub row_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportCohortApiRequest {
    #[serde(default)]
    pub cohort_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportCohortApiResponse {
    pub file_id: String,
    pub file_name: String,
    pub row_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportDepartmentApiRequest {
    #[serde(default)]
    pub department_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportDepartmentApiResponse {
    pub file_id: String,
    pub file_name: String,
    pub row_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportDocumentApiRequest {
    #[serde(default)]
    pub document_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportDocumentApiResponse {
    pub content: String,
    pub file_name: String,
    pub mime_type: String,
    pub row_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportEvalApiRequest {
    #[serde(default)]
    pub eval_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportEvalApiResponse {
    pub file_id: String,
    pub file_name: String,
    pub row_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportFieldApiRequest {
    #[serde(default)]
    pub field_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportFieldApiResponse {
    pub file_id: String,
    pub file_name: String,
    pub row_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportModelApiRequest {
    #[serde(default)]
    pub model_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportModelApiResponse {
    pub file_id: String,
    pub file_name: String,
    pub row_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportParameterApiRequest {
    #[serde(default)]
    pub parameter_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportParameterApiResponse {
    pub file_id: String,
    pub file_name: String,
    pub row_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportPersonaApiRequest {
    #[serde(default)]
    pub persona_id: Option<String>,
    #[serde(default)]
    pub snapshot_key: Option<String>,
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub scenario_ids: Option<Vec<String>>,
    #[serde(default)]
    pub field_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_department_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportPersonaApiResponse {
    pub file_id: String,
    pub file_name: String,
    pub row_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportProfileApiRequest {
    #[serde(default)]
    pub profile_export_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportProfileApiResponse {
    pub content: String,
    pub file_name: String,
    pub mime_type: String,
    pub row_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportProviderApiRequest {
    #[serde(default)]
    pub provider_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportProviderApiResponse {
    pub file_id: String,
    pub file_name: String,
    pub row_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportRubricApiRequest {
    pub rubric_id: String,
    #[serde(default)]
    pub chat_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportRubricApiResponse {
    pub file_id: String,
    pub file_name: String,
    pub row_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportScenarioApiRequest {
    #[serde(default)]
    pub scenario_id: Option<String>,
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub persona_ids: Option<Vec<String>>,
    #[serde(default)]
    pub simulation_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_department_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportScenarioApiResponse {
    pub content: String,
    pub file_name: String,
    pub mime_type: String,
    pub row_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportSettingApiRequest {
    #[serde(default)]
    pub setting_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportSettingApiResponse {
    pub file_id: String,
    pub file_name: String,
    pub row_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportSimulationApiRequest {
    #[serde(default)]
    pub simulation_id: Option<String>,
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub filter_scenario_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_cohort_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_department_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportSimulationApiResponse {
    pub file_id: String,
    pub file_name: String,
    pub row_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportSystemApiRequest {
    pub view: String,
    #[serde(default)]
    pub session_id: Option<String>,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub mode: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportSystemApiResponse {
    pub file_id: String,
    pub file_name: String,
    pub row_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportTestApiRequest {
    #[serde(default)]
    pub view: Option<String>,
    #[serde(default)]
    pub test_id: Option<String>,
    #[serde(default)]
    pub invocation_id: Option<String>,
    #[serde(default)]
    pub draft_id: Option<String>,
    #[serde(default)]
    pub mode: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportTestApiResponse {
    pub file_id: String,
    pub file_name: String,
    pub row_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportToolApiRequest {
    #[serde(default)]
    pub tool_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportToolApiResponse {
    pub file_id: String,
    pub file_name: String,
    pub row_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackEntry {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub standard_id: Option<String>,
    #[serde(default)]
    pub standard_group_id: Option<String>,
    #[serde(default)]
    pub total: Option<f64>,
    #[serde(default)]
    pub feedback: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldConditionalParameterResource {
    #[serde(default)]
    pub parameter_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldDepartmentResource {
    #[serde(default)]
    pub department_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldDescriptionResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldFieldError {
    pub field: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldFlagResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub value: Option<bool>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub icon_id: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldNameResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldResultItem {
    pub success: bool,
    pub message: String,
    #[serde(default)]
    pub field_id: Option<String>,
    #[serde(default)]
    pub errors: Option<Vec<FieldFieldError>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDownloadAgentApiRequest {
    pub file_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDownloadAttemptApiRequest {
    pub file_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDownloadAuthApiRequest {
    pub file_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDownloadCohortApiRequest {
    pub file_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDownloadDepartmentApiRequest {
    pub file_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDownloadDocumentApiRequest {
    pub file_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDownloadEvalApiRequest {
    pub file_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDownloadFieldApiRequest {
    pub file_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDownloadGroupApiRequest {
    pub file_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDownloadModelApiRequest {
    pub file_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDownloadParameterApiRequest {
    pub file_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDownloadPersonaApiRequest {
    pub file_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDownloadProfileApiRequest {
    pub file_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDownloadProviderApiRequest {
    pub file_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDownloadRubricApiRequest {
    pub file_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDownloadScenarioApiRequest {
    pub file_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDownloadSettingApiRequest {
    pub file_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDownloadSimulationApiRequest {
    pub file_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDownloadTestApiRequest {
    pub file_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDownloadToolApiRequest {
    pub file_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilePreviewAttemptApiRequest {
    pub file_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilePreviewDocumentApiRequest {
    pub file_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilePreviewGroupApiRequest {
    pub file_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilePreviewScenarioApiRequest {
    pub file_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileUploadDocumentApiResponse {
    pub file_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterOption {
    pub value: String,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FooterNumericAttemptFact {
    #[serde(default)]
    pub parameter_id: Option<String>,
    #[serde(default)]
    pub level_label: Option<String>,
    #[serde(default)]
    pub level_value: Option<f64>,
    #[serde(default)]
    pub score: Option<f64>,
    #[serde(default)]
    pub attempts: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FooterNumericScenarioFact {
    #[serde(default)]
    pub parameter_id: Option<String>,
    #[serde(default)]
    pub scenario_id: Option<String>,
    #[serde(default)]
    pub level_label: Option<String>,
    #[serde(default)]
    pub level_value: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FooterScenarioAttributeAttemptFact {
    #[serde(default)]
    pub parameter_id: Option<String>,
    #[serde(default)]
    pub parameter_item_id: Option<String>,
    #[serde(default)]
    pub date: Option<String>,
    #[serde(default)]
    pub timestamp: Option<i64>,
    #[serde(default)]
    pub avg_score: Option<f64>,
    #[serde(default)]
    pub attempts: Option<i64>,
    #[serde(default)]
    pub passed_attempts: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FooterScenarioAttributeScenarioFact {
    #[serde(default)]
    pub parameter_id: Option<String>,
    #[serde(default)]
    pub parameter_item_id: Option<String>,
    #[serde(default)]
    pub scenario_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FooterScenarioComposition {
    #[serde(default)]
    pub scenario_summaries: Option<Vec<FooterScenarioCompositionSummary>>,
    #[serde(default)]
    pub chat_parameter_facts: Option<Vec<FooterScenarioCompositionParamFact>>,
    #[serde(default)]
    pub valid_scenario_ids: Option<Vec<String>>,
    #[serde(default)]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FooterScenarioCompositionParamFact {
    #[serde(default)]
    pub scenario_id: Option<String>,
    #[serde(default)]
    pub group: Option<String>,
    #[serde(default)]
    pub parameter_id: Option<String>,
    #[serde(default)]
    pub parameter_item_id: Option<String>,
    #[serde(default)]
    pub chat_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FooterScenarioCompositionSummary {
    #[serde(default)]
    pub scenario_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub total_chats: Option<i64>,
    #[serde(default)]
    pub high_count: Option<i64>,
    #[serde(default)]
    pub low_count: Option<i64>,
    #[serde(default)]
    pub high_avg_score: Option<f64>,
    #[serde(default)]
    pub low_avg_score: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FooterScenarioPerformance {
    #[serde(default)]
    pub attribute_attempt_facts: Option<Vec<FooterScenarioAttributeAttemptFact>>,
    #[serde(default)]
    pub attribute_scenario_facts: Option<Vec<FooterScenarioAttributeScenarioFact>>,
    #[serde(default)]
    pub valid_parameter_ids: Option<Vec<String>>,
    #[serde(default)]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FooterScenarioSimulationFact {
    #[serde(default)]
    pub scenario_id: Option<String>,
    #[serde(default)]
    pub simulation_id: Option<String>,
    #[serde(default)]
    pub simulation_name: Option<String>,
    #[serde(default)]
    pub avg_score: Option<f64>,
    #[serde(default)]
    pub success_rate: Option<f64>,
    #[serde(default)]
    pub total_attempts: Option<i64>,
    #[serde(default)]
    pub completed_attempts: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FooterScenarioSimulationPerformance {
    #[serde(default)]
    pub simulation_facts: Option<Vec<FooterScenarioSimulationFact>>,
    #[serde(default)]
    pub valid_scenario_ids: Option<Vec<String>>,
    #[serde(default)]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FooterScenarioStats {
    #[serde(default)]
    pub numeric_attempt_facts: Option<Vec<FooterNumericAttemptFact>>,
    #[serde(default)]
    pub numeric_scenario_facts: Option<Vec<FooterNumericScenarioFact>>,
    #[serde(default)]
    pub valid_numeric_parameter_ids: Option<Vec<String>>,
    #[serde(default)]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateConfig {
    #[serde(default)]
    pub operations: Option<Vec<String>>,
    #[serde(default)]
    pub dangerous: Option<bool>,
    #[serde(default)]
    pub params: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(default)]
    pub group_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationsAgentApiRequest {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub date_from: Option<String>,
    #[serde(default)]
    pub date_to: Option<String>,
    #[serde(default)]
    pub page_limit: Option<i64>,
    #[serde(default)]
    pub page_offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationsAgentApiResponse {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub items: Option<Vec<GenerationsAgentListItem>>,
    #[serde(default)]
    pub total_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationsAgentListItem {
    pub group_id: String,
    #[serde(default)]
    pub session_id: Option<String>,
    #[serde(default)]
    pub group_name: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationsAttemptApiRequest {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub date_from: Option<String>,
    #[serde(default)]
    pub date_to: Option<String>,
    #[serde(default)]
    pub page_limit: Option<i64>,
    #[serde(default)]
    pub page_offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationsAttemptApiResponse {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub items: Option<Vec<GenerationsAttemptListItem>>,
    #[serde(default)]
    pub total_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationsAttemptListItem {
    pub group_id: String,
    #[serde(default)]
    pub session_id: Option<String>,
    #[serde(default)]
    pub group_name: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationsAuthApiRequest {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub date_from: Option<String>,
    #[serde(default)]
    pub date_to: Option<String>,
    #[serde(default)]
    pub page_limit: Option<i64>,
    #[serde(default)]
    pub page_offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationsAuthApiResponse {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub items: Option<Vec<GenerationsAuthListItem>>,
    #[serde(default)]
    pub total_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationsAuthListItem {
    pub group_id: String,
    #[serde(default)]
    pub session_id: Option<String>,
    #[serde(default)]
    pub group_name: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationsCohortApiRequest {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub date_from: Option<String>,
    #[serde(default)]
    pub date_to: Option<String>,
    #[serde(default)]
    pub page_limit: Option<i64>,
    #[serde(default)]
    pub page_offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationsCohortApiResponse {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub items: Option<Vec<GenerationsCohortListItem>>,
    #[serde(default)]
    pub total_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationsCohortListItem {
    pub group_id: String,
    #[serde(default)]
    pub session_id: Option<String>,
    #[serde(default)]
    pub group_name: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationsDepartmentApiRequest {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub date_from: Option<String>,
    #[serde(default)]
    pub date_to: Option<String>,
    #[serde(default)]
    pub page_limit: Option<i64>,
    #[serde(default)]
    pub page_offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationsDepartmentApiResponse {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub items: Option<Vec<GenerationsDepartmentListItem>>,
    #[serde(default)]
    pub total_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationsDepartmentListItem {
    pub group_id: String,
    #[serde(default)]
    pub session_id: Option<String>,
    #[serde(default)]
    pub group_name: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationsDocumentApiRequest {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub date_from: Option<String>,
    #[serde(default)]
    pub date_to: Option<String>,
    #[serde(default)]
    pub page_limit: Option<i64>,
    #[serde(default)]
    pub page_offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationsDocumentApiResponse {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub items: Option<Vec<GenerationsDocumentListItem>>,
    #[serde(default)]
    pub total_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationsDocumentListItem {
    pub group_id: String,
    #[serde(default)]
    pub session_id: Option<String>,
    #[serde(default)]
    pub group_name: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationsEvalApiRequest {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub date_from: Option<String>,
    #[serde(default)]
    pub date_to: Option<String>,
    #[serde(default)]
    pub page_limit: Option<i64>,
    #[serde(default)]
    pub page_offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationsEvalApiResponse {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub items: Option<Vec<GenerationsEvalListItem>>,
    #[serde(default)]
    pub total_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationsEvalListItem {
    pub group_id: String,
    #[serde(default)]
    pub session_id: Option<String>,
    #[serde(default)]
    pub group_name: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationsFieldApiRequest {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub date_from: Option<String>,
    #[serde(default)]
    pub date_to: Option<String>,
    #[serde(default)]
    pub page_limit: Option<i64>,
    #[serde(default)]
    pub page_offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationsFieldApiResponse {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub items: Option<Vec<GenerationsFieldListItem>>,
    #[serde(default)]
    pub total_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationsFieldListItem {
    pub group_id: String,
    #[serde(default)]
    pub session_id: Option<String>,
    #[serde(default)]
    pub group_name: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationsModelApiRequest {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub date_from: Option<String>,
    #[serde(default)]
    pub date_to: Option<String>,
    #[serde(default)]
    pub page_limit: Option<i64>,
    #[serde(default)]
    pub page_offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationsModelApiResponse {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub items: Option<Vec<GenerationsModelListItem>>,
    #[serde(default)]
    pub total_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationsModelListItem {
    pub group_id: String,
    #[serde(default)]
    pub session_id: Option<String>,
    #[serde(default)]
    pub group_name: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationsParameterApiRequest {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub date_from: Option<String>,
    #[serde(default)]
    pub date_to: Option<String>,
    #[serde(default)]
    pub page_limit: Option<i64>,
    #[serde(default)]
    pub page_offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationsParameterApiResponse {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub items: Option<Vec<GenerationsParameterListItem>>,
    #[serde(default)]
    pub total_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationsParameterListItem {
    pub group_id: String,
    #[serde(default)]
    pub session_id: Option<String>,
    #[serde(default)]
    pub group_name: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationsPersonaApiRequest {
    #[serde(default)]
    pub snapshot_key: Option<String>,
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub date_from: Option<String>,
    #[serde(default)]
    pub date_to: Option<String>,
    #[serde(default)]
    pub page_limit: Option<i64>,
    #[serde(default)]
    pub page_offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationsPersonaApiResponse {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub items: Option<Vec<GenerationsPersonaListItem>>,
    #[serde(default)]
    pub total_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationsPersonaListItem {
    pub group_id: String,
    #[serde(default)]
    pub session_id: Option<String>,
    #[serde(default)]
    pub group_name: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationsProfileApiRequest {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub date_from: Option<String>,
    #[serde(default)]
    pub date_to: Option<String>,
    #[serde(default)]
    pub page_limit: Option<i64>,
    #[serde(default)]
    pub page_offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationsProfileApiResponse {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub items: Option<Vec<GenerationsProfileListItem>>,
    #[serde(default)]
    pub total_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationsProfileListItem {
    pub group_id: String,
    #[serde(default)]
    pub session_id: Option<String>,
    #[serde(default)]
    pub group_name: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationsProviderApiRequest {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub date_from: Option<String>,
    #[serde(default)]
    pub date_to: Option<String>,
    #[serde(default)]
    pub page_limit: Option<i64>,
    #[serde(default)]
    pub page_offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationsProviderApiResponse {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub items: Option<Vec<GenerationsProviderListItem>>,
    #[serde(default)]
    pub total_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationsProviderListItem {
    pub group_id: String,
    #[serde(default)]
    pub session_id: Option<String>,
    #[serde(default)]
    pub group_name: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationsRubricApiRequest {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub date_from: Option<String>,
    #[serde(default)]
    pub date_to: Option<String>,
    #[serde(default)]
    pub page_limit: Option<i64>,
    #[serde(default)]
    pub page_offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationsRubricApiResponse {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub items: Option<Vec<GenerationsRubricListItem>>,
    #[serde(default)]
    pub total_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationsRubricListItem {
    pub group_id: String,
    #[serde(default)]
    pub session_id: Option<String>,
    #[serde(default)]
    pub group_name: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationsScenarioApiRequest {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub date_from: Option<String>,
    #[serde(default)]
    pub date_to: Option<String>,
    #[serde(default)]
    pub page_limit: Option<i64>,
    #[serde(default)]
    pub page_offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationsScenarioApiResponse {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub items: Option<Vec<GenerationsScenarioListItem>>,
    #[serde(default)]
    pub total_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationsScenarioListItem {
    pub group_id: String,
    #[serde(default)]
    pub session_id: Option<String>,
    #[serde(default)]
    pub group_name: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationsSettingApiRequest {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub date_from: Option<String>,
    #[serde(default)]
    pub date_to: Option<String>,
    #[serde(default)]
    pub page_limit: Option<i64>,
    #[serde(default)]
    pub page_offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationsSettingApiResponse {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub items: Option<Vec<GenerationsSettingListItem>>,
    #[serde(default)]
    pub total_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationsSettingListItem {
    pub group_id: String,
    #[serde(default)]
    pub session_id: Option<String>,
    #[serde(default)]
    pub group_name: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationsSimulationApiRequest {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub date_from: Option<String>,
    #[serde(default)]
    pub date_to: Option<String>,
    #[serde(default)]
    pub page_limit: Option<i64>,
    #[serde(default)]
    pub page_offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationsSimulationApiResponse {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub items: Option<Vec<GenerationsSimulationListItem>>,
    #[serde(default)]
    pub total_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationsSimulationListItem {
    pub group_id: String,
    #[serde(default)]
    pub session_id: Option<String>,
    #[serde(default)]
    pub group_name: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationsSystemApiRequest {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub date_from: Option<String>,
    #[serde(default)]
    pub date_to: Option<String>,
    #[serde(default)]
    pub page_limit: Option<i64>,
    #[serde(default)]
    pub page_offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationsSystemApiResponse {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub items: Option<Vec<GenerationsSystemListItem>>,
    #[serde(default)]
    pub total_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationsSystemListItem {
    pub group_id: String,
    #[serde(default)]
    pub session_id: Option<String>,
    #[serde(default)]
    pub group_name: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationsTestApiRequest {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub date_from: Option<String>,
    #[serde(default)]
    pub date_to: Option<String>,
    #[serde(default)]
    pub page_limit: Option<i64>,
    #[serde(default)]
    pub page_offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationsTestApiResponse {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub items: Option<Vec<GenerationsTestListItem>>,
    #[serde(default)]
    pub total_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationsTestListItem {
    pub group_id: String,
    #[serde(default)]
    pub session_id: Option<String>,
    #[serde(default)]
    pub group_name: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationsToolApiRequest {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub date_from: Option<String>,
    #[serde(default)]
    pub date_to: Option<String>,
    #[serde(default)]
    pub page_limit: Option<i64>,
    #[serde(default)]
    pub page_offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationsToolApiResponse {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub items: Option<Vec<GenerationsToolListItem>>,
    #[serde(default)]
    pub total_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationsToolListItem {
    pub group_id: String,
    #[serde(default)]
    pub session_id: Option<String>,
    #[serde(default)]
    pub group_name: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAgentApiRequest {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub agent_id: Option<String>,
    #[serde(default)]
    pub draft_id: Option<String>,
    #[serde(default)]
    pub snapshot_key: Option<String>,
    #[serde(default)]
    pub names: Option<App__infra__agent__types__SectionFilter>,
    #[serde(default)]
    pub descriptions: Option<App__infra__agent__types__SectionFilter>,
    #[serde(default)]
    pub models: Option<App__infra__agent__types__SectionFilter>,
    #[serde(default)]
    pub prompts: Option<App__infra__agent__types__SectionFilter>,
    #[serde(default)]
    pub instructions: Option<App__infra__agent__types__SectionFilter>,
    #[serde(default)]
    pub flags: Option<App__infra__agent__types__SectionFilter>,
    #[serde(default)]
    pub departments: Option<App__infra__agent__types__SectionFilter>,
    #[serde(default)]
    pub tools: Option<App__infra__agent__types__SectionFilter>,
    #[serde(default)]
    pub temperature_levels: Option<App__infra__agent__types__SectionFilter>,
    #[serde(default)]
    pub reasoning_levels: Option<App__infra__agent__types__SectionFilter>,
    #[serde(default)]
    pub voices: Option<App__infra__agent__types__SectionFilter>,
    #[serde(default)]
    pub qualities: Option<App__infra__agent__types__SectionFilter>,
    #[serde(default)]
    pub rubrics: Option<App__infra__agent__types__SectionFilter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAgentApiResponse {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub agent_exists: Option<bool>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub disabled_reason: Option<String>,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub draft_name: Option<String>,
    #[serde(default)]
    pub agent_id: Option<String>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub basic_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub general_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub pending_ids: Option<Vec<String>>,
    #[serde(default)]
    pub names: Option<Vec<AgentNameResource>>,
    #[serde(default)]
    pub descriptions: Option<Vec<AgentDescriptionResource>>,
    #[serde(default)]
    pub models: Option<Vec<AgentModelResource>>,
    #[serde(default)]
    pub prompts: Option<Vec<AgentPromptResource>>,
    #[serde(default)]
    pub instructions: Option<Vec<AgentInstructionResource>>,
    #[serde(default)]
    pub flags: Option<Vec<AgentFlagResource>>,
    #[serde(default)]
    pub departments: Option<Vec<AgentDepartmentResource>>,
    #[serde(default)]
    pub tools: Option<Vec<AgentToolResource>>,
    #[serde(default)]
    pub temperature_levels: Option<Vec<AgentTemperatureLevelResource>>,
    #[serde(default)]
    pub reasoning_levels: Option<Vec<AgentReasoningLevelResource>>,
    #[serde(default)]
    pub voices: Option<Vec<AgentVoiceResource>>,
    #[serde(default)]
    pub qualities: Option<Vec<AgentQualityResource>>,
    #[serde(default)]
    pub rubrics: Option<Vec<AgentRubricResource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAgentDraftResponse {
    pub id: String,
    pub created_at: String,
    pub generated: bool,
    pub mcp: bool,
    pub active: bool,
    pub session_id: String,
    pub name_ids: Vec<String>,
    pub description_ids: Vec<String>,
    pub flag_ids: Vec<String>,
    pub department_ids: Vec<String>,
    pub model_ids: Vec<String>,
    pub tool_ids: Vec<String>,
    pub profile_ids: Vec<String>,
    pub reasoning_level_ids: Vec<String>,
    pub temperature_level_ids: Vec<String>,
    pub voice_ids: Vec<String>,
    pub quality_ids: Vec<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub rubric_ids: Option<Vec<String>>,
    #[serde(default)]
    pub prompt_ids: Option<Vec<String>>,
    #[serde(default)]
    pub instruction_ids: Option<Vec<String>>,
    #[serde(default)]
    pub agent_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_name_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_description_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_model_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_tool_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_reasoning_level_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_temperature_level_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_voice_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_quality_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_rubric_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_prompt_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_instruction_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_agent_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAgentDraftsApiRequest {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub date_from: Option<String>,
    #[serde(default)]
    pub date_to: Option<String>,
    #[serde(default)]
    pub page_limit: Option<i64>,
    #[serde(default)]
    pub page_offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAgentDraftsApiResponse {
    #[serde(default)]
    pub entries: Option<Vec<GetAgentDraftResponse>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAttemptDetailRequest {
    pub attempt_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAttemptDetailResponse {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub attempt_exists: Option<bool>,
    #[serde(default)]
    pub access_denied: Option<bool>,
    #[serde(default)]
    pub attempt: Option<AttemptData>,
    #[serde(default)]
    pub simulation: Option<SimulationData>,
    #[serde(default)]
    pub timer: Option<TimerData>,
    #[serde(default)]
    pub aggregated_results: Option<AggregatedResults>,
    #[serde(default)]
    pub current_chat_index: Option<i64>,
    #[serde(default)]
    pub expected_chat_count: Option<i64>,
    #[serde(default)]
    pub is_active: Option<bool>,
    #[serde(default)]
    pub is_lobby: Option<bool>,
    #[serde(default)]
    pub show_results: Option<bool>,
    #[serde(default)]
    pub should_show_controls: Option<bool>,
    #[serde(default)]
    pub is_own_attempt: Option<bool>,
    #[serde(default)]
    pub current_chat_id: Option<String>,
    #[serde(default)]
    pub has_messages: Option<bool>,
    #[serde(default)]
    pub available_continuation_options: Option<AvailableContinuationOptions>,
    #[serde(default)]
    pub rubric_structure: Option<RubricStructureData>,
    #[serde(default)]
    pub training_id: Option<String>,
    #[serde(default)]
    pub chat_entry_id: Option<String>,
    #[serde(default)]
    pub next_chat_entry_id: Option<String>,
    #[serde(default)]
    pub resources: Option<AttemptResources>,
    #[serde(default)]
    pub entries: Option<AttemptEntries>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAttemptResponse {
    pub attempt_id: String,
    pub simulation_id: Option<String>,
    pub profile_id: Option<String>,
    pub user_persona_id: Option<String>,
    pub personas_id: Option<String>,
    pub cohort_id: Option<String>,
    pub department_id: Option<String>,
    pub practice: bool,
    pub attempt_created_at: String,
    pub infinite_mode: bool,
    pub num_chats: i64,
    pub scenario_ids: Vec<String>,
    pub chat_entry_id: Option<String>,
    pub attempt_chat_id: Option<String>,
    #[serde(default)]
    pub role_id: Option<String>,
    #[serde(default)]
    pub is_archived: Option<bool>,
    #[serde(default)]
    pub is_completed: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAuthApiRequest {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub auth_id: Option<String>,
    #[serde(default)]
    pub draft_id: Option<String>,
    #[serde(default)]
    pub snapshot_key: Option<String>,
    #[serde(default)]
    pub names: Option<App__infra__auth__types__SectionFilter>,
    #[serde(default)]
    pub descriptions: Option<App__infra__auth__types__SectionFilter>,
    #[serde(default)]
    pub flags: Option<App__infra__auth__types__SectionFilter>,
    #[serde(default)]
    pub departments: Option<App__infra__auth__types__SectionFilter>,
    #[serde(default)]
    pub protocols: Option<App__infra__auth__types__SectionFilter>,
    #[serde(default)]
    pub slugs: Option<App__infra__auth__types__SectionFilter>,
    #[serde(default)]
    pub items: Option<App__infra__auth__types__SectionFilter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAuthApiResponse {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub auth_exists: Option<bool>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub disabled_reason: Option<String>,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub draft_name: Option<String>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub basic_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub pending_ids: Option<Vec<String>>,
    #[serde(default)]
    pub names: Option<Vec<AuthNameResource>>,
    #[serde(default)]
    pub descriptions: Option<Vec<AuthDescriptionResource>>,
    #[serde(default)]
    pub flags: Option<Vec<AuthFlagResource>>,
    #[serde(default)]
    pub departments: Option<Vec<AuthDepartmentResource>>,
    #[serde(default)]
    pub protocols: Option<Vec<AuthProtocolResource>>,
    #[serde(default)]
    pub slugs: Option<Vec<AuthSlugResource>>,
    #[serde(default)]
    pub items: Option<Vec<AuthItemResource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAuthDraftResponse {
    pub id: String,
    pub created_at: String,
    pub generated: bool,
    pub mcp: bool,
    pub active: bool,
    pub session_id: String,
    pub department_ids: Vec<String>,
    pub description_ids: Vec<String>,
    pub flag_ids: Vec<String>,
    pub item_ids: Vec<String>,
    pub name_ids: Vec<String>,
    pub profile_ids: Vec<String>,
    pub protocol_ids: Vec<String>,
    pub slug_ids: Vec<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub pending_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_description_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_item_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_name_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_protocol_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_slug_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAuthDraftsApiRequest {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub date_from: Option<String>,
    #[serde(default)]
    pub date_to: Option<String>,
    #[serde(default)]
    pub page_limit: Option<i64>,
    #[serde(default)]
    pub page_offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAuthDraftsApiResponse {
    #[serde(default)]
    pub entries: Option<Vec<GetAuthDraftResponse>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetChatDraftResponse {
    pub id: String,
    pub created_at: String,
    pub generated: bool,
    pub mcp: bool,
    pub active: bool,
    pub session_id: String,
    pub department_ids: Vec<String>,
    pub description_ids: Vec<String>,
    pub document_ids: Vec<String>,
    pub field_ids: Vec<String>,
    pub flag_ids: Vec<String>,
    pub image_ids: Vec<String>,
    pub name_ids: Vec<String>,
    pub objective_ids: Vec<String>,
    pub option_ids: Vec<String>,
    pub parameter_field_ids: Vec<String>,
    pub parameter_ids: Vec<String>,
    pub persona_ids: Vec<String>,
    pub problem_statement_ids: Vec<String>,
    pub profile_ids: Vec<String>,
    pub question_ids: Vec<String>,
    pub scenario_ids: Vec<String>,
    pub video_ids: Vec<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub pending_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_description_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_document_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_field_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_image_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_name_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_objective_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_option_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_parameter_field_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_parameter_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_persona_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_problem_statement_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_question_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_scenario_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_video_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetChatDraftsApiRequest {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub date_from: Option<String>,
    #[serde(default)]
    pub date_to: Option<String>,
    #[serde(default)]
    pub page_limit: Option<i64>,
    #[serde(default)]
    pub page_offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetChatDraftsApiResponse {
    #[serde(default)]
    pub entries: Option<Vec<GetChatDraftResponse>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetChatRequest {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub chat_entry_id: Option<String>,
    #[serde(default)]
    pub attempt_id: Option<String>,
    #[serde(default)]
    pub draft_id: Option<String>,
    #[serde(default)]
    pub snapshot_key: Option<String>,
    #[serde(default)]
    pub names: Option<App__infra__attempt__chat__types__SectionFilter>,
    #[serde(default)]
    pub descriptions: Option<App__infra__attempt__chat__types__SectionFilter>,
    #[serde(default)]
    pub flags: Option<App__infra__attempt__chat__types__SectionFilter>,
    #[serde(default)]
    pub departments: Option<App__infra__attempt__chat__types__SectionFilter>,
    #[serde(default)]
    pub personas: Option<App__infra__attempt__chat__types__SectionFilter>,
    #[serde(default)]
    pub documents: Option<App__infra__attempt__chat__types__SectionFilter>,
    #[serde(default)]
    pub parameter_fields: Option<App__infra__attempt__chat__types__SectionFilter>,
    #[serde(default)]
    pub scenarios: Option<App__infra__attempt__chat__types__SectionFilter>,
    #[serde(default)]
    pub fields: Option<App__infra__attempt__chat__types__SectionFilter>,
    #[serde(default)]
    pub questions: Option<App__infra__attempt__chat__types__SectionFilter>,
    #[serde(default)]
    pub options: Option<App__infra__attempt__chat__types__SectionFilter>,
    #[serde(default)]
    pub videos: Option<App__infra__attempt__chat__types__SectionFilter>,
    #[serde(default)]
    pub images: Option<App__infra__attempt__chat__types__SectionFilter>,
    #[serde(default)]
    pub problem_statements: Option<App__infra__attempt__chat__types__SectionFilter>,
    #[serde(default)]
    pub objectives: Option<App__infra__attempt__chat__types__SectionFilter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetChatResponse {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub chat_exists: Option<bool>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub disabled_reason: Option<String>,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub profile_has_access: Option<bool>,
    #[serde(default)]
    pub simulation_name: Option<String>,
    #[serde(default)]
    pub chat_entry_id: Option<String>,
    #[serde(default)]
    pub attempt_id: Option<String>,
    #[serde(default)]
    pub names: Option<Vec<ChatNameResource>>,
    #[serde(default)]
    pub descriptions: Option<Vec<ChatDescriptionResource>>,
    #[serde(default)]
    pub flags: Option<Vec<ChatFlagResource>>,
    #[serde(default)]
    pub departments: Option<Vec<ChatDepartmentResource>>,
    #[serde(default)]
    pub personas: Option<Vec<ChatPersonaResource>>,
    #[serde(default)]
    pub documents: Option<Vec<ChatDocumentResource>>,
    #[serde(default)]
    pub parameter_fields: Option<Vec<ChatParameterFieldResource>>,
    #[serde(default)]
    pub scenarios: Option<Vec<ChatScenarioResource>>,
    #[serde(default)]
    pub fields: Option<Vec<ChatFieldResource>>,
    #[serde(default)]
    pub questions: Option<Vec<ChatQuestionResource>>,
    #[serde(default)]
    pub options: Option<Vec<ChatOptionResource>>,
    #[serde(default)]
    pub videos: Option<Vec<ChatVideoResource>>,
    #[serde(default)]
    pub images: Option<Vec<ChatImageResource>>,
    #[serde(default)]
    pub problem_statements: Option<Vec<ChatProblemStatementResource>>,
    #[serde(default)]
    pub objectives: Option<Vec<ChatObjectiveResource>>,
    #[serde(default)]
    pub name_ids: Option<Vec<String>>,
    #[serde(default)]
    pub description_ids: Option<Vec<String>>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub persona_ids: Option<Vec<String>>,
    #[serde(default)]
    pub document_ids: Option<Vec<String>>,
    #[serde(default)]
    pub parameter_field_ids: Option<Vec<String>>,
    #[serde(default)]
    pub scenario_ids: Option<Vec<String>>,
    #[serde(default)]
    pub field_ids: Option<Vec<String>>,
    #[serde(default)]
    pub question_ids: Option<Vec<String>>,
    #[serde(default)]
    pub option_ids: Option<Vec<String>>,
    #[serde(default)]
    pub video_ids: Option<Vec<String>>,
    #[serde(default)]
    pub image_ids: Option<Vec<String>>,
    #[serde(default)]
    pub problem_statement_ids: Option<Vec<String>>,
    #[serde(default)]
    pub objective_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCohortApiRequest {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub cohort_id: Option<String>,
    #[serde(default)]
    pub draft_id: Option<String>,
    #[serde(default)]
    pub snapshot_key: Option<String>,
    #[serde(default)]
    pub names: Option<App__infra__cohort__types__SectionFilter>,
    #[serde(default)]
    pub descriptions: Option<App__infra__cohort__types__SectionFilter>,
    #[serde(default)]
    pub flags: Option<App__infra__cohort__types__SectionFilter>,
    #[serde(default)]
    pub departments: Option<App__infra__cohort__types__SectionFilter>,
    #[serde(default)]
    pub simulations: Option<App__infra__cohort__types__SectionFilter>,
    #[serde(default)]
    pub simulation_positions: Option<App__infra__cohort__types__SectionFilter>,
    #[serde(default)]
    pub simulation_availability: Option<App__infra__cohort__types__SectionFilter>,
    #[serde(default)]
    pub profiles: Option<App__infra__cohort__types__SectionFilter>,
    #[serde(default)]
    pub profile_personas: Option<App__infra__cohort__types__SectionFilter>,
    #[serde(default)]
    pub personas: Option<App__infra__cohort__types__SectionFilter>,
    #[serde(default)]
    pub descriptions_search: Option<String>,
    #[serde(default)]
    pub simulation_search: Option<String>,
    #[serde(default)]
    pub simulation_show_selected: Option<bool>,
    #[serde(default)]
    pub profile_search: Option<String>,
    #[serde(default)]
    pub profile_show_selected: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCohortApiResponse {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub cohort_exists: Option<bool>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub disabled_reason: Option<String>,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub draft_name: Option<String>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub names: Option<Vec<CohortNameResource>>,
    #[serde(default)]
    pub descriptions: Option<Vec<CohortDescriptionResource>>,
    #[serde(default)]
    pub flags: Option<Vec<CohortFlagResource>>,
    #[serde(default)]
    pub departments: Option<Vec<CohortDepartment>>,
    #[serde(default)]
    pub simulations: Option<Vec<CohortSimulation>>,
    #[serde(default)]
    pub simulation_positions: Option<Vec<CohortSimulationPosition>>,
    #[serde(default)]
    pub simulation_availability: Option<Vec<CohortSimulationAvailability>>,
    #[serde(default)]
    pub profiles: Option<Vec<CohortProfile>>,
    #[serde(default)]
    pub profile_personas: Option<Vec<CohortProfilePersona>>,
    #[serde(default)]
    pub personas: Option<Vec<CohortPersonaResource>>,
    #[serde(default)]
    pub pending_ids: Option<Vec<String>>,
    #[serde(default)]
    pub basic_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub simulations_step_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub profiles_step_show_ai_generate: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCohortDraftResponse {
    pub id: String,
    pub created_at: String,
    pub generated: bool,
    pub mcp: bool,
    pub active: bool,
    pub session_id: String,
    pub department_ids: Vec<String>,
    pub description_ids: Vec<String>,
    pub flag_ids: Vec<String>,
    pub name_ids: Vec<String>,
    pub profile_persona_ids: Vec<String>,
    pub profile_ids: Vec<String>,
    pub simulation_availability_ids: Vec<String>,
    pub simulation_position_ids: Vec<String>,
    pub simulation_ids: Vec<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub pending_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_description_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_name_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_profile_persona_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_profile_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_simulation_availability_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_simulation_position_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_simulation_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCohortDraftsApiRequest {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub date_from: Option<String>,
    #[serde(default)]
    pub date_to: Option<String>,
    #[serde(default)]
    pub page_limit: Option<i64>,
    #[serde(default)]
    pub page_offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCohortDraftsApiResponse {
    #[serde(default)]
    pub entries: Option<Vec<GetCohortDraftResponse>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDepartmentApiRequest {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub department_id: Option<String>,
    #[serde(default)]
    pub draft_id: Option<String>,
    #[serde(default)]
    pub snapshot_key: Option<String>,
    #[serde(default)]
    pub names: Option<App__infra__department__types__SectionFilter>,
    #[serde(default)]
    pub descriptions: Option<App__infra__department__types__SectionFilter>,
    #[serde(default)]
    pub flags: Option<App__infra__department__types__SectionFilter>,
    #[serde(default)]
    pub settings: Option<App__infra__department__types__SectionFilter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDepartmentApiResponse {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub department_exists: Option<bool>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub disabled_reason: Option<String>,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub draft_name: Option<String>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub basic_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub pending_ids: Option<Vec<String>>,
    #[serde(default)]
    pub names: Option<Vec<DepartmentNameResource>>,
    #[serde(default)]
    pub descriptions: Option<Vec<DepartmentDescriptionResource>>,
    #[serde(default)]
    pub flags: Option<Vec<DepartmentFlagResource>>,
    #[serde(default)]
    pub settings: Option<Vec<DepartmentSettingResource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDepartmentDraftResponse {
    pub id: String,
    pub created_at: String,
    pub generated: bool,
    pub mcp: bool,
    pub active: bool,
    pub session_id: String,
    pub description_ids: Vec<String>,
    pub flag_ids: Vec<String>,
    pub name_ids: Vec<String>,
    pub profile_ids: Vec<String>,
    pub setting_ids: Vec<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub pending_description_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_name_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_setting_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDepartmentDraftsApiRequest {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub date_from: Option<String>,
    #[serde(default)]
    pub date_to: Option<String>,
    #[serde(default)]
    pub page_limit: Option<i64>,
    #[serde(default)]
    pub page_offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDepartmentDraftsApiResponse {
    #[serde(default)]
    pub entries: Option<Vec<GetDepartmentDraftResponse>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocumentApiRequest {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub document_id: Option<String>,
    #[serde(default)]
    pub draft_id: Option<String>,
    #[serde(default)]
    pub snapshot_key: Option<String>,
    #[serde(default)]
    pub names: Option<App__infra__document__types__SectionFilter>,
    #[serde(default)]
    pub descriptions: Option<App__infra__document__types__SectionFilter>,
    #[serde(default)]
    pub flags: Option<App__infra__document__types__SectionFilter>,
    #[serde(default)]
    pub departments: Option<App__infra__document__types__SectionFilter>,
    #[serde(default)]
    pub parameter_fields: Option<App__infra__document__types__SectionFilter>,
    #[serde(default)]
    pub parameters: Option<App__infra__document__types__SectionFilter>,
    #[serde(default)]
    pub files: Option<App__infra__document__types__SectionFilter>,
    #[serde(default)]
    pub images: Option<App__infra__document__types__SectionFilter>,
    #[serde(default)]
    pub texts: Option<App__infra__document__types__SectionFilter>,
    #[serde(default)]
    pub fields: Option<App__infra__document__types__SectionFilter>,
    #[serde(default)]
    pub uploads: Option<App__infra__document__types__SectionFilter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocumentApiResponse {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub document_exists: Option<bool>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub disabled_reason: Option<String>,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub draft_name: Option<String>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub basic_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub content_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub pending_ids: Option<Vec<String>>,
    #[serde(default)]
    pub names: Option<Vec<DocumentNameResource>>,
    #[serde(default)]
    pub descriptions: Option<Vec<DocumentDescriptionResource>>,
    #[serde(default)]
    pub flags: Option<Vec<DocumentFlagResource>>,
    #[serde(default)]
    pub departments: Option<Vec<DocumentDepartmentResource>>,
    #[serde(default)]
    pub parameter_fields: Option<Vec<DocumentParameterFieldResource>>,
    #[serde(default)]
    pub parameters: Option<Vec<DocumentParameterResource>>,
    #[serde(default)]
    pub files: Option<Vec<DocumentFileResource>>,
    #[serde(default)]
    pub images: Option<Vec<DocumentImageResource>>,
    #[serde(default)]
    pub texts: Option<Vec<DocumentTextResource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocumentDraftResponse {
    pub id: String,
    pub created_at: String,
    pub generated: bool,
    pub mcp: bool,
    pub active: bool,
    pub session_id: String,
    pub department_ids: Vec<String>,
    pub description_ids: Vec<String>,
    pub file_ids: Vec<String>,
    pub flag_ids: Vec<String>,
    pub image_ids: Vec<String>,
    pub name_ids: Vec<String>,
    pub parameter_field_ids: Vec<String>,
    pub parameter_ids: Vec<String>,
    pub profile_ids: Vec<String>,
    pub text_ids: Vec<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub pending_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_description_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_file_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_image_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_name_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_parameter_field_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_parameter_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_text_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocumentDraftsApiRequest {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub date_from: Option<String>,
    #[serde(default)]
    pub date_to: Option<String>,
    #[serde(default)]
    pub page_limit: Option<i64>,
    #[serde(default)]
    pub page_offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocumentDraftsApiResponse {
    #[serde(default)]
    pub entries: Option<Vec<GetDocumentDraftResponse>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetEvalApiRequest {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub eval_id: Option<String>,
    #[serde(default)]
    pub draft_id: Option<String>,
    #[serde(default)]
    pub snapshot_key: Option<String>,
    #[serde(default)]
    pub names: Option<App__infra__eval__types__SectionFilter>,
    #[serde(default)]
    pub descriptions: Option<App__infra__eval__types__SectionFilter>,
    #[serde(default)]
    pub flags: Option<App__infra__eval__types__SectionFilter>,
    #[serde(default)]
    pub departments: Option<App__infra__eval__types__SectionFilter>,
    #[serde(default)]
    pub models: Option<App__infra__eval__types__SectionFilter>,
    #[serde(default)]
    pub model_flags: Option<App__infra__eval__types__SectionFilter>,
    #[serde(default)]
    pub model_rubrics: Option<App__infra__eval__types__SectionFilter>,
    #[serde(default)]
    pub model_positions: Option<App__infra__eval__types__SectionFilter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetEvalApiResponse {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub eval_exists: Option<bool>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub disabled_reason: Option<String>,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub draft_name: Option<String>,
    #[serde(default)]
    pub basic_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub model_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub pending_ids: Option<Vec<String>>,
    #[serde(default)]
    pub names: Option<Vec<EvalNameResource>>,
    #[serde(default)]
    pub descriptions: Option<Vec<EvalDescriptionResource>>,
    #[serde(default)]
    pub flags: Option<Vec<EvalFlagResource>>,
    #[serde(default)]
    pub departments: Option<Vec<EvalDepartmentResource>>,
    #[serde(default)]
    pub models: Option<Vec<EvalModelResource>>,
    #[serde(default)]
    pub model_flags: Option<Vec<EvalModelFlagResource>>,
    #[serde(default)]
    pub model_flag_options: Option<Vec<EvalModelFlagOptionResource>>,
    #[serde(default)]
    pub model_rubrics: Option<Vec<EvalModelRubricResource>>,
    #[serde(default)]
    pub model_positions: Option<Vec<EvalModelPositionResource>>,
    #[serde(default)]
    pub rubrics: Option<Vec<EvalRubricResource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetEvalDraftResponse {
    pub id: String,
    pub created_at: String,
    pub generated: bool,
    pub mcp: bool,
    pub active: bool,
    pub session_id: String,
    pub department_ids: Vec<String>,
    pub description_ids: Vec<String>,
    pub flag_ids: Vec<String>,
    pub model_ids: Vec<String>,
    pub name_ids: Vec<String>,
    pub profile_ids: Vec<String>,
    pub rubric_ids: Vec<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub model_flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub model_position_ids: Option<Vec<String>>,
    #[serde(default)]
    pub model_rubric_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_description_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_model_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_name_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_rubric_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_model_flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_model_position_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_model_rubric_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetEvalDraftsApiRequest {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub date_from: Option<String>,
    #[serde(default)]
    pub date_to: Option<String>,
    #[serde(default)]
    pub page_limit: Option<i64>,
    #[serde(default)]
    pub page_offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetEvalDraftsApiResponse {
    #[serde(default)]
    pub entries: Option<Vec<GetEvalDraftResponse>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFieldApiRequest {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub field_id: Option<String>,
    #[serde(default)]
    pub draft_id: Option<String>,
    #[serde(default)]
    pub snapshot_key: Option<String>,
    #[serde(default)]
    pub names: Option<App__infra__field__types__SectionFilter>,
    #[serde(default)]
    pub descriptions: Option<App__infra__field__types__SectionFilter>,
    #[serde(default)]
    pub flags: Option<App__infra__field__types__SectionFilter>,
    #[serde(default)]
    pub departments: Option<App__infra__field__types__SectionFilter>,
    #[serde(default)]
    pub conditional_parameters: Option<App__infra__field__types__SectionFilter>,
    #[serde(default)]
    pub descriptions_search: Option<String>,
    #[serde(default)]
    pub conditional_parameter_search: Option<String>,
    #[serde(default)]
    pub conditional_parameter_show_selected: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFieldApiResponse {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub field_exists: Option<bool>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub disabled_reason: Option<String>,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub draft_name: Option<String>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub basic_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub pending_ids: Option<Vec<String>>,
    #[serde(default)]
    pub names: Option<Vec<FieldNameResource>>,
    #[serde(default)]
    pub descriptions: Option<Vec<FieldDescriptionResource>>,
    #[serde(default)]
    pub flags: Option<Vec<FieldFlagResource>>,
    #[serde(default)]
    pub departments: Option<Vec<FieldDepartmentResource>>,
    #[serde(default)]
    pub conditional_parameters: Option<Vec<FieldConditionalParameterResource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFieldDraftResponse {
    pub id: String,
    pub created_at: String,
    pub generated: bool,
    pub mcp: bool,
    pub active: bool,
    pub session_id: String,
    pub conditional_parameter_ids: Vec<String>,
    pub department_ids: Vec<String>,
    pub description_ids: Vec<String>,
    pub flag_ids: Vec<String>,
    pub name_ids: Vec<String>,
    pub profile_ids: Vec<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub pending_conditional_parameter_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_description_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_name_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFieldDraftsApiRequest {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub date_from: Option<String>,
    #[serde(default)]
    pub date_to: Option<String>,
    #[serde(default)]
    pub page_limit: Option<i64>,
    #[serde(default)]
    pub page_offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFieldDraftsApiResponse {
    #[serde(default)]
    pub entries: Option<Vec<GetFieldDraftResponse>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFieldResponse {
    pub id: String,
    pub name: String,
    pub description: String,
    pub value: String,
    pub department_ids: Vec<String>,
    pub conditional_parameter_ids: Vec<String>,
    pub created_at: String,
    pub active: bool,
    pub generated: bool,
    pub mcp: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetHealthResponse {
    pub date_hour: String,
    pub service: String,
    pub check_count: i64,
    pub ok_count: i64,
    pub fail_count: i64,
    pub uptime_percent: f64,
    pub avg_latency_ms: f64,
    pub min_latency_ms: f64,
    pub max_latency_ms: f64,
    pub latest_ok: bool,
    pub latest_error: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetHomeResponse {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub items: Option<Vec<ChatSimulationOperational>>,
    #[serde(default)]
    pub rubrics: Option<Vec<RubricMapping>>,
    #[serde(default)]
    pub standard_groups: Option<Vec<StandardGroupMapping>>,
    #[serde(default)]
    pub standards: Option<Vec<StandardMapping>>,
    #[serde(default)]
    pub analytics: Option<AnalyticsFacets>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetInvocationApiRequest {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub invocation_id: Option<String>,
    #[serde(default)]
    pub benchmark_bundle_entry_id: Option<String>,
    #[serde(default)]
    pub test_id: Option<String>,
    #[serde(default)]
    pub draft_id: Option<String>,
    #[serde(default)]
    pub snapshot_key: Option<String>,
    #[serde(default)]
    pub names: Option<App__infra__invocation__types__SectionFilter>,
    #[serde(default)]
    pub descriptions: Option<App__infra__invocation__types__SectionFilter>,
    #[serde(default)]
    pub values: Option<App__infra__invocation__types__SectionFilter>,
    #[serde(default)]
    pub flags: Option<App__infra__invocation__types__SectionFilter>,
    #[serde(default)]
    pub departments: Option<App__infra__invocation__types__SectionFilter>,
    #[serde(default)]
    pub keys: Option<App__infra__invocation__types__SectionFilter>,
    #[serde(default)]
    pub endpoints: Option<App__infra__invocation__types__SectionFilter>,
    #[serde(default)]
    pub modalities: Option<App__infra__invocation__types__SectionFilter>,
    #[serde(default)]
    pub temperature_levels: Option<App__infra__invocation__types__SectionFilter>,
    #[serde(default)]
    pub pricing: Option<App__infra__invocation__types__SectionFilter>,
    #[serde(default)]
    pub reasoning_levels: Option<App__infra__invocation__types__SectionFilter>,
    #[serde(default)]
    pub qualities: Option<App__infra__invocation__types__SectionFilter>,
    #[serde(default)]
    pub voices: Option<App__infra__invocation__types__SectionFilter>,
    #[serde(default)]
    pub descriptions_search: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetInvocationDraftResponse {
    pub id: String,
    pub created_at: String,
    pub generated: bool,
    pub mcp: bool,
    pub active: bool,
    pub session_id: String,
    pub department_ids: Vec<String>,
    pub description_ids: Vec<String>,
    pub flag_ids: Vec<String>,
    pub key_ids: Vec<String>,
    pub model_flag_ids: Vec<String>,
    pub model_position_ids: Vec<String>,
    pub model_rubric_ids: Vec<String>,
    pub name_ids: Vec<String>,
    pub profile_ids: Vec<String>,
    pub reasoning_level_ids: Vec<String>,
    pub temperature_level_ids: Vec<String>,
    pub voice_ids: Vec<String>,
    pub pricing_ids: Vec<String>,
    pub endpoint_ids: Vec<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub modality_ids: Option<Vec<String>>,
    #[serde(default)]
    pub quality_ids: Option<Vec<String>>,
    #[serde(default)]
    pub value_id: Option<String>,
    #[serde(default)]
    pub pending_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_description_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_key_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_modality_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_quality_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_model_flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_model_position_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_model_rubric_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_name_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_reasoning_level_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_temperature_level_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_voice_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_value_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_pricing_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_endpoint_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetInvocationDraftsApiRequest {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub date_from: Option<String>,
    #[serde(default)]
    pub date_to: Option<String>,
    #[serde(default)]
    pub page_limit: Option<i64>,
    #[serde(default)]
    pub page_offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetInvocationDraftsApiResponse {
    #[serde(default)]
    pub entries: Option<Vec<GetInvocationDraftResponse>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMetricsSearchResponse {
    pub date_hour: String,
    pub sample_count: i64,
    pub avg_cpu_percent: f64,
    pub min_cpu_percent: f64,
    pub max_cpu_percent: f64,
    pub avg_latency_ms: f64,
    pub min_latency_ms: f64,
    pub max_latency_ms: f64,
    pub avg_memory_bytes: i64,
    pub min_memory_bytes: i64,
    pub max_memory_bytes: i64,
    pub max_requests_total: i64,
    pub max_errors_total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetModelApiRequest {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub model_id: Option<String>,
    #[serde(default)]
    pub draft_id: Option<String>,
    #[serde(default)]
    pub snapshot_key: Option<String>,
    #[serde(default)]
    pub names: Option<App__infra__model__types__SectionFilter>,
    #[serde(default)]
    pub descriptions: Option<App__infra__model__types__SectionFilter>,
    #[serde(default)]
    pub values: Option<App__infra__model__types__SectionFilter>,
    #[serde(default)]
    pub providers: Option<App__infra__model__types__SectionFilter>,
    #[serde(default)]
    pub flags: Option<App__infra__model__types__SectionFilter>,
    #[serde(default)]
    pub departments: Option<App__infra__model__types__SectionFilter>,
    #[serde(default)]
    pub modalities: Option<App__infra__model__types__SectionFilter>,
    #[serde(default)]
    pub temperature_levels: Option<App__infra__model__types__SectionFilter>,
    #[serde(default)]
    pub pricing: Option<App__infra__model__types__SectionFilter>,
    #[serde(default)]
    pub reasoning_levels: Option<App__infra__model__types__SectionFilter>,
    #[serde(default)]
    pub qualities: Option<App__infra__model__types__SectionFilter>,
    #[serde(default)]
    pub voices: Option<App__infra__model__types__SectionFilter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetModelApiResponse {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub model_exists: Option<bool>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub disabled_reason: Option<String>,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub draft_name: Option<String>,
    #[serde(default)]
    pub model_id: Option<String>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub basic_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub provider_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub features_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub pending_ids: Option<Vec<String>>,
    #[serde(default)]
    pub names: Option<Vec<ModelNameResource>>,
    #[serde(default)]
    pub descriptions: Option<Vec<ModelDescriptionResource>>,
    #[serde(default)]
    pub values: Option<Vec<ModelValueResource>>,
    #[serde(default)]
    pub providers: Option<Vec<ModelProviderResource>>,
    #[serde(default)]
    pub flags: Option<Vec<ModelFlagResource>>,
    #[serde(default)]
    pub departments: Option<Vec<ModelDepartmentResource>>,
    #[serde(default)]
    pub modalities: Option<Vec<ModelModalityResource>>,
    #[serde(default)]
    pub temperature_levels: Option<Vec<ModelTemperatureLevelResource>>,
    #[serde(default)]
    pub pricing: Option<Vec<ModelPricingResource>>,
    #[serde(default)]
    pub reasoning_levels: Option<Vec<ModelReasoningLevelResource>>,
    #[serde(default)]
    pub qualities: Option<Vec<ModelQualityResource>>,
    #[serde(default)]
    pub voices: Option<Vec<ModelVoiceResource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetModelDraftResponse {
    pub id: String,
    pub created_at: String,
    pub generated: bool,
    pub mcp: bool,
    pub active: bool,
    pub session_id: String,
    pub department_ids: Vec<String>,
    pub description_ids: Vec<String>,
    pub flag_ids: Vec<String>,
    pub modality_ids: Vec<String>,
    pub name_ids: Vec<String>,
    pub pricing_ids: Vec<String>,
    pub profile_ids: Vec<String>,
    pub provider_ids: Vec<String>,
    pub quality_ids: Vec<String>,
    pub reasoning_level_ids: Vec<String>,
    pub temperature_level_ids: Vec<String>,
    pub voice_ids: Vec<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub value_id: Option<String>,
    #[serde(default)]
    pub pending_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_description_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_modality_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_name_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_pricing_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_provider_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_quality_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_reasoning_level_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_temperature_level_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_value_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_voice_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetModelDraftsApiRequest {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub date_from: Option<String>,
    #[serde(default)]
    pub date_to: Option<String>,
    #[serde(default)]
    pub page_limit: Option<i64>,
    #[serde(default)]
    pub page_offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetModelDraftsApiResponse {
    #[serde(default)]
    pub entries: Option<Vec<GetModelDraftResponse>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetParameterApiRequest {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub parameter_id: Option<String>,
    #[serde(default)]
    pub draft_id: Option<String>,
    #[serde(default)]
    pub snapshot_key: Option<String>,
    #[serde(default)]
    pub names: Option<App__infra__parameter__types__SectionFilter>,
    #[serde(default)]
    pub descriptions: Option<App__infra__parameter__types__SectionFilter>,
    #[serde(default)]
    pub flags: Option<App__infra__parameter__types__SectionFilter>,
    #[serde(default)]
    pub departments: Option<App__infra__parameter__types__SectionFilter>,
    #[serde(default)]
    pub parameter_fields: Option<App__infra__parameter__types__SectionFilter>,
    #[serde(default)]
    pub fields: Option<App__infra__parameter__types__SectionFilter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetParameterApiResponse {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub parameter_exists: Option<bool>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub disabled_reason: Option<String>,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub draft_name: Option<String>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub basic_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub fields_step_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub pending_ids: Option<Vec<String>>,
    #[serde(default)]
    pub names: Option<Vec<ParameterNameResource>>,
    #[serde(default)]
    pub descriptions: Option<Vec<ParameterDescriptionResource>>,
    #[serde(default)]
    pub flags: Option<Vec<ParameterFlagResource>>,
    #[serde(default)]
    pub departments: Option<Vec<ParameterDepartmentResource>>,
    #[serde(default)]
    pub parameter_fields: Option<Vec<ParameterFieldResource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetParameterDraftResponse {
    pub id: String,
    pub created_at: String,
    pub generated: bool,
    pub mcp: bool,
    pub active: bool,
    pub session_id: String,
    pub department_ids: Vec<String>,
    pub description_ids: Vec<String>,
    pub field_ids: Vec<String>,
    pub flag_ids: Vec<String>,
    pub name_ids: Vec<String>,
    pub profile_ids: Vec<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub pending_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_description_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_field_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_name_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetParameterDraftsApiRequest {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub date_from: Option<String>,
    #[serde(default)]
    pub date_to: Option<String>,
    #[serde(default)]
    pub page_limit: Option<i64>,
    #[serde(default)]
    pub page_offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetParameterDraftsApiResponse {
    #[serde(default)]
    pub entries: Option<Vec<GetParameterDraftResponse>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPersonaApiRequest {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub draft_id: Option<String>,
    #[serde(default)]
    pub snapshot_key: Option<String>,
    #[serde(default)]
    pub names: Option<App__infra__persona__types__SectionFilter>,
    #[serde(default)]
    pub descriptions: Option<App__infra__persona__types__SectionFilter>,
    #[serde(default)]
    pub colors: Option<App__infra__persona__types__SectionFilter>,
    #[serde(default)]
    pub icons: Option<App__infra__persona__types__SectionFilter>,
    #[serde(default)]
    pub instructions: Option<App__infra__persona__types__SectionFilter>,
    #[serde(default)]
    pub departments: Option<App__infra__persona__types__SectionFilter>,
    #[serde(default)]
    pub examples: Option<App__infra__persona__types__SectionFilter>,
    #[serde(default)]
    pub parameter_fields: Option<App__infra__persona__types__SectionFilter>,
    #[serde(default)]
    pub voices: Option<App__infra__persona__types__SectionFilter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPersonaApiResponse {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub persona_exists: Option<bool>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub disabled_reason: Option<String>,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub draft_name: Option<String>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub names: Option<Vec<PersonaNameResource>>,
    #[serde(default)]
    pub descriptions: Option<Vec<PersonaDescriptionResource>>,
    #[serde(default)]
    pub colors: Option<Vec<PersonaColorResource>>,
    #[serde(default)]
    pub icons: Option<Vec<PersonaIconResource>>,
    #[serde(default)]
    pub instructions: Option<Vec<PersonaInstructionResource>>,
    #[serde(default)]
    pub flags: Option<Vec<PersonaFlagResource>>,
    #[serde(default)]
    pub departments: Option<Vec<PersonaDepartmentResource>>,
    #[serde(default)]
    pub parameter_fields: Option<Vec<PersonaParameterFieldResource>>,
    #[serde(default)]
    pub examples: Option<Vec<PersonaExampleResource>>,
    #[serde(default)]
    pub parameters: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub voices: Option<Vec<PersonaVoiceResource>>,
    #[serde(default)]
    pub fields: Option<Vec<GetFieldResponse>>,
    #[serde(default)]
    pub resolved_parameter_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPersonaDraftResponse {
    pub id: String,
    pub created_at: String,
    pub generated: bool,
    pub mcp: bool,
    pub active: bool,
    pub session_id: String,
    pub color_ids: Vec<String>,
    pub department_ids: Vec<String>,
    pub description_ids: Vec<String>,
    pub example_ids: Vec<String>,
    pub flag_ids: Vec<String>,
    pub icon_ids: Vec<String>,
    pub instruction_ids: Vec<String>,
    pub name_ids: Vec<String>,
    pub parameter_field_ids: Vec<String>,
    pub profile_ids: Vec<String>,
    pub voice_ids: Vec<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub pending_color_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_description_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_example_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_icon_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_instruction_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_name_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_parameter_field_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_voice_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPersonaDraftsApiRequest {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub date_from: Option<String>,
    #[serde(default)]
    pub date_to: Option<String>,
    #[serde(default)]
    pub page_limit: Option<i64>,
    #[serde(default)]
    pub page_offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPersonaDraftsApiResponse {
    #[serde(default)]
    pub entries: Option<Vec<GetPersonaDraftResponse>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPracticeResponse {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub items: Option<Vec<ChatSimulationOperational>>,
    #[serde(default)]
    pub rubrics: Option<Vec<RubricMapping>>,
    #[serde(default)]
    pub standard_groups: Option<Vec<StandardGroupMapping>>,
    #[serde(default)]
    pub standards: Option<Vec<StandardMapping>>,
    #[serde(default)]
    pub analytics: Option<AnalyticsFacets>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetProfileApiRequest {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub target_profile_id: Option<String>,
    #[serde(default)]
    pub draft_id: Option<String>,
    #[serde(default)]
    pub snapshot_key: Option<String>,
    #[serde(default)]
    pub names: Option<App__infra__profile__types__SectionFilter>,
    #[serde(default)]
    pub emails: Option<App__infra__profile__types__SectionFilter>,
    #[serde(default)]
    pub flags: Option<App__infra__profile__types__SectionFilter>,
    #[serde(default)]
    pub departments: Option<App__infra__profile__types__SectionFilter>,
    #[serde(default)]
    pub roles: Option<App__infra__profile__types__SectionFilter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetProfileApiResponse {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub profile_exists: Option<bool>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub disabled_reason: Option<String>,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub draft_name: Option<String>,
    #[serde(default)]
    pub profile_id: Option<String>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub basic_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub contact_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub pending_ids: Option<Vec<String>>,
    #[serde(default)]
    pub role_options: Option<Vec<String>>,
    #[serde(default)]
    pub names: Option<Vec<ProfileNameResource>>,
    #[serde(default)]
    pub emails: Option<Vec<ProfileEmailResource>>,
    #[serde(default)]
    pub flags: Option<Vec<ProfileFlagResource>>,
    #[serde(default)]
    pub departments: Option<Vec<ProfileDepartmentResource>>,
    #[serde(default)]
    pub roles: Option<Vec<ProfileRoleResource>>,
    #[serde(default)]
    pub permissions: Option<Vec<ProfilePermissionResource>>,
    #[serde(default)]
    pub request_limits: Option<Vec<ProfileRequestLimitResource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetProfileDraftResponse {
    pub id: String,
    pub created_at: String,
    pub generated: bool,
    pub mcp: bool,
    pub active: bool,
    pub session_id: String,
    pub profile_ids: Vec<String>,
    pub department_ids: Vec<String>,
    pub email_ids: Vec<String>,
    pub flag_ids: Vec<String>,
    pub name_ids: Vec<String>,
    pub role_ids: Vec<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub primary_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_email_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_name_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_role_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_primary_department_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetProfileDraftsApiRequest {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub date_from: Option<String>,
    #[serde(default)]
    pub date_to: Option<String>,
    #[serde(default)]
    pub page_limit: Option<i64>,
    #[serde(default)]
    pub page_offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetProfileDraftsApiResponse {
    #[serde(default)]
    pub entries: Option<Vec<GetProfileDraftResponse>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetProviderApiRequest {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub provider_id: Option<String>,
    #[serde(default)]
    pub draft_id: Option<String>,
    #[serde(default)]
    pub snapshot_key: Option<String>,
    #[serde(default)]
    pub names: Option<App__infra__provider__types__SectionFilter>,
    #[serde(default)]
    pub descriptions: Option<App__infra__provider__types__SectionFilter>,
    #[serde(default)]
    pub flags: Option<App__infra__provider__types__SectionFilter>,
    #[serde(default)]
    pub departments: Option<App__infra__provider__types__SectionFilter>,
    #[serde(default)]
    pub values: Option<App__infra__provider__types__SectionFilter>,
    #[serde(default)]
    pub endpoints: Option<App__infra__provider__types__SectionFilter>,
    #[serde(default)]
    pub keys: Option<App__infra__provider__types__SectionFilter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetProviderApiResponse {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub provider_exists: Option<bool>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub disabled_reason: Option<String>,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub draft_name: Option<String>,
    #[serde(default)]
    pub provider_id: Option<String>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub basic_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub integrations_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub pending_ids: Option<Vec<String>>,
    #[serde(default)]
    pub names: Option<Vec<ProviderNameResource>>,
    #[serde(default)]
    pub descriptions: Option<Vec<ProviderDescriptionResource>>,
    #[serde(default)]
    pub flags: Option<Vec<ProviderFlagResource>>,
    #[serde(default)]
    pub departments: Option<Vec<ProviderDepartmentResource>>,
    #[serde(default)]
    pub values: Option<Vec<ProviderValueResource>>,
    #[serde(default)]
    pub endpoints: Option<Vec<ProviderEndpointResource>>,
    #[serde(default)]
    pub keys: Option<Vec<ProviderKeyResource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetProviderDraftResponse {
    pub id: String,
    pub created_at: String,
    pub generated: bool,
    pub mcp: bool,
    pub active: bool,
    pub session_id: String,
    pub department_ids: Vec<String>,
    pub description_ids: Vec<String>,
    pub endpoint_ids: Vec<String>,
    pub flag_ids: Vec<String>,
    pub key_ids: Vec<String>,
    pub name_ids: Vec<String>,
    pub profile_ids: Vec<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub value_id: Option<String>,
    #[serde(default)]
    pub pending_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_description_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_endpoint_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_key_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_name_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_value_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetProviderDraftsApiRequest {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub date_from: Option<String>,
    #[serde(default)]
    pub date_to: Option<String>,
    #[serde(default)]
    pub page_limit: Option<i64>,
    #[serde(default)]
    pub page_offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetProviderDraftsApiResponse {
    #[serde(default)]
    pub entries: Option<Vec<GetProviderDraftResponse>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRubricApiRequest {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub rubric_id: Option<String>,
    #[serde(default)]
    pub draft_id: Option<String>,
    #[serde(default)]
    pub snapshot_key: Option<String>,
    #[serde(default)]
    pub names: Option<App__infra__rubric__types__SectionFilter>,
    #[serde(default)]
    pub descriptions: Option<App__infra__rubric__types__SectionFilter>,
    #[serde(default)]
    pub flags: Option<App__infra__rubric__types__SectionFilter>,
    #[serde(default)]
    pub departments: Option<App__infra__rubric__types__SectionFilter>,
    #[serde(default)]
    pub points: Option<App__infra__rubric__types__SectionFilter>,
    #[serde(default)]
    pub standard_groups: Option<App__infra__rubric__types__SectionFilter>,
    #[serde(default)]
    pub standards: Option<App__infra__rubric__types__SectionFilter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRubricApiResponse {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub rubric_exists: Option<bool>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub disabled_reason: Option<String>,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub draft_name: Option<String>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub basic_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub content_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub pending_ids: Option<Vec<String>>,
    #[serde(default)]
    pub names: Option<Vec<RubricNameResource>>,
    #[serde(default)]
    pub descriptions: Option<Vec<RubricDescriptionResource>>,
    #[serde(default)]
    pub flags: Option<Vec<RubricFlagResource>>,
    #[serde(default)]
    pub departments: Option<Vec<RubricDepartmentResource>>,
    #[serde(default)]
    pub points: Option<Vec<RubricPointResource>>,
    #[serde(default)]
    pub standard_groups: Option<Vec<RubricStandardGroupResource>>,
    #[serde(default)]
    pub standards: Option<Vec<RubricStandardResource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRubricDraftResponse {
    pub id: String,
    pub created_at: String,
    pub generated: bool,
    pub mcp: bool,
    pub active: bool,
    pub session_id: String,
    pub department_ids: Vec<String>,
    pub description_ids: Vec<String>,
    pub flag_ids: Vec<String>,
    pub name_ids: Vec<String>,
    pub point_ids: Vec<String>,
    pub profile_ids: Vec<String>,
    pub standard_group_ids: Vec<String>,
    pub standard_ids: Vec<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub pending_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_description_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_name_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_point_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_standard_group_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_standard_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRubricDraftsApiRequest {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub date_from: Option<String>,
    #[serde(default)]
    pub date_to: Option<String>,
    #[serde(default)]
    pub page_limit: Option<i64>,
    #[serde(default)]
    pub page_offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRubricDraftsApiResponse {
    #[serde(default)]
    pub entries: Option<Vec<GetRubricDraftResponse>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRunListViewResponse {
    #[serde(default)]
    pub items: Option<Vec<RunViewItem>>,
    #[serde(default)]
    pub total_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetScenarioApiRequest {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub draft_id: Option<String>,
    #[serde(default)]
    pub names: Option<App__infra__persona__types__SectionFilter>,
    #[serde(default)]
    pub descriptions: Option<App__infra__persona__types__SectionFilter>,
    #[serde(default)]
    pub problem_statements: Option<App__infra__persona__types__SectionFilter>,
    #[serde(default)]
    pub flags: Option<App__infra__persona__types__SectionFilter>,
    #[serde(default)]
    pub departments: Option<App__infra__persona__types__SectionFilter>,
    #[serde(default)]
    pub personas: Option<App__infra__persona__types__SectionFilter>,
    #[serde(default)]
    pub documents: Option<App__infra__persona__types__SectionFilter>,
    #[serde(default)]
    pub parameters: Option<App__infra__persona__types__SectionFilter>,
    #[serde(default)]
    pub parameter_fields: Option<App__infra__persona__types__SectionFilter>,
    #[serde(default)]
    pub objectives: Option<App__infra__persona__types__SectionFilter>,
    #[serde(default)]
    pub images: Option<App__infra__persona__types__SectionFilter>,
    #[serde(default)]
    pub videos: Option<App__infra__persona__types__SectionFilter>,
    #[serde(default)]
    pub questions: Option<App__infra__persona__types__SectionFilter>,
    #[serde(default)]
    pub options: Option<App__infra__persona__types__SectionFilter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetScenarioApiResponse {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub scenario_exists: Option<bool>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub disabled_reason: Option<String>,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub draft_name: Option<String>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub resolved_parameter_ids: Option<Vec<String>>,
    #[serde(default)]
    pub names: Option<Vec<ScenarioNameResource>>,
    #[serde(default)]
    pub descriptions: Option<Vec<ScenarioDescriptionResource>>,
    #[serde(default)]
    pub problem_statements: Option<Vec<ScenarioProblemStatement>>,
    #[serde(default)]
    pub flags: Option<Vec<ScenarioFlagResource>>,
    #[serde(default)]
    pub departments: Option<Vec<ScenarioDepartment>>,
    #[serde(default)]
    pub personas: Option<Vec<ScenarioPersona>>,
    #[serde(default)]
    pub documents: Option<Vec<ScenarioDocument>>,
    #[serde(default)]
    pub parameters: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub parameter_fields: Option<Vec<ScenarioField>>,
    #[serde(default)]
    pub objectives: Option<Vec<ScenarioObjective>>,
    #[serde(default)]
    pub images: Option<Vec<ScenarioImage>>,
    #[serde(default)]
    pub videos: Option<Vec<ScenarioVideo>>,
    #[serde(default)]
    pub questions: Option<Vec<ScenarioQuestion>>,
    #[serde(default)]
    pub options: Option<Vec<ScenarioOption>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetScenarioDraftResponse {
    pub id: String,
    pub created_at: String,
    pub generated: bool,
    pub mcp: bool,
    pub active: bool,
    pub session_id: String,
    pub department_ids: Vec<String>,
    pub description_ids: Vec<String>,
    pub document_ids: Vec<String>,
    pub flag_ids: Vec<String>,
    pub image_ids: Vec<String>,
    pub name_ids: Vec<String>,
    pub objective_ids: Vec<String>,
    pub option_ids: Vec<String>,
    pub parameter_field_ids: Vec<String>,
    pub persona_ids: Vec<String>,
    pub problem_statement_ids: Vec<String>,
    pub profile_ids: Vec<String>,
    pub question_ids: Vec<String>,
    pub video_ids: Vec<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub pending_name_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_description_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_problem_statement_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_persona_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_document_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_objective_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_image_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_video_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_question_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_option_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_parameter_field_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetScenarioDraftsApiRequest {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub date_from: Option<String>,
    #[serde(default)]
    pub date_to: Option<String>,
    #[serde(default)]
    pub page_limit: Option<i64>,
    #[serde(default)]
    pub page_offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetScenarioDraftsApiResponse {
    #[serde(default)]
    pub entries: Option<Vec<GetScenarioDraftResponse>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSessionDetailRequest {
    pub session_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSessionDetailResponse {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub session_exists: Option<bool>,
    #[serde(default)]
    pub session_id: Option<String>,
    #[serde(default)]
    pub profile_id: Option<String>,
    #[serde(default)]
    pub profile_name: Option<String>,
    #[serde(default)]
    pub session_created_at: Option<String>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub groups: Option<Vec<ArtifactSessionGroup>>,
    #[serde(default)]
    pub timeline: Option<Vec<SessionTimelineItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSettingApiRequest {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub setting_id: Option<String>,
    #[serde(default)]
    pub settings_id: Option<String>,
    #[serde(default)]
    pub draft_id: Option<String>,
    #[serde(default)]
    pub snapshot_key: Option<String>,
    #[serde(default)]
    pub names: Option<App__infra__setting__types__SectionFilter>,
    #[serde(default)]
    pub descriptions: Option<App__infra__setting__types__SectionFilter>,
    #[serde(default)]
    pub colors: Option<App__infra__setting__types__SectionFilter>,
    #[serde(default)]
    pub flags: Option<App__infra__setting__types__SectionFilter>,
    #[serde(default)]
    pub departments: Option<App__infra__setting__types__SectionFilter>,
    #[serde(default)]
    pub logins: Option<App__infra__setting__types__SectionFilter>,
    #[serde(default)]
    pub systems: Option<App__infra__setting__types__SectionFilter>,
    #[serde(default)]
    pub mcp: Option<App__infra__setting__types__SectionFilter>,
    #[serde(default)]
    pub thresholds: Option<App__infra__setting__types__SectionFilter>,
    #[serde(default)]
    pub provider_keys: Option<App__infra__setting__types__SectionFilter>,
    #[serde(default)]
    pub auth_item_keys: Option<App__infra__setting__types__SectionFilter>,
    #[serde(default)]
    pub auth_item_values: Option<App__infra__setting__types__SectionFilter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSettingApiResponse {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub setting_exists: Option<bool>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub disabled_reason: Option<String>,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub draft_name: Option<String>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub basic_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub pending_ids: Option<Vec<String>>,
    #[serde(default)]
    pub names: Option<Vec<SettingNameResource>>,
    #[serde(default)]
    pub descriptions: Option<Vec<SettingDescriptionResource>>,
    #[serde(default)]
    pub colors: Option<Vec<SettingColorResource>>,
    #[serde(default)]
    pub flags: Option<Vec<SettingFlagResource>>,
    #[serde(default)]
    pub departments: Option<Vec<SettingDepartmentResource>>,
    #[serde(default)]
    pub logins: Option<Vec<SettingLoginsResource>>,
    #[serde(default)]
    pub systems: Option<Vec<SettingSystemResource>>,
    #[serde(default)]
    pub mcp: Option<Vec<SettingMcpResource>>,
    #[serde(default)]
    pub thresholds: Option<Vec<SettingThresholdResource>>,
    #[serde(default)]
    pub provider_keys: Option<Vec<SettingProviderKeyResource>>,
    #[serde(default)]
    pub auth_item_keys: Option<Vec<SettingAuthItemKeyResource>>,
    #[serde(default)]
    pub auth_item_values: Option<Vec<SettingAuthItemValueResource>>,
    #[serde(default)]
    pub providers: Option<Vec<SettingProviderCatalogResource>>,
    #[serde(default)]
    pub keys: Option<Vec<SettingKeyCatalogResource>>,
    #[serde(default)]
    pub items: Option<Vec<SettingItemCatalogResource>>,
    #[serde(default)]
    pub profiles: Option<Vec<SettingProfileCatalogResource>>,
    #[serde(default)]
    pub auths: Option<Vec<SettingAuthCatalogResource>>,
    #[serde(default)]
    pub icons: Option<Vec<SettingIconCatalogResource>>,
    #[serde(default)]
    pub agents: Option<Vec<SettingAgentCatalogResource>>,
    #[serde(default)]
    pub provider_key_options: Option<Vec<SettingProviderKeyOption>>,
    #[serde(default)]
    pub auth_item_key_options: Option<Vec<SettingAuthItemKeyOption>>,
    #[serde(default)]
    pub auth_item_value_options: Option<Vec<SettingAuthItemValueOption>>,
    #[serde(default)]
    pub mcp_options: Option<Vec<SettingMcpOption>>,
    #[serde(default)]
    pub login_options: Option<Vec<SettingLoginOption>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSettingDraftResponse {
    pub id: String,
    pub created_at: String,
    pub generated: bool,
    pub mcp: bool,
    pub active: bool,
    pub session_id: String,
    pub agent_ids: Vec<String>,
    pub auth_item_key_ids: Vec<String>,
    pub auth_ids: Vec<String>,
    pub color_ids: Vec<String>,
    pub department_ids: Vec<String>,
    pub description_ids: Vec<String>,
    pub flag_ids: Vec<String>,
    pub item_ids: Vec<String>,
    pub name_ids: Vec<String>,
    pub provider_ids: Vec<String>,
    pub provider_key_ids: Vec<String>,
    pub threshold_ids: Vec<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub mcp_ids: Option<Vec<String>>,
    #[serde(default)]
    pub logins_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_agent_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_auth_item_key_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_auth_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_color_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_description_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_item_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_name_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_provider_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_provider_key_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_threshold_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_mcp_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_logins_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSettingDraftsApiRequest {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub date_from: Option<String>,
    #[serde(default)]
    pub date_to: Option<String>,
    #[serde(default)]
    pub page_limit: Option<i64>,
    #[serde(default)]
    pub page_offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSettingDraftsApiResponse {
    #[serde(default)]
    pub entries: Option<Vec<GetSettingDraftResponse>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSimulationApiRequest {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub simulation_id: Option<String>,
    #[serde(default)]
    pub draft_id: Option<String>,
    #[serde(default)]
    pub snapshot_key: Option<String>,
    #[serde(default)]
    pub names: Option<App__infra__simulation__types__SectionFilter>,
    #[serde(default)]
    pub descriptions: Option<App__infra__simulation__types__SectionFilter>,
    #[serde(default)]
    pub flags: Option<App__infra__simulation__types__SectionFilter>,
    #[serde(default)]
    pub departments: Option<App__infra__simulation__types__SectionFilter>,
    #[serde(default)]
    pub scenarios: Option<App__infra__simulation__types__SectionFilter>,
    #[serde(default)]
    pub scenario_flags: Option<App__infra__simulation__types__SectionFilter>,
    #[serde(default)]
    pub scenario_positions: Option<App__infra__simulation__types__SectionFilter>,
    #[serde(default)]
    pub scenario_rubrics: Option<App__infra__simulation__types__SectionFilter>,
    #[serde(default)]
    pub scenario_time_limits: Option<App__infra__simulation__types__SectionFilter>,
    #[serde(default)]
    pub filter_scenario_ids: Option<Vec<String>>,
    #[serde(default)]
    pub scenario_search: Option<String>,
    #[serde(default)]
    pub scenario_show_selected: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSimulationApiResponse {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub simulation_exists: Option<bool>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub disabled_reason: Option<String>,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub draft_name: Option<String>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub basic_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub names: Option<Vec<SimulationNameResource>>,
    #[serde(default)]
    pub descriptions: Option<Vec<SimulationDescriptionResource>>,
    #[serde(default)]
    pub flags: Option<Vec<SimulationFlagResource>>,
    #[serde(default)]
    pub departments: Option<Vec<SimulationDepartment>>,
    #[serde(default)]
    pub scenarios: Option<Vec<SimulationScenario>>,
    #[serde(default)]
    pub scenario_flags: Option<Vec<SimulationScenarioFlag>>,
    #[serde(default)]
    pub scenario_flag_options: Option<Vec<SimulationScenarioFlagOption>>,
    #[serde(default)]
    pub scenario_positions: Option<Vec<SimulationScenarioPosition>>,
    #[serde(default)]
    pub scenario_rubrics: Option<Vec<SimulationScenarioRubric>>,
    #[serde(default)]
    pub scenario_time_limits: Option<Vec<SimulationScenarioTimeLimit>>,
    #[serde(default)]
    pub rubrics: Option<Vec<SimulationRubric>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSimulationDraftResponse {
    pub id: String,
    pub created_at: String,
    pub generated: bool,
    pub mcp: bool,
    pub active: bool,
    pub session_id: String,
    pub department_ids: Vec<String>,
    pub description_ids: Vec<String>,
    pub flag_ids: Vec<String>,
    pub name_ids: Vec<String>,
    pub profile_ids: Vec<String>,
    pub scenario_flag_ids: Vec<String>,
    pub scenario_position_ids: Vec<String>,
    pub scenario_rubric_ids: Vec<String>,
    pub scenario_time_limit_ids: Vec<String>,
    pub scenario_ids: Vec<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub pending_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_description_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_name_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_scenario_flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_scenario_position_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_scenario_rubric_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_scenario_time_limit_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_scenario_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSimulationDraftsApiRequest {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub date_from: Option<String>,
    #[serde(default)]
    pub date_to: Option<String>,
    #[serde(default)]
    pub page_limit: Option<i64>,
    #[serde(default)]
    pub page_offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSimulationDraftsApiResponse {
    #[serde(default)]
    pub entries: Option<Vec<GetSimulationDraftResponse>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSuiteResponse {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub test_id: Option<String>,
    #[serde(default)]
    pub invocation_id: Option<String>,
    #[serde(default)]
    pub profile_has_access: Option<bool>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub disabled_reason: Option<String>,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub pending_ids: Option<Vec<String>>,
    #[serde(default)]
    pub names: Option<Vec<InvocationNameResource>>,
    #[serde(default)]
    pub descriptions: Option<Vec<InvocationDescriptionResource>>,
    #[serde(default)]
    pub values: Option<Vec<InvocationValueResource>>,
    #[serde(default)]
    pub flags: Option<Vec<InvocationFlagResource>>,
    #[serde(default)]
    pub departments: Option<Vec<InvocationDepartmentResource>>,
    #[serde(default)]
    pub keys: Option<Vec<InvocationKeyResource>>,
    #[serde(default)]
    pub endpoints: Option<Vec<InvocationEndpointResource>>,
    #[serde(default)]
    pub modalities: Option<Vec<InvocationModalityResource>>,
    #[serde(default)]
    pub temperature_levels: Option<Vec<InvocationTemperatureLevelResource>>,
    #[serde(default)]
    pub pricing: Option<Vec<InvocationPricingResource>>,
    #[serde(default)]
    pub reasoning_levels: Option<Vec<InvocationReasoningLevelResource>>,
    #[serde(default)]
    pub qualities: Option<Vec<InvocationQualityResource>>,
    #[serde(default)]
    pub voices: Option<Vec<InvocationVoiceResource>>,
    #[serde(default)]
    pub model_flags: Option<Vec<InvocationModelFlagResource>>,
    #[serde(default)]
    pub model_flag_options: Option<Vec<InvocationModelFlagOptionResource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTestArtifactRequest {
    pub test_id: String,
    #[serde(default)]
    pub configs_groups_page: Option<i64>,
    #[serde(default)]
    pub configs_groups_page_size: Option<i64>,
    #[serde(default)]
    pub configs_expanded: Option<Vec<String>>,
    #[serde(default)]
    pub configs_expanded_page_size: Option<i64>,
    #[serde(default)]
    pub configs_search: Option<String>,
    #[serde(default)]
    pub configs_selected: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTestArtifactResponse {
    #[serde(default)]
    pub test: Option<GetTestResponse>,
    #[serde(default)]
    pub invocations: Option<Vec<GetTestInvocationResponse>>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub eval_name: Option<String>,
    #[serde(default)]
    pub eval_description: Option<String>,
    #[serde(default)]
    pub rubric_name: Option<String>,
    #[serde(default)]
    pub infinite_mode: Option<bool>,
    #[serde(default)]
    pub runs: Option<Vec<TestRunItem>>,
    #[serde(default)]
    pub configs: Option<Vec<TestConfigItem>>,
    #[serde(default)]
    pub configs_groups: Option<Vec<TestConfigGroup>>,
    #[serde(default)]
    pub configs_total: Option<i64>,
    #[serde(default)]
    pub configs_groups_total: Option<i64>,
    #[serde(default)]
    pub configs_per_group_total: Option<std::collections::HashMap<String, i64>>,
    #[serde(default)]
    pub status_summary: Option<TestStatusSummary>,
    #[serde(default)]
    pub show_controls: Option<bool>,
    #[serde(default)]
    pub current_invocation_id: Option<String>,
    #[serde(default)]
    pub has_runs_or_groups: Option<bool>,
    #[serde(default)]
    pub next_invocation_id: Option<String>,
    #[serde(default)]
    pub invocation_details: Option<Vec<InvocationDetail>>,
    #[serde(default)]
    pub entries: Option<TestEntries>,
    #[serde(default)]
    pub resources: Option<TestResources>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTestFeedbackResponse {
    pub feedback_id: String,
    pub grade_id: String,
    pub call_id: String,
    pub tool_call_id: String,
    pub total: i64,
    pub feedback: String,
    pub total_points: i64,
    pub pass_points: i64,
    pub created_at: String,
    #[serde(default)]
    pub standard_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTestGradeResponse {
    pub id: String,
    pub invocation_id: String,
    pub created_at: String,
    pub updated_at: String,
    pub passed: bool,
    pub score: i64,
    pub time_taken: Option<i64>,
    pub generated: bool,
    pub mcp: bool,
    pub active: bool,
    pub call_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTestInvocationResponse {
    pub invocation_id: String,
    pub test_id: Option<String>,
    pub group_id: Option<String>,
    pub invocation_created_at: String,
    pub invocation_title: String,
    pub use_custom: bool,
    pub position: i64,
    pub invocation_completed: bool,
    pub grade_id: Option<String>,
    pub grade_score: Option<f64>,
    pub grade_passed: Option<bool>,
    pub grade_time_taken: Option<f64>,
    pub rubric_id: Option<String>,
    #[serde(default)]
    pub agent_ids: Option<Vec<String>>,
    #[serde(default)]
    pub quality_id: Option<String>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub voice_id: Option<String>,
    #[serde(default)]
    pub temperature_level_id: Option<String>,
    #[serde(default)]
    pub reasoning_level_id: Option<String>,
    #[serde(default)]
    pub modality_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTestInvocationRunsResponse {
    pub id: String,
    pub test_invocation_id: String,
    pub created_at: String,
    pub updated_at: String,
    pub generated: bool,
    pub mcp: bool,
    pub active: bool,
    #[serde(default)]
    pub test_invocation_traces_id: Option<String>,
    #[serde(default)]
    pub run_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTestInvocationTracesResponse {
    pub id: String,
    pub test_invocation_id: String,
    pub created_at: String,
    pub updated_at: String,
    pub generated: bool,
    pub mcp: bool,
    pub active: bool,
    #[serde(default)]
    pub run_id: Option<String>,
    #[serde(default)]
    pub reasoning_level_ids: Option<Vec<String>>,
    #[serde(default)]
    pub temperature_level_ids: Option<Vec<String>>,
    #[serde(default)]
    pub voice_ids: Option<Vec<String>>,
    #[serde(default)]
    pub prompt_ids: Option<Vec<String>>,
    #[serde(default)]
    pub instruction_ids: Option<Vec<String>>,
    #[serde(default)]
    pub tool_ids: Option<Vec<String>>,
    #[serde(default)]
    pub quality_ids: Option<Vec<String>>,
    #[serde(default)]
    pub modality_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTestResponse {
    pub test_id: String,
    pub call_id: Option<String>,
    pub eval_id: Option<String>,
    pub profile_id: Option<String>,
    pub department_ids: Vec<String>,
    pub test_name: String,
    pub test_description: String,
    pub num_invocations: i64,
    pub infinite_mode: bool,
    pub is_dynamic: bool,
    pub archived: bool,
    pub test_created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetToolApiRequest {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub draft_id: Option<String>,
    #[serde(default)]
    pub snapshot_key: Option<String>,
    #[serde(default)]
    pub names: Option<App__infra__tool__types__SectionFilter>,
    #[serde(default)]
    pub descriptions: Option<App__infra__tool__types__SectionFilter>,
    #[serde(default)]
    pub flags: Option<App__infra__tool__types__SectionFilter>,
    #[serde(default)]
    pub args: Option<App__infra__tool__types__SectionFilter>,
    #[serde(default)]
    pub arg_positions: Option<App__infra__tool__types__SectionFilter>,
    #[serde(default)]
    pub args_outputs: Option<App__infra__tool__types__SectionFilter>,
    #[serde(default)]
    pub permissions: Option<App__infra__tool__types__SectionFilter>,
    #[serde(default)]
    pub instructions: Option<App__infra__tool__types__SectionFilter>,
    #[serde(default)]
    pub departments: Option<App__infra__tool__types__SectionFilter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetToolApiResponse {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub tool_exists: Option<bool>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub disabled_reason: Option<String>,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub draft_name: Option<String>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub basic_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub args_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub permissions_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub pending_ids: Option<Vec<String>>,
    #[serde(default)]
    pub names: Option<Vec<ToolNameResource>>,
    #[serde(default)]
    pub descriptions: Option<Vec<ToolDescriptionResource>>,
    #[serde(default)]
    pub flags: Option<Vec<ToolFlagResource>>,
    #[serde(default)]
    pub args: Option<Vec<ToolArgResource>>,
    #[serde(default)]
    pub arg_positions: Option<Vec<ToolArgPositionResource>>,
    #[serde(default)]
    pub args_outputs: Option<Vec<ToolArgOutputResource>>,
    #[serde(default)]
    pub permissions: Option<Vec<ToolPermissionResource>>,
    #[serde(default)]
    pub instructions: Option<Vec<ToolInstructionResource>>,
    #[serde(default)]
    pub departments: Option<Vec<ToolDepartmentResource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetToolDraftResponse {
    pub id: String,
    pub created_at: String,
    pub generated: bool,
    pub mcp: bool,
    pub active: bool,
    pub session_id: String,
    pub arg_position_ids: Vec<String>,
    pub arg_ids: Vec<String>,
    pub args_output_ids: Vec<String>,
    pub department_ids: Vec<String>,
    pub description_ids: Vec<String>,
    pub flag_ids: Vec<String>,
    pub name_ids: Vec<String>,
    pub permission_ids: Vec<String>,
    pub profile_ids: Vec<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub instruction_ids: Option<Vec<String>>,
    #[serde(default)]
    pub agent_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_arg_position_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_arg_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_args_output_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_description_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_name_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_instruction_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_permission_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_agent_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetToolDraftsApiRequest {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub date_from: Option<String>,
    #[serde(default)]
    pub date_to: Option<String>,
    #[serde(default)]
    pub page_limit: Option<i64>,
    #[serde(default)]
    pub page_offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetToolDraftsApiResponse {
    #[serde(default)]
    pub entries: Option<Vec<GetToolDraftResponse>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradeAttemptApiRequest {
    pub chat_id: String,
    pub score: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradeAttemptApiResponse {
    pub chat_id: String,
    #[serde(default)]
    pub grade_id: Option<String>,
    #[serde(default)]
    pub score: Option<i64>,
    #[serde(default)]
    pub passed: Option<bool>,
    #[serde(default)]
    pub time_taken: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradeData {
    #[serde(default)]
    pub score: Option<f64>,
    #[serde(default)]
    pub passed: Option<bool>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub time_taken: Option<i64>,
    #[serde(default)]
    pub total_points: Option<f64>,
    #[serde(default)]
    pub pass_points: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradingStateData {
    #[serde(default)]
    pub achieved_standards: Option<std::collections::HashMap<String, bool>>,
    #[serde(default)]
    pub passed_standards: Option<std::collections::HashMap<String, bool>>,
    #[serde(default)]
    pub feedback_by_standard_id: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupAgentApiRequest {
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub snapshot_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupAgentApiResponse {
    pub group_id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub snapshot_key: Option<String>,
    #[serde(default)]
    pub runs: Option<Vec<GroupRun>>,
    #[serde(default)]
    pub group_exists: Option<bool>,
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub total_message_count: Option<i64>,
    #[serde(default)]
    pub models: Option<Vec<GroupResource>>,
    #[serde(default)]
    pub agents: Option<Vec<GroupResource>>,
    #[serde(default)]
    pub profiles: Option<Vec<GroupResource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupAttemptApiRequest {
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub snapshot_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupAttemptApiResponse {
    pub group_id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub snapshot_key: Option<String>,
    #[serde(default)]
    pub runs: Option<Vec<GroupRun>>,
    #[serde(default)]
    pub group_exists: Option<bool>,
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub total_message_count: Option<i64>,
    #[serde(default)]
    pub models: Option<Vec<GroupResource>>,
    #[serde(default)]
    pub agents: Option<Vec<GroupResource>>,
    #[serde(default)]
    pub profiles: Option<Vec<GroupResource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupAuthApiRequest {
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub snapshot_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupAuthApiResponse {
    pub group_id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub snapshot_key: Option<String>,
    #[serde(default)]
    pub runs: Option<Vec<GroupRun>>,
    #[serde(default)]
    pub group_exists: Option<bool>,
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub total_message_count: Option<i64>,
    #[serde(default)]
    pub models: Option<Vec<GroupResource>>,
    #[serde(default)]
    pub agents: Option<Vec<GroupResource>>,
    #[serde(default)]
    pub profiles: Option<Vec<GroupResource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupCall {
    pub id: String,
    #[serde(default)]
    pub tool_name: Option<String>,
    #[serde(default)]
    pub template_name: Option<String>,
    #[serde(default)]
    pub tool: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(default)]
    pub ledger_status: Option<String>,
    #[serde(default)]
    pub ledger_operation: Option<String>,
    #[serde(default)]
    pub ledger_artifact: Option<String>,
    #[serde(default)]
    pub ledger_artifact_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupCohortApiRequest {
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub snapshot_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupCohortApiResponse {
    pub group_id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub snapshot_key: Option<String>,
    #[serde(default)]
    pub runs: Option<Vec<GroupRun>>,
    #[serde(default)]
    pub group_exists: Option<bool>,
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub total_message_count: Option<i64>,
    #[serde(default)]
    pub models: Option<Vec<GroupResource>>,
    #[serde(default)]
    pub agents: Option<Vec<GroupResource>>,
    #[serde(default)]
    pub profiles: Option<Vec<GroupResource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupDepartmentApiRequest {
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub snapshot_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupDepartmentApiResponse {
    pub group_id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub snapshot_key: Option<String>,
    #[serde(default)]
    pub runs: Option<Vec<GroupRun>>,
    #[serde(default)]
    pub group_exists: Option<bool>,
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub total_message_count: Option<i64>,
    #[serde(default)]
    pub models: Option<Vec<GroupResource>>,
    #[serde(default)]
    pub agents: Option<Vec<GroupResource>>,
    #[serde(default)]
    pub profiles: Option<Vec<GroupResource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupDocumentApiRequest {
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub snapshot_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupDocumentApiResponse {
    pub group_id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub snapshot_key: Option<String>,
    #[serde(default)]
    pub runs: Option<Vec<GroupRun>>,
    #[serde(default)]
    pub group_exists: Option<bool>,
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub total_message_count: Option<i64>,
    #[serde(default)]
    pub models: Option<Vec<GroupResource>>,
    #[serde(default)]
    pub agents: Option<Vec<GroupResource>>,
    #[serde(default)]
    pub profiles: Option<Vec<GroupResource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupEvalApiRequest {
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub snapshot_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupEvalApiResponse {
    pub group_id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub snapshot_key: Option<String>,
    #[serde(default)]
    pub runs: Option<Vec<GroupRun>>,
    #[serde(default)]
    pub group_exists: Option<bool>,
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub total_message_count: Option<i64>,
    #[serde(default)]
    pub models: Option<Vec<GroupResource>>,
    #[serde(default)]
    pub agents: Option<Vec<GroupResource>>,
    #[serde(default)]
    pub profiles: Option<Vec<GroupResource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupFieldApiRequest {
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub snapshot_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupFieldApiResponse {
    pub group_id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub snapshot_key: Option<String>,
    #[serde(default)]
    pub runs: Option<Vec<GroupRun>>,
    #[serde(default)]
    pub group_exists: Option<bool>,
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub total_message_count: Option<i64>,
    #[serde(default)]
    pub models: Option<Vec<GroupResource>>,
    #[serde(default)]
    pub agents: Option<Vec<GroupResource>>,
    #[serde(default)]
    pub profiles: Option<Vec<GroupResource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupMessage {
    pub id: String,
    pub role: String,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub text_ids: Option<Vec<String>>,
    #[serde(default)]
    pub audio_ids: Option<Vec<String>>,
    #[serde(default)]
    pub image_ids: Option<Vec<String>>,
    #[serde(default)]
    pub video_ids: Option<Vec<String>>,
    #[serde(default)]
    pub file_ids: Option<Vec<String>>,
    #[serde(default)]
    pub call_ids: Option<Vec<String>>,
    #[serde(default)]
    pub calls: Option<Vec<GroupCall>>,
    #[serde(default)]
    pub reasoning: Option<bool>,
    #[serde(default)]
    pub in_context: Option<bool>,
    #[serde(default)]
    pub in_context_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupModelApiRequest {
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub snapshot_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupModelApiResponse {
    pub group_id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub snapshot_key: Option<String>,
    #[serde(default)]
    pub runs: Option<Vec<GroupRun>>,
    #[serde(default)]
    pub group_exists: Option<bool>,
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub total_message_count: Option<i64>,
    #[serde(default)]
    pub models: Option<Vec<GroupResource>>,
    #[serde(default)]
    pub agents: Option<Vec<GroupResource>>,
    #[serde(default)]
    pub profiles: Option<Vec<GroupResource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupParameterApiRequest {
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub snapshot_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupParameterApiResponse {
    pub group_id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub snapshot_key: Option<String>,
    #[serde(default)]
    pub runs: Option<Vec<GroupRun>>,
    #[serde(default)]
    pub group_exists: Option<bool>,
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub total_message_count: Option<i64>,
    #[serde(default)]
    pub models: Option<Vec<GroupResource>>,
    #[serde(default)]
    pub agents: Option<Vec<GroupResource>>,
    #[serde(default)]
    pub profiles: Option<Vec<GroupResource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupPersonaApiRequest {
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub snapshot_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupPersonaApiResponse {
    pub group_id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub snapshot_key: Option<String>,
    #[serde(default)]
    pub runs: Option<Vec<GroupRun>>,
    #[serde(default)]
    pub group_exists: Option<bool>,
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub total_message_count: Option<i64>,
    #[serde(default)]
    pub models: Option<Vec<GroupResource>>,
    #[serde(default)]
    pub agents: Option<Vec<GroupResource>>,
    #[serde(default)]
    pub profiles: Option<Vec<GroupResource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupProfileApiRequest {
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub snapshot_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupProfileApiResponse {
    pub group_id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub snapshot_key: Option<String>,
    #[serde(default)]
    pub runs: Option<Vec<GroupRun>>,
    #[serde(default)]
    pub group_exists: Option<bool>,
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub total_message_count: Option<i64>,
    #[serde(default)]
    pub models: Option<Vec<GroupResource>>,
    #[serde(default)]
    pub agents: Option<Vec<GroupResource>>,
    #[serde(default)]
    pub profiles: Option<Vec<GroupResource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupProviderApiRequest {
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub snapshot_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupProviderApiResponse {
    pub group_id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub snapshot_key: Option<String>,
    #[serde(default)]
    pub runs: Option<Vec<GroupRun>>,
    #[serde(default)]
    pub group_exists: Option<bool>,
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub total_message_count: Option<i64>,
    #[serde(default)]
    pub models: Option<Vec<GroupResource>>,
    #[serde(default)]
    pub agents: Option<Vec<GroupResource>>,
    #[serde(default)]
    pub profiles: Option<Vec<GroupResource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupResource {
    pub id: String,
    #[serde(default)]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupRubricApiRequest {
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub snapshot_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupRubricApiResponse {
    pub group_id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub snapshot_key: Option<String>,
    #[serde(default)]
    pub runs: Option<Vec<GroupRun>>,
    #[serde(default)]
    pub group_exists: Option<bool>,
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub total_message_count: Option<i64>,
    #[serde(default)]
    pub models: Option<Vec<GroupResource>>,
    #[serde(default)]
    pub agents: Option<Vec<GroupResource>>,
    #[serde(default)]
    pub profiles: Option<Vec<GroupResource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupRun {
    pub id: String,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub input_tokens: Option<i64>,
    #[serde(default)]
    pub output_tokens: Option<i64>,
    #[serde(default)]
    pub cached_input_tokens: Option<i64>,
    #[serde(default)]
    pub cost: Option<f64>,
    #[serde(default)]
    pub model_id: Option<String>,
    #[serde(default)]
    pub agent_id: Option<String>,
    #[serde(default)]
    pub profile_id: Option<String>,
    #[serde(default)]
    pub previous_context_start_index: Option<i64>,
    #[serde(default)]
    pub messages: Option<Vec<GroupMessage>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupScenarioApiRequest {
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub snapshot_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupScenarioApiResponse {
    pub group_id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub snapshot_key: Option<String>,
    #[serde(default)]
    pub runs: Option<Vec<GroupRun>>,
    #[serde(default)]
    pub group_exists: Option<bool>,
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub total_message_count: Option<i64>,
    #[serde(default)]
    pub models: Option<Vec<GroupResource>>,
    #[serde(default)]
    pub agents: Option<Vec<GroupResource>>,
    #[serde(default)]
    pub profiles: Option<Vec<GroupResource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupSettingApiRequest {
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub snapshot_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupSettingApiResponse {
    pub group_id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub snapshot_key: Option<String>,
    #[serde(default)]
    pub runs: Option<Vec<GroupRun>>,
    #[serde(default)]
    pub group_exists: Option<bool>,
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub total_message_count: Option<i64>,
    #[serde(default)]
    pub models: Option<Vec<GroupResource>>,
    #[serde(default)]
    pub agents: Option<Vec<GroupResource>>,
    #[serde(default)]
    pub profiles: Option<Vec<GroupResource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupSimulationApiRequest {
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub snapshot_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupSimulationApiResponse {
    pub group_id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub snapshot_key: Option<String>,
    #[serde(default)]
    pub runs: Option<Vec<GroupRun>>,
    #[serde(default)]
    pub group_exists: Option<bool>,
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub total_message_count: Option<i64>,
    #[serde(default)]
    pub models: Option<Vec<GroupResource>>,
    #[serde(default)]
    pub agents: Option<Vec<GroupResource>>,
    #[serde(default)]
    pub profiles: Option<Vec<GroupResource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupSystemApiRequest {
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub snapshot_key: Option<String>,
    #[serde(default)]
    pub include_detail: Option<bool>,
    #[serde(default)]
    pub message_limit: Option<i64>,
    #[serde(default)]
    pub message_offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupSystemApiResponse {
    pub group_id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub snapshot_key: Option<String>,
    #[serde(default)]
    pub runs: Option<Vec<GroupRun>>,
    #[serde(default)]
    pub group_exists: Option<bool>,
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub total_message_count: Option<i64>,
    #[serde(default)]
    pub models: Option<Vec<GroupResource>>,
    #[serde(default)]
    pub agents: Option<Vec<GroupResource>>,
    #[serde(default)]
    pub profiles: Option<Vec<GroupResource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupTestApiRequest {
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub snapshot_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupTestApiResponse {
    pub group_id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub snapshot_key: Option<String>,
    #[serde(default)]
    pub runs: Option<Vec<GroupRun>>,
    #[serde(default)]
    pub group_exists: Option<bool>,
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub total_message_count: Option<i64>,
    #[serde(default)]
    pub models: Option<Vec<GroupResource>>,
    #[serde(default)]
    pub agents: Option<Vec<GroupResource>>,
    #[serde(default)]
    pub profiles: Option<Vec<GroupResource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupToolApiRequest {
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub snapshot_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupToolApiResponse {
    pub group_id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub snapshot_key: Option<String>,
    #[serde(default)]
    pub runs: Option<Vec<GroupRun>>,
    #[serde(default)]
    pub group_exists: Option<bool>,
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub total_message_count: Option<i64>,
    #[serde(default)]
    pub models: Option<Vec<GroupResource>>,
    #[serde(default)]
    pub agents: Option<Vec<GroupResource>>,
    #[serde(default)]
    pub profiles: Option<Vec<GroupResource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HTTPValidationError {
    #[serde(default)]
    pub detail: Option<Vec<ValidationError>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthRequest {
    #[serde(default)]
    pub service: Option<String>,
    #[serde(default)]
    pub date_from: Option<String>,
    #[serde(default)]
    pub date_to: Option<String>,
    #[serde(default)]
    pub page_limit: Option<i64>,
    #[serde(default)]
    pub page_offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    #[serde(default)]
    pub views: Option<HealthViews>,
    #[serde(default)]
    pub total_count: Option<i64>,
    #[serde(default)]
    pub analytics: Option<AnalyticsFacets>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthViews {
    #[serde(default)]
    pub service_hourly: Option<Vec<GetHealthResponse>>,
    #[serde(default)]
    pub metrics_hourly: Option<Vec<GetMetricsSearchResponse>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HighlightEntry {
    #[serde(default)]
    pub section: Option<String>,
    #[serde(default)]
    pub idx: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HintEntry {
    #[serde(default)]
    pub hint: Option<String>,
    #[serde(default)]
    pub idx: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryItem {
    pub attempt_id: String,
    #[serde(default)]
    pub date: Option<String>,
    #[serde(default)]
    pub profile_id: Option<String>,
    #[serde(default)]
    pub profile_name: Option<String>,
    #[serde(default)]
    pub simulation_id: Option<String>,
    #[serde(default)]
    pub simulation_name: Option<String>,
    #[serde(default)]
    pub num_scenarios: Option<i64>,
    #[serde(default)]
    pub num_scenarios_completed: Option<i64>,
    #[serde(default)]
    pub infinite_mode: Option<bool>,
    #[serde(default)]
    pub time_limit: Option<i64>,
    #[serde(default)]
    pub persona_names_junction: Option<Vec<String>>,
    #[serde(default)]
    pub persona_colors_junction: Option<Vec<String>>,
    #[serde(default)]
    pub scenario_ids: Option<Vec<String>>,
    #[serde(default)]
    pub scenario_titles: Option<Vec<String>>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub score: Option<i64>,
    #[serde(default)]
    pub score_status: Option<String>,
    #[serde(default)]
    pub pass_pct: Option<i64>,
    #[serde(default)]
    pub show_view: Option<bool>,
    #[serde(default)]
    pub show_continue: Option<bool>,
    #[serde(default)]
    pub is_archived: Option<bool>,
    #[serde(default)]
    pub practice_simulation: Option<bool>,
    #[serde(default)]
    pub practice_scenario_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryResponse {
    #[serde(default)]
    pub data: Option<Vec<HistoryItem>>,
    #[serde(default)]
    pub total_count: Option<i64>,
    #[serde(default)]
    pub page: Option<i64>,
    #[serde(default)]
    pub page_size: Option<i64>,
    #[serde(default)]
    pub total_pages: Option<i64>,
    #[serde(default)]
    pub simulation_options: Option<Vec<FilterOption>>,
    #[serde(default)]
    pub scenario_options: Option<Vec<FilterOption>>,
    #[serde(default)]
    pub profile_options: Option<Vec<FilterOption>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageDownloadAttemptApiRequest {
    pub image_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageDownloadGroupApiRequest {
    pub image_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageDownloadScenarioApiRequest {
    pub image_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageEntry {
    #[serde(default)]
    pub image_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageUploadScenarioApiResponse {
    pub image_id: String,
    pub upload_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportField {
    pub key: String,
    pub label: String,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub multi: Option<bool>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub example: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvocationDepartmentResource {
    #[serde(default)]
    pub department_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvocationDescriptionResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvocationDetail {
    pub invocation_id: String,
    #[serde(default)]
    pub rubric_id: Option<String>,
    #[serde(default)]
    pub rubric_structure: Option<RubricStructureData>,
    #[serde(default)]
    pub primary_run_id: Option<String>,
    #[serde(default)]
    pub agent_id: Option<String>,
    #[serde(default)]
    pub model_id: Option<String>,
    #[serde(default)]
    pub voice_id: Option<String>,
    #[serde(default)]
    pub temperature_level_id: Option<String>,
    #[serde(default)]
    pub reasoning_level_id: Option<String>,
    #[serde(default)]
    pub quality_id: Option<String>,
    #[serde(default)]
    pub modality_ids: Option<Vec<String>>,
    #[serde(default)]
    pub runs: Option<Vec<InvocationRunDetail>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvocationEndpointResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub base_url: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvocationFlagResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub value: Option<bool>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub icon_id: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvocationKeyResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub key_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub key_masked: Option<String>,
    #[serde(default)]
    pub masked_key: Option<String>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvocationModalityResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub modality_id: Option<String>,
    #[serde(default)]
    pub modality: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub is_input: Option<bool>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvocationModelFlagOptionResource {
    #[serde(default)]
    pub model_id: Option<String>,
    #[serde(default)]
    pub flag_id: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub value: Option<bool>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvocationModelFlagResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub model_id: Option<String>,
    #[serde(default)]
    pub flag_id: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub value: Option<bool>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvocationModelFlagValue {
    pub model_id: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub value: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvocationNameResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvocationPricingResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub pricing_id: Option<String>,
    #[serde(default)]
    pub pricing_type: Option<String>,
    #[serde(default)]
    pub price: Option<f64>,
    #[serde(default)]
    pub unit_name: Option<String>,
    #[serde(default)]
    pub unit_category: Option<String>,
    #[serde(default)]
    pub unit_value: Option<i64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvocationQualityResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub quality_id: Option<String>,
    #[serde(default)]
    pub quality: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvocationReasoningLevelResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub reasoning_level: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvocationRunDetail {
    pub run_id: String,
    pub binding_id: String,
    #[serde(default)]
    pub grade_id: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub completed: Option<bool>,
    #[serde(default)]
    pub grade: Option<GradeData>,
    #[serde(default)]
    pub grading_state: Option<GradingStateData>,
    #[serde(default)]
    pub feedbacks: Option<Vec<FeedbackEntry>>,
    #[serde(default)]
    pub analyses: Option<Vec<AnalysisEntry>>,
    #[serde(default)]
    pub message_ids: Option<Vec<String>>,
    #[serde(default)]
    pub call_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvocationSlot {
    pub invocation_id: String,
    pub agent_id: String,
    #[serde(default)]
    pub rubric_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvocationTemperatureLevelResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub temperature: Option<f64>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvocationValueResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub value: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvocationVoiceResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub voice: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardAccoladeWinner {
    #[serde(default)]
    pub profile_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub value: Option<serde_json::Value>,
    #[serde(default)]
    pub details: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardAccoladeWinners {
    #[serde(default)]
    pub highest_scorer: Option<LeaderboardAccoladeWinner>,
    #[serde(default)]
    pub perfect_score: Option<LeaderboardAccoladeWinner>,
    #[serde(default)]
    pub longest_convo: Option<LeaderboardAccoladeWinner>,
    #[serde(default)]
    pub response_times: Option<LeaderboardAccoladeWinner>,
    #[serde(default)]
    pub quickest_pass: Option<LeaderboardAccoladeWinner>,
    #[serde(default)]
    pub the_persistent: Option<LeaderboardAccoladeWinner>,
    #[serde(default)]
    pub marathon_runner: Option<LeaderboardAccoladeWinner>,
    #[serde(default)]
    pub rapid_riser: Option<LeaderboardAccoladeWinner>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardDataRow {
    #[serde(default)]
    pub rank: Option<i64>,
    #[serde(default)]
    pub profile_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub simulation_ids: Option<Vec<String>>,
    #[serde(default)]
    pub scenario_ids: Option<Vec<String>>,
    #[serde(default)]
    pub metrics_entry: Option<LeaderboardMetricsEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardHeaderMetrics {
    #[serde(default)]
    pub total_profiles: Option<LeaderboardMetric>,
    #[serde(default)]
    pub total_attempts: Option<LeaderboardMetric>,
    #[serde(default)]
    pub average_score: Option<LeaderboardMetric>,
    #[serde(default)]
    pub perfect_scores: Option<LeaderboardMetric>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardMetric {
    #[serde(default)]
    pub has_data: Option<bool>,
    #[serde(default)]
    pub method: Option<String>,
    #[serde(default)]
    pub current_value: Option<serde_json::Value>,
    #[serde(default)]
    pub key_field: Option<String>,
    #[serde(default)]
    pub trend_data: Option<Vec<String>>,
    #[serde(default)]
    pub data_points: Option<Vec<String>>,
    #[serde(default)]
    pub hover: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardMetricsEntry {
    #[serde(default)]
    pub total_attempts: Option<LeaderboardMetric>,
    #[serde(default)]
    pub highest_score_avg: Option<LeaderboardMetric>,
    #[serde(default)]
    pub messages_per_session: Option<LeaderboardMetric>,
    #[serde(default)]
    pub persona_response_seconds: Option<LeaderboardMetric>,
    #[serde(default)]
    pub time_spent_minutes: Option<LeaderboardMetric>,
    #[serde(default)]
    pub improvement_rate_per_day: Option<LeaderboardMetric>,
    #[serde(default)]
    pub perfect_score_count: Option<LeaderboardMetric>,
    #[serde(default)]
    pub quickest_pass_minutes: Option<LeaderboardMetric>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardProfileResource {
    #[serde(default)]
    pub profile_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub role: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardRequest {
    #[serde(default)]
    pub start_date: Option<String>,
    #[serde(default)]
    pub end_date: Option<String>,
    #[serde(default)]
    pub cohort_ids: Option<Vec<String>>,
    #[serde(default)]
    pub simulation_ids: Option<Vec<String>>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub simulation_filters: Option<Vec<String>>,
    #[serde(default)]
    pub target_profile_id: Option<String>,
    #[serde(default)]
    pub cohort_id: Option<String>,
    #[serde(default)]
    pub simulation_id: Option<String>,
    #[serde(default)]
    pub scenario_ids: Option<Vec<String>>,
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub sort_by: Option<String>,
    #[serde(default)]
    pub sort_order: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardResources {
    #[serde(default)]
    pub profiles: Option<std::collections::HashMap<String, LeaderboardProfileResource>>,
    #[serde(default)]
    pub simulations: Option<std::collections::HashMap<String, LeaderboardSimulationResource>>,
    #[serde(default)]
    pub scenarios: Option<std::collections::HashMap<String, LeaderboardScenarioResource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardResponse {
    #[serde(default)]
    pub sections: Option<LeaderboardSections>,
    #[serde(default)]
    pub resources: Option<LeaderboardResources>,
    #[serde(default)]
    pub analytics: Option<AnalyticsFacets>,
    #[serde(default)]
    pub data: Option<Vec<LeaderboardDataRow>>,
    #[serde(default)]
    pub total_count: Option<i64>,
    #[serde(default)]
    pub simulation_options: Option<Vec<FilterOption>>,
    #[serde(default)]
    pub profile_options: Option<Vec<FilterOption>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardScenarioResource {
    #[serde(default)]
    pub scenario_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardSectionStatus {
    #[serde(default)]
    pub has_data: Option<bool>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardSections {
    #[serde(default)]
    pub header_metrics: Option<LeaderboardHeaderMetrics>,
    #[serde(default)]
    pub rankings: Option<LeaderboardSectionStatus>,
    #[serde(default)]
    pub accolades: Option<LeaderboardSectionStatus>,
    #[serde(default)]
    pub trends: Option<LeaderboardSectionStatus>,
    #[serde(default)]
    pub filters: Option<LeaderboardSectionStatus>,
    #[serde(default)]
    pub accolade_winners: Option<LeaderboardAccoladeWinners>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardSimulationResource {
    #[serde(default)]
    pub simulation_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListActivityRequest {
    #[serde(default)]
    pub date_from: Option<String>,
    #[serde(default)]
    pub date_to: Option<String>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub role_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub page: Option<i64>,
    #[serde(default)]
    pub page_size: Option<i64>,
    #[serde(default)]
    pub sort_order: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListActivityResponse {
    #[serde(default)]
    pub data: Option<Vec<SessionListItem>>,
    #[serde(default)]
    pub total_count: Option<i64>,
    #[serde(default)]
    pub page: Option<i64>,
    #[serde(default)]
    pub page_size: Option<i64>,
    #[serde(default)]
    pub total_pages: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListAgentApiAgent {
    #[serde(default)]
    pub agent_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub reasoning: Option<String>,
    #[serde(default)]
    pub temperature: Option<f64>,
    #[serde(default)]
    pub model_id: Option<String>,
    #[serde(default)]
    pub model_name: Option<String>,
    #[serde(default)]
    pub model_description: Option<String>,
    #[serde(default)]
    pub role: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub is_inactive: Option<bool>,
    #[serde(default)]
    pub is_mcp: Option<bool>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub can_duplicate: Option<bool>,
    #[serde(default)]
    pub can_delete: Option<bool>,
    #[serde(default)]
    pub pending_status: Option<String>,
    #[serde(default)]
    pub pending_operation: Option<String>,
    #[serde(default)]
    pub pending_call_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListAgentApiResponse {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub agents: Option<Vec<ListAgentApiAgent>>,
    #[serde(default)]
    pub department_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub model_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub tool_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub flag_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub total_count: Option<i64>,
    #[serde(default)]
    pub import_fields: Option<Vec<ImportField>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListAuthApiAuth {
    #[serde(default)]
    pub auth_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub item_count: Option<i64>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub setting_ids: Option<Vec<String>>,
    #[serde(default)]
    pub auth_item_key_ids: Option<Vec<String>>,
    #[serde(default)]
    pub is_inactive: Option<bool>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub can_duplicate: Option<bool>,
    #[serde(default)]
    pub can_delete: Option<bool>,
    #[serde(default)]
    pub pending_status: Option<String>,
    #[serde(default)]
    pub pending_operation: Option<String>,
    #[serde(default)]
    pub pending_call_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListAuthApiResponse {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub auths: Option<Vec<ListAuthApiAuth>>,
    #[serde(default)]
    pub department_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub flag_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub settings_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub auth_item_keys_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub total_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCohortApiCohort {
    #[serde(default)]
    pub cohort_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub is_inactive: Option<bool>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub mcp: Option<bool>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub profile_ids: Option<Vec<String>>,
    #[serde(default)]
    pub simulation_ids: Option<Vec<String>>,
    #[serde(default)]
    pub usage_count: Option<i64>,
    #[serde(default)]
    pub num_members: Option<i64>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub can_delete: Option<bool>,
    #[serde(default)]
    pub can_duplicate: Option<bool>,
    #[serde(default)]
    pub can_leave: Option<bool>,
    #[serde(default)]
    pub updated_at: Option<String>,
    #[serde(default)]
    pub pending_status: Option<String>,
    #[serde(default)]
    pub pending_operation: Option<String>,
    #[serde(default)]
    pub pending_call_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCohortApiDepartment {
    #[serde(default)]
    pub department_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCohortApiProfile {
    #[serde(default)]
    pub profile_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCohortApiResponse {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub user_role: Option<String>,
    #[serde(default)]
    pub cohorts: Option<Vec<ListCohortApiCohort>>,
    #[serde(default)]
    pub profiles: Option<Vec<ListCohortApiProfile>>,
    #[serde(default)]
    pub simulations: Option<Vec<ListCohortApiSimulation>>,
    #[serde(default)]
    pub departments: Option<Vec<ListCohortApiDepartment>>,
    #[serde(default)]
    pub simulation_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub profile_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub department_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub flag_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub total_count: Option<i64>,
    #[serde(default)]
    pub import_fields: Option<Vec<ImportField>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCohortApiSimulation {
    #[serde(default)]
    pub simulation_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListDepartmentApiDepartment {
    #[serde(default)]
    pub department_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub staff_count: Option<i64>,
    #[serde(default)]
    pub profile_ids: Option<Vec<String>>,
    #[serde(default)]
    pub setting_ids: Option<Vec<String>>,
    #[serde(default)]
    pub login_ids: Option<Vec<String>>,
    #[serde(default)]
    pub is_inactive: Option<bool>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub can_duplicate: Option<bool>,
    #[serde(default)]
    pub can_delete: Option<bool>,
    #[serde(default)]
    pub updated_at: Option<String>,
    #[serde(default)]
    pub pending_status: Option<String>,
    #[serde(default)]
    pub pending_operation: Option<String>,
    #[serde(default)]
    pub pending_call_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListDepartmentApiResponse {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub departments: Option<Vec<ListDepartmentApiDepartment>>,
    #[serde(default)]
    pub flag_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub profile_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub settings_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub logins_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub total_count: Option<i64>,
    #[serde(default)]
    pub import_fields: Option<Vec<ImportField>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListDocumentApiDocument {
    #[serde(default)]
    pub document_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub scenario_ids: Option<Vec<String>>,
    #[serde(default)]
    pub field_ids: Option<Vec<String>>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub is_inactive: Option<bool>,
    #[serde(default)]
    pub is_template: Option<bool>,
    #[serde(default)]
    pub pending_status: Option<String>,
    #[serde(default)]
    pub pending_operation: Option<String>,
    #[serde(default)]
    pub pending_call_id: Option<String>,
    #[serde(default)]
    pub extension: Option<String>,
    #[serde(default)]
    pub num_scenarios: Option<i64>,
    #[serde(default)]
    pub active_scenario_count: Option<i64>,
    #[serde(default)]
    pub file_id: Option<String>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub can_duplicate: Option<bool>,
    #[serde(default)]
    pub can_delete: Option<bool>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListDocumentApiResponse {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub documents: Option<Vec<ListDocumentApiDocument>>,
    #[serde(default)]
    pub scenario_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub field_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub department_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub flag_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub total_count: Option<i64>,
    #[serde(default)]
    pub import_fields: Option<Vec<ImportField>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListEvalApiEval {
    #[serde(default)]
    pub eval_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub model_ids: Option<Vec<String>>,
    #[serde(default)]
    pub rubric_ids: Option<Vec<String>>,
    #[serde(default)]
    pub is_inactive: Option<bool>,
    #[serde(default)]
    pub is_dynamic: Option<bool>,
    #[serde(default)]
    pub use_groups: Option<bool>,
    #[serde(default)]
    pub pending_status: Option<String>,
    #[serde(default)]
    pub pending_operation: Option<String>,
    #[serde(default)]
    pub pending_call_id: Option<String>,
    #[serde(default)]
    pub num_runs: Option<i64>,
    #[serde(default)]
    pub num_groups: Option<i64>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub can_duplicate: Option<bool>,
    #[serde(default)]
    pub can_delete: Option<bool>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListEvalApiResponse {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub evals: Option<Vec<ListEvalApiEval>>,
    #[serde(default)]
    pub department_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub flag_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub model_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub rubric_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub total_count: Option<i64>,
    #[serde(default)]
    pub user_role: Option<String>,
    #[serde(default)]
    pub import_fields: Option<Vec<ImportField>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListFieldApiField {
    #[serde(default)]
    pub field_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub conditional_parameter_ids: Option<Vec<String>>,
    #[serde(default)]
    pub persona_ids: Option<Vec<String>>,
    #[serde(default)]
    pub is_inactive: Option<bool>,
    #[serde(default)]
    pub pending_status: Option<String>,
    #[serde(default)]
    pub pending_operation: Option<String>,
    #[serde(default)]
    pub pending_call_id: Option<String>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub can_duplicate: Option<bool>,
    #[serde(default)]
    pub can_delete: Option<bool>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListFieldApiResponse {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub fields: Option<Vec<ListFieldApiField>>,
    #[serde(default)]
    pub parameter_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub persona_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub department_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub flag_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub total_count: Option<i64>,
    #[serde(default)]
    pub import_fields: Option<Vec<ImportField>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListFilterOption {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub count: Option<i64>,
    #[serde(default)]
    pub hex_code: Option<String>,
    #[serde(default)]
    pub value: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListFilterSection {
    #[serde(default)]
    pub options: Option<Vec<ListFilterOption>>,
    #[serde(default)]
    pub selected_ids: Option<Vec<String>>,
    #[serde(default)]
    pub search: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListModelApiModel {
    #[serde(default)]
    pub model_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub provider_id: Option<String>,
    #[serde(default)]
    pub provider_name: Option<String>,
    #[serde(default)]
    pub base_url: Option<String>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub is_inactive: Option<bool>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub image_model: Option<bool>,
    #[serde(default)]
    pub pending_status: Option<String>,
    #[serde(default)]
    pub pending_operation: Option<String>,
    #[serde(default)]
    pub pending_call_id: Option<String>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub can_duplicate: Option<bool>,
    #[serde(default)]
    pub can_delete: Option<bool>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListModelApiResponse {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub models: Option<Vec<ListModelApiModel>>,
    #[serde(default)]
    pub provider_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub department_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub agent_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub flag_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub total_count: Option<i64>,
    #[serde(default)]
    pub import_fields: Option<Vec<ImportField>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListParameterApiParameter {
    #[serde(default)]
    pub parameter_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub is_inactive: Option<bool>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub scenario_ids: Option<Vec<String>>,
    #[serde(default)]
    pub document_ids: Option<Vec<String>>,
    #[serde(default)]
    pub num_items: Option<i64>,
    #[serde(default)]
    pub sample_items: Option<Vec<String>>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub can_duplicate: Option<bool>,
    #[serde(default)]
    pub can_delete: Option<bool>,
    #[serde(default)]
    pub updated_at: Option<String>,
    #[serde(default)]
    pub pending_status: Option<String>,
    #[serde(default)]
    pub pending_operation: Option<String>,
    #[serde(default)]
    pub pending_call_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListParameterApiResponse {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub parameters: Option<Vec<ListParameterApiParameter>>,
    #[serde(default)]
    pub scenario_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub field_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub department_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub flag_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub total_count: Option<i64>,
    #[serde(default)]
    pub import_fields: Option<Vec<ImportField>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPersonaApiPersona {
    #[serde(default)]
    pub persona_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub scenario_ids: Option<Vec<String>>,
    #[serde(default)]
    pub field_ids: Option<Vec<String>>,
    #[serde(default)]
    pub is_inactive: Option<bool>,
    #[serde(default)]
    pub pending_status: Option<String>,
    #[serde(default)]
    pub pending_operation: Option<String>,
    #[serde(default)]
    pub pending_call_id: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub mcp: Option<bool>,
    #[serde(default)]
    pub num_scenarios: Option<i64>,
    #[serde(default)]
    pub num_profiles: Option<i64>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub can_duplicate: Option<bool>,
    #[serde(default)]
    pub can_delete: Option<bool>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPersonaApiResponse {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub personas: Option<Vec<ListPersonaApiPersona>>,
    #[serde(default)]
    pub scenario_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub field_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub department_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub color_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub icon_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub voice_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub instruction_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub flag_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub total_count: Option<i64>,
    #[serde(default)]
    pub import_fields: Option<Vec<ImportField>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPricingRequest {
    #[serde(default)]
    pub start_date: Option<String>,
    #[serde(default)]
    pub end_date: Option<String>,
    #[serde(default)]
    pub date_from: Option<String>,
    #[serde(default)]
    pub date_to: Option<String>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub model_id: Option<String>,
    #[serde(default)]
    pub model_ids: Option<Vec<String>>,
    #[serde(default)]
    pub profile_ids: Option<Vec<String>>,
    #[serde(default)]
    pub agent_ids: Option<Vec<String>>,
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub page: Option<i64>,
    #[serde(default)]
    pub page_size: Option<i64>,
    #[serde(default)]
    pub sort_by: Option<String>,
    #[serde(default)]
    pub sort_order: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPricingResponse {
    #[serde(default)]
    pub data: Option<Vec<PricingGroupItem>>,
    #[serde(default)]
    pub total_count: Option<i64>,
    #[serde(default)]
    pub page: Option<i64>,
    #[serde(default)]
    pub page_size: Option<i64>,
    #[serde(default)]
    pub total_pages: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListProfilesApiProfile {
    #[serde(default)]
    pub profile_id: Option<String>,
    #[serde(default)]
    pub emails: Option<Vec<String>>,
    #[serde(default)]
    pub primary_email: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub role: Option<String>,
    #[serde(default)]
    pub role_name: Option<String>,
    #[serde(default)]
    pub initials: Option<String>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub primary_department_id: Option<String>,
    #[serde(default)]
    pub permission_ids: Option<Vec<String>>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub can_duplicate: Option<bool>,
    #[serde(default)]
    pub can_delete: Option<bool>,
    #[serde(default)]
    pub can_emulate: Option<bool>,
    #[serde(default)]
    pub is_emulated: Option<bool>,
    #[serde(default)]
    pub is_inactive: Option<bool>,
    #[serde(default)]
    pub pending_status: Option<String>,
    #[serde(default)]
    pub pending_operation: Option<String>,
    #[serde(default)]
    pub pending_call_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListProfilesApiResponse {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub profiles: Option<Vec<ListProfilesApiProfile>>,
    #[serde(default)]
    pub department_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub role_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub flag_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub permissions_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub total_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListProviderApiProvider {
    #[serde(default)]
    pub provider_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub value: Option<String>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub is_inactive: Option<bool>,
    #[serde(default)]
    pub updated_at: Option<String>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub model_usage_count: Option<i64>,
    #[serde(default)]
    pub model_ids: Option<Vec<String>>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub can_delete: Option<bool>,
    #[serde(default)]
    pub can_duplicate: Option<bool>,
    #[serde(default)]
    pub pending_status: Option<String>,
    #[serde(default)]
    pub pending_operation: Option<String>,
    #[serde(default)]
    pub pending_call_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListProviderApiResponse {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub providers: Option<Vec<ListProviderApiProvider>>,
    #[serde(default)]
    pub department_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub model_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub status_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub flag_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub total_count: Option<i64>,
    #[serde(default)]
    pub import_fields: Option<Vec<ImportField>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListRubricApiResponse {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub rubrics: Option<Vec<ListRubricApiRubric>>,
    #[serde(default)]
    pub standard_groups: Option<Vec<ListRubricApiStandardGroup>>,
    #[serde(default)]
    pub standards: Option<Vec<ListRubricApiStandard>>,
    #[serde(default)]
    pub department_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub simulation_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub flag_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub eval_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub total_count: Option<i64>,
    #[serde(default)]
    pub import_fields: Option<Vec<ImportField>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListRubricApiRubric {
    #[serde(default)]
    pub rubric_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub points: Option<i64>,
    #[serde(default)]
    pub pass_points: Option<i64>,
    #[serde(default)]
    pub pass_percentage: Option<i64>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub simulation_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active_simulation_count: Option<i64>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub can_delete: Option<bool>,
    #[serde(default)]
    pub can_duplicate: Option<bool>,
    #[serde(default)]
    pub standard_group_ids: Option<Vec<String>>,
    #[serde(default)]
    pub eval_ids: Option<Vec<String>>,
    #[serde(default)]
    pub is_inactive: Option<bool>,
    #[serde(default)]
    pub pending_status: Option<String>,
    #[serde(default)]
    pub pending_operation: Option<String>,
    #[serde(default)]
    pub pending_call_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListRubricApiStandard {
    #[serde(default)]
    pub standard_id: Option<String>,
    #[serde(default)]
    pub standard_group_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub points: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListRubricApiStandardGroup {
    #[serde(default)]
    pub standard_group_id: Option<String>,
    #[serde(default)]
    pub rubric_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub points: Option<i64>,
    #[serde(default)]
    pub pass_points: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListScenarioApiCohort {
    #[serde(default)]
    pub cohort_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListScenarioApiDepartment {
    #[serde(default)]
    pub department_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListScenarioApiField {
    #[serde(default)]
    pub field_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListScenarioApiObjective {
    #[serde(default)]
    pub objective_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListScenarioApiPersona {
    #[serde(default)]
    pub persona_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListScenarioApiResponse {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub scenarios: Option<Vec<ListScenarioApiScenario>>,
    #[serde(default)]
    pub objectives: Option<Vec<ListScenarioApiObjective>>,
    #[serde(default)]
    pub fields: Option<Vec<ListScenarioApiField>>,
    #[serde(default)]
    pub cohorts: Option<Vec<ListScenarioApiCohort>>,
    #[serde(default)]
    pub personas: Option<Vec<ListScenarioApiPersona>>,
    #[serde(default)]
    pub simulations: Option<Vec<ListScenarioApiSimulation>>,
    #[serde(default)]
    pub departments: Option<Vec<ListScenarioApiDepartment>>,
    #[serde(default)]
    pub persona_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub simulation_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub department_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub flag_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub total_count: Option<i64>,
    #[serde(default)]
    pub import_fields: Option<Vec<ImportField>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListScenarioApiScenario {
    #[serde(default)]
    pub scenario_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub problem_statement: Option<String>,
    #[serde(default)]
    pub is_inactive: Option<bool>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub mcp: Option<bool>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub objective_ids: Option<Vec<String>>,
    #[serde(default)]
    pub persona_ids: Option<Vec<String>>,
    #[serde(default)]
    pub field_ids: Option<Vec<String>>,
    #[serde(default)]
    pub simulation_ids: Option<Vec<String>>,
    #[serde(default)]
    pub num_simulations: Option<i64>,
    #[serde(default)]
    pub active_simulation_count: Option<i64>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub can_delete: Option<bool>,
    #[serde(default)]
    pub can_duplicate: Option<bool>,
    #[serde(default)]
    pub cohort_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_status: Option<String>,
    #[serde(default)]
    pub pending_operation: Option<String>,
    #[serde(default)]
    pub pending_call_id: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListScenarioApiSimulation {
    #[serde(default)]
    pub simulation_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListSettingApiKey {
    #[serde(default)]
    pub key_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub key_masked: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListSettingApiResponse {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub user_role: Option<String>,
    #[serde(default)]
    pub settings: Option<Vec<ListSettingApiSetting>>,
    #[serde(default)]
    pub keys: Option<Vec<ListSettingApiKey>>,
    #[serde(default)]
    pub flag_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub providers_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub auth_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub systems_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub import_fields: Option<Vec<ImportField>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListSettingApiSetting {
    #[serde(default)]
    pub settings_id: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub is_inactive: Option<bool>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub provider_ids: Option<Vec<String>>,
    #[serde(default)]
    pub auth_ids: Option<Vec<String>>,
    #[serde(default)]
    pub system_ids: Option<Vec<String>>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub can_delete: Option<bool>,
    #[serde(default)]
    pub can_duplicate: Option<bool>,
    #[serde(default)]
    pub pending_status: Option<String>,
    #[serde(default)]
    pub pending_operation: Option<String>,
    #[serde(default)]
    pub pending_call_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListSimulationApiPersona {
    #[serde(default)]
    pub persona_id: Option<String>,
    #[serde(default)]
    pub color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListSimulationApiResponse {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub simulations: Option<Vec<ListSimulationApiSimulation>>,
    #[serde(default)]
    pub scenarios: Option<Vec<ListSimulationApiScenario>>,
    #[serde(default)]
    pub scenario_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub cohort_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub department_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub flag_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub total_count: Option<i64>,
    #[serde(default)]
    pub import_fields: Option<Vec<ImportField>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListSimulationApiScenario {
    #[serde(default)]
    pub scenario_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub persona_ids: Option<Vec<String>>,
    #[serde(default)]
    pub persona_mapping: Option<Vec<ListSimulationApiPersona>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListSimulationApiSimulation {
    #[serde(default)]
    pub simulation_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub is_inactive: Option<bool>,
    #[serde(default)]
    pub is_practice: Option<bool>,
    #[serde(default)]
    pub practice_simulation: Option<bool>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub mcp: Option<bool>,
    #[serde(default)]
    pub scenario_ids: Option<Vec<String>>,
    #[serde(default)]
    pub num_cohorts: Option<i64>,
    #[serde(default)]
    pub cohort_usage_count: Option<i64>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub can_delete: Option<bool>,
    #[serde(default)]
    pub can_duplicate: Option<bool>,
    #[serde(default)]
    pub cohort_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_status: Option<String>,
    #[serde(default)]
    pub pending_operation: Option<String>,
    #[serde(default)]
    pub pending_call_id: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListToolApiResponse {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub tools: Option<Vec<ListToolApiTool>>,
    #[serde(default)]
    pub department_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub creatable_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub agent_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub flag_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub permissions_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub total_count: Option<i64>,
    #[serde(default)]
    pub import_fields: Option<Vec<ImportField>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListToolApiTool {
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub is_inactive: Option<bool>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub permission_ids: Option<Vec<String>>,
    #[serde(default)]
    pub agent_ids: Option<Vec<String>>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub updated_at: Option<String>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub can_duplicate: Option<bool>,
    #[serde(default)]
    pub can_delete: Option<bool>,
    #[serde(default)]
    pub pending_status: Option<String>,
    #[serde(default)]
    pub pending_operation: Option<String>,
    #[serde(default)]
    pub pending_call_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageData {
    pub id: String,
    #[serde(default)]
    pub chat_id: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub completed: Option<bool>,
    #[serde(default)]
    pub contents: Option<Vec<ContentEntry>>,
    #[serde(default)]
    pub feedbacks: Option<Vec<MessageFeedbackEntry>>,
    #[serde(default)]
    pub hints: Option<Vec<HintEntry>>,
    #[serde(default)]
    pub parent_message_id: Option<String>,
    #[serde(default)]
    pub sibling_index: Option<i64>,
    #[serde(default)]
    pub sibling_count: Option<i64>,
    #[serde(default)]
    pub audios_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageFeedbackEntry {
    pub id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub highlights: Option<Vec<HighlightEntry>>,
    #[serde(default)]
    pub replaces: Option<Vec<ReplacementEntry>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelDepartmentResource {
    #[serde(default)]
    pub department_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelDescriptionResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelFieldError {
    pub field: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelFlagResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub value: Option<bool>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub icon_id: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelModalityResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub modality: Option<String>,
    #[serde(default)]
    pub is_input: Option<bool>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelNameResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelPricingResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub pricing_type: Option<String>,
    #[serde(default)]
    pub price: Option<f64>,
    #[serde(default)]
    pub unit_name: Option<String>,
    #[serde(default)]
    pub unit_category: Option<String>,
    #[serde(default)]
    pub unit_value: Option<f64>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelProviderResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub value: Option<String>,
    #[serde(default)]
    pub base_url: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelQualityResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub quality: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelReasoningLevelResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub reasoning_level: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelResultItem {
    pub success: bool,
    pub message: String,
    #[serde(default)]
    pub model_id: Option<String>,
    #[serde(default)]
    pub errors: Option<Vec<ModelFieldError>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelTemperatureLevelResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub temperature: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelValueResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub value: Option<String>,
    #[serde(default)]
    pub value_type: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelVoiceResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub voice: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MvInfo {
    pub name: String,
    pub definition: String,
    pub columns: Vec<ColumnInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectiveEntry {
    #[serde(default)]
    pub objective_id: Option<String>,
    #[serde(default)]
    pub objective: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationInfo {
    pub name: String,
    pub description: String,
    pub params: Vec<ParamInfo>,
    #[serde(default)]
    pub returns: Option<std::collections::HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationPrompts {
    #[serde(default)]
    pub prompts: Option<std::collections::HashMap<String, Vec<StarterPrompt>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionEntry {
    #[serde(default)]
    pub option_id: Option<String>,
    #[serde(default)]
    pub question_id: Option<String>,
    #[serde(default)]
    pub option_text: Option<String>,
    #[serde(default)]
    pub is_correct: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageMetaItem {
    pub title: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamInfo {
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub required: bool,
    #[serde(default)]
    pub default: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterDepartmentResource {
    #[serde(default)]
    pub department_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterDescriptionResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterFieldError {
    pub field: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterFieldResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub field_id: Option<String>,
    #[serde(default)]
    pub parameter_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub conditional_parameter_id: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterFlagResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub value: Option<bool>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub icon_id: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterNameResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterResultItem {
    pub success: bool,
    pub message: String,
    #[serde(default)]
    pub parameter_id: Option<String>,
    #[serde(default)]
    pub errors: Option<Vec<ParameterFieldError>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseAgentCsvApiResponse {
    pub upload_id: String,
    pub items: Vec<CreateAgentItem>,
    pub mapped_fields: Vec<String>,
    pub row_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseAuthCsvApiResponse {
    pub upload_id: String,
    pub items: Vec<CreateAuthItem>,
    pub mapped_fields: Vec<String>,
    pub row_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseCohortCsvApiResponse {
    pub upload_id: String,
    pub items: Vec<CreateCohortItem>,
    pub mapped_fields: Vec<String>,
    pub row_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseDepartmentCsvApiResponse {
    pub upload_id: String,
    pub items: Vec<CreateDepartmentItem>,
    pub mapped_fields: Vec<String>,
    pub row_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseDocumentCsvApiResponse {
    pub upload_id: String,
    pub items: Vec<CreateDocumentItem>,
    pub mapped_fields: Vec<String>,
    pub row_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseEvalCsvApiResponse {
    pub upload_id: String,
    pub items: Vec<CreateEvalItem>,
    pub mapped_fields: Vec<String>,
    pub row_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseFieldCsvApiResponse {
    pub upload_id: String,
    pub items: Vec<CreateFieldItem>,
    pub mapped_fields: Vec<String>,
    pub row_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseModelCsvApiResponse {
    pub upload_id: String,
    pub items: Vec<CreateModelItem>,
    pub mapped_fields: Vec<String>,
    pub row_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseParameterCsvApiResponse {
    pub upload_id: String,
    pub items: Vec<CreateParameterItem>,
    pub mapped_fields: Vec<String>,
    pub row_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsePersonaCsvApiResponse {
    pub upload_id: String,
    pub items: Vec<CreatePersonaItem>,
    pub mapped_fields: Vec<String>,
    pub row_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseProfileCsvApiResponse {
    pub upload_id: String,
    pub items: Vec<CreateProfileItem>,
    pub mapped_fields: Vec<String>,
    pub row_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseProviderCsvApiResponse {
    pub upload_id: String,
    pub items: Vec<CreateProviderItem>,
    pub mapped_fields: Vec<String>,
    pub row_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseRubricCsvApiResponse {
    pub upload_id: String,
    pub items: Vec<CreateRubricItem>,
    pub mapped_fields: Vec<String>,
    pub row_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseScenarioCsvApiResponse {
    pub upload_id: String,
    pub items: Vec<CreateScenarioItem>,
    pub mapped_fields: Vec<String>,
    pub row_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseSettingCsvApiResponse {
    pub upload_id: String,
    pub items: Vec<CreateSettingItem>,
    pub mapped_fields: Vec<String>,
    pub row_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseSimulationCsvApiResponse {
    pub upload_id: String,
    pub items: Vec<CreateSimulationItem>,
    pub mapped_fields: Vec<String>,
    pub row_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseToolCsvApiResponse {
    pub upload_id: String,
    pub items: Vec<CreateToolItem>,
    pub mapped_fields: Vec<String>,
    pub row_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchAgentDraftApiRequest {
    #[serde(default)]
    pub draft_id: Option<String>,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub input_draft_id: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub model_id: Option<String>,
    #[serde(default)]
    pub tool_ids: Option<Vec<String>>,
    #[serde(default)]
    pub reasoning_level: Option<String>,
    #[serde(default)]
    pub reasoning_level_id: Option<String>,
    #[serde(default)]
    pub temperature_level: Option<String>,
    #[serde(default)]
    pub temperature_level_id: Option<String>,
    #[serde(default)]
    pub voices: Option<Vec<String>>,
    #[serde(default)]
    pub voice_ids: Option<Vec<String>>,
    #[serde(default)]
    pub qualities: Option<Vec<String>>,
    #[serde(default)]
    pub quality_ids: Option<Vec<String>>,
    #[serde(default)]
    pub prompt_id: Option<String>,
    #[serde(default)]
    pub prompt: Option<CreatePromptInput>,
    #[serde(default)]
    pub instruction_id: Option<String>,
    #[serde(default)]
    pub instructions_id: Option<String>,
    #[serde(default)]
    pub rubric_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchAgentDraftApiResponse {
    pub success: bool,
    pub draft_id: String,
    pub message: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub form_state: Option<App__infra__agent__types__DraftFormState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchAuthDraftApiRequest {
    #[serde(default)]
    pub draft_id: Option<String>,
    #[serde(default)]
    pub input_draft_id: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub protocols: Option<Vec<String>>,
    #[serde(default)]
    pub protocol_ids: Option<Vec<String>>,
    #[serde(default)]
    pub slugs: Option<Vec<String>>,
    #[serde(default)]
    pub slug_ids: Option<Vec<String>>,
    #[serde(default)]
    pub item_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchAuthDraftApiResponse {
    pub success: bool,
    pub draft_id: String,
    pub message: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub form_state: Option<App__infra__auth__types__DraftFormState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchChatDraftApiRequest {
    #[serde(default)]
    pub draft_id: Option<String>,
    #[serde(default)]
    pub input_draft_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub problem_statement: Option<String>,
    #[serde(default)]
    pub problem_statement_id: Option<String>,
    #[serde(default)]
    pub objectives: Option<Vec<String>>,
    #[serde(default)]
    pub objective_ids: Option<Vec<String>>,
    #[serde(default)]
    pub images: Option<Vec<App__infra__attempt__chat__types__DraftImageValue>>,
    #[serde(default)]
    pub image_ids: Option<Vec<String>>,
    #[serde(default)]
    pub videos: Option<Vec<App__infra__attempt__chat__types__DraftVideoValue>>,
    #[serde(default)]
    pub video_ids: Option<Vec<String>>,
    #[serde(default)]
    pub questions: Option<Vec<App__infra__attempt__chat__types__DraftQuestionValue>>,
    #[serde(default)]
    pub question_ids: Option<Vec<String>>,
    #[serde(default)]
    pub options: Option<Vec<App__infra__attempt__chat__types__DraftOptionValue>>,
    #[serde(default)]
    pub option_ids: Option<Vec<String>>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub document_ids: Option<Vec<String>>,
    #[serde(default)]
    pub field_ids: Option<Vec<String>>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub parameter_field_ids: Option<Vec<String>>,
    #[serde(default)]
    pub parameter_ids: Option<Vec<String>>,
    #[serde(default)]
    pub persona_ids: Option<Vec<String>>,
    #[serde(default)]
    pub scenario_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_ids: Option<Vec<String>>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchChatDraftApiResponse {
    pub success: bool,
    pub draft_id: String,
    pub message: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub form_state: Option<ChatDraftFormState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchCohortDraftApiRequest {
    #[serde(default)]
    pub draft_id: Option<String>,
    #[serde(default)]
    pub input_draft_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub simulation_ids: Option<Vec<String>>,
    #[serde(default)]
    pub simulations: Option<Vec<String>>,
    #[serde(default)]
    pub profile_ids: Option<Vec<String>>,
    #[serde(default)]
    pub profiles: Option<Vec<String>>,
    #[serde(default)]
    pub simulation_position_ids: Option<Vec<String>>,
    #[serde(default)]
    pub simulation_positions: Option<Vec<DraftSimulationPositionValue>>,
    #[serde(default)]
    pub simulation_availability_ids: Option<Vec<String>>,
    #[serde(default)]
    pub simulation_availability: Option<Vec<DraftSimulationAvailabilityValue>>,
    #[serde(default)]
    pub profile_persona_ids: Option<Vec<String>>,
    #[serde(default)]
    pub profile_personas: Option<Vec<DraftProfilePersonaValue>>,
    #[serde(default)]
    pub pending_ids: Option<Vec<String>>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchCohortDraftApiResponse {
    pub success: bool,
    pub draft_id: String,
    pub idempotency_key: String,
    pub message: String,
    #[serde(default)]
    pub form_state: Option<App__infra__cohort__types__DraftFormState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchDepartmentDraftApiRequest {
    #[serde(default)]
    pub draft_id: Option<String>,
    #[serde(default)]
    pub input_draft_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub settings: Option<Vec<String>>,
    #[serde(default)]
    pub setting_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_ids: Option<Vec<String>>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchDepartmentDraftApiResponse {
    pub success: bool,
    pub draft_id: String,
    pub message: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub form_state: Option<App__infra__department__types__DraftFormState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchDocumentDraftApiRequest {
    #[serde(default)]
    pub draft_id: Option<String>,
    #[serde(default)]
    pub input_draft_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub files: Option<Vec<DraftFileValue>>,
    #[serde(default)]
    pub file_ids: Option<Vec<String>>,
    #[serde(default)]
    pub texts: Option<Vec<DraftTextValue>>,
    #[serde(default)]
    pub text_ids: Option<Vec<String>>,
    #[serde(default)]
    pub images: Option<Vec<App__infra__document__types__DraftImageValue>>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub image_ids: Option<Vec<String>>,
    #[serde(default)]
    pub parameter_field_ids: Option<Vec<String>>,
    #[serde(default)]
    pub parameter_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_ids: Option<Vec<String>>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchDocumentDraftApiResponse {
    pub success: bool,
    pub draft_id: String,
    pub idempotency_key: String,
    pub message: String,
    #[serde(default)]
    pub form_state: Option<App__infra__document__types__DraftFormState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchEvalDraftApiRequest {
    #[serde(default)]
    pub draft_id: Option<String>,
    #[serde(default)]
    pub input_draft_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub model_ids: Option<Vec<String>>,
    #[serde(default)]
    pub model_flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub model_flags: Option<Vec<std::collections::HashMap<String, serde_json::Value>>>,
    #[serde(default)]
    pub model_flag_values: Option<Vec<EvalModelFlagValue>>,
    #[serde(default)]
    pub model_position_ids: Option<Vec<String>>,
    #[serde(default)]
    pub model_rubric_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_ids: Option<Vec<String>>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchEvalDraftApiResponse {
    pub success: bool,
    pub draft_id: String,
    pub message: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub form_state: Option<App__infra__eval__types__DraftFormState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchFieldDraftApiRequest {
    #[serde(default)]
    pub draft_id: Option<String>,
    #[serde(default)]
    pub input_draft_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub conditional_parameter_ids: Option<Vec<String>>,
    #[serde(default)]
    pub conditional_parameters: Option<Vec<String>>,
    #[serde(default)]
    pub pending_ids: Option<Vec<String>>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchFieldDraftApiResponse {
    pub success: bool,
    pub draft_id: String,
    pub idempotency_key: String,
    pub message: String,
    #[serde(default)]
    pub form_state: Option<App__infra__field__types__DraftFormState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchInvocationDraftApiRequest {
    #[serde(default)]
    pub draft_id: Option<String>,
    #[serde(default)]
    pub input_draft_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub value_id: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub key_id: Option<String>,
    #[serde(default)]
    pub endpoint_id: Option<String>,
    #[serde(default)]
    pub modality_ids: Option<Vec<String>>,
    #[serde(default)]
    pub temperature_level_id: Option<String>,
    #[serde(default)]
    pub pricing_id: Option<String>,
    #[serde(default)]
    pub reasoning_level_id: Option<String>,
    #[serde(default)]
    pub quality_ids: Option<Vec<String>>,
    #[serde(default)]
    pub voice_ids: Option<Vec<String>>,
    #[serde(default)]
    pub model_flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub model_flags: Option<Vec<std::collections::HashMap<String, serde_json::Value>>>,
    #[serde(default)]
    pub model_flag_values: Option<Vec<InvocationModelFlagValue>>,
    #[serde(default)]
    pub model_position_ids: Option<Vec<String>>,
    #[serde(default)]
    pub model_rubric_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_ids: Option<Vec<String>>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchInvocationDraftApiResponse {
    pub success: bool,
    pub draft_id: String,
    pub message: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub form_state: Option<App__infra__invocation__types__DraftFormState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchModelDraftApiRequest {
    #[serde(default)]
    pub draft_id: Option<String>,
    #[serde(default)]
    pub input_draft_id: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub value: Option<String>,
    #[serde(default)]
    pub value_id: Option<String>,
    #[serde(default)]
    pub provider: Option<String>,
    #[serde(default)]
    pub provider_id: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub modalities_enabled: Option<bool>,
    #[serde(default)]
    pub temperature_enabled: Option<bool>,
    #[serde(default)]
    pub pricing_enabled: Option<bool>,
    #[serde(default)]
    pub voices_enabled: Option<bool>,
    #[serde(default)]
    pub reasoning_levels_enabled: Option<bool>,
    #[serde(default)]
    pub qualities_enabled: Option<bool>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub modalities: Option<Vec<String>>,
    #[serde(default)]
    pub modality_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pricing: Option<Vec<PricingDraftValue>>,
    #[serde(default)]
    pub pricing_ids: Option<Vec<String>>,
    #[serde(default)]
    pub qualities: Option<Vec<String>>,
    #[serde(default)]
    pub quality_ids: Option<Vec<String>>,
    #[serde(default)]
    pub reasoning_levels: Option<Vec<String>>,
    #[serde(default)]
    pub reasoning_level_ids: Option<Vec<String>>,
    #[serde(default)]
    pub temperature_levels: Option<Vec<String>>,
    #[serde(default)]
    pub temperature_level_ids: Option<Vec<String>>,
    #[serde(default)]
    pub voices: Option<Vec<String>>,
    #[serde(default)]
    pub voice_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchModelDraftApiResponse {
    pub success: bool,
    pub draft_id: String,
    pub message: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub form_state: Option<App__infra__model__types__DraftFormState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchParameterDraftApiRequest {
    #[serde(default)]
    pub draft_id: Option<String>,
    #[serde(default)]
    pub input_draft_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub field_ids: Option<Vec<String>>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub parameter_fields: Option<Vec<String>>,
    #[serde(default)]
    pub pending_ids: Option<Vec<String>>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchParameterDraftApiResponse {
    pub success: bool,
    pub draft_id: String,
    pub idempotency_key: String,
    pub message: String,
    #[serde(default)]
    pub form_state: Option<App__infra__parameter__types__DraftFormState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchPersonaDraftApiRequest {
    #[serde(default)]
    pub draft_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub color_id: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub icon_id: Option<String>,
    #[serde(default)]
    pub instructions: Option<String>,
    #[serde(default)]
    pub instructions_id: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub examples: Option<Vec<String>>,
    #[serde(default)]
    pub example_ids: Option<Vec<String>>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub parameter_field_ids: Option<Vec<String>>,
    #[serde(default)]
    pub parameter_fields: Option<Vec<String>>,
    #[serde(default)]
    pub voice_ids: Option<Vec<String>>,
    #[serde(default)]
    pub voices: Option<Vec<String>>,
    #[serde(default)]
    pub pending_ids: Option<Vec<String>>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchPersonaDraftApiResponse {
    pub success: bool,
    pub draft_id: String,
    pub idempotency_key: String,
    pub message: String,
    pub form_state: App__infra__persona__types__DraftFormState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchProfileDraftApiRequest {
    #[serde(default)]
    pub draft_id: Option<String>,
    #[serde(default)]
    pub input_draft_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub emails: Option<Vec<String>>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub email_ids: Option<Vec<String>>,
    #[serde(default)]
    pub role: Option<String>,
    #[serde(default)]
    pub role_id: Option<String>,
    #[serde(default)]
    pub role_draft: Option<ProfileRoleDraftValue>,
    #[serde(default)]
    pub primary_department_id: Option<String>,
    #[serde(default)]
    pub pending_ids: Option<Vec<String>>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchProfileDraftApiResponse {
    pub success: bool,
    pub draft_id: String,
    pub message: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub form_state: Option<App__infra__profile__types__DraftFormState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchProviderDraftApiRequest {
    #[serde(default)]
    pub draft_id: Option<String>,
    #[serde(default)]
    pub input_draft_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub endpoint: Option<String>,
    #[serde(default)]
    pub endpoint_id: Option<String>,
    #[serde(default)]
    pub endpoint_ids: Option<Vec<String>>,
    #[serde(default)]
    pub key: Option<String>,
    #[serde(default)]
    pub key_name: Option<String>,
    #[serde(default)]
    pub key_description: Option<String>,
    #[serde(default)]
    pub key_id: Option<String>,
    #[serde(default)]
    pub key_ids: Option<Vec<String>>,
    #[serde(default)]
    pub value: Option<String>,
    #[serde(default)]
    pub value_id: Option<String>,
    #[serde(default)]
    pub pending_ids: Option<Vec<String>>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchProviderDraftApiResponse {
    pub success: bool,
    pub draft_id: String,
    pub message: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub form_state: Option<App__infra__provider__types__DraftFormState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchRubricDraftApiRequest {
    #[serde(default)]
    pub draft_id: Option<String>,
    #[serde(default)]
    pub input_draft_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub simulation_rubric: Option<bool>,
    #[serde(default)]
    pub video_rubric: Option<bool>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pass_points_id: Option<String>,
    #[serde(default)]
    pub pass_points: Option<i64>,
    #[serde(default)]
    pub standard_group_ids: Option<Vec<String>>,
    #[serde(default)]
    pub standard_groups: Option<Vec<RubricStandardGroupDraftValue>>,
    #[serde(default)]
    pub standard_ids: Option<Vec<String>>,
    #[serde(default)]
    pub standards: Option<Vec<RubricStandardDraftValue>>,
    #[serde(default)]
    pub pending_ids: Option<Vec<String>>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchRubricDraftApiResponse {
    pub success: bool,
    pub draft_id: String,
    pub message: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub form_state: Option<App__infra__rubric__types__DraftFormState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchScenarioDraftApiRequest {
    #[serde(default)]
    pub input_draft_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub problem_statement: Option<String>,
    #[serde(default)]
    pub problem_statement_id: Option<String>,
    #[serde(default)]
    pub objectives: Option<Vec<String>>,
    #[serde(default)]
    pub objective_ids: Option<Vec<String>>,
    #[serde(default)]
    pub images: Option<Vec<App__infra__scenario__types__DraftImageValue>>,
    #[serde(default)]
    pub image_ids: Option<Vec<String>>,
    #[serde(default)]
    pub videos: Option<Vec<App__infra__scenario__types__DraftVideoValue>>,
    #[serde(default)]
    pub video_ids: Option<Vec<String>>,
    #[serde(default)]
    pub questions: Option<Vec<App__infra__scenario__types__DraftQuestionValue>>,
    #[serde(default)]
    pub question_ids: Option<Vec<String>>,
    #[serde(default)]
    pub options: Option<Vec<App__infra__scenario__types__DraftOptionValue>>,
    #[serde(default)]
    pub option_ids: Option<Vec<String>>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub video_enabled: Option<bool>,
    #[serde(default)]
    pub problem_statement_enabled: Option<bool>,
    #[serde(default)]
    pub objectives_enabled: Option<bool>,
    #[serde(default)]
    pub images_enabled: Option<bool>,
    #[serde(default)]
    pub questions_enabled: Option<bool>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub persona_ids: Option<Vec<String>>,
    #[serde(default)]
    pub document_ids: Option<Vec<String>>,
    #[serde(default)]
    pub parameter_field_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_ids: Option<Vec<String>>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchScenarioDraftApiResponse {
    pub success: bool,
    pub draft_id: String,
    pub idempotency_key: String,
    pub message: String,
    pub form_state: ScenarioDraftFormState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchSettingDraftApiRequest {
    #[serde(default)]
    pub draft_id: Option<String>,
    #[serde(default)]
    pub input_draft_id: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub mcp: Option<bool>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub color_ids: Option<Vec<String>>,
    #[serde(default)]
    pub logins_ids: Option<Vec<String>>,
    #[serde(default)]
    pub system_ids: Option<Vec<String>>,
    #[serde(default)]
    pub mcp_id: Option<String>,
    #[serde(default)]
    pub threshold_ids: Option<Vec<String>>,
    #[serde(default)]
    pub provider_key_ids: Option<Vec<String>>,
    #[serde(default)]
    pub auth_item_key_ids: Option<Vec<String>>,
    #[serde(default)]
    pub auth_item_value_ids: Option<Vec<String>>,
    #[serde(default)]
    pub auth_ids: Option<Vec<String>>,
    #[serde(default)]
    pub provider_ids: Option<Vec<String>>,
    #[serde(default)]
    pub provider_keys: Option<Vec<SettingProviderKeyDraftValue>>,
    #[serde(default)]
    pub auth_item_keys: Option<Vec<SettingAuthItemKeyDraftValue>>,
    #[serde(default)]
    pub auth_item_values: Option<Vec<SettingAuthItemValueDraftValue>>,
    #[serde(default)]
    pub mcp_values: Option<Vec<SettingMcpDraftValue>>,
    #[serde(default)]
    pub system_values: Option<Vec<SettingSystemDraftValue>>,
    #[serde(default)]
    pub threshold_values: Option<Vec<SettingThresholdDraftValue>>,
    #[serde(default)]
    pub logins: Option<Vec<SettingLoginDraftValue>>,
    #[serde(default)]
    pub pending_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchSettingDraftApiResponse {
    pub success: bool,
    pub draft_id: String,
    pub message: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub form_state: Option<App__infra__setting__types__DraftFormState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchSimulationDraftApiRequest {
    #[serde(default)]
    pub draft_id: Option<String>,
    #[serde(default)]
    pub input_draft_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub practice: Option<bool>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub scenario_ids: Option<Vec<String>>,
    #[serde(default)]
    pub scenario_flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub scenario_flags: Option<Vec<DraftScenarioFlagValue>>,
    #[serde(default)]
    pub scenario_flag_values: Option<Vec<DraftScenarioFlagDenormValue>>,
    #[serde(default)]
    pub scenario_position_ids: Option<Vec<String>>,
    #[serde(default)]
    pub scenario_positions: Option<Vec<DraftScenarioPositionValue>>,
    #[serde(default)]
    pub scenario_rubric_ids: Option<Vec<String>>,
    #[serde(default)]
    pub scenario_rubrics: Option<Vec<DraftScenarioRubricValue>>,
    #[serde(default)]
    pub scenario_time_limit_ids: Option<Vec<String>>,
    #[serde(default)]
    pub scenario_time_limits: Option<Vec<DraftScenarioTimeLimitValue>>,
    #[serde(default)]
    pub pending_ids: Option<Vec<String>>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchSimulationDraftApiResponse {
    pub success: bool,
    pub draft_id: String,
    pub idempotency_key: String,
    pub message: String,
    pub form_state: App__infra__simulation__types__DraftFormState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchToolDraftApiRequest {
    #[serde(default)]
    pub draft_id: Option<String>,
    #[serde(default)]
    pub input_draft_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub arg_ids: Option<Vec<String>>,
    #[serde(default)]
    pub args: Option<Vec<CreateArgInput>>,
    #[serde(default)]
    pub arg_position_ids: Option<Vec<String>>,
    #[serde(default)]
    pub arg_positions: Option<Vec<CreateArgPositionInput>>,
    #[serde(default)]
    pub args_output_ids: Option<Vec<String>>,
    #[serde(default)]
    pub args_outputs_ids: Option<Vec<String>>,
    #[serde(default)]
    pub args_outputs: Option<Vec<CreateArgsOutputInput>>,
    #[serde(default)]
    pub args_drafts: Option<Vec<ToolArgDraftValue>>,
    #[serde(default)]
    pub instruction_id: Option<String>,
    #[serde(default)]
    pub instruction_ids: Option<Vec<String>>,
    #[serde(default)]
    pub permission_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_ids: Option<Vec<String>>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchToolDraftApiResponse {
    pub success: bool,
    pub draft_id: String,
    pub message: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub form_state: Option<App__infra__tool__types__DraftFormState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaChartRow {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub score: Option<f64>,
    #[serde(default)]
    pub sessions: Option<i64>,
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub trend_data: Option<Vec<PersonaTrendPoint>>,
    #[serde(default)]
    pub simulation_ids: Option<Vec<String>>,
    #[serde(default)]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaColorJunction {
    #[serde(default)]
    pub persona_name: Option<String>,
    #[serde(default)]
    pub color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaColorResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub hex_code: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaDepartmentResource {
    #[serde(default)]
    pub department_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaDescriptionResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaEntry {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub entry_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub instructions: Option<String>,
    #[serde(default)]
    pub examples: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaExampleResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub example: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaFieldError {
    pub field: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaFlagResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub value: Option<bool>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub icon_id: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaIconResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub value: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaInstructionResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub template: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaNameResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaParameterFieldResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub field_id: Option<String>,
    #[serde(default)]
    pub parameter_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub conditional_parameter_id: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaResultItem {
    pub success: bool,
    pub message: String,
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub errors: Option<Vec<PersonaFieldError>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaTrendPoint {
    #[serde(default)]
    pub date: Option<String>,
    #[serde(default)]
    pub score: Option<f64>,
    #[serde(default)]
    pub timestamp: Option<i64>,
    #[serde(default)]
    pub simulation_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaVoiceResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub voice: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewToolApiRequest {
    #[serde(default)]
    pub args: Option<Vec<ToolPreviewArg>>,
    #[serde(default)]
    pub outputs: Option<Vec<ToolPreviewOutput>>,
    #[serde(default)]
    pub mock: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewToolApiResponse {
    #[serde(default)]
    pub outputs: Option<Vec<ToolPreviewOutputResult>>,
    #[serde(default)]
    pub type_hints: Option<Vec<ToolPreviewArgHint>>,
    #[serde(default)]
    pub undeclared: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviousChatOption {
    #[serde(default)]
    pub chat_entry_id: Option<String>,
    #[serde(default)]
    pub scenario_name: Option<String>,
    #[serde(default)]
    pub attempt_chat_id: Option<String>,
    #[serde(default)]
    pub score: Option<f64>,
    #[serde(default)]
    pub percentage: Option<f64>,
    #[serde(default)]
    pub time_taken: Option<f64>,
    #[serde(default)]
    pub position: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingDailyItem {
    pub date_key: String,
    #[serde(default)]
    pub model_id: Option<String>,
    #[serde(default)]
    pub total_cost: Option<String>,
    #[serde(default)]
    pub run_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingDraftValue {
    pub pricing_type: String,
    pub price: f64,
    pub unit_name: String,
    pub unit_category: String,
    pub unit_value: i64,
    #[serde(default)]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingGroupItem {
    pub group_id: String,
    #[serde(default)]
    pub session_id: Option<String>,
    #[serde(default)]
    pub group_name: Option<String>,
    #[serde(default)]
    pub first_run_at: Option<String>,
    #[serde(default)]
    pub last_run_at: Option<String>,
    #[serde(default)]
    pub run_count: Option<i64>,
    #[serde(default)]
    pub total_input_tokens: Option<i64>,
    #[serde(default)]
    pub total_output_tokens: Option<i64>,
    #[serde(default)]
    pub total_tokens: Option<i64>,
    #[serde(default)]
    pub total_cost: Option<String>,
    #[serde(default)]
    pub agent_ids: Option<Vec<String>>,
    #[serde(default)]
    pub model_ids: Option<Vec<String>>,
    #[serde(default)]
    pub profile_ids: Option<Vec<String>>,
    #[serde(default)]
    pub agent_names: Option<Vec<String>>,
    #[serde(default)]
    pub model_names: Option<Vec<String>>,
    #[serde(default)]
    pub profile_names: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingHistoryResponse {
    #[serde(default)]
    pub items: Option<Vec<PricingGroupItem>>,
    #[serde(default)]
    pub total_count: Option<i64>,
    #[serde(default)]
    pub page: Option<i64>,
    #[serde(default)]
    pub page_size: Option<i64>,
    #[serde(default)]
    pub total_pages: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingRequest {
    #[serde(default)]
    pub start_date: Option<String>,
    #[serde(default)]
    pub end_date: Option<String>,
    #[serde(default)]
    pub date_from: Option<String>,
    #[serde(default)]
    pub date_to: Option<String>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub page_limit: Option<i64>,
    #[serde(default)]
    pub page_offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingResources {
    #[serde(default)]
    pub agents: Option<std::collections::HashMap<String, std::collections::HashMap<String, serde_json::Value>>>,
    #[serde(default)]
    pub models: Option<std::collections::HashMap<String, std::collections::HashMap<String, serde_json::Value>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingResponse {
    #[serde(default)]
    pub daily: Option<Vec<PricingDailyItem>>,
    #[serde(default)]
    pub resources: Option<PricingResources>,
    #[serde(default)]
    pub total_count: Option<i64>,
    #[serde(default)]
    pub model_options: Option<Vec<FilterOption>>,
    #[serde(default)]
    pub agent_options: Option<Vec<FilterOption>>,
    #[serde(default)]
    pub analytics: Option<AnalyticsFacets>,
    #[serde(default)]
    pub history: Option<PricingHistoryResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimaryPersonaPerformance {
    #[serde(default)]
    pub chart_data: Option<Vec<PersonaChartRow>>,
    #[serde(default)]
    pub valid_simulation_ids: Option<Vec<String>>,
    #[serde(default)]
    pub persona_colors_junction: Option<Vec<PersonaColorJunction>>,
    #[serde(default)]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimaryRubricHeatmap {
    #[serde(default)]
    pub matrices: Option<Vec<RubricHeatmapMatrix>>,
    #[serde(default)]
    pub valid_rubric_ids: Option<Vec<String>>,
    #[serde(default)]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimaryRubricTrend {
    #[serde(default)]
    pub trend_data: Option<Vec<PrimaryRubricTrendPoint>>,
    #[serde(default)]
    pub valid_rubric_ids: Option<Vec<String>>,
    #[serde(default)]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimaryRubricTrendPoint {
    #[serde(default)]
    pub date: Option<String>,
    #[serde(default)]
    pub standard_group_id: Option<String>,
    #[serde(default)]
    pub standard_group_name: Option<String>,
    #[serde(default)]
    pub avg_pct: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemAgentApiRequest {
    #[serde(rename = "type")]
    pub r#type: String,
    pub message: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemAgentApiResponse {
    pub problem_id: String,
    #[serde(default)]
    pub success: Option<bool>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemAttemptApiRequest {
    #[serde(rename = "type")]
    pub r#type: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemAttemptApiResponse {
    pub problem_id: String,
    #[serde(default)]
    pub success: Option<bool>,
    #[serde(default)]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemAuthApiRequest {
    #[serde(rename = "type")]
    pub r#type: String,
    pub message: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemAuthApiResponse {
    pub problem_id: String,
    #[serde(default)]
    pub success: Option<bool>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemCohortApiRequest {
    #[serde(rename = "type")]
    pub r#type: String,
    pub message: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemCohortApiResponse {
    pub problem_id: String,
    #[serde(default)]
    pub success: Option<bool>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemDepartmentApiRequest {
    #[serde(rename = "type")]
    pub r#type: String,
    pub message: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemDepartmentApiResponse {
    pub problem_id: String,
    #[serde(default)]
    pub success: Option<bool>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemDocumentApiRequest {
    #[serde(rename = "type")]
    pub r#type: String,
    pub message: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemDocumentApiResponse {
    pub problem_id: String,
    #[serde(default)]
    pub success: Option<bool>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemEvalApiRequest {
    #[serde(rename = "type")]
    pub r#type: String,
    pub message: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemEvalApiResponse {
    pub problem_id: String,
    #[serde(default)]
    pub success: Option<bool>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemFieldApiRequest {
    #[serde(rename = "type")]
    pub r#type: String,
    pub message: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemFieldApiResponse {
    pub problem_id: String,
    #[serde(default)]
    pub success: Option<bool>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemModelApiRequest {
    #[serde(rename = "type")]
    pub r#type: String,
    pub message: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemModelApiResponse {
    pub problem_id: String,
    #[serde(default)]
    pub success: Option<bool>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemParameterApiRequest {
    #[serde(rename = "type")]
    pub r#type: String,
    pub message: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemParameterApiResponse {
    pub problem_id: String,
    #[serde(default)]
    pub success: Option<bool>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemPersonaApiRequest {
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemPersonaApiResponse {
    pub problem_id: String,
    #[serde(default)]
    pub success: Option<bool>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemProfileApiRequest {
    #[serde(rename = "type")]
    pub r#type: String,
    pub message: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemProfileApiResponse {
    pub problem_id: String,
    #[serde(default)]
    pub success: Option<bool>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemProviderApiRequest {
    #[serde(rename = "type")]
    pub r#type: String,
    pub message: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemProviderApiResponse {
    pub problem_id: String,
    #[serde(default)]
    pub success: Option<bool>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemRubricApiRequest {
    #[serde(rename = "type")]
    pub r#type: String,
    pub message: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemRubricApiResponse {
    pub problem_id: String,
    #[serde(default)]
    pub success: Option<bool>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemScenarioApiRequest {
    #[serde(rename = "type")]
    pub r#type: String,
    pub message: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemScenarioApiResponse {
    pub problem_id: String,
    #[serde(default)]
    pub success: Option<bool>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemSettingApiRequest {
    #[serde(rename = "type")]
    pub r#type: String,
    pub message: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemSettingApiResponse {
    pub problem_id: String,
    #[serde(default)]
    pub success: Option<bool>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemSimulationApiRequest {
    #[serde(rename = "type")]
    pub r#type: String,
    pub message: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemSimulationApiResponse {
    pub problem_id: String,
    #[serde(default)]
    pub success: Option<bool>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemStatementEntry {
    #[serde(default)]
    pub problem_statement_id: Option<String>,
    #[serde(default)]
    pub problem_statement: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemSystemApiRequest {
    #[serde(rename = "type")]
    pub r#type: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemSystemApiResponse {
    pub problem_id: String,
    #[serde(default)]
    pub success: Option<bool>,
    #[serde(default)]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemTestApiRequest {
    #[serde(rename = "type")]
    pub r#type: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemTestApiResponse {
    pub problem_id: String,
    #[serde(default)]
    pub success: Option<bool>,
    #[serde(default)]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemToolApiRequest {
    #[serde(rename = "type")]
    pub r#type: String,
    pub message: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemToolApiResponse {
    pub problem_id: String,
    #[serde(default)]
    pub success: Option<bool>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProducedMedia {
    pub modality: String,
    pub resource_id: String,
    pub upload_id: String,
    #[serde(default)]
    pub mime_type: Option<String>,
    #[serde(default)]
    pub file_size: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileDepartmentResource {
    #[serde(default)]
    pub department_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileEmailResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub is_primary: Option<bool>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileFieldError {
    pub field: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileFlagResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub value: Option<bool>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub icon_id: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileNameResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfilePermissionResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub artifact: Option<String>,
    #[serde(default)]
    pub operation: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileRequestLimitDraftValue {
    pub limit: i64,
    pub interval: String,
    #[serde(default)]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileRequestLimitResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub interval: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileResultItem {
    pub success: bool,
    pub message: String,
    #[serde(default)]
    pub profile_id: Option<String>,
    #[serde(default)]
    pub errors: Option<Vec<ProfileFieldError>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileRoleDraftValue {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub icon_id: Option<String>,
    #[serde(default)]
    pub color_id: Option<String>,
    #[serde(default)]
    pub level: Option<i64>,
    #[serde(default)]
    pub permission_ids: Option<Vec<String>>,
    #[serde(default)]
    pub request_limit_ids: Option<Vec<String>>,
    #[serde(default)]
    pub request_limits: Option<Vec<ProfileRequestLimitDraftValue>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileRoleResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub role: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub icon_id: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub icon_value: Option<String>,
    #[serde(default)]
    pub color_id: Option<String>,
    #[serde(default)]
    pub color_hex: Option<String>,
    #[serde(default)]
    pub level: Option<i64>,
    #[serde(default)]
    pub permission_ids: Option<Vec<String>>,
    #[serde(default)]
    pub request_limit_ids: Option<Vec<String>>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileSummary {
    pub name: String,
    pub role: String,
    pub role_level: i64,
    pub department_ids: Vec<String>,
    pub artifact_access: Vec<String>,
    pub role_permissions: Vec<Vec<serde_json::Value>>,
    pub is_active: bool,
    pub id: String,
    #[serde(default)]
    pub theme: Option<ThemeBundle>,
    #[serde(default)]
    pub session_id: Option<String>,
    #[serde(default)]
    pub is_emulation: Option<bool>,
    #[serde(default)]
    pub role_resources: Option<Vec<QGetProfileContextV4RoleResource>>,
    #[serde(default)]
    pub scoped_roles: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileSummaryItem {
    #[serde(default)]
    pub profile_id: Option<String>,
    #[serde(default)]
    pub profile_name: Option<String>,
    #[serde(default)]
    pub sessions_count: Option<i64>,
    #[serde(default)]
    pub logins_count: Option<i64>,
    #[serde(default)]
    pub grants_count: Option<i64>,
    #[serde(default)]
    pub problems_count: Option<i64>,
    #[serde(default)]
    pub activity_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderDepartmentResource {
    #[serde(default)]
    pub department_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderDescriptionResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderEndpointResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub base_url: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderFieldError {
    pub field: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderFlagResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub value: Option<bool>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub icon_id: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderKeyResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub key: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderNameResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderResultItem {
    pub success: bool,
    pub message: String,
    #[serde(default)]
    pub provider_id: Option<String>,
    #[serde(default)]
    pub errors: Option<Vec<ProviderFieldError>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderValueResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub value: Option<String>,
    #[serde(default)]
    pub value_type: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QGetProfileContextV4RoleResource {
    #[serde(default)]
    pub role: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub icon_value: Option<String>,
    #[serde(default)]
    pub color_hex: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionEntry {
    #[serde(default)]
    pub question_id: Option<String>,
    #[serde(default)]
    pub question_text: Option<String>,
    #[serde(default)]
    pub allow_multiple: Option<bool>,
    #[serde(default)]
    pub times: Option<Vec<i64>>,
    #[serde(default)]
    pub options: Option<Vec<OptionEntry>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizResponse {
    #[serde(default)]
    pub question_id: Option<String>,
    #[serde(default)]
    pub option_id: Option<String>,
    #[serde(default)]
    pub completed: Option<bool>,
    #[serde(default)]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshAgentApiRequest {
    #[serde(default)]
    pub targets: Option<Vec<String>>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshAuthApiRequest {
    #[serde(default)]
    pub targets: Option<Vec<String>>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshCohortApiRequest {
    #[serde(default)]
    pub targets: Option<Vec<String>>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshDepartmentApiRequest {
    #[serde(default)]
    pub targets: Option<Vec<String>>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshDocumentApiRequest {
    #[serde(default)]
    pub targets: Option<Vec<String>>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshEvalApiRequest {
    #[serde(default)]
    pub targets: Option<Vec<String>>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshFieldApiRequest {
    #[serde(default)]
    pub targets: Option<Vec<String>>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshModelApiRequest {
    #[serde(default)]
    pub targets: Option<Vec<String>>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshParameterApiRequest {
    #[serde(default)]
    pub targets: Option<Vec<String>>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshProfileApiRequest {
    #[serde(default)]
    pub targets: Option<Vec<String>>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshProviderApiRequest {
    #[serde(default)]
    pub targets: Option<Vec<String>>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshResponse {
    pub success: bool,
    pub refreshed_views: Vec<String>,
    pub invalidated_tags: Vec<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshRubricApiRequest {
    #[serde(default)]
    pub targets: Option<Vec<String>>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshScenarioApiRequest {
    #[serde(default)]
    pub targets: Option<Vec<String>>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshSettingApiRequest {
    #[serde(default)]
    pub targets: Option<Vec<String>>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshSimulationApiRequest {
    #[serde(default)]
    pub targets: Option<Vec<String>>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshToolApiRequest {
    #[serde(default)]
    pub targets: Option<Vec<String>>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplacementEntry {
    #[serde(default)]
    pub section: Option<String>,
    #[serde(default)]
    pub replace: Option<String>,
    #[serde(default)]
    pub idx: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportsCohortResource {
    #[serde(default)]
    pub cohort_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportsDataPoint {
    #[serde(default)]
    pub profile_id: Option<String>,
    #[serde(default)]
    pub date: Option<String>,
    #[serde(default)]
    pub value: Option<serde_json::Value>,
    #[serde(default)]
    pub simulation_id: Option<String>,
    #[serde(default)]
    pub scenario_id: Option<String>,
    #[serde(default)]
    pub attempt_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportsHeaderMetrics {
    #[serde(default)]
    pub total_attempts: Option<ReportsMetric>,
    #[serde(default)]
    pub average_score: Option<ReportsMetric>,
    #[serde(default)]
    pub completion_percentage: Option<ReportsMetric>,
    #[serde(default)]
    pub first_attempt_pass_rate: Option<ReportsMetric>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportsHistoryRow {
    #[serde(default)]
    pub attempt_id: Option<String>,
    #[serde(default)]
    pub profile_id: Option<String>,
    #[serde(default)]
    pub simulation_id: Option<String>,
    #[serde(default)]
    pub cohort_id: Option<String>,
    #[serde(default)]
    pub attempt_created_at: Option<String>,
    #[serde(default)]
    pub attempt_type: Option<String>,
    #[serde(default)]
    pub is_archived: Option<bool>,
    #[serde(default)]
    pub infinite_mode: Option<bool>,
    #[serde(default)]
    pub score_percent: Option<f64>,
    #[serde(default)]
    pub has_passed: Option<bool>,
    #[serde(default)]
    pub num_chats: Option<i64>,
    #[serde(default)]
    pub num_chats_completed: Option<i64>,
    #[serde(default)]
    pub total_time_seconds: Option<i64>,
    #[serde(default)]
    pub scenario_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportsHistorySection {
    #[serde(default)]
    pub status: Option<ReportsSectionStatus>,
    #[serde(default)]
    pub rows: Option<Vec<ReportsHistoryRow>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportsLeaderboardRow {
    pub rank: i64,
    #[serde(default)]
    pub profile_id: Option<String>,
    #[serde(default)]
    pub total_attempts: Option<i64>,
    #[serde(default)]
    pub average_score: Option<f64>,
    #[serde(default)]
    pub highest_score: Option<f64>,
    #[serde(default)]
    pub completion_percentage: Option<f64>,
    #[serde(default)]
    pub first_attempt_pass_rate: Option<f64>,
    #[serde(default)]
    pub profile_metrics: Option<ReportsProfileMetrics>,
    #[serde(default)]
    pub simulation_ids: Option<Vec<String>>,
    #[serde(default)]
    pub scenario_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportsLeaderboardSection {
    #[serde(default)]
    pub status: Option<ReportsSectionStatus>,
    #[serde(default)]
    pub rows: Option<Vec<ReportsLeaderboardRow>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportsMetric {
    #[serde(default)]
    pub current_value: Option<serde_json::Value>,
    #[serde(default)]
    pub has_data: Option<bool>,
    #[serde(default)]
    pub method: Option<String>,
    #[serde(default)]
    pub data_points: Option<Vec<ReportsDataPoint>>,
    #[serde(default)]
    pub hover: Option<ReportsMetricHover>,
    #[serde(default)]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportsMetricHover {
    #[serde(default)]
    pub mean: Option<i64>,
    #[serde(default)]
    pub median: Option<i64>,
    #[serde(default)]
    pub mode: Option<i64>,
    #[serde(default)]
    pub count: Option<i64>,
    #[serde(default)]
    pub completed: Option<i64>,
    #[serde(default)]
    pub total: Option<i64>,
    #[serde(default)]
    pub percent: Option<i64>,
    #[serde(default)]
    pub top: Option<Vec<i64>>,
    #[serde(default)]
    pub mean_seconds: Option<i64>,
    #[serde(default)]
    pub median_seconds: Option<i64>,
    #[serde(default)]
    pub samples: Option<i64>,
    #[serde(default)]
    pub avg_score_percent: Option<i64>,
    #[serde(default)]
    pub avg_minutes: Option<i64>,
    #[serde(default)]
    pub efficiency: Option<i64>,
    #[serde(default)]
    pub tracked: Option<i64>,
    #[serde(default)]
    pub stagnant: Option<i64>,
    #[serde(default)]
    pub rate_percent: Option<i64>,
    #[serde(default)]
    pub total_minutes: Option<i64>,
    #[serde(default)]
    pub total_hours: Option<f64>,
    #[serde(default)]
    pub attempts: Option<i64>,
    #[serde(default)]
    pub unique_simulations: Option<i64>,
    #[serde(default)]
    pub per_simulation_mean: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportsOverviewRow {
    #[serde(default)]
    pub simulation_id: Option<String>,
    #[serde(default)]
    pub attempts: Option<i64>,
    #[serde(default)]
    pub completed_attempts: Option<i64>,
    #[serde(default)]
    pub passed_attempts: Option<i64>,
    #[serde(default)]
    pub average_score: Option<f64>,
    #[serde(default)]
    pub completion_percentage: Option<f64>,
    #[serde(default)]
    pub pass_rate: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportsOverviewSection {
    #[serde(default)]
    pub status: Option<ReportsSectionStatus>,
    #[serde(default)]
    pub rows: Option<Vec<ReportsOverviewRow>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportsPersonaResource {
    #[serde(default)]
    pub persona_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportsProfileMetrics {
    #[serde(default)]
    pub average_score: Option<ReportsMetric>,
    #[serde(default)]
    pub completion_percentage: Option<ReportsMetric>,
    #[serde(default)]
    pub first_attempt_pass_rate: Option<ReportsMetric>,
    #[serde(default)]
    pub highest_score: Option<ReportsMetric>,
    #[serde(default)]
    pub messages_per_session: Option<ReportsMetric>,
    #[serde(default)]
    pub persona_response_times: Option<ReportsMetric>,
    #[serde(default)]
    pub session_efficiency: Option<ReportsMetric>,
    #[serde(default)]
    pub stagnation_rate: Option<ReportsMetric>,
    #[serde(default)]
    pub time_spent: Option<ReportsMetric>,
    #[serde(default)]
    pub total_attempts: Option<ReportsMetric>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportsProfileResource {
    #[serde(default)]
    pub profile_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub role_id: Option<String>,
    #[serde(default)]
    pub role: Option<String>,
    #[serde(default)]
    pub emails: Option<Vec<String>>,
    #[serde(default)]
    pub primary_email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportsRequest {
    #[serde(default)]
    pub start_date: Option<String>,
    #[serde(default)]
    pub end_date: Option<String>,
    #[serde(default)]
    pub cohort_ids: Option<Vec<String>>,
    #[serde(default)]
    pub simulation_ids: Option<Vec<String>>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub role_ids: Option<Vec<String>>,
    #[serde(default)]
    pub simulation_filters: Option<Vec<String>>,
    #[serde(default)]
    pub actor_profile_id: Option<String>,
    #[serde(default)]
    pub target_profile_id: Option<String>,
    #[serde(default)]
    pub profile_ids: Option<Vec<String>>,
    #[serde(default)]
    pub scenario_ids: Option<Vec<String>>,
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub sort_by: Option<String>,
    #[serde(default)]
    pub sort_order: Option<String>,
    #[serde(default)]
    pub page_limit: Option<i64>,
    #[serde(default)]
    pub page_offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportsResources {
    #[serde(default)]
    pub simulations: Option<std::collections::HashMap<String, ReportsSimulationResource>>,
    #[serde(default)]
    pub profiles: Option<std::collections::HashMap<String, ReportsProfileResource>>,
    #[serde(default)]
    pub roles: Option<std::collections::HashMap<String, ReportsRoleResource>>,
    #[serde(default)]
    pub scenarios: Option<std::collections::HashMap<String, ReportsScenarioResource>>,
    #[serde(default)]
    pub cohorts: Option<std::collections::HashMap<String, ReportsCohortResource>>,
    #[serde(default)]
    pub personas: Option<std::collections::HashMap<String, ReportsPersonaResource>>,
    #[serde(default)]
    pub rubrics: Option<std::collections::HashMap<String, ReportsRubricResource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportsResponse {
    #[serde(default)]
    pub sections: Option<ReportsSections>,
    #[serde(default)]
    pub views: Option<ReportsViews>,
    #[serde(default)]
    pub resources: Option<ReportsResources>,
    #[serde(default)]
    pub total_count: Option<i64>,
    #[serde(default)]
    pub simulation_options: Option<Vec<FilterOption>>,
    #[serde(default)]
    pub profile_options: Option<Vec<FilterOption>>,
    #[serde(default)]
    pub scenario_options: Option<Vec<FilterOption>>,
    #[serde(default)]
    pub analytics: Option<AnalyticsFacets>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportsRoleResource {
    #[serde(default)]
    pub role_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub icon_id: Option<String>,
    #[serde(default)]
    pub color_id: Option<String>,
    #[serde(default)]
    pub level: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportsRubricResource {
    #[serde(default)]
    pub rubric_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportsScenarioResource {
    #[serde(default)]
    pub scenario_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportsSectionStatus {
    #[serde(default)]
    pub has_data: Option<bool>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportsSections {
    #[serde(default)]
    pub header_metrics: Option<ReportsHeaderMetrics>,
    #[serde(default)]
    pub overview: Option<ReportsOverviewSection>,
    #[serde(default)]
    pub leaderboard: Option<ReportsLeaderboardSection>,
    #[serde(default)]
    pub trends: Option<ReportsTrendsSection>,
    #[serde(default)]
    pub history: Option<ReportsHistorySection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportsSimulationResource {
    #[serde(default)]
    pub simulation_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportsTrendPoint {
    #[serde(default)]
    pub date: Option<String>,
    #[serde(default)]
    pub attempts: Option<i64>,
    #[serde(default)]
    pub completed_attempts: Option<i64>,
    #[serde(default)]
    pub passed_attempts: Option<i64>,
    #[serde(default)]
    pub average_score: Option<f64>,
    #[serde(default)]
    pub completion_percentage: Option<f64>,
    #[serde(default)]
    pub pass_rate: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportsTrendsSection {
    #[serde(default)]
    pub status: Option<ReportsSectionStatus>,
    #[serde(default)]
    pub chart_data: Option<Vec<ReportsTrendPoint>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportsViews {
    #[serde(default)]
    pub attempt_facts: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub chat_facts: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub daily_metrics: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub profile_metrics: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolveProblemApiRequest {
    pub problem_id: String,
    #[serde(default)]
    pub resolved: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolveProblemApiResponse {
    pub problem_id: String,
    pub resolved: bool,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseAttemptApiResponse {
    pub success: bool,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub is_correct: Option<bool>,
    #[serde(default)]
    pub response_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RubricDepartmentResource {
    #[serde(default)]
    pub department_id: Option<String>,
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub setting_ids: Option<Vec<String>>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RubricDescriptionResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RubricEntry {
    #[serde(default)]
    pub rubric_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub total_points: Option<f64>,
    #[serde(default)]
    pub pass_points: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RubricFieldError {
    pub field: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RubricFlagResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub value: Option<bool>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub icon_id: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RubricHeatmapCell {
    #[serde(default)]
    pub rubric_id: Option<String>,
    #[serde(default)]
    pub correlation: Option<f64>,
    #[serde(default)]
    pub p_value: Option<f64>,
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub strength: Option<String>,
    #[serde(default)]
    pub data_points: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RubricHeatmapMatrix {
    #[serde(default)]
    pub rubric_id: Option<String>,
    #[serde(default)]
    pub standard_groups: Option<Vec<RubricHeatmapStandardGroup>>,
    #[serde(default)]
    pub matrix: Option<Vec<RubricHeatmapMatrixRow>>,
    #[serde(default)]
    pub insights: Option<String>,
    #[serde(default)]
    pub has_data: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RubricHeatmapMatrixRow {
    #[serde(default)]
    pub cells: Option<Vec<RubricHeatmapCell>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RubricHeatmapStandardGroup {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub short_name: Option<String>,
    #[serde(default)]
    pub rubric_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RubricMapping {
    pub rubric_id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub standard_group_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RubricNameResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RubricPointResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub value: Option<i64>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RubricResultItem {
    pub success: bool,
    pub message: String,
    #[serde(default)]
    pub rubric_id: Option<String>,
    #[serde(default)]
    pub errors: Option<Vec<RubricFieldError>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RubricStandardDraftValue {
    pub name: String,
    pub points: i64,
    pub standard_group_id: String,
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RubricStandardGroupDraftValue {
    pub name: String,
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub points: Option<i64>,
    #[serde(default)]
    pub pass_points: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RubricStandardGroupResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub standard_group_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub short_name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub points: Option<i64>,
    #[serde(default)]
    pub pass_points: Option<i64>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RubricStandardResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub standard_id: Option<String>,
    #[serde(default)]
    pub standard_group_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub points: Option<i64>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RubricStructureData {
    #[serde(default)]
    pub standard_groups: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(default)]
    pub standard_groups_mapping: Option<std::collections::HashMap<String, StandardGroupMeta>>,
    #[serde(default)]
    pub standards_mapping: Option<std::collections::HashMap<String, StandardMeta>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunPricingItem {
    #[serde(default)]
    pub pricing_type: Option<String>,
    #[serde(default)]
    pub count: Option<i64>,
    #[serde(default)]
    pub pricing_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunViewItem {
    pub run_id: String,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub profiles_id: Option<String>,
    #[serde(default)]
    pub input_tokens: Option<i64>,
    #[serde(default)]
    pub output_tokens: Option<i64>,
    #[serde(default)]
    pub cached_input_tokens: Option<i64>,
    #[serde(default)]
    pub run_created_at: Option<String>,
    #[serde(default)]
    pub agent_ids: Option<Vec<String>>,
    #[serde(default)]
    pub model_ids: Option<Vec<String>>,
    #[serde(default)]
    pub provider_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pricing: Option<Vec<RunPricingItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioDepartment {
    #[serde(default)]
    pub department_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioDescriptionResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioDocument {
    #[serde(default)]
    pub document_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub file_id: Option<String>,
    #[serde(default)]
    pub file_path: Option<String>,
    #[serde(default)]
    pub mime_type: Option<String>,
    #[serde(default)]
    pub upload_id: Option<String>,
    #[serde(default)]
    pub html: Option<bool>,
    #[serde(default)]
    pub parameter_ids: Option<Vec<String>>,
    #[serde(default)]
    pub field_ids: Option<Vec<String>>,
    #[serde(default)]
    pub parent_document_id: Option<String>,
    #[serde(default)]
    pub video_document: Option<bool>,
    #[serde(default)]
    pub non_video_document: Option<bool>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioDraftFormState {
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub problem_statement_id: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub video_enabled: Option<bool>,
    #[serde(default)]
    pub problem_statement_enabled: Option<bool>,
    #[serde(default)]
    pub objectives_enabled: Option<bool>,
    #[serde(default)]
    pub images_enabled: Option<bool>,
    #[serde(default)]
    pub questions_enabled: Option<bool>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub persona_ids: Option<Vec<String>>,
    #[serde(default)]
    pub document_ids: Option<Vec<String>>,
    #[serde(default)]
    pub parameter_field_ids: Option<Vec<String>>,
    #[serde(default)]
    pub objective_ids: Option<Vec<String>>,
    #[serde(default)]
    pub image_ids: Option<Vec<String>>,
    #[serde(default)]
    pub video_ids: Option<Vec<String>>,
    #[serde(default)]
    pub question_ids: Option<Vec<String>>,
    #[serde(default)]
    pub option_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioEntry {
    #[serde(default)]
    pub scenario_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioField {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub field_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub parameter_id: Option<String>,
    #[serde(default)]
    pub parameter_name: Option<String>,
    #[serde(default)]
    pub conditional_parameter_ids: Option<Vec<String>>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioFieldError {
    pub field: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioFlagResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub value: Option<bool>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub icon_id: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioImage {
    #[serde(default)]
    pub image_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub file_path: Option<String>,
    #[serde(default)]
    pub mime_type: Option<String>,
    #[serde(default)]
    pub upload_id: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioNameResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioObjective {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub objective: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioOption {
    #[serde(default)]
    pub option_id: Option<String>,
    #[serde(default)]
    pub option_text: Option<String>,
    #[serde(default)]
    pub is_correct: Option<bool>,
    #[serde(default)]
    pub question_id: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioPersona {
    #[serde(default)]
    pub persona_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub image_model: Option<bool>,
    #[serde(default)]
    pub parameter_ids: Option<Vec<String>>,
    #[serde(default)]
    pub field_ids: Option<Vec<String>>,
    #[serde(default)]
    pub example: Option<String>,
    #[serde(default)]
    pub video_persona: Option<bool>,
    #[serde(default)]
    pub non_video_persona: Option<bool>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioProblemStatement {
    #[serde(default)]
    pub problem_statement_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub problem_statement: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioQuestion {
    #[serde(default)]
    pub question_id: Option<String>,
    #[serde(default)]
    pub question_text: Option<String>,
    #[serde(default)]
    pub allow_multiple: Option<bool>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioResultItem {
    pub success: bool,
    pub message: String,
    #[serde(default)]
    pub scenario_id: Option<String>,
    #[serde(default)]
    pub errors: Option<Vec<ScenarioFieldError>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioVideo {
    #[serde(default)]
    pub video_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub file_path: Option<String>,
    #[serde(default)]
    pub mime_type: Option<String>,
    #[serde(default)]
    pub upload_id: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchAgentApiRequest {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub filter_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_model_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_tool_ids: Option<Vec<String>>,
    #[serde(default)]
    pub department_search: Option<String>,
    #[serde(default)]
    pub model_search: Option<String>,
    #[serde(default)]
    pub tool_search: Option<String>,
    #[serde(default)]
    pub page_size: Option<i64>,
    #[serde(default)]
    pub page_offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchAttemptApiRequest {
    #[serde(default)]
    pub target_profile_id: Option<String>,
    #[serde(default)]
    pub profile_ids: Option<Vec<String>>,
    #[serde(default)]
    pub cohort_ids: Option<Vec<String>>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub role_ids: Option<Vec<String>>,
    #[serde(default)]
    pub simulation_ids: Option<Vec<String>>,
    #[serde(default)]
    pub scenario_ids: Option<Vec<String>>,
    #[serde(default)]
    pub practice: Option<bool>,
    #[serde(default)]
    pub infinite_mode: Option<bool>,
    #[serde(default)]
    pub show_archived: Option<bool>,
    #[serde(default)]
    pub start_date: Option<String>,
    #[serde(default)]
    pub end_date: Option<String>,
    #[serde(default)]
    pub simulation_search: Option<String>,
    #[serde(default)]
    pub scenario_search: Option<String>,
    #[serde(default)]
    pub profile_search: Option<String>,
    #[serde(default)]
    pub sort_by: Option<String>,
    #[serde(default)]
    pub sort_order: Option<String>,
    #[serde(default)]
    pub page: Option<i64>,
    #[serde(default)]
    pub page_size: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchAuthApiRequest {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub filter_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub department_search: Option<String>,
    #[serde(default)]
    pub page_size: Option<i64>,
    #[serde(default)]
    pub page_offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchCohortApiRequest {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub filter_profile_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_simulation_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub profile_search: Option<String>,
    #[serde(default)]
    pub simulation_search: Option<String>,
    #[serde(default)]
    pub department_search: Option<String>,
    #[serde(default)]
    pub flag_search: Option<String>,
    #[serde(default)]
    pub page_size: Option<i64>,
    #[serde(default)]
    pub page_offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchDepartmentApiRequest {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub page_size: Option<i64>,
    #[serde(default)]
    pub page_offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchDocumentApiRequest {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub scenario_ids: Option<Vec<String>>,
    #[serde(default)]
    pub field_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub scenario_search: Option<String>,
    #[serde(default)]
    pub field_search: Option<String>,
    #[serde(default)]
    pub department_search: Option<String>,
    #[serde(default)]
    pub page_size: Option<i64>,
    #[serde(default)]
    pub page_offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchEvalApiRequest {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub filter_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub department_search: Option<String>,
    #[serde(default)]
    pub page_size: Option<i64>,
    #[serde(default)]
    pub page_offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchFieldApiRequest {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub parameter_ids: Option<Vec<String>>,
    #[serde(default)]
    pub persona_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub parameter_search: Option<String>,
    #[serde(default)]
    pub persona_search: Option<String>,
    #[serde(default)]
    pub department_search: Option<String>,
    #[serde(default)]
    pub page_size: Option<i64>,
    #[serde(default)]
    pub page_offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchMessageResponse {
    pub message_id: String,
    pub run_id: String,
    pub role: String,
    pub message_created_at: String,
    pub text_ids: Vec<String>,
    pub audio_ids: Vec<String>,
    pub image_ids: Vec<String>,
    pub video_ids: Vec<String>,
    pub file_ids: Vec<String>,
    pub call_ids: Vec<String>,
    #[serde(default)]
    pub reasoning: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchModelApiRequest {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub filter_provider_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_agent_ids: Option<Vec<String>>,
    #[serde(default)]
    pub provider_search: Option<String>,
    #[serde(default)]
    pub department_search: Option<String>,
    #[serde(default)]
    pub agent_search: Option<String>,
    #[serde(default)]
    pub page_size: Option<i64>,
    #[serde(default)]
    pub page_offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchParameterApiRequest {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub scenario_ids: Option<Vec<String>>,
    #[serde(default)]
    pub field_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub scenario_search: Option<String>,
    #[serde(default)]
    pub field_search: Option<String>,
    #[serde(default)]
    pub department_search: Option<String>,
    #[serde(default)]
    pub page_size: Option<i64>,
    #[serde(default)]
    pub page_offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchPersonaApiRequest {
    #[serde(default)]
    pub snapshot_key: Option<String>,
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub scenario_ids: Option<Vec<String>>,
    #[serde(default)]
    pub field_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub scenario_search: Option<String>,
    #[serde(default)]
    pub field_search: Option<String>,
    #[serde(default)]
    pub department_search: Option<String>,
    #[serde(default)]
    pub color_search: Option<String>,
    #[serde(default)]
    pub icon_search: Option<String>,
    #[serde(default)]
    pub voice_search: Option<String>,
    #[serde(default)]
    pub instruction_search: Option<String>,
    #[serde(default)]
    pub page_size: Option<i64>,
    #[serde(default)]
    pub page_offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchProfileApiRequest {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub cohort_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub role_filter: Option<String>,
    #[serde(default)]
    pub cohort_search: Option<String>,
    #[serde(default)]
    pub department_search: Option<String>,
    #[serde(default)]
    pub role_search: Option<String>,
    #[serde(default)]
    pub page_size: Option<i64>,
    #[serde(default)]
    pub page_offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchProviderApiRequest {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub filter_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_model_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_status: Option<Vec<String>>,
    #[serde(default)]
    pub department_search: Option<String>,
    #[serde(default)]
    pub model_search: Option<String>,
    #[serde(default)]
    pub page_size: Option<i64>,
    #[serde(default)]
    pub page_offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchRubricApiRequest {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub filter_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_simulation_ids: Option<Vec<String>>,
    #[serde(default)]
    pub department_search: Option<String>,
    #[serde(default)]
    pub simulation_search: Option<String>,
    #[serde(default)]
    pub page_size: Option<i64>,
    #[serde(default)]
    pub page_offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchScenarioApiRequest {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub persona_ids: Option<Vec<String>>,
    #[serde(default)]
    pub simulation_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub persona_search: Option<String>,
    #[serde(default)]
    pub simulation_search: Option<String>,
    #[serde(default)]
    pub department_search: Option<String>,
    #[serde(default)]
    pub flag_search: Option<String>,
    #[serde(default)]
    pub page_size: Option<i64>,
    #[serde(default)]
    pub page_offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchSimulationApiRequest {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub filter_scenario_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_cohort_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub scenario_search: Option<String>,
    #[serde(default)]
    pub cohort_search: Option<String>,
    #[serde(default)]
    pub department_search: Option<String>,
    #[serde(default)]
    pub flag_search: Option<String>,
    #[serde(default)]
    pub page_size: Option<i64>,
    #[serde(default)]
    pub page_offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchTestApiRequest {
    #[serde(default)]
    pub eval_ids: Option<Vec<String>>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub is_archived: Option<bool>,
    #[serde(default)]
    pub start_date: Option<String>,
    #[serde(default)]
    pub end_date: Option<String>,
    #[serde(default)]
    pub eval_search: Option<String>,
    #[serde(default)]
    pub department_search: Option<String>,
    #[serde(default)]
    pub page_size: Option<i64>,
    #[serde(default)]
    pub page_offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchTestApiResponse {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub tests: Option<Vec<SearchTestItem>>,
    #[serde(default)]
    pub eval_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub department_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub total_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchTestItem {
    pub test_id: String,
    #[serde(default)]
    pub eval_id: Option<String>,
    #[serde(default)]
    pub eval_name: Option<String>,
    #[serde(default)]
    pub eval_description: Option<String>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub test_name: Option<String>,
    #[serde(default)]
    pub test_description: Option<String>,
    #[serde(default)]
    pub num_invocations: Option<i64>,
    #[serde(default)]
    pub infinite_mode: Option<bool>,
    #[serde(default)]
    pub is_dynamic: Option<bool>,
    #[serde(default)]
    pub archived: Option<bool>,
    #[serde(default)]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchToolApiRequest {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub filter_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_creatable: Option<Vec<String>>,
    #[serde(default)]
    pub department_search: Option<String>,
    #[serde(default)]
    pub page_size: Option<i64>,
    #[serde(default)]
    pub page_offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecondaryAttemptImprovement {
    #[serde(default)]
    pub chart_data: Option<Vec<SecondaryAttemptImprovementChart>>,
    #[serde(default)]
    pub facts: Option<Vec<SecondaryAttemptImprovementFact>>,
    #[serde(default)]
    pub valid_simulation_ids: Option<Vec<String>>,
    #[serde(default)]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecondaryAttemptImprovementChart {
    #[serde(default)]
    pub attempt: Option<String>,
    #[serde(default)]
    pub average_score: Option<f64>,
    #[serde(default)]
    pub average_time: Option<f64>,
    #[serde(default)]
    pub pass_rate: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecondaryAttemptImprovementFact {
    #[serde(default)]
    pub simulation_id: Option<String>,
    #[serde(default)]
    pub attempt_no: Option<i64>,
    #[serde(default)]
    pub avg_grade: Option<f64>,
    #[serde(default)]
    pub avg_minutes: Option<f64>,
    #[serde(default)]
    pub pass_rate: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecondaryCohortDaily {
    #[serde(default)]
    pub date: Option<String>,
    #[serde(default)]
    pub avg_score: Option<f64>,
    #[serde(default)]
    pub cohort_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecondaryCohortData {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub pass_rate: Option<f64>,
    #[serde(default)]
    pub avg_percentage_score: Option<f64>,
    #[serde(default)]
    pub total_students: Option<i64>,
    #[serde(default)]
    pub passed_students: Option<i64>,
    #[serde(default)]
    pub total_attempts: Option<i64>,
    #[serde(default)]
    pub passed_attempts: Option<i64>,
    #[serde(default)]
    pub simulation_count: Option<i64>,
    #[serde(default)]
    pub required_simulations: Option<i64>,
    #[serde(default)]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecondaryCohortPerformance {
    #[serde(default)]
    pub cohort_data: Option<Vec<SecondaryCohortData>>,
    #[serde(default)]
    pub daily_data: Option<Vec<SecondaryCohortDaily>>,
    #[serde(default)]
    pub simulation_facts: Option<Vec<SecondarySimulationFact>>,
    #[serde(default)]
    pub daily_facts: Option<Vec<SecondaryDailyFact>>,
    #[serde(default)]
    pub valid_simulation_ids: Option<Vec<String>>,
    #[serde(default)]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecondaryDailyFact {
    #[serde(default)]
    pub date: Option<String>,
    #[serde(default)]
    pub simulation_id: Option<String>,
    #[serde(default)]
    pub avg_score: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecondaryGroupFact {
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub group_name: Option<String>,
    #[serde(default)]
    pub group_description: Option<String>,
    #[serde(default)]
    pub simulation_id: Option<String>,
    #[serde(default)]
    pub score: Option<f64>,
    #[serde(default)]
    pub points: Option<f64>,
    #[serde(default)]
    pub avg_pct: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecondaryRadarPoint {
    #[serde(default)]
    pub metric: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub value: Option<f64>,
    #[serde(default)]
    pub full_mark: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecondarySimulationFact {
    #[serde(default)]
    pub cohort_id: Option<String>,
    #[serde(default)]
    pub simulation_id: Option<String>,
    #[serde(default)]
    pub pass_rate: Option<f64>,
    #[serde(default)]
    pub avg_score: Option<f64>,
    #[serde(default)]
    pub attempts: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecondarySkillPackage {
    #[serde(default)]
    pub rubric_id: Option<String>,
    #[serde(default)]
    pub radar_data: Option<Vec<SecondaryRadarPoint>>,
    #[serde(default)]
    pub group_facts: Option<Vec<SecondaryGroupFact>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecondarySkillPerformance {
    #[serde(default)]
    pub packages: Option<Vec<SecondarySkillPackage>>,
    #[serde(default)]
    pub valid_rubric_ids: Option<Vec<String>>,
    #[serde(default)]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionListItem {
    pub session_id: String,
    #[serde(default)]
    pub profile_id: Option<String>,
    #[serde(default)]
    pub profile_name: Option<String>,
    #[serde(default)]
    pub session_created_at: Option<String>,
    #[serde(default)]
    pub session_updated_at: Option<String>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub group_count: Option<i64>,
    #[serde(default)]
    pub run_count: Option<i64>,
    #[serde(default)]
    pub first_run_at: Option<String>,
    #[serde(default)]
    pub last_run_at: Option<String>,
    #[serde(default)]
    pub total_tokens: Option<i64>,
    #[serde(default)]
    pub total_cost: Option<String>,
    #[serde(default)]
    pub chat_count: Option<i64>,
    #[serde(default)]
    pub attempt_count: Option<i64>,
    #[serde(default)]
    pub message_count: Option<i64>,
    #[serde(default)]
    pub problem_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionTimelineItem {
    #[serde(default)]
    pub event_type: Option<String>,
    #[serde(default)]
    pub entity_id: Option<String>,
    #[serde(default)]
    pub entity_name: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub extra_1: Option<String>,
    #[serde(default)]
    pub extra_2: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingAgentCatalogResource {
    #[serde(default)]
    pub agent_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingAuthCatalogResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub auth_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub slug: Option<String>,
    #[serde(default)]
    pub protocol: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingAuthItemKeyDraftValue {
    pub auth_id: String,
    pub item_id: String,
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub key_id: Option<String>,
    #[serde(default)]
    pub key_value: Option<String>,
    #[serde(default)]
    pub key_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingAuthItemKeyOption {
    #[serde(default)]
    pub auth_id: Option<String>,
    #[serde(default)]
    pub item_id: Option<String>,
    #[serde(default)]
    pub key_id: Option<String>,
    #[serde(default)]
    pub auth_name: Option<String>,
    #[serde(default)]
    pub item_name: Option<String>,
    #[serde(default)]
    pub key_name: Option<String>,
    #[serde(default)]
    pub masked_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingAuthItemKeyResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub auth_id: Option<String>,
    #[serde(default)]
    pub item_id: Option<String>,
    #[serde(default)]
    pub key_id: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingAuthItemValueDraftValue {
    pub auth_id: String,
    pub item_id: String,
    pub value: String,
    #[serde(default)]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingAuthItemValueOption {
    #[serde(default)]
    pub auth_id: Option<String>,
    #[serde(default)]
    pub item_id: Option<String>,
    #[serde(default)]
    pub auth_name: Option<String>,
    #[serde(default)]
    pub item_name: Option<String>,
    #[serde(default)]
    pub item_description: Option<String>,
    #[serde(default)]
    pub encrypted: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingAuthItemValueResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub auth_id: Option<String>,
    #[serde(default)]
    pub item_id: Option<String>,
    #[serde(default)]
    pub value: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingColorResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub hex_code: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingDepartmentResource {
    #[serde(default)]
    pub department_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingDescriptionResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingFieldError {
    pub field: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingFlagResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub value: Option<bool>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub icon_id: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingIconCatalogResource {
    #[serde(default)]
    pub icon_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingItemCatalogResource {
    #[serde(default)]
    pub item_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub encrypted: Option<bool>,
    #[serde(default)]
    pub position: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingKeyCatalogResource {
    #[serde(default)]
    pub key_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub masked_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingLoginDraftValue {
    pub login_type: String,
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub auth_id: Option<String>,
    #[serde(default)]
    pub profile_id: Option<String>,
    #[serde(default)]
    pub display_name: Option<String>,
    #[serde(default)]
    pub icon_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingLoginOption {
    #[serde(default)]
    pub login_type: Option<String>,
    #[serde(default)]
    pub auth_id: Option<String>,
    #[serde(default)]
    pub profile_id: Option<String>,
    #[serde(default)]
    pub display_name: Option<String>,
    #[serde(default)]
    pub icon_id: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingLoginsResource {
    #[serde(default)]
    pub logins_id: Option<String>,
    #[serde(default)]
    pub profile_id: Option<String>,
    #[serde(default)]
    pub auth_id: Option<String>,
    #[serde(default)]
    pub icon_id: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub display_name: Option<String>,
    #[serde(default)]
    pub login_type: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingMcpDraftValue {
    pub agent_id: String,
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingMcpOption {
    #[serde(default)]
    pub agent_id: Option<String>,
    #[serde(default)]
    pub agent_name: Option<String>,
    #[serde(default)]
    pub agent_description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingMcpResource {
    #[serde(default)]
    pub mcp_id: Option<String>,
    #[serde(default)]
    pub agent_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingNameResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingProfileCatalogResource {
    #[serde(default)]
    pub profile_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingProviderCatalogResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub provider_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingProviderKeyDraftValue {
    pub provider_id: String,
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub key_id: Option<String>,
    #[serde(default)]
    pub key_value: Option<String>,
    #[serde(default)]
    pub key_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingProviderKeyOption {
    #[serde(default)]
    pub provider_id: Option<String>,
    #[serde(default)]
    pub key_id: Option<String>,
    #[serde(default)]
    pub provider_name: Option<String>,
    #[serde(default)]
    pub key_name: Option<String>,
    #[serde(default)]
    pub masked_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingProviderKeyResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub provider_id: Option<String>,
    #[serde(default)]
    pub key_id: Option<String>,
    #[serde(default)]
    pub key: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingResultItem {
    pub success: bool,
    pub message: String,
    #[serde(default)]
    pub setting_id: Option<String>,
    #[serde(default)]
    pub errors: Option<Vec<SettingFieldError>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingSystemDraftValue {
    pub name: String,
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub agent_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingSystemResource {
    #[serde(default)]
    pub system_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub agent_ids: Option<Vec<String>>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingThresholdDraftValue {
    #[serde(rename = "type")]
    pub r#type: String,
    pub value: i64,
    #[serde(default)]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingThresholdResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub value: Option<i64>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationData {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub time_limit: Option<i64>,
    #[serde(default)]
    pub hints_enabled: Option<bool>,
    #[serde(default)]
    pub objectives_enabled: Option<bool>,
    #[serde(default)]
    pub image_input_active: Option<bool>,
    #[serde(default)]
    pub copy_paste_allowed: Option<bool>,
    #[serde(default)]
    pub practice_simulation: Option<bool>,
    #[serde(default)]
    pub rubric_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationDepartment {
    #[serde(default)]
    pub department_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationDescriptionResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationFieldError {
    pub field: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationFlagResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub value: Option<bool>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub icon_id: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationNameResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationResultItem {
    pub success: bool,
    pub message: String,
    #[serde(default)]
    pub simulation_id: Option<String>,
    #[serde(default)]
    pub errors: Option<Vec<SimulationFieldError>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationRubric {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub standard_group_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationScenario {
    #[serde(default)]
    pub scenario_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub show_problem_statement: Option<bool>,
    #[serde(default)]
    pub show_objectives: Option<bool>,
    #[serde(default)]
    pub show_video: Option<bool>,
    #[serde(default)]
    pub show_text: Option<bool>,
    #[serde(default)]
    pub show_audio: Option<bool>,
    #[serde(default)]
    pub show_copy_paste: Option<bool>,
    #[serde(default)]
    pub show_images: Option<bool>,
    #[serde(default)]
    pub show_questions: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationScenarioFlag {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub scenario_id: Option<String>,
    #[serde(default)]
    pub flag_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub value: Option<bool>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationScenarioFlagOption {
    #[serde(default)]
    pub scenario_id: Option<String>,
    #[serde(default)]
    pub flag_id: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub value: Option<bool>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationScenarioPosition {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub scenario_id: Option<String>,
    #[serde(default)]
    pub value: Option<i64>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationScenarioRubric {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub scenario_id: Option<String>,
    #[serde(default)]
    pub rubric_id: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationScenarioTimeLimit {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub scenario_id: Option<String>,
    #[serde(default)]
    pub time_limit_seconds: Option<i64>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub negative: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillFeedback {
    #[serde(default)]
    pub skill_name: Option<String>,
    #[serde(default)]
    pub feedback: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillScore {
    #[serde(default)]
    pub skill_name: Option<String>,
    #[serde(default)]
    pub score: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandardEntry {
    #[serde(default)]
    pub standard_id: Option<String>,
    #[serde(default)]
    pub standard_group_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub points: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandardGroupEntry {
    #[serde(default)]
    pub standard_group_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub points: Option<f64>,
    #[serde(default)]
    pub pass_points: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandardGroupMapping {
    pub standard_group_id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub points: Option<i64>,
    #[serde(default)]
    pub pass_points: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandardGroupMeta {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub points: Option<f64>,
    #[serde(default)]
    pub pass_points: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandardMapping {
    pub standard_id: String,
    #[serde(default)]
    pub standard_group_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub points: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandardMeta {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub points: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartTestApiResponse {
    pub test_id: String,
    #[serde(default)]
    pub invocation_id: Option<String>,
    #[serde(default)]
    pub benchmark_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarterPrompt {
    pub title: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StopTestApiResponse {
    pub invocation_id: String,
    pub success: bool,
    #[serde(default)]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableInfo {
    pub name: String,
    pub columns: Vec<ColumnInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCompleteInternalResult {
    pub test_id: String,
    #[serde(default)]
    pub success: Option<bool>,
    #[serde(default)]
    pub completed_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCompletePayload {
    pub test_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestConfigGroup {
    pub group_id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub run_count: Option<i64>,
    #[serde(default)]
    pub last_run_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestConfigItem {
    pub run_id: String,
    pub label: String,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub agent_name: Option<String>,
    #[serde(default)]
    pub model_name: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub prompt_ids: Option<Vec<String>>,
    #[serde(default)]
    pub tool_ids: Option<Vec<String>>,
    #[serde(default)]
    pub instruction_ids: Option<Vec<String>>,
    #[serde(default)]
    pub model_id: Option<String>,
    #[serde(default)]
    pub temperature: Option<f64>,
    #[serde(default)]
    pub reasoning: Option<String>,
    #[serde(default)]
    pub quality: Option<String>,
    #[serde(default)]
    pub permissions: Option<Vec<Vec<serde_json::Value>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestEntries {
    #[serde(default)]
    pub tests: Option<Vec<GetTestResponse>>,
    #[serde(default)]
    pub invocations: Option<Vec<GetTestInvocationResponse>>,
    #[serde(default)]
    pub runs: Option<Vec<GetTestInvocationRunsResponse>>,
    #[serde(default)]
    pub groups: Option<Vec<GetTestInvocationTracesResponse>>,
    #[serde(default)]
    pub grades: Option<Vec<GetTestGradeResponse>>,
    #[serde(default)]
    pub feedback: Option<Vec<GetTestFeedbackResponse>>,
    #[serde(default)]
    pub messages: Option<Vec<SearchMessageResponse>>,
    #[serde(default)]
    pub calls: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestInvocationCompleteInternalResult {
    pub invocation_id: String,
    #[serde(default)]
    pub completion_id: Option<String>,
    #[serde(default)]
    pub success: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestInvocationCompletePayload {
    pub test_id: String,
    pub test_invocation_id: String,
    #[serde(default)]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResources {
    #[serde(default)]
    pub evals: Option<std::collections::HashMap<String, std::collections::HashMap<String, serde_json::Value>>>,
    #[serde(default)]
    pub rubrics: Option<std::collections::HashMap<String, std::collections::HashMap<String, serde_json::Value>>>,
    #[serde(default)]
    pub agents: Option<std::collections::HashMap<String, std::collections::HashMap<String, serde_json::Value>>>,
    #[serde(default)]
    pub models: Option<std::collections::HashMap<String, std::collections::HashMap<String, serde_json::Value>>>,
    #[serde(default)]
    pub voices: Option<std::collections::HashMap<String, std::collections::HashMap<String, serde_json::Value>>>,
    #[serde(default)]
    pub temperature_levels: Option<std::collections::HashMap<String, std::collections::HashMap<String, serde_json::Value>>>,
    #[serde(default)]
    pub reasoning_levels: Option<std::collections::HashMap<String, std::collections::HashMap<String, serde_json::Value>>>,
    #[serde(default)]
    pub modalities: Option<std::collections::HashMap<String, std::collections::HashMap<String, serde_json::Value>>>,
    #[serde(default)]
    pub prompts: Option<std::collections::HashMap<String, std::collections::HashMap<String, serde_json::Value>>>,
    #[serde(default)]
    pub instructions: Option<std::collections::HashMap<String, std::collections::HashMap<String, serde_json::Value>>>,
    #[serde(default)]
    pub tools: Option<std::collections::HashMap<String, std::collections::HashMap<String, serde_json::Value>>>,
    #[serde(default)]
    pub qualities: Option<std::collections::HashMap<String, std::collections::HashMap<String, serde_json::Value>>>,
    #[serde(default)]
    pub standard_groups: Option<std::collections::HashMap<String, std::collections::HashMap<String, serde_json::Value>>>,
    #[serde(default)]
    pub standards: Option<std::collections::HashMap<String, std::collections::HashMap<String, serde_json::Value>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestRunEndPayload {
    pub test_invocation_run_id: String,
    #[serde(default)]
    pub success: Option<bool>,
    #[serde(default)]
    pub error: Option<bool>,
    #[serde(default)]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestRunEndResponse {
    pub test_invocation_run_id: String,
    pub completion_id: String,
    #[serde(default)]
    pub success: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestRunInternalResult {
    pub test_invocation_run_id: String,
    #[serde(default)]
    pub success: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestRunItem {
    pub chat_id: String,
    pub invocation_id: String,
    #[serde(default)]
    pub run_id: Option<String>,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub suite_entry_id: Option<String>,
    #[serde(default)]
    pub model_name: Option<String>,
    #[serde(default)]
    pub agent_name: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub grade_score: Option<f64>,
    #[serde(default)]
    pub grade_passed: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestRunPayload {
    pub test_id: String,
    pub test_invocation_id: String,
    pub run_id: String,
    #[serde(default)]
    pub test_invocation_trace_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestStartPayload {
    pub eval_id: String,
    #[serde(default)]
    pub infinite_mode: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestStatusSummary {
    #[serde(default)]
    pub total: Option<i64>,
    #[serde(default)]
    pub completed: Option<i64>,
    #[serde(default)]
    pub in_progress: Option<i64>,
    #[serde(default)]
    pub not_started: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestStopPayload {
    pub invocation_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestTraceInternalResult {
    pub test_invocation_trace_id: String,
    #[serde(default)]
    pub success: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestTracePayload {
    pub test_id: String,
    pub test_invocation_id: String,
    #[serde(default)]
    pub run_id: Option<String>,
    #[serde(default)]
    pub tool_ids: Option<Vec<String>>,
    #[serde(default)]
    pub modality_ids: Option<Vec<String>>,
    #[serde(default)]
    pub voice_ids: Option<Vec<String>>,
    #[serde(default)]
    pub temperature_level_ids: Option<Vec<String>>,
    #[serde(default)]
    pub reasoning_level_ids: Option<Vec<String>>,
    #[serde(default)]
    pub quality_ids: Option<Vec<String>>,
    #[serde(default)]
    pub prompt_ids: Option<Vec<String>>,
    #[serde(default)]
    pub instruction_ids: Option<Vec<String>>,
    #[serde(default)]
    pub prompt_text: Option<String>,
    #[serde(default)]
    pub instructions: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextDownloadAgentApiRequest {
    pub text_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextDownloadAttemptApiRequest {
    pub text_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextDownloadAuthApiRequest {
    pub text_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextDownloadCohortApiRequest {
    pub text_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextDownloadDepartmentApiRequest {
    pub text_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextDownloadDocumentApiRequest {
    pub text_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextDownloadEvalApiRequest {
    pub text_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextDownloadFieldApiRequest {
    pub text_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextDownloadGroupApiRequest {
    pub text_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextDownloadModelApiRequest {
    pub text_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextDownloadParameterApiRequest {
    pub text_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextDownloadPersonaApiRequest {
    pub text_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextDownloadProfileApiRequest {
    pub text_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextDownloadProviderApiRequest {
    pub text_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextDownloadRubricApiRequest {
    pub text_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextDownloadScenarioApiRequest {
    pub text_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextDownloadSettingApiRequest {
    pub text_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextDownloadSimulationApiRequest {
    pub text_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextDownloadTestApiRequest {
    pub text_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextDownloadToolApiRequest {
    pub text_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextUploadDocumentApiResponse {
    pub text_id: String,
    pub upload_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeBundle {
    #[serde(default)]
    pub primitives: Option<ThemePrimitives>,
    #[serde(default)]
    pub tokens: Option<ThemeTokens>,
    #[serde(default)]
    pub dark_primitives: Option<ThemePrimitives>,
    #[serde(default)]
    pub dark_tokens: Option<ThemeTokens>,
    #[serde(default)]
    pub thresholds: Option<Thresholds>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemePrimitives {
    #[serde(default)]
    pub background: Option<String>,
    #[serde(default)]
    pub primary: Option<String>,
    #[serde(default)]
    pub accent: Option<String>,
    #[serde(default)]
    pub card: Option<String>,
    #[serde(default)]
    pub sidebar: Option<String>,
    #[serde(default)]
    pub muted_foreground: Option<String>,
    #[serde(default)]
    pub ring: Option<String>,
    #[serde(default)]
    pub border: Option<String>,
    #[serde(default)]
    pub destructive: Option<String>,
    #[serde(default)]
    pub success: Option<String>,
    #[serde(default)]
    pub warning: Option<String>,
    #[serde(default)]
    pub info: Option<String>,
    #[serde(default)]
    pub chart1: Option<String>,
    #[serde(default)]
    pub chart2: Option<String>,
    #[serde(default)]
    pub chart3: Option<String>,
    #[serde(default)]
    pub chart4: Option<String>,
    #[serde(default)]
    pub chart5: Option<String>,
    #[serde(default)]
    pub foreground: Option<String>,
    #[serde(default)]
    pub card_foreground: Option<String>,
    #[serde(default)]
    pub popover: Option<String>,
    #[serde(default)]
    pub popover_foreground: Option<String>,
    #[serde(default)]
    pub primary_foreground: Option<String>,
    #[serde(default)]
    pub secondary: Option<String>,
    #[serde(default)]
    pub secondary_foreground: Option<String>,
    #[serde(default)]
    pub muted: Option<String>,
    #[serde(default)]
    pub accent_foreground: Option<String>,
    #[serde(default)]
    pub destructive_foreground: Option<String>,
    #[serde(default)]
    pub danger: Option<String>,
    #[serde(default)]
    pub danger_foreground: Option<String>,
    #[serde(default)]
    pub input: Option<String>,
    #[serde(default)]
    pub success_foreground: Option<String>,
    #[serde(default)]
    pub warning_foreground: Option<String>,
    #[serde(default)]
    pub info_foreground: Option<String>,
    #[serde(default)]
    pub sidebar_foreground: Option<String>,
    #[serde(default)]
    pub sidebar_primary: Option<String>,
    #[serde(default)]
    pub sidebar_primary_foreground: Option<String>,
    #[serde(default)]
    pub sidebar_accent: Option<String>,
    #[serde(default)]
    pub sidebar_accent_foreground: Option<String>,
    #[serde(default)]
    pub sidebar_border: Option<String>,
    #[serde(default)]
    pub sidebar_ring: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeTokens {
    #[serde(default)]
    pub background: Option<String>,
    #[serde(default)]
    pub foreground: Option<String>,
    #[serde(default)]
    pub card: Option<String>,
    #[serde(default)]
    pub card_foreground: Option<String>,
    #[serde(default)]
    pub popover: Option<String>,
    #[serde(default)]
    pub popover_foreground: Option<String>,
    #[serde(default)]
    pub primary: Option<String>,
    #[serde(default)]
    pub primary_foreground: Option<String>,
    #[serde(default)]
    pub secondary: Option<String>,
    #[serde(default)]
    pub secondary_foreground: Option<String>,
    #[serde(default)]
    pub muted: Option<String>,
    #[serde(default)]
    pub muted_foreground: Option<String>,
    #[serde(default)]
    pub accent: Option<String>,
    #[serde(default)]
    pub accent_foreground: Option<String>,
    #[serde(default)]
    pub destructive: Option<String>,
    #[serde(default)]
    pub destructive_foreground: Option<String>,
    #[serde(default)]
    pub danger: Option<String>,
    #[serde(default)]
    pub danger_foreground: Option<String>,
    #[serde(default)]
    pub border: Option<String>,
    #[serde(default)]
    pub input: Option<String>,
    #[serde(default)]
    pub ring: Option<String>,
    #[serde(default)]
    pub success: Option<String>,
    #[serde(default)]
    pub success_foreground: Option<String>,
    #[serde(default)]
    pub warning: Option<String>,
    #[serde(default)]
    pub warning_foreground: Option<String>,
    #[serde(default)]
    pub info: Option<String>,
    #[serde(default)]
    pub info_foreground: Option<String>,
    #[serde(default)]
    pub chart1: Option<String>,
    #[serde(default)]
    pub chart2: Option<String>,
    #[serde(default)]
    pub chart3: Option<String>,
    #[serde(default)]
    pub chart4: Option<String>,
    #[serde(default)]
    pub chart5: Option<String>,
    #[serde(default)]
    pub sidebar: Option<String>,
    #[serde(default)]
    pub sidebar_foreground: Option<String>,
    #[serde(default)]
    pub sidebar_primary: Option<String>,
    #[serde(default)]
    pub sidebar_primary_foreground: Option<String>,
    #[serde(default)]
    pub sidebar_accent: Option<String>,
    #[serde(default)]
    pub sidebar_accent_foreground: Option<String>,
    #[serde(default)]
    pub sidebar_border: Option<String>,
    #[serde(default)]
    pub sidebar_ring: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Thresholds {
    pub success: i64,
    pub warning: i64,
    pub danger: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimerData {
    #[serde(default)]
    pub elapsed: Option<i64>,
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub exceeded: Option<bool>,
    #[serde(default)]
    pub formatted: Option<String>,
    #[serde(default)]
    pub negative: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleAgentApiRequest {
    pub group_id: String,
    pub title: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleAgentApiResponse {
    pub group_id: String,
    pub group_name_id: String,
    pub title: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleAttemptApiRequest {
    pub group_id: String,
    pub title: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleAttemptApiResponse {
    pub group_id: String,
    pub group_name_id: String,
    pub title: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleAuthApiRequest {
    pub group_id: String,
    pub title: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleAuthApiResponse {
    pub group_id: String,
    pub group_name_id: String,
    pub title: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleCohortApiRequest {
    pub group_id: String,
    pub title: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleCohortApiResponse {
    pub group_id: String,
    pub group_name_id: String,
    pub title: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleDepartmentApiRequest {
    pub group_id: String,
    pub title: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleDepartmentApiResponse {
    pub group_id: String,
    pub group_name_id: String,
    pub title: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleDocumentApiRequest {
    pub group_id: String,
    pub title: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleDocumentApiResponse {
    pub group_id: String,
    pub group_name_id: String,
    pub title: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleEvalApiRequest {
    pub group_id: String,
    pub title: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleEvalApiResponse {
    pub group_id: String,
    pub group_name_id: String,
    pub title: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleFieldApiRequest {
    pub group_id: String,
    pub title: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleFieldApiResponse {
    pub group_id: String,
    pub group_name_id: String,
    pub title: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleModelApiRequest {
    pub group_id: String,
    pub title: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleModelApiResponse {
    pub group_id: String,
    pub group_name_id: String,
    pub title: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleParameterApiRequest {
    pub group_id: String,
    pub title: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleParameterApiResponse {
    pub group_id: String,
    pub group_name_id: String,
    pub title: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitlePersonaApiRequest {
    pub group_id: String,
    pub title: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitlePersonaApiResponse {
    pub group_id: String,
    pub group_name_id: String,
    pub title: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleProfileApiRequest {
    pub group_id: String,
    pub title: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleProfileApiResponse {
    pub group_id: String,
    pub group_name_id: String,
    pub title: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleProviderApiRequest {
    pub group_id: String,
    pub title: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleProviderApiResponse {
    pub group_id: String,
    pub group_name_id: String,
    pub title: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleRubricApiRequest {
    pub group_id: String,
    pub title: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleRubricApiResponse {
    pub group_id: String,
    pub group_name_id: String,
    pub title: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleScenarioApiRequest {
    pub group_id: String,
    pub title: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleScenarioApiResponse {
    pub group_id: String,
    pub group_name_id: String,
    pub title: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleSettingApiRequest {
    pub group_id: String,
    pub title: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleSettingApiResponse {
    pub group_id: String,
    pub group_name_id: String,
    pub title: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleSimulationApiRequest {
    pub group_id: String,
    pub title: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleSimulationApiResponse {
    pub group_id: String,
    pub group_name_id: String,
    pub title: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleSystemApiRequest {
    pub group_id: String,
    pub title: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleSystemApiResponse {
    pub group_id: String,
    pub group_name_id: String,
    pub title: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleTestApiRequest {
    pub group_id: String,
    pub title: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleTestApiResponse {
    pub group_id: String,
    pub group_name_id: String,
    pub title: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleToolApiRequest {
    pub group_id: String,
    pub title: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleToolApiResponse {
    pub group_id: String,
    pub group_name_id: String,
    pub title: String,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolArgDraftValue {
    pub name: String,
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub field_type: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub default_value: Option<String>,
    #[serde(default)]
    pub outputs: Option<Vec<ToolArgOutputDraftValue>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolArgOutputDraftValue {
    pub name: String,
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub template: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolArgOutputResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub args_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub template: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolArgPositionResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub args_id: Option<String>,
    #[serde(default)]
    pub value: Option<i64>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolArgResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub field_type: Option<String>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub default_value: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDepartmentResource {
    #[serde(default)]
    pub department_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDescriptionResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolFieldError {
    pub field: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolFlagResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub value: Option<bool>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub icon_id: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolInstructionResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub template: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolNameResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolPermissionResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub artifact: Option<String>,
    #[serde(default)]
    pub operation: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolPreviewArg {
    pub name: String,
    #[serde(default)]
    pub field_type: Option<String>,
    #[serde(default)]
    pub default_value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolPreviewArgHint {
    pub name: String,
    #[serde(default)]
    pub used: Option<bool>,
    #[serde(default)]
    pub filters: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolPreviewOutput {
    pub name: String,
    #[serde(default)]
    pub template: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolPreviewOutputResult {
    pub name: String,
    #[serde(default)]
    pub compiled: Option<String>,
    #[serde(default)]
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResultItem {
    pub success: bool,
    pub message: String,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub errors: Option<Vec<ToolFieldError>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnemulateProfileApiRequest {
    pub target_profile_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnemulateProfileApiResponse {
    pub ok: bool,
    #[serde(default)]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAgentApiRequest {
    #[serde(default)]
    pub agents: Option<Vec<UpdateAgentItem>>,
    #[serde(default)]
    pub all: Option<bool>,
    #[serde(default)]
    pub excluded_ids: Option<Vec<String>>,
    #[serde(default)]
    pub patch: Option<UpdateAgentPatch>,
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub filter_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_model_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_tool_ids: Option<Vec<String>>,
    #[serde(default)]
    pub department_search: Option<String>,
    #[serde(default)]
    pub model_search: Option<String>,
    #[serde(default)]
    pub tool_search: Option<String>,
    #[serde(default)]
    pub flag_search: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAgentApiResponse {
    pub results: Vec<AgentResultItem>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAgentItem {
    pub id: String,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub model_id: Option<String>,
    #[serde(default)]
    pub reasoning_level_ids: Option<Vec<String>>,
    #[serde(default)]
    pub temperature_level_ids: Option<Vec<String>>,
    #[serde(default)]
    pub tool_ids: Option<Vec<String>>,
    #[serde(default)]
    pub voice_ids: Option<Vec<String>>,
    #[serde(default)]
    pub agent_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAgentPatch {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub model_id: Option<String>,
    #[serde(default)]
    pub reasoning_level_ids: Option<Vec<String>>,
    #[serde(default)]
    pub temperature_level_ids: Option<Vec<String>>,
    #[serde(default)]
    pub tool_ids: Option<Vec<String>>,
    #[serde(default)]
    pub voice_ids: Option<Vec<String>>,
    #[serde(default)]
    pub agent_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAuthApiRequest {
    #[serde(default)]
    pub auths: Option<Vec<UpdateAuthItem>>,
    #[serde(default)]
    pub all: Option<bool>,
    #[serde(default)]
    pub excluded_ids: Option<Vec<String>>,
    #[serde(default)]
    pub patch: Option<UpdateAuthPatch>,
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub filter_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub department_search: Option<String>,
    #[serde(default)]
    pub flag_search: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAuthApiResponse {
    pub results: Vec<AuthResultItem>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub auths: Option<Vec<ListAuthApiAuth>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAuthItem {
    pub id: String,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub slug_id: Option<String>,
    #[serde(default)]
    pub slug: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub protocol_ids: Option<Vec<String>>,
    #[serde(default)]
    pub protocol: Option<String>,
    #[serde(default)]
    pub item_ids: Option<Vec<String>>,
    #[serde(default)]
    pub auth_resource_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAuthPatch {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub slug_id: Option<String>,
    #[serde(default)]
    pub slug: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub protocol_ids: Option<Vec<String>>,
    #[serde(default)]
    pub protocol: Option<String>,
    #[serde(default)]
    pub item_ids: Option<Vec<String>>,
    #[serde(default)]
    pub auth_resource_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCohortApiRequest {
    #[serde(default)]
    pub cohorts: Option<Vec<UpdateCohortItem>>,
    #[serde(default)]
    pub all: Option<bool>,
    #[serde(default)]
    pub excluded_ids: Option<Vec<String>>,
    #[serde(default)]
    pub patch: Option<UpdateCohortPatch>,
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub filter_profile_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_simulation_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub profile_search: Option<String>,
    #[serde(default)]
    pub simulation_search: Option<String>,
    #[serde(default)]
    pub department_search: Option<String>,
    #[serde(default)]
    pub flag_search: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCohortApiResponse {
    pub results: Vec<CohortResultItem>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCohortItem {
    pub id: String,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub simulation_ids: Option<Vec<String>>,
    #[serde(default)]
    pub simulation_position_ids: Option<Vec<String>>,
    #[serde(default)]
    pub simulation_availability_ids: Option<Vec<String>>,
    #[serde(default)]
    pub profile_ids: Option<Vec<String>>,
    #[serde(default)]
    pub profile_persona_ids: Option<Vec<String>>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub simulations: Option<Vec<String>>,
    #[serde(default)]
    pub profiles: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCohortPatch {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub simulation_ids: Option<Vec<String>>,
    #[serde(default)]
    pub simulation_position_ids: Option<Vec<String>>,
    #[serde(default)]
    pub simulation_availability_ids: Option<Vec<String>>,
    #[serde(default)]
    pub profile_ids: Option<Vec<String>>,
    #[serde(default)]
    pub profile_persona_ids: Option<Vec<String>>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub simulations: Option<Vec<String>>,
    #[serde(default)]
    pub profiles: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDepartmentApiRequest {
    #[serde(default)]
    pub departments: Option<Vec<UpdateDepartmentItem>>,
    #[serde(default)]
    pub all: Option<bool>,
    #[serde(default)]
    pub excluded_ids: Option<Vec<String>>,
    #[serde(default)]
    pub patch: Option<UpdateDepartmentPatch>,
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub flag_search: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDepartmentApiResponse {
    pub results: Vec<DepartmentResultItem>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub departments: Option<Vec<ListDepartmentApiDepartment>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDepartmentItem {
    pub id: String,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub settings_ids: Option<Vec<String>>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDepartmentPatch {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub settings_ids: Option<Vec<String>>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDocumentApiRequest {
    #[serde(default)]
    pub documents: Option<Vec<UpdateDocumentItem>>,
    #[serde(default)]
    pub all: Option<bool>,
    #[serde(default)]
    pub excluded_ids: Option<Vec<String>>,
    #[serde(default)]
    pub patch: Option<UpdateDocumentPatch>,
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub scenario_ids: Option<Vec<String>>,
    #[serde(default)]
    pub field_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub scenario_search: Option<String>,
    #[serde(default)]
    pub field_search: Option<String>,
    #[serde(default)]
    pub department_search: Option<String>,
    #[serde(default)]
    pub flag_search: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDocumentApiResponse {
    pub results: Vec<DocumentResultItem>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDocumentItem {
    pub id: String,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub parameter_field_ids: Option<Vec<String>>,
    #[serde(default)]
    pub upload_ids: Option<Vec<String>>,
    #[serde(default)]
    pub image_ids: Option<Vec<String>>,
    #[serde(default)]
    pub text_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDocumentPatch {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub parameter_field_ids: Option<Vec<String>>,
    #[serde(default)]
    pub upload_ids: Option<Vec<String>>,
    #[serde(default)]
    pub image_ids: Option<Vec<String>>,
    #[serde(default)]
    pub text_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateEvalApiRequest {
    #[serde(default)]
    pub evals: Option<Vec<UpdateEvalItem>>,
    #[serde(default)]
    pub all: Option<bool>,
    #[serde(default)]
    pub excluded_ids: Option<Vec<String>>,
    #[serde(default)]
    pub patch: Option<UpdateEvalPatch>,
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub filter_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub department_search: Option<String>,
    #[serde(default)]
    pub flag_search: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateEvalApiResponse {
    pub results: Vec<EvalResultItem>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub evals: Option<Vec<ListEvalApiEval>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateEvalItem {
    pub id: String,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub model_ids: Option<Vec<String>>,
    #[serde(default)]
    pub model_flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub model_rubric_ids: Option<Vec<String>>,
    #[serde(default)]
    pub model_position_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateEvalPatch {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub model_ids: Option<Vec<String>>,
    #[serde(default)]
    pub model_flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub model_rubric_ids: Option<Vec<String>>,
    #[serde(default)]
    pub model_position_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFieldApiRequest {
    #[serde(default)]
    pub fields: Option<Vec<UpdateFieldItem>>,
    #[serde(default)]
    pub all: Option<bool>,
    #[serde(default)]
    pub excluded_ids: Option<Vec<String>>,
    #[serde(default)]
    pub patch: Option<UpdateFieldPatch>,
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub parameter_ids: Option<Vec<String>>,
    #[serde(default)]
    pub persona_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub parameter_search: Option<String>,
    #[serde(default)]
    pub persona_search: Option<String>,
    #[serde(default)]
    pub department_search: Option<String>,
    #[serde(default)]
    pub flag_search: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFieldApiResponse {
    pub results: Vec<FieldResultItem>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFieldItem {
    pub id: String,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub conditional_parameter_ids: Option<Vec<String>>,
    #[serde(default)]
    pub field_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFieldPatch {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub conditional_parameter_ids: Option<Vec<String>>,
    #[serde(default)]
    pub field_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateModelApiRequest {
    #[serde(default)]
    pub models: Option<Vec<UpdateModelItem>>,
    #[serde(default)]
    pub all: Option<bool>,
    #[serde(default)]
    pub excluded_ids: Option<Vec<String>>,
    #[serde(default)]
    pub patch: Option<UpdateModelPatch>,
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub filter_provider_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_agent_ids: Option<Vec<String>>,
    #[serde(default)]
    pub provider_search: Option<String>,
    #[serde(default)]
    pub department_search: Option<String>,
    #[serde(default)]
    pub agent_search: Option<String>,
    #[serde(default)]
    pub flag_search: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateModelApiResponse {
    pub results: Vec<ModelResultItem>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub models: Option<Vec<ListModelApiModel>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateModelItem {
    pub id: String,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub modality_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pricing_ids: Option<Vec<String>>,
    #[serde(default)]
    pub provider_id: Option<String>,
    #[serde(default)]
    pub quality_ids: Option<Vec<String>>,
    #[serde(default)]
    pub reasoning_level_ids: Option<Vec<String>>,
    #[serde(default)]
    pub temperature_level_ids: Option<Vec<String>>,
    #[serde(default)]
    pub value_id: Option<String>,
    #[serde(default)]
    pub voice_ids: Option<Vec<String>>,
    #[serde(default)]
    pub model_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateModelPatch {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub modality_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pricing_ids: Option<Vec<String>>,
    #[serde(default)]
    pub provider_id: Option<String>,
    #[serde(default)]
    pub quality_ids: Option<Vec<String>>,
    #[serde(default)]
    pub reasoning_level_ids: Option<Vec<String>>,
    #[serde(default)]
    pub temperature_level_ids: Option<Vec<String>>,
    #[serde(default)]
    pub value_id: Option<String>,
    #[serde(default)]
    pub voice_ids: Option<Vec<String>>,
    #[serde(default)]
    pub model_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateParameterApiRequest {
    #[serde(default)]
    pub parameters: Option<Vec<UpdateParameterItem>>,
    #[serde(default)]
    pub all: Option<bool>,
    #[serde(default)]
    pub excluded_ids: Option<Vec<String>>,
    #[serde(default)]
    pub patch: Option<UpdateParameterPatch>,
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub scenario_ids: Option<Vec<String>>,
    #[serde(default)]
    pub field_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub scenario_search: Option<String>,
    #[serde(default)]
    pub field_search: Option<String>,
    #[serde(default)]
    pub department_search: Option<String>,
    #[serde(default)]
    pub flag_search: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateParameterApiResponse {
    pub results: Vec<ParameterResultItem>,
    #[serde(default)]
    pub parameters: Option<Vec<ListParameterApiParameter>>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateParameterItem {
    pub id: String,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub field_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateParameterPatch {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub field_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePersonaApiRequest {
    #[serde(default)]
    pub personas: Option<Vec<UpdatePersonaItem>>,
    #[serde(default)]
    pub all: Option<bool>,
    #[serde(default)]
    pub excluded_ids: Option<Vec<String>>,
    #[serde(default)]
    pub patch: Option<UpdatePersonaPatch>,
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub scenario_ids: Option<Vec<String>>,
    #[serde(default)]
    pub field_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub scenario_search: Option<String>,
    #[serde(default)]
    pub field_search: Option<String>,
    #[serde(default)]
    pub department_search: Option<String>,
    #[serde(default)]
    pub color_search: Option<String>,
    #[serde(default)]
    pub icon_search: Option<String>,
    #[serde(default)]
    pub voice_search: Option<String>,
    #[serde(default)]
    pub instruction_search: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePersonaApiResponse {
    pub results: Vec<PersonaResultItem>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub personas: Option<Vec<ListPersonaApiPersona>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePersonaItem {
    pub id: String,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub color_id: Option<String>,
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub icon_id: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub instructions_id: Option<String>,
    #[serde(default)]
    pub instructions: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub parameter_field_ids: Option<Vec<String>>,
    #[serde(default)]
    pub parameter_fields: Option<Vec<String>>,
    #[serde(default)]
    pub example_ids: Option<Vec<String>>,
    #[serde(default)]
    pub examples: Option<Vec<String>>,
    #[serde(default)]
    pub voice_ids: Option<Vec<String>>,
    #[serde(default)]
    pub voices: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePersonaPatch {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub color_id: Option<String>,
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub icon_id: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub instructions_id: Option<String>,
    #[serde(default)]
    pub instructions: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub parameter_field_ids: Option<Vec<String>>,
    #[serde(default)]
    pub parameter_fields: Option<Vec<String>>,
    #[serde(default)]
    pub example_ids: Option<Vec<String>>,
    #[serde(default)]
    pub examples: Option<Vec<String>>,
    #[serde(default)]
    pub voice_ids: Option<Vec<String>>,
    #[serde(default)]
    pub voices: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProfileApiRequest {
    #[serde(default)]
    pub profiles: Option<Vec<UpdateProfileItem>>,
    #[serde(default)]
    pub all: Option<bool>,
    #[serde(default)]
    pub excluded_ids: Option<Vec<String>>,
    #[serde(default)]
    pub patch: Option<UpdateProfilePatch>,
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub cohort_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub role_filter: Option<String>,
    #[serde(default)]
    pub cohort_search: Option<String>,
    #[serde(default)]
    pub department_search: Option<String>,
    #[serde(default)]
    pub role_search: Option<String>,
    #[serde(default)]
    pub flag_search: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProfileApiResponse {
    pub results: Vec<ProfileResultItem>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub profiles: Option<Vec<ListProfilesApiProfile>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProfileItem {
    pub profile_id: String,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub email_ids: Option<Vec<String>>,
    #[serde(default)]
    pub role_id: Option<String>,
    #[serde(default)]
    pub primary_department_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProfilePatch {
    #[serde(default)]
    pub profile_id: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub email_ids: Option<Vec<String>>,
    #[serde(default)]
    pub role_id: Option<String>,
    #[serde(default)]
    pub primary_department_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProviderApiRequest {
    #[serde(default)]
    pub providers: Option<Vec<UpdateProviderItem>>,
    #[serde(default)]
    pub all: Option<bool>,
    #[serde(default)]
    pub excluded_ids: Option<Vec<String>>,
    #[serde(default)]
    pub patch: Option<UpdateProviderPatch>,
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub filter_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_model_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_status: Option<Vec<String>>,
    #[serde(default)]
    pub department_search: Option<String>,
    #[serde(default)]
    pub model_search: Option<String>,
    #[serde(default)]
    pub flag_search: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProviderApiResponse {
    pub results: Vec<ProviderResultItem>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub providers: Option<Vec<ListProviderApiProvider>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProviderItem {
    pub id: String,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub endpoint_ids: Option<Vec<String>>,
    #[serde(default)]
    pub key_ids: Option<Vec<String>>,
    #[serde(default)]
    pub value_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProviderPatch {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub endpoint_ids: Option<Vec<String>>,
    #[serde(default)]
    pub key_ids: Option<Vec<String>>,
    #[serde(default)]
    pub value_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRubricApiRequest {
    #[serde(default)]
    pub rubrics: Option<Vec<UpdateRubricItem>>,
    #[serde(default)]
    pub all: Option<bool>,
    #[serde(default)]
    pub excluded_ids: Option<Vec<String>>,
    #[serde(default)]
    pub patch: Option<UpdateRubricPatch>,
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub filter_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_simulation_ids: Option<Vec<String>>,
    #[serde(default)]
    pub department_search: Option<String>,
    #[serde(default)]
    pub simulation_search: Option<String>,
    #[serde(default)]
    pub flag_search: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRubricApiResponse {
    pub results: Vec<RubricResultItem>,
    #[serde(default)]
    pub rubrics: Option<Vec<ListRubricApiRubric>>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRubricItem {
    pub id: String,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub simulation_rubric: Option<bool>,
    #[serde(default)]
    pub video_rubric: Option<bool>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub pass_points_id: Option<String>,
    #[serde(default)]
    pub pass_points: Option<i64>,
    #[serde(default)]
    pub standard_group_ids: Option<Vec<String>>,
    #[serde(default)]
    pub standard_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRubricPatch {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub simulation_rubric: Option<bool>,
    #[serde(default)]
    pub video_rubric: Option<bool>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub pass_points_id: Option<String>,
    #[serde(default)]
    pub pass_points: Option<i64>,
    #[serde(default)]
    pub standard_group_ids: Option<Vec<String>>,
    #[serde(default)]
    pub standard_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateScenarioApiRequest {
    #[serde(default)]
    pub scenarios: Option<Vec<UpdateScenarioItem>>,
    #[serde(default)]
    pub all: Option<bool>,
    #[serde(default)]
    pub excluded_ids: Option<Vec<String>>,
    #[serde(default)]
    pub patch: Option<UpdateScenarioPatch>,
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub persona_ids: Option<Vec<String>>,
    #[serde(default)]
    pub simulation_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub persona_search: Option<String>,
    #[serde(default)]
    pub simulation_search: Option<String>,
    #[serde(default)]
    pub department_search: Option<String>,
    #[serde(default)]
    pub flag_search: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateScenarioApiResponse {
    pub results: Vec<ScenarioResultItem>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub scenarios: Option<Vec<ListScenarioApiScenario>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateScenarioItem {
    pub id: String,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub problem_statement_id: Option<String>,
    #[serde(default)]
    pub problem_statement: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub persona_ids: Option<Vec<String>>,
    #[serde(default)]
    pub document_ids: Option<Vec<String>>,
    #[serde(default)]
    pub parameter_ids: Option<Vec<String>>,
    #[serde(default)]
    pub parameter_field_ids: Option<Vec<String>>,
    #[serde(default)]
    pub image_ids: Option<Vec<String>>,
    #[serde(default)]
    pub objective_ids: Option<Vec<String>>,
    #[serde(default)]
    pub video_ids: Option<Vec<String>>,
    #[serde(default)]
    pub question_ids: Option<Vec<String>>,
    #[serde(default)]
    pub option_ids: Option<Vec<String>>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub personas: Option<Vec<String>>,
    #[serde(default)]
    pub documents: Option<Vec<String>>,
    #[serde(default)]
    pub parameter_fields: Option<Vec<String>>,
    #[serde(default)]
    pub objectives: Option<Vec<String>>,
    #[serde(default)]
    pub images: Option<Vec<String>>,
    #[serde(default)]
    pub videos: Option<Vec<String>>,
    #[serde(default)]
    pub questions: Option<Vec<String>>,
    #[serde(default)]
    pub options: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateScenarioPatch {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub problem_statement_id: Option<String>,
    #[serde(default)]
    pub problem_statement: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub persona_ids: Option<Vec<String>>,
    #[serde(default)]
    pub document_ids: Option<Vec<String>>,
    #[serde(default)]
    pub parameter_ids: Option<Vec<String>>,
    #[serde(default)]
    pub parameter_field_ids: Option<Vec<String>>,
    #[serde(default)]
    pub image_ids: Option<Vec<String>>,
    #[serde(default)]
    pub objective_ids: Option<Vec<String>>,
    #[serde(default)]
    pub video_ids: Option<Vec<String>>,
    #[serde(default)]
    pub question_ids: Option<Vec<String>>,
    #[serde(default)]
    pub option_ids: Option<Vec<String>>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub personas: Option<Vec<String>>,
    #[serde(default)]
    pub documents: Option<Vec<String>>,
    #[serde(default)]
    pub parameter_fields: Option<Vec<String>>,
    #[serde(default)]
    pub objectives: Option<Vec<String>>,
    #[serde(default)]
    pub images: Option<Vec<String>>,
    #[serde(default)]
    pub videos: Option<Vec<String>>,
    #[serde(default)]
    pub questions: Option<Vec<String>>,
    #[serde(default)]
    pub options: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSettingApiRequest {
    #[serde(default)]
    pub settings: Option<Vec<UpdateSettingItem>>,
    #[serde(default)]
    pub all: Option<bool>,
    #[serde(default)]
    pub excluded_ids: Option<Vec<String>>,
    #[serde(default)]
    pub patch: Option<UpdateSettingPatch>,
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub provider_ids: Option<Vec<String>>,
    #[serde(default)]
    pub auth_ids: Option<Vec<String>>,
    #[serde(default)]
    pub system_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub flag_search: Option<String>,
    #[serde(default)]
    pub provider_search: Option<String>,
    #[serde(default)]
    pub auth_search: Option<String>,
    #[serde(default)]
    pub system_search: Option<String>,
    #[serde(default)]
    pub department_search: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSettingApiResponse {
    pub results: Vec<SettingResultItem>,
    #[serde(default)]
    pub settings: Option<Vec<ListSettingApiSetting>>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSettingItem {
    pub id: String,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active_flag_id: Option<String>,
    #[serde(default)]
    pub active_flag: Option<bool>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub color_ids: Option<Vec<String>>,
    #[serde(default)]
    pub logins_ids: Option<Vec<String>>,
    #[serde(default)]
    pub system_ids: Option<Vec<String>>,
    #[serde(default)]
    pub mcp_id: Option<String>,
    #[serde(default)]
    pub threshold_ids: Option<Vec<String>>,
    #[serde(default)]
    pub provider_key_ids: Option<Vec<String>>,
    #[serde(default)]
    pub auth_item_key_ids: Option<Vec<String>>,
    #[serde(default)]
    pub auth_item_value_ids: Option<Vec<String>>,
    #[serde(default)]
    pub auth_ids: Option<Vec<String>>,
    #[serde(default)]
    pub provider_ids: Option<Vec<String>>,
    #[serde(default)]
    pub setting_resource_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSettingPatch {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active_flag_id: Option<String>,
    #[serde(default)]
    pub active_flag: Option<bool>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub color_ids: Option<Vec<String>>,
    #[serde(default)]
    pub logins_ids: Option<Vec<String>>,
    #[serde(default)]
    pub system_ids: Option<Vec<String>>,
    #[serde(default)]
    pub mcp_id: Option<String>,
    #[serde(default)]
    pub threshold_ids: Option<Vec<String>>,
    #[serde(default)]
    pub provider_key_ids: Option<Vec<String>>,
    #[serde(default)]
    pub auth_item_key_ids: Option<Vec<String>>,
    #[serde(default)]
    pub auth_item_value_ids: Option<Vec<String>>,
    #[serde(default)]
    pub auth_ids: Option<Vec<String>>,
    #[serde(default)]
    pub provider_ids: Option<Vec<String>>,
    #[serde(default)]
    pub setting_resource_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSimulationApiRequest {
    #[serde(default)]
    pub simulations: Option<Vec<UpdateSimulationItem>>,
    #[serde(default)]
    pub all: Option<bool>,
    #[serde(default)]
    pub excluded_ids: Option<Vec<String>>,
    #[serde(default)]
    pub patch: Option<UpdateSimulationPatch>,
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub filter_scenario_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_cohort_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub scenario_search: Option<String>,
    #[serde(default)]
    pub cohort_search: Option<String>,
    #[serde(default)]
    pub department_search: Option<String>,
    #[serde(default)]
    pub flag_search: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSimulationApiResponse {
    pub results: Vec<SimulationResultItem>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSimulationItem {
    pub id: String,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub scenario_ids: Option<Vec<String>>,
    #[serde(default)]
    pub scenario_flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub scenario_position_ids: Option<Vec<String>>,
    #[serde(default)]
    pub scenario_rubric_ids: Option<Vec<String>>,
    #[serde(default)]
    pub scenario_time_limit_ids: Option<Vec<String>>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub scenarios: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSimulationPatch {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub scenario_ids: Option<Vec<String>>,
    #[serde(default)]
    pub scenario_flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub scenario_position_ids: Option<Vec<String>>,
    #[serde(default)]
    pub scenario_rubric_ids: Option<Vec<String>>,
    #[serde(default)]
    pub scenario_time_limit_ids: Option<Vec<String>>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub scenarios: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateToolApiRequest {
    #[serde(default)]
    pub tools: Option<Vec<UpdateToolItem>>,
    #[serde(default)]
    pub all: Option<bool>,
    #[serde(default)]
    pub excluded_ids: Option<Vec<String>>,
    #[serde(default)]
    pub patch: Option<UpdateToolPatch>,
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub filter_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_creatable: Option<Vec<String>>,
    #[serde(default)]
    pub filter_agent_ids: Option<Vec<String>>,
    #[serde(default)]
    pub department_search: Option<String>,
    #[serde(default)]
    pub flag_search: Option<String>,
    #[serde(default)]
    pub agent_search: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub accept: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateToolApiResponse {
    pub results: Vec<ToolResultItem>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub tools: Option<Vec<ListToolApiTool>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateToolItem {
    pub id: String,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub arg_positions_ids: Option<Vec<String>>,
    #[serde(default)]
    pub args_ids: Option<Vec<String>>,
    #[serde(default)]
    pub args_outputs_ids: Option<Vec<String>>,
    #[serde(default)]
    pub permission_ids: Option<Vec<String>>,
    #[serde(default)]
    pub instruction_id: Option<String>,
    #[serde(default)]
    pub tool_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateToolPatch {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub arg_positions_ids: Option<Vec<String>>,
    #[serde(default)]
    pub args_ids: Option<Vec<String>>,
    #[serde(default)]
    pub args_outputs_ids: Option<Vec<String>>,
    #[serde(default)]
    pub permission_ids: Option<Vec<String>>,
    #[serde(default)]
    pub instruction_id: Option<String>,
    #[serde(default)]
    pub tool_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    pub loc: Vec<serde_json::Value>,
    pub msg: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoDownloadAttemptApiRequest {
    pub video_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoDownloadGroupApiRequest {
    pub video_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoDownloadScenarioApiRequest {
    pub video_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoEntry {
    #[serde(default)]
    pub video_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub length_seconds: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoUploadScenarioApiResponse {
    pub video_id: String,
    pub upload_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App__infra__agent__types__DraftFormState {
    pub department_ids: Vec<String>,
    pub tool_ids: Vec<String>,
    pub voice_ids: Vec<String>,
    pub quality_ids: Vec<String>,
    pub rubric_ids: Vec<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub model_id: Option<String>,
    #[serde(default)]
    pub reasoning_level_id: Option<String>,
    #[serde(default)]
    pub temperature_level_id: Option<String>,
    #[serde(default)]
    pub prompt_id: Option<String>,
    #[serde(default)]
    pub instruction_id: Option<String>,
    #[serde(default)]
    pub pending_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App__infra__agent__types__SectionFilter {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub include: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App__infra__attempt__chat__types__DraftImageValue {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub upload_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App__infra__attempt__chat__types__DraftOptionValue {
    pub option_text: String,
    #[serde(default)]
    pub question_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App__infra__attempt__chat__types__DraftQuestionValue {
    pub question_text: String,
    #[serde(default)]
    pub time: Option<i64>,
    #[serde(default)]
    pub allow_multiple: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App__infra__attempt__chat__types__DraftVideoValue {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub upload_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App__infra__attempt__chat__types__SectionFilter {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub include: Option<bool>,
    #[serde(default)]
    pub parameter_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App__infra__auth__types__DraftFormState {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub protocol_ids: Option<Vec<String>>,
    #[serde(default)]
    pub slug_ids: Option<Vec<String>>,
    #[serde(default)]
    pub item_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App__infra__auth__types__SectionFilter {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub include: Option<bool>,
    #[serde(default)]
    pub parameter_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App__infra__cohort__types__DraftFormState {
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub simulation_ids: Option<Vec<String>>,
    #[serde(default)]
    pub simulations: Option<Vec<String>>,
    #[serde(default)]
    pub simulation_position_ids: Option<Vec<String>>,
    #[serde(default)]
    pub simulation_positions: Option<Vec<DraftSimulationPositionValue>>,
    #[serde(default)]
    pub simulation_availability_ids: Option<Vec<String>>,
    #[serde(default)]
    pub simulation_availability: Option<Vec<DraftSimulationAvailabilityValue>>,
    #[serde(default)]
    pub profile_ids: Option<Vec<String>>,
    #[serde(default)]
    pub profiles: Option<Vec<String>>,
    #[serde(default)]
    pub profile_persona_ids: Option<Vec<String>>,
    #[serde(default)]
    pub profile_personas: Option<Vec<DraftProfilePersonaValue>>,
    #[serde(default)]
    pub pending_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App__infra__cohort__types__SectionFilter {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub include: Option<bool>,
    #[serde(default)]
    pub parameter_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App__infra__department__types__DraftFormState {
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub setting_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App__infra__department__types__SectionFilter {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub include: Option<bool>,
    #[serde(default)]
    pub parameter_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App__infra__document__types__DraftFormState {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub file_ids: Option<Vec<String>>,
    #[serde(default)]
    pub image_ids: Option<Vec<String>>,
    #[serde(default)]
    pub text_ids: Option<Vec<String>>,
    #[serde(default)]
    pub parameter_field_ids: Option<Vec<String>>,
    #[serde(default)]
    pub parameter_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App__infra__document__types__DraftImageValue {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub upload_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App__infra__document__types__SectionFilter {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub include: Option<bool>,
    #[serde(default)]
    pub parameter_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App__infra__eval__types__DraftFormState {
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub model_ids: Option<Vec<String>>,
    #[serde(default)]
    pub model_flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub model_flag_values: Option<Vec<EvalModelFlagValue>>,
    #[serde(default)]
    pub model_position_ids: Option<Vec<String>>,
    #[serde(default)]
    pub model_rubric_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App__infra__eval__types__SectionFilter {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub include: Option<bool>,
    #[serde(default)]
    pub parameter_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App__infra__field__types__DraftFormState {
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub conditional_parameter_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App__infra__field__types__SectionFilter {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub include: Option<bool>,
    #[serde(default)]
    pub parameter_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App__infra__invocation__types__DraftFormState {
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub value_id: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub key_id: Option<String>,
    #[serde(default)]
    pub endpoint_id: Option<String>,
    #[serde(default)]
    pub modality_ids: Option<Vec<String>>,
    #[serde(default)]
    pub temperature_level_id: Option<String>,
    #[serde(default)]
    pub pricing_id: Option<String>,
    #[serde(default)]
    pub reasoning_level_id: Option<String>,
    #[serde(default)]
    pub quality_ids: Option<Vec<String>>,
    #[serde(default)]
    pub voice_ids: Option<Vec<String>>,
    #[serde(default)]
    pub model_flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub model_flag_values: Option<Vec<InvocationModelFlagValue>>,
    #[serde(default)]
    pub model_position_ids: Option<Vec<String>>,
    #[serde(default)]
    pub model_rubric_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App__infra__invocation__types__SectionFilter {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub include: Option<bool>,
    #[serde(default)]
    pub parameter_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App__infra__model__types__DraftFormState {
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub value_id: Option<String>,
    #[serde(default)]
    pub value: Option<String>,
    #[serde(default)]
    pub provider_id: Option<String>,
    #[serde(default)]
    pub provider: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub modalities_enabled: Option<bool>,
    #[serde(default)]
    pub temperature_enabled: Option<bool>,
    #[serde(default)]
    pub pricing_enabled: Option<bool>,
    #[serde(default)]
    pub voices_enabled: Option<bool>,
    #[serde(default)]
    pub reasoning_levels_enabled: Option<bool>,
    #[serde(default)]
    pub qualities_enabled: Option<bool>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub modality_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pricing_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pricing: Option<Vec<PricingDraftValue>>,
    #[serde(default)]
    pub quality_ids: Option<Vec<String>>,
    #[serde(default)]
    pub reasoning_level_ids: Option<Vec<String>>,
    #[serde(default)]
    pub temperature_level_ids: Option<Vec<String>>,
    #[serde(default)]
    pub voice_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App__infra__model__types__SectionFilter {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub include: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App__infra__parameter__types__DraftFormState {
    pub flag_ids: Vec<String>,
    pub department_ids: Vec<String>,
    pub field_ids: Vec<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub pending_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App__infra__parameter__types__SectionFilter {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub include: Option<bool>,
    #[serde(default)]
    pub parameter_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App__infra__persona__types__DraftFormState {
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub instructions_id: Option<String>,
    #[serde(default)]
    pub instructions: Option<String>,
    #[serde(default)]
    pub color_id: Option<String>,
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub icon_id: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub example_ids: Option<Vec<String>>,
    #[serde(default)]
    pub parameter_field_ids: Option<Vec<String>>,
    #[serde(default)]
    pub voice_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App__infra__persona__types__SectionFilter {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub include: Option<bool>,
    #[serde(default)]
    pub parameter_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App__infra__profile__types__DraftFormState {
    pub department_ids: Vec<String>,
    pub email_ids: Vec<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub emails: Option<Vec<String>>,
    #[serde(default)]
    pub role: Option<String>,
    #[serde(default)]
    pub role_id: Option<String>,
    #[serde(default)]
    pub role_draft: Option<ProfileRoleDraftValue>,
    #[serde(default)]
    pub primary_department_id: Option<String>,
    #[serde(default)]
    pub pending_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App__infra__profile__types__SectionFilter {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub include: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App__infra__provider__types__DraftFormState {
    pub department_ids: Vec<String>,
    pub endpoint_ids: Vec<String>,
    pub key_ids: Vec<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub endpoint: Option<String>,
    #[serde(default)]
    pub endpoint_id: Option<String>,
    #[serde(default)]
    pub key: Option<String>,
    #[serde(default)]
    pub key_name: Option<String>,
    #[serde(default)]
    pub key_description: Option<String>,
    #[serde(default)]
    pub key_id: Option<String>,
    #[serde(default)]
    pub value: Option<String>,
    #[serde(default)]
    pub value_id: Option<String>,
    #[serde(default)]
    pub pending_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App__infra__provider__types__SectionFilter {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub include: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App__infra__rubric__types__DraftFormState {
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub simulation_rubric: Option<bool>,
    #[serde(default)]
    pub video_rubric: Option<bool>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pass_points_id: Option<String>,
    #[serde(default)]
    pub pass_points: Option<i64>,
    #[serde(default)]
    pub total_points_id: Option<String>,
    #[serde(default)]
    pub total_points: Option<i64>,
    #[serde(default)]
    pub standard_group_ids: Option<Vec<String>>,
    #[serde(default)]
    pub standard_groups: Option<Vec<RubricStandardGroupDraftValue>>,
    #[serde(default)]
    pub standard_ids: Option<Vec<String>>,
    #[serde(default)]
    pub standards: Option<Vec<RubricStandardDraftValue>>,
    #[serde(default)]
    pub pending_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App__infra__rubric__types__SectionFilter {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub include: Option<bool>,
    #[serde(default)]
    pub parameter_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App__infra__scenario__types__DraftImageValue {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub upload_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App__infra__scenario__types__DraftOptionValue {
    pub option_text: String,
    #[serde(default)]
    pub question_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App__infra__scenario__types__DraftQuestionValue {
    pub question_text: String,
    #[serde(default)]
    pub time: Option<i64>,
    #[serde(default)]
    pub allow_multiple: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App__infra__scenario__types__DraftVideoValue {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub upload_id: Option<String>,
    #[serde(default)]
    pub length_seconds: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App__infra__setting__types__DraftFormState {
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub mcp: Option<bool>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub color_ids: Option<Vec<String>>,
    #[serde(default)]
    pub logins_ids: Option<Vec<String>>,
    #[serde(default)]
    pub system_ids: Option<Vec<String>>,
    #[serde(default)]
    pub mcp_id: Option<String>,
    #[serde(default)]
    pub threshold_ids: Option<Vec<String>>,
    #[serde(default)]
    pub provider_key_ids: Option<Vec<String>>,
    #[serde(default)]
    pub auth_item_key_ids: Option<Vec<String>>,
    #[serde(default)]
    pub auth_item_value_ids: Option<Vec<String>>,
    #[serde(default)]
    pub auth_ids: Option<Vec<String>>,
    #[serde(default)]
    pub provider_ids: Option<Vec<String>>,
    #[serde(default)]
    pub provider_keys: Option<Vec<SettingProviderKeyDraftValue>>,
    #[serde(default)]
    pub auth_item_keys: Option<Vec<SettingAuthItemKeyDraftValue>>,
    #[serde(default)]
    pub auth_item_values: Option<Vec<SettingAuthItemValueDraftValue>>,
    #[serde(default)]
    pub mcp_values: Option<Vec<SettingMcpDraftValue>>,
    #[serde(default)]
    pub system_values: Option<Vec<SettingSystemDraftValue>>,
    #[serde(default)]
    pub threshold_values: Option<Vec<SettingThresholdDraftValue>>,
    #[serde(default)]
    pub logins: Option<Vec<SettingLoginDraftValue>>,
    #[serde(default)]
    pub pending_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App__infra__setting__types__SectionFilter {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub include: Option<bool>,
    #[serde(default)]
    pub parameter_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App__infra__simulation__types__DraftFormState {
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub practice: Option<bool>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub scenario_ids: Option<Vec<String>>,
    #[serde(default)]
    pub scenario_flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub scenario_flag_values: Option<Vec<DraftScenarioFlagDenormValue>>,
    #[serde(default)]
    pub scenario_position_ids: Option<Vec<String>>,
    #[serde(default)]
    pub scenario_rubric_ids: Option<Vec<String>>,
    #[serde(default)]
    pub scenario_time_limit_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App__infra__simulation__types__SectionFilter {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub include: Option<bool>,
    #[serde(default)]
    pub parameter_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App__infra__tool__types__DraftFormState {
    pub arg_ids: Vec<String>,
    pub arg_position_ids: Vec<String>,
    pub args_output_ids: Vec<String>,
    pub args_outputs_ids: Vec<String>,
    pub permission_ids: Vec<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub args_drafts: Option<Vec<ToolArgDraftValue>>,
    #[serde(default)]
    pub instruction_id: Option<String>,
    #[serde(default)]
    pub instruction_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pending_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App__infra__tool__types__SectionFilter {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    pub suggested: Option<bool>,
    #[serde(default)]
    pub include: Option<bool>,
}
