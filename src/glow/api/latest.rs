// Auto-generated from GLOW API OpenAPI spec v2.6.0
// Do not edit manually — regenerated on each API release.
// Schemas: 1206

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityRequest {
    #[serde(default)]
    pub date_from: Option<String>,
    #[serde(default)]
    pub date_to: Option<String>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub roles: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityResources {
    #[serde(default)]
    pub profiles: Option<
        std::collections::HashMap<String, std::collections::HashMap<String, serde_json::Value>>,
    >,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityResponseInput {
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
    pub resources: Option<ActivityResources>,
    #[serde(default)]
    pub analytics: Option<AnalyticsFacetsInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityResponseOutput {
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
    pub resources: Option<ActivityResources>,
    #[serde(default)]
    pub analytics: Option<AnalyticsFacetsOutput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentDepartmentSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentDescriptionSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub resource: Option<serde_json::Value>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentDraftFormState {
    pub flag_ids: Vec<String>,
    pub department_ids: Vec<String>,
    pub model_ids: Vec<String>,
    pub tool_ids: Vec<String>,
    pub reasoning_level_ids: Vec<String>,
    pub temperature_level_ids: Vec<String>,
    pub voice_ids: Vec<String>,
    pub rubric_ids: Vec<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentFieldError {
    pub field: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentFlagConfig {
    pub key: String,
    pub label: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub icon_id: Option<String>,
    #[serde(default)]
    pub flag_option_id: Option<String>,
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub generated: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentFlagSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<AgentFlagConfig>>,
    #[serde(default)]
    pub resources: Option<Vec<AgentFlagConfig>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentInstructionSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub resource: Option<serde_json::Value>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentModelSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub resource: Option<serde_json::Value>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentNameSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub resource: Option<serde_json::Value>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentPromptSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub resource: Option<serde_json::Value>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentQualitySection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentReasoningLevelSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub resource: Option<serde_json::Value>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
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
pub struct AgentRubricSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentTemperatureLevelSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub resource: Option<serde_json::Value>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentToolSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentVoiceSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
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
pub struct AnalyticsFacetsInput {
    pub fields: AnalyticsFilterFields,
    #[serde(default)]
    pub department_options: Option<Vec<AnalyticsFilterOption>>,
    #[serde(default)]
    pub cohort_options: Option<Vec<AnalyticsFilterOption>>,
    #[serde(default)]
    pub role_options: Option<Vec<String>>,
    #[serde(default)]
    pub attempt_options: Option<Vec<String>>,
    #[serde(default)]
    pub date_range_earliest: Option<String>,
    #[serde(default)]
    pub date_range_latest: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsFacetsOutput {
    pub fields: AnalyticsFilterFields,
    #[serde(default)]
    pub department_options: Option<Vec<AnalyticsFilterOption>>,
    #[serde(default)]
    pub cohort_options: Option<Vec<AnalyticsFilterOption>>,
    #[serde(default)]
    pub role_options: Option<Vec<String>>,
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
pub struct ArtifactSessionGroupInput {
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
    pub total_cost: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactSessionGroupOutput {
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
pub struct ArtifactTypeItem {
    pub name: String,
    pub operation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptAssistantCompleteEvent {
    pub chat_id: String,
    pub message_id: String,
    #[serde(default)]
    pub content: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptAssistantHintsEvent {
    pub chat_id: String,
    pub hints: Vec<std::collections::HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptAssistantProgressEvent {
    pub chat_id: String,
    pub content_type: String,
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub audio: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptAssistantStartEvent {
    pub chat_id: String,
    pub message_id: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptAudioEndedEvent {
    pub chat_id: String,
    pub success: bool,
    #[serde(default)]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptAudioReadyEvent {
    pub chat_id: String,
    pub success: bool,
    #[serde(default)]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptAudioStartPayload {
    pub chat_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptAudioStopPayload {
    pub chat_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptChatEndedEvent {
    pub chat_id: String,
    #[serde(default)]
    pub is_attempt_finished: Option<bool>,
    #[serde(default)]
    pub grade_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptChatStartedEvent {
    pub attempt_id: String,
    pub chat_id: String,
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
    pub department_id: Option<String>,
    #[serde(default)]
    pub cohort_id: Option<String>,
    #[serde(default)]
    pub is_archived: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptEndAllPayload {
    pub attempt_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptEndPayload {
    pub attempt_id: String,
    pub chat_id: String,
    #[serde(default)]
    pub grade: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptEndedEvent {
    pub attempt_id: String,
    pub success: bool,
    #[serde(default)]
    pub all_scenarios_complete: Option<bool>,
    #[serde(default)]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptEntriesInput {
    #[serde(default)]
    pub attempt: Option<Vec<GetAttemptResponse>>,
    #[serde(default)]
    pub attempt_chat: Option<Vec<ChatDataInput>>,
    #[serde(default)]
    pub attempt_message: Option<Vec<MessageDataInput>>,
    #[serde(default)]
    pub runs: Option<GetRunListViewResponseInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptEntriesOutput {
    #[serde(default)]
    pub attempt: Option<Vec<GetAttemptResponse>>,
    #[serde(default)]
    pub attempt_chat: Option<Vec<ChatDataOutput>>,
    #[serde(default)]
    pub attempt_message: Option<Vec<MessageDataOutput>>,
    #[serde(default)]
    pub runs: Option<GetRunListViewResponseOutput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptErrorEvent {
    pub message: String,
    #[serde(default)]
    pub chat_id: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptGradeAnalysisEntry {
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptGradeCompleteEvent {
    pub chat_id: String,
    #[serde(default)]
    pub grade_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptGradeFeedbackEntry {
    pub feedback: String,
    #[serde(default)]
    pub total: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptGradeHighlightEntry {
    pub section: String,
    #[serde(default)]
    pub strength_id: Option<String>,
    #[serde(default)]
    pub idx: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptGradeImprovementEntry {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub message_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptGradePayload {
    pub attempt_id: String,
    #[serde(default)]
    pub chat_id: Option<String>,
    #[serde(default)]
    pub resource_types: Option<Vec<String>>,
    #[serde(default)]
    pub user_instructions: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptGradeProgressEvent {
    pub chat_id: String,
    #[serde(default)]
    pub grade_id: Option<String>,
    #[serde(default)]
    pub resource_type: Option<String>,
    #[serde(default)]
    pub entry: Option<std::collections::HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptGradeReplacementEntry {
    pub section: String,
    pub replace: String,
    #[serde(default)]
    pub improvement_id: Option<String>,
    #[serde(default)]
    pub idx: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptGradeStartEvent {
    pub chat_id: String,
    #[serde(default)]
    pub grade_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptGradeStrengthEntry {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub message_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptJoinPayload {
    pub chat_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptJoinRequest {
    pub sid: String,
    pub chat_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptJoinResponse {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptJoinedEvent {
    pub chat_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptLeavePayload {
    pub chat_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptLeaveRequest {
    pub sid: String,
    pub chat_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptLeaveResponse {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptMessagePayload {
    pub attempt_id: String,
    pub chat_id: String,
    pub message: String,
    #[serde(default)]
    pub parent_message_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptNextPayload {
    pub attempt_id: String,
    #[serde(default)]
    pub draft_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptResourcesInput {
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
pub struct AttemptResourcesOutput {
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
pub struct AttemptResponseResultEvent {
    pub success: bool,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub is_correct: Option<bool>,
    #[serde(default)]
    pub response_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptStartPayload {
    #[serde(default)]
    pub home_id: Option<String>,
    #[serde(default)]
    pub practice_id: Option<String>,
    #[serde(default)]
    pub infinite_mode: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptStartedEvent {
    pub attempt_id: String,
    pub chat_entry_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptStopPayload {
    pub chat_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptStoppedEvent {
    pub chat_id: String,
    pub success: bool,
    #[serde(default)]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptUsePreviousPayload {
    pub attempt_id: String,
    pub previous_chat_map: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptUserCompleteEvent {
    pub chat_id: String,
    pub message_id: String,
    pub content: String,
    pub created_at: String,
    #[serde(default)]
    pub item_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptUserDeltaEvent {
    pub chat_id: String,
    pub item_id: String,
    pub transcript: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptUserProgressEvent {
    pub chat_id: String,
    pub transcript: String,
    #[serde(default)]
    pub item_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptUserStartEvent {
    pub chat_id: String,
    pub message_id: String,
    pub created_at: String,
    #[serde(default)]
    pub item_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioFramePayload {
    pub chat_id: String,
    #[serde(default)]
    pub upload_id: Option<String>,
    #[serde(default)]
    pub audio: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioFrameResponse {
    pub accepted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioMutePayload {
    pub chat_id: String,
    #[serde(default)]
    pub muted: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioMuteResponse {
    pub accepted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioStartInternalResult {
    pub chat_id: String,
    pub run_id: String,
    pub group_id: String,
    pub attempt_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioStartPayload {
    pub chat_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioStopInternalResult {
    pub chat_id: String,
    #[serde(default)]
    pub stopped: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioStopPayload {
    pub chat_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthDescriptionSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub resource: Option<serde_json::Value>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthDraftFormState {
    pub department_ids: Vec<String>,
    pub protocol_ids: Vec<String>,
    pub slug_ids: Vec<String>,
    pub item_ids: Vec<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub flag_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthFieldError {
    pub field: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthFlagConfig {
    pub key: String,
    pub label: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub icon_id: Option<String>,
    #[serde(default)]
    pub flag_option_id: Option<String>,
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub generated: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthFlagSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<AuthFlagConfig>>,
    #[serde(default)]
    pub resources: Option<Vec<AuthFlagConfig>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthItemResource {
    #[serde(default)]
    pub auth_item_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub position: Option<i64>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub value_masked: Option<String>,
    #[serde(default)]
    pub key_id: Option<String>,
    #[serde(default)]
    pub encrypted: Option<bool>,
    #[serde(default)]
    pub generated: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthItemSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<AuthItemResource>>,
    #[serde(default)]
    pub resources: Option<Vec<AuthItemResource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthNameSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub resource: Option<serde_json::Value>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthProtocolSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
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
pub struct AuthSlugSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailableContinuationOptionsInput {
    pub options: Vec<ContinuationOption>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailableContinuationOptionsOutput {
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
    pub eval_id: Option<String>,
    #[serde(default)]
    pub eval_name: Option<String>,
    #[serde(default)]
    pub eval_description: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub archived: Option<bool>,
    #[serde(default)]
    pub infinite_mode: Option<bool>,
    #[serde(default)]
    pub total_invocations: Option<i64>,
    #[serde(default)]
    pub completed_invocations: Option<i64>,
    #[serde(default)]
    pub pending_invocations: Option<i64>,
    #[serde(default)]
    pub best_score: Option<f64>,
    #[serde(default)]
    pub has_passed: Option<bool>,
    #[serde(default)]
    pub status: Option<String>,
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
pub struct BenchmarkResponseInput {
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
    pub analytics: Option<AnalyticsFacetsInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResponseOutput {
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
    pub analytics: Option<AnalyticsFacetsOutput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body_parse_agent_csv_agents_csv_post {
    pub file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body_parse_cohort_csv_cohorts_csv_post {
    pub file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body_parse_department_csv_departments_csv_post {
    pub file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body_parse_document_csv_documents_csv_post {
    pub file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body_parse_eval_csv_evals_csv_post {
    pub file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body_parse_field_csv_fields_csv_post {
    pub file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body_parse_model_csv_models_csv_post {
    pub file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body_parse_parameter_csv_parameters_csv_post {
    pub file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body_parse_persona_csv_personas_csv_post {
    pub file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body_parse_provider_csv_providers_csv_post {
    pub file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body_parse_rubric_csv_rubrics_csv_post {
    pub file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body_parse_scenario_csv_scenarios_csv_post {
    pub file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body_parse_setting_csv_settings_csv_post {
    pub file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body_parse_simulation_csv_simulations_csv_post {
    pub file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body_parse_tool_csv_tools_csv_post {
    pub file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatDataInput {
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
pub struct ChatDataOutput {
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
pub struct ChatDepartmentSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatDescriptionSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatDocumentSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatDraftFormState {
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
    pub parameter_ids: Option<Vec<String>>,
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
pub struct ChatFieldSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatFlagSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatImageSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatNameSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatObjectiveSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatOptionSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatParameterFieldSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatPersonaSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatProblemStatementSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatQuestionSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatScenarioSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
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
pub struct ChatVideoSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CohortDepartmentSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<CohortDepartment>>,
    #[serde(default)]
    pub resources: Option<Vec<CohortDepartment>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CohortDescriptionResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CohortDescriptionSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub resource: Option<CohortDescriptionResource>,
    #[serde(default)]
    pub resources: Option<Vec<CohortDescriptionResource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CohortDraftFormState {
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub flag_id: Option<String>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CohortFieldError {
    pub field: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CohortFlagConfig {
    #[serde(default)]
    pub key: Option<String>,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub icon_id: Option<String>,
    #[serde(default)]
    pub flag_option_id: Option<String>,
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub generated: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CohortFlagSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub resource: Option<CohortFlagConfig>,
    #[serde(default)]
    pub resources: Option<Vec<CohortFlagConfig>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CohortNameResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CohortNameSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub resource: Option<CohortNameResource>,
    #[serde(default)]
    pub resources: Option<Vec<CohortNameResource>>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CohortProfilePersonaSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<CohortProfilePersona>>,
    #[serde(default)]
    pub resources: Option<Vec<CohortProfilePersona>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CohortProfileSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<CohortProfile>>,
    #[serde(default)]
    pub resources: Option<Vec<CohortProfile>>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CohortSimulationAvailabilitySection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<CohortSimulationAvailability>>,
    #[serde(default)]
    pub resources: Option<Vec<CohortSimulationAvailability>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CohortSimulationPosition {
    #[serde(default)]
    pub simulation_id: Option<String>,
    #[serde(default)]
    pub value: Option<i64>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub mcp: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CohortSimulationPositionSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<CohortSimulationPosition>>,
    #[serde(default)]
    pub resources: Option<Vec<CohortSimulationPosition>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CohortSimulationSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<CohortSimulation>>,
    #[serde(default)]
    pub resources: Option<Vec<CohortSimulation>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnInfo {
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub nullable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComposedDocsResponseInput {
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub description: String,
    pub entries: Vec<DocsResponseInput>,
    pub resources: Vec<DocsResponseInput>,
    pub permissions: Vec<OperationInfo>,
    pub api_operations: Vec<OperationInfo>,
    #[serde(default)]
    pub artifact: Option<DocsResponseInput>,
    #[serde(default)]
    pub page_metadata: Option<DocsApiResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComposedDocsResponseOutput {
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub description: String,
    pub entries: Vec<DocsResponseOutput>,
    pub resources: Vec<DocsResponseOutput>,
    pub permissions: Vec<OperationInfo>,
    pub api_operations: Vec<OperationInfo>,
    #[serde(default)]
    pub artifact: Option<DocsResponseOutput>,
    #[serde(default)]
    pub page_metadata: Option<DocsApiResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectResponse {
    pub sid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionConfirmedPayload {
    pub sid: String,
    pub profile_id: Option<String>,
    pub guest_id: Option<String>,
    pub server_time: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentEntryOutput {
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAgentApiResponseInput {
    pub results: Vec<AgentResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAgentApiResponseOutput {
    pub results: Vec<AgentResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAgentItem {
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
    pub model_ids: Option<Vec<String>>,
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
pub struct CreateAuthApiRequest {
    pub auths: Vec<CreateAuthItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAuthApiResponseInput {
    pub results: Vec<AuthResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAuthApiResponseOutput {
    pub results: Vec<AuthResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAuthItem {
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
    pub active_flag_id: Option<String>,
    #[serde(default)]
    pub active_flag: Option<bool>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCohortApiResponseInput {
    pub results: Vec<CohortResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCohortApiResponseOutput {
    pub results: Vec<CohortResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCohortItem {
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
    pub flag_id: Option<String>,
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
    pub is_inactive: Option<bool>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDepartmentApiResponseInput {
    pub results: Vec<DepartmentResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDepartmentApiResponseOutput {
    pub results: Vec<DepartmentResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDepartmentItem {
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
    pub active_flag_id: Option<String>,
    #[serde(default)]
    pub active_flag: Option<bool>,
    #[serde(default)]
    pub settings_ids: Option<Vec<String>>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDocumentApiRequest {
    pub documents: Vec<CreateDocumentItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDocumentApiResponseInput {
    pub results: Vec<DocumentResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDocumentApiResponseOutput {
    pub results: Vec<DocumentResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDocumentItem {
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
    pub flag_id: Option<String>,
    #[serde(default)]
    pub is_inactive: Option<bool>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub field_ids: Option<Vec<String>>,
    #[serde(default)]
    pub upload_ids: Option<Vec<String>>,
    #[serde(default)]
    pub image_ids: Option<Vec<String>>,
    #[serde(default)]
    pub text_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEvalApiRequest {
    pub evals: Vec<CreateEvalItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEvalApiResponseInput {
    pub results: Vec<EvalResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEvalApiResponseOutput {
    pub results: Vec<EvalResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEvalItem {
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFieldApiRequest {
    pub fields: Vec<CreateFieldItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFieldApiResponseInput {
    pub results: Vec<FieldResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFieldApiResponseOutput {
    pub results: Vec<FieldResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFieldItem {
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
    pub flag_id: Option<String>,
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
pub struct CreateModelApiRequest {
    pub models: Vec<CreateModelItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateModelApiResponseInput {
    pub results: Vec<ModelResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateModelApiResponseOutput {
    pub results: Vec<ModelResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateModelItem {
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
    pub provider_ids: Option<Vec<String>>,
    #[serde(default)]
    pub quality_ids: Option<Vec<String>>,
    #[serde(default)]
    pub reasoning_level_ids: Option<Vec<String>>,
    #[serde(default)]
    pub temperature_level_ids: Option<Vec<String>>,
    #[serde(default)]
    pub value_ids: Option<Vec<String>>,
    #[serde(default)]
    pub voice_ids: Option<Vec<String>>,
    #[serde(default)]
    pub model_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateParameterApiRequest {
    pub parameters: Vec<CreateParameterItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateParameterApiResponseInput {
    pub results: Vec<ParameterResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateParameterApiResponseOutput {
    pub results: Vec<ParameterResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateParameterItem {
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
pub struct CreatePersonaApiRequest {
    pub personas: Vec<CreatePersonaItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePersonaApiResponseInput {
    pub results: Vec<PersonaResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePersonaApiResponseOutput {
    pub results: Vec<PersonaResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePersonaItem {
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
    pub active_flag_id: Option<String>,
    #[serde(default)]
    pub active_flag: Option<bool>,
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
pub struct CreateProblemRequest {
    #[serde(rename = "type")]
    pub r#type: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProblemResponse {
    pub problem_id: String,
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProfileApiRequest {
    pub profiles: Vec<CreateProfileItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProfileApiResponseInput {
    pub results: Vec<ProfileResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProfileApiResponseOutput {
    pub results: Vec<ProfileResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProfileItem {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub request_limit_id: Option<String>,
    #[serde(default)]
    pub flag_id: Option<String>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub email_ids: Option<Vec<String>>,
    #[serde(default)]
    pub role_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProviderApiRequest {
    pub providers: Vec<CreateProviderItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProviderApiResponseInput {
    pub results: Vec<ProviderResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProviderApiResponseOutput {
    pub results: Vec<ProviderResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProviderItem {
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
    pub active_flag_id: Option<String>,
    #[serde(default)]
    pub active_flag: Option<bool>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub endpoint_ids: Option<Vec<String>>,
    #[serde(default)]
    pub key_ids: Option<Vec<String>>,
    #[serde(default)]
    pub value_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRubricApiRequest {
    pub rubrics: Vec<CreateRubricItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRubricApiResponseInput {
    pub results: Vec<RubricResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRubricApiResponseOutput {
    pub results: Vec<RubricResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRubricItem {
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
    pub active_flag_id: Option<String>,
    #[serde(default)]
    pub active_flag: Option<bool>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub point_ids: Option<Vec<String>>,
    #[serde(default)]
    pub standard_group_ids: Option<Vec<String>>,
    #[serde(default)]
    pub standard_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateScenarioApiRequest {
    pub scenarios: Vec<CreateScenarioItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateScenarioApiResponseInput {
    pub results: Vec<ScenarioResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateScenarioApiResponseOutput {
    pub results: Vec<ScenarioResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateScenarioItem {
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
    pub active_flag_id: Option<String>,
    #[serde(default)]
    pub objectives_enabled_flag_id: Option<String>,
    #[serde(default)]
    pub images_enabled_flag_id: Option<String>,
    #[serde(default)]
    pub video_enabled_flag_id: Option<String>,
    #[serde(default)]
    pub questions_enabled_flag_id: Option<String>,
    #[serde(default)]
    pub problem_statement_enabled_flag_id: Option<String>,
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
    pub active_flag: Option<bool>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSettingApiResponseInput {
    pub results: Vec<SettingResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSettingApiResponseOutput {
    pub results: Vec<SettingResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSettingItem {
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
    pub profile_ids: Option<Vec<String>>,
    #[serde(default)]
    pub auth_ids: Option<Vec<String>>,
    #[serde(default)]
    pub provider_key_ids: Option<Vec<String>>,
    #[serde(default)]
    pub auth_item_key_ids: Option<Vec<String>>,
    #[serde(default)]
    pub auth_item_value_ids: Option<Vec<String>>,
    #[serde(default)]
    pub system_ids: Option<Vec<String>>,
    #[serde(default)]
    pub threshold_ids: Option<Vec<String>>,
    #[serde(default)]
    pub setting_resource_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSimulationApiRequest {
    pub simulations: Vec<CreateSimulationItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSimulationApiResponseInput {
    pub results: Vec<SimulationResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSimulationApiResponseOutput {
    pub results: Vec<SimulationResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSimulationItem {
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
    pub is_inactive: Option<bool>,
    #[serde(default)]
    pub is_practice: Option<bool>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub scenarios: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateToolApiRequest {
    pub tools: Vec<CreateToolItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateToolApiResponseInput {
    pub results: Vec<ToolResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateToolApiResponseOutput {
    pub results: Vec<ToolResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateToolItem {
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
    pub artifact_ids: Option<Vec<String>>,
    #[serde(default)]
    pub operation_ids: Option<Vec<String>>,
    #[serde(default)]
    pub tool_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardBundleResponseInput {
    #[serde(default)]
    pub header_metrics: Option<DashboardHeaderMetricsInput>,
    #[serde(default)]
    pub primary_metrics: Option<DashboardPrimaryMetricsInput>,
    #[serde(default)]
    pub secondary_metrics: Option<DashboardSecondaryMetricsInput>,
    #[serde(default)]
    pub footer_metrics: Option<DashboardFooterMetricsInput>,
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
    pub analytics: Option<AnalyticsFacetsInput>,
    #[serde(default)]
    pub history: Option<HistoryResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardBundleResponseOutput {
    #[serde(default)]
    pub header_metrics: Option<DashboardHeaderMetricsOutput>,
    #[serde(default)]
    pub primary_metrics: Option<DashboardPrimaryMetricsOutput>,
    #[serde(default)]
    pub secondary_metrics: Option<DashboardSecondaryMetricsOutput>,
    #[serde(default)]
    pub footer_metrics: Option<DashboardFooterMetricsOutput>,
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
    pub analytics: Option<AnalyticsFacetsOutput>,
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
pub struct DashboardFooterMetricsInput {
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
pub struct DashboardFooterMetricsOutput {
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
pub struct DashboardHeaderMetricsInput {
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
pub struct DashboardHeaderMetricsOutput {
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
pub struct DashboardPrimaryMetricsInput {
    #[serde(default)]
    pub rubric_heatmap: Option<PrimaryRubricHeatmapInput>,
    #[serde(default)]
    pub rubric_trend: Option<PrimaryRubricTrend>,
    #[serde(default)]
    pub skill_performance: Option<SecondarySkillPerformanceInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardPrimaryMetricsOutput {
    #[serde(default)]
    pub rubric_heatmap: Option<PrimaryRubricHeatmapOutput>,
    #[serde(default)]
    pub rubric_trend: Option<PrimaryRubricTrend>,
    #[serde(default)]
    pub skill_performance: Option<SecondarySkillPerformanceOutput>,
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
    pub roles: Option<Vec<String>>,
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
    #[serde(default)]
    pub history_practice: Option<bool>,
    #[serde(default)]
    pub history_scenario_ids: Option<Vec<String>>,
    #[serde(default)]
    pub history_infinite_mode: Option<bool>,
    #[serde(default)]
    pub history_show_archived: Option<bool>,
    #[serde(default)]
    pub history_sort_by: Option<String>,
    #[serde(default)]
    pub history_sort_order: Option<String>,
    #[serde(default)]
    pub history_page: Option<i64>,
    #[serde(default)]
    pub history_page_size: Option<i64>,
    #[serde(default)]
    pub history_simulation_search: Option<String>,
    #[serde(default)]
    pub history_scenario_search: Option<String>,
    #[serde(default)]
    pub history_profile_search: Option<String>,
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
pub struct DashboardSecondaryMetricsInput {
    #[serde(default)]
    pub persona_performance: Option<PrimaryPersonaPerformanceInput>,
    #[serde(default)]
    pub cohort_performance: Option<SecondaryCohortPerformance>,
    #[serde(default)]
    pub attempt_improvement: Option<SecondaryAttemptImprovement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardSecondaryMetricsOutput {
    #[serde(default)]
    pub persona_performance: Option<PrimaryPersonaPerformanceOutput>,
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
    pub agent_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteAgentApiResponse {
    pub results: Vec<DeleteAgentResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteAgentResult {
    pub success: bool,
    pub agent_id: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteAuthApiRequest {
    pub auth_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteAuthApiResponse {
    pub results: Vec<DeleteAuthResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteAuthResult {
    pub success: bool,
    pub auth_id: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteCohortApiRequest {
    pub cohort_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteCohortApiResponse {
    pub results: Vec<DeleteCohortResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteCohortResult {
    pub success: bool,
    pub cohort_id: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteDepartmentApiRequest {
    pub department_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteDepartmentApiResponse {
    pub results: Vec<DeleteDepartmentResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteDepartmentResult {
    pub success: bool,
    pub department_id: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteDocumentApiRequest {
    pub document_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteDocumentApiResponse {
    pub results: Vec<DeleteDocumentResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteDocumentResult {
    pub success: bool,
    pub document_id: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteEvalApiRequest {
    pub eval_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteEvalApiResponse {
    pub results: Vec<DeleteEvalResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteEvalResult {
    pub success: bool,
    pub eval_id: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteFieldApiRequest {
    pub field_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteFieldApiResponse {
    pub results: Vec<DeleteFieldResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteFieldResult {
    pub success: bool,
    pub field_id: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteModelApiRequest {
    pub model_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteModelApiResponse {
    pub results: Vec<DeleteModelResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteModelResult {
    pub success: bool,
    pub model_id: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteParameterApiRequest {
    pub parameter_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteParameterApiResponse {
    pub results: Vec<DeleteParameterResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteParameterResult {
    pub success: bool,
    pub parameter_id: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletePersonaApiRequest {
    pub persona_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletePersonaApiResponse {
    pub results: Vec<DeletePersonaResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletePersonaResult {
    pub success: bool,
    pub persona_id: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteProfileApiRequest {
    pub profile_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteProfileApiResponse {
    pub results: Vec<DeleteProfileResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteProfileResult {
    pub success: bool,
    pub profile_id: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteProviderApiRequest {
    pub provider_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteProviderApiResponse {
    pub results: Vec<DeleteProviderResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteProviderResult {
    pub success: bool,
    pub provider_id: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteRubricApiRequest {
    pub rubric_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteRubricApiResponse {
    pub results: Vec<DeleteRubricResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteRubricResult {
    pub success: bool,
    pub rubric_id: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteScenarioApiRequest {
    pub scenario_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteScenarioApiResponse {
    pub results: Vec<DeleteScenarioResult>,
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
    pub setting_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSettingApiResponse {
    pub results: Vec<DeleteSettingResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSettingResult {
    pub success: bool,
    pub setting_id: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSimulationApiRequest {
    pub simulation_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSimulationApiResponse {
    pub results: Vec<DeleteSimulationResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSimulationResult {
    pub success: bool,
    pub simulation_id: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteToolApiRequest {
    pub tool_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteToolApiResponse {
    pub results: Vec<DeleteToolResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteToolResult {
    pub success: bool,
    pub tool_id: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepartmentDescriptionSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub resource: Option<serde_json::Value>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepartmentDraftFormState {
    pub setting_ids: Vec<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub flag_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepartmentFieldError {
    pub field: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepartmentFlagConfig {
    pub key: String,
    pub label: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub icon_id: Option<String>,
    #[serde(default)]
    pub flag_option_id: Option<String>,
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub generated: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepartmentFlagSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<DepartmentFlagConfig>>,
    #[serde(default)]
    pub resources: Option<Vec<DepartmentFlagConfig>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepartmentNameSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub resource: Option<serde_json::Value>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
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
pub struct DepartmentSettingSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisconnectRequest {
    pub sid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisconnectResponse {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocsApiRequest {
    #[serde(default)]
    pub entity_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocsApiResponse {
    pub list: PageMetaItem,
    pub detail: PageMetaItem,
    pub new: PageMetaItem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocsResponseInput {
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
pub struct DocsResponseOutput {
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
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentDepartmentSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<DocumentDepartmentResource>>,
    #[serde(default)]
    pub resources: Option<Vec<DocumentDepartmentResource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentDescriptionResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentDescriptionSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub resource: Option<DocumentDescriptionResource>,
    #[serde(default)]
    pub resources: Option<Vec<DocumentDescriptionResource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentDraftFormState {
    pub flag_ids: Vec<String>,
    pub department_ids: Vec<String>,
    pub file_ids: Vec<String>,
    pub image_ids: Vec<String>,
    pub text_ids: Vec<String>,
    pub parameter_field_ids: Vec<String>,
    pub parameter_ids: Vec<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentEntry {
    #[serde(default)]
    pub document_id: Option<String>,
    #[serde(default)]
    pub upload_id: Option<String>,
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
pub struct DocumentFieldSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<DocumentParameterFieldResource>>,
    #[serde(default)]
    pub resources: Option<Vec<DocumentParameterFieldResource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentFileResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentFlagConfig {
    pub key: String,
    pub label: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub flag_option_id: Option<String>,
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub generated: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentFlagSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<DocumentFlagConfig>>,
    #[serde(default)]
    pub resources: Option<Vec<DocumentFlagConfig>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentImageResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentImageSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<DocumentImageResource>>,
    #[serde(default)]
    pub resources: Option<Vec<DocumentImageResource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentNameResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentNameSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub resource: Option<DocumentNameResource>,
    #[serde(default)]
    pub resources: Option<Vec<DocumentNameResource>>,
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
    pub generated: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentParameterSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<GetParameterResponse>>,
    #[serde(default)]
    pub resources: Option<Vec<GetParameterResponse>>,
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
    pub generated: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentTextSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<DocumentTextResource>>,
    #[serde(default)]
    pub resources: Option<Vec<DocumentTextResource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentUploadSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<DocumentFileResource>>,
    #[serde(default)]
    pub resources: Option<Vec<DocumentFileResource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DraftFileValue {
    pub upload_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DraftFormState {
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub instructions_id: Option<String>,
    #[serde(default)]
    pub color_id: Option<String>,
    #[serde(default)]
    pub icon_id: Option<String>,
    #[serde(default)]
    pub active_flag_id: Option<String>,
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
pub struct DraftProfilePersonaValue {
    pub profile_id: String,
    pub persona_id: String,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateAgentApiResponse {
    pub success: bool,
    pub agent_id: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateAuthApiRequest {
    pub auth_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateAuthApiResponse {
    pub success: bool,
    pub auth_id: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateCohortApiRequest {
    pub cohort_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateCohortApiResponse {
    pub success: bool,
    pub cohort_id: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateDepartmentApiRequest {
    pub department_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateDepartmentApiResponse {
    pub success: bool,
    pub department_id: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateDocumentApiRequest {
    pub document_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateDocumentApiResponse {
    pub success: bool,
    pub document_id: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateEvalApiRequest {
    pub eval_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateEvalApiResponse {
    pub success: bool,
    pub eval_id: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateFieldApiRequest {
    pub field_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateFieldApiResponse {
    pub success: bool,
    pub field_id: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateModelApiRequest {
    pub model_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateModelApiResponse {
    pub success: bool,
    pub model_id: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateParameterApiRequest {
    pub parameter_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateParameterApiResponse {
    pub success: bool,
    pub parameter_id: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicatePersonaApiRequest {
    pub persona_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicatePersonaApiResponse {
    pub success: bool,
    pub persona_id: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateProfileApiRequest {
    pub target_profile_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateProfileApiResponse {
    pub success: bool,
    pub profile_id: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateProviderApiRequest {
    pub provider_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateProviderApiResponse {
    pub success: bool,
    pub provider_id: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateRubricApiRequest {
    pub rubric_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateRubricApiResponse {
    pub success: bool,
    pub rubric_id: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateScenarioApiRequest {
    pub scenario_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateScenarioApiResponse {
    pub success: bool,
    pub scenario_id: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateSettingApiRequest {
    pub setting_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateSettingApiResponse {
    pub success: bool,
    pub setting_id: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateSimulationApiRequest {
    pub simulation_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateSimulationApiResponse {
    pub success: bool,
    pub simulation_id: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateToolApiRequest {
    pub tool_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateToolApiResponse {
    pub success: bool,
    pub tool_id: String,
    pub message: String,
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
pub struct EndAllAttemptApiResponse {
    pub attempt_id: String,
    pub success: bool,
    #[serde(default)]
    pub all_scenarios_complete: Option<bool>,
    #[serde(default)]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndAttemptApiRequest {
    pub attempt_id: String,
    pub chat_id: String,
    #[serde(default)]
    pub grade: Option<bool>,
    #[serde(default)]
    pub score: Option<i64>,
    #[serde(default)]
    pub passed: Option<bool>,
    #[serde(default)]
    pub time_taken: Option<i64>,
    #[serde(default)]
    pub feedbacks: Option<Vec<AttemptGradeFeedbackEntry>>,
    #[serde(default)]
    pub strengths: Option<Vec<AttemptGradeStrengthEntry>>,
    #[serde(default)]
    pub improvements: Option<Vec<AttemptGradeImprovementEntry>>,
    #[serde(default)]
    pub analyses: Option<Vec<AttemptGradeAnalysisEntry>>,
    #[serde(default)]
    pub highlights: Option<Vec<AttemptGradeHighlightEntry>>,
    #[serde(default)]
    pub replacements: Option<Vec<AttemptGradeReplacementEntry>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndAttemptApiResponse {
    pub chat_id: String,
    #[serde(default)]
    pub is_attempt_finished: Option<bool>,
    #[serde(default)]
    pub grade_id: Option<String>,
    #[serde(default)]
    pub score: Option<i64>,
    #[serde(default)]
    pub passed: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndTestApiRequest {
    pub test_id: String,
    pub test_invocation_id: String,
    pub run_id: String,
    #[serde(default)]
    pub grade: Option<bool>,
    #[serde(default)]
    pub score: Option<f64>,
    #[serde(default)]
    pub passed: Option<bool>,
    #[serde(default)]
    pub feedback: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndTestApiResponse {
    pub invocation_id: String,
    #[serde(default)]
    pub grade_id: Option<String>,
    #[serde(default)]
    pub score: Option<f64>,
    #[serde(default)]
    pub passed: Option<bool>,
    #[serde(default)]
    pub feedback: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntryTypeItem {
    pub name: String,
    pub operation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalDepartmentSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalDescriptionSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub resource: Option<serde_json::Value>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalDraftFormState {
    pub flag_ids: Vec<String>,
    pub department_ids: Vec<String>,
    pub model_ids: Vec<String>,
    pub rubric_ids: Vec<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalFieldError {
    pub field: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalFlagConfig {
    pub key: String,
    pub label: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub icon_id: Option<String>,
    #[serde(default)]
    pub flag_option_id: Option<String>,
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub generated: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalFlagSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub resource: Option<EvalFlagConfig>,
    #[serde(default)]
    pub resources: Option<Vec<EvalFlagConfig>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalModelFlagSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalModelPositionSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalModelRubricSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalModelSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalNameSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub resource: Option<serde_json::Value>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
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
pub struct ExportActivityApiResponse {
    pub content: String,
    pub file_name: String,
    pub mime_type: String,
    pub row_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportAgentApiRequest {
    #[serde(default)]
    pub agent_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportAgentApiResponse {
    pub content: String,
    pub file_name: String,
    pub mime_type: String,
    pub row_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportAttemptApiRequest {
    pub attempt_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportAttemptApiResponse {
    pub content: String,
    pub file_name: String,
    pub mime_type: String,
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
pub struct ExportBenchmarkApiResponse {
    pub content: String,
    pub file_name: String,
    pub mime_type: String,
    pub row_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportChatApiRequest {
    pub chat_entry_id: String,
    #[serde(default)]
    pub attempt_id: Option<String>,
    #[serde(default)]
    pub draft_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportChatApiResponse {
    pub content: String,
    pub file_name: String,
    pub mime_type: String,
    pub row_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportCohortApiResponse {
    pub content: String,
    pub file_name: String,
    pub mime_type: String,
    pub row_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportDashboardApiResponse {
    pub content: String,
    pub file_name: String,
    pub mime_type: String,
    pub row_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportDepartmentApiRequest {
    #[serde(default)]
    pub department_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportDepartmentApiResponse {
    pub content: String,
    pub file_name: String,
    pub mime_type: String,
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
    pub content: String,
    pub file_name: String,
    pub mime_type: String,
    pub row_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportFieldApiRequest {
    #[serde(default)]
    pub field_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportFieldApiResponse {
    pub content: String,
    pub file_name: String,
    pub mime_type: String,
    pub row_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportGroupApiRequest {
    pub group_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportGroupApiResponse {
    pub content: String,
    pub file_name: String,
    pub mime_type: String,
    pub row_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportHealthApiResponse {
    pub content: String,
    pub file_name: String,
    pub mime_type: String,
    pub row_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportHomeApiResponse {
    pub content: String,
    pub file_name: String,
    pub mime_type: String,
    pub row_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportLeaderboardApiResponse {
    pub content: String,
    pub file_name: String,
    pub mime_type: String,
    pub row_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportModelApiRequest {
    #[serde(default)]
    pub model_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportModelApiResponse {
    pub content: String,
    pub file_name: String,
    pub mime_type: String,
    pub row_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportParameterApiRequest {
    #[serde(default)]
    pub parameter_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportParameterApiResponse {
    pub content: String,
    pub file_name: String,
    pub mime_type: String,
    pub row_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportPersonaApiRequest {
    #[serde(default)]
    pub persona_id: Option<String>,
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
    pub content: String,
    pub file_name: String,
    pub mime_type: String,
    pub row_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportPracticeApiResponse {
    pub content: String,
    pub file_name: String,
    pub mime_type: String,
    pub row_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportPricingApiResponse {
    pub content: String,
    pub file_name: String,
    pub mime_type: String,
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
    pub content: String,
    pub file_name: String,
    pub mime_type: String,
    pub row_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportRecordApiRequest {
    pub target_profile_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportRecordApiResponse {
    pub content: String,
    pub file_name: String,
    pub mime_type: String,
    pub row_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportReportsApiResponse {
    pub content: String,
    pub file_name: String,
    pub mime_type: String,
    pub row_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportRubricApiRequest {
    #[serde(default)]
    pub rubric_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportRubricApiResponse {
    pub content: String,
    pub file_name: String,
    pub mime_type: String,
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
pub struct ExportSessionApiRequest {
    pub target_session_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportSessionApiResponse {
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
    pub content: String,
    pub file_name: String,
    pub mime_type: String,
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
    pub content: String,
    pub file_name: String,
    pub mime_type: String,
    pub row_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportTestApiRequest {
    pub test_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportTestApiResponse {
    pub content: String,
    pub file_name: String,
    pub mime_type: String,
    pub row_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportToolApiRequest {
    #[serde(default)]
    pub tool_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportToolApiResponse {
    pub content: String,
    pub file_name: String,
    pub mime_type: String,
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
pub struct FieldConditionalParameterSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<GetParameterResponse>>,
    #[serde(default)]
    pub resources: Option<Vec<GetParameterResponse>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldDepartmentSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldDescriptionSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub resource: Option<serde_json::Value>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldDraftFormState {
    pub department_ids: Vec<String>,
    pub conditional_parameter_ids: Vec<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub flag_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldFieldError {
    pub field: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldFlagConfig {
    pub key: String,
    pub label: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub icon_id: Option<String>,
    #[serde(default)]
    pub flag_option_id: Option<String>,
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub generated: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldFlagSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub resource: Option<FieldFlagConfig>,
    #[serde(default)]
    pub resources: Option<Vec<FieldFlagConfig>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldNameSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub resource: Option<serde_json::Value>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
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
pub struct GenerateApiResponse {
    pub group_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratePayload {
    pub artifact_types: Vec<ArtifactTypeItem>,
    pub resource_types: Vec<ResourceTypeItem>,
    #[serde(default)]
    pub artifact_id: Option<serde_json::Value>,
    #[serde(default)]
    pub draft_id: Option<serde_json::Value>,
    #[serde(default)]
    pub entry_types: Option<Vec<EntryTypeItem>>,
    #[serde(default)]
    pub user_instructions: Option<Vec<String>>,
    #[serde(default)]
    pub save: Option<bool>,
    #[serde(default)]
    pub run_id: Option<String>,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub modality: Option<String>,
    #[serde(default)]
    pub extra_messages: Option<Vec<std::collections::HashMap<String, String>>>,
    #[serde(default)]
    pub metadata: Option<std::collections::HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationCompleteEvent {
    pub artifact_type: String,
    pub group_id: String,
    pub run_id: String,
    #[serde(default)]
    pub success: Option<bool>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub artifact_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationErrorEvent {
    pub artifact_type: String,
    pub message: String,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub resource_type: Option<String>,
    #[serde(default)]
    pub resource_types: Option<Vec<String>>,
    #[serde(default)]
    pub resource_id: Option<String>,
    #[serde(default)]
    pub run_id: Option<String>,
    #[serde(default)]
    pub success: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationMediaCompleteEvent {
    pub modality: String,
    pub artifact_type: String,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub run_id: Option<String>,
    #[serde(default)]
    pub resource_type: Option<String>,
    #[serde(default)]
    pub resource_id: Option<String>,
    #[serde(default)]
    pub file_path: Option<String>,
    #[serde(default)]
    pub mime_type: Option<String>,
    #[serde(default)]
    pub file_size: Option<i64>,
    #[serde(default)]
    pub upload_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationMediaProgressEvent {
    pub modality: String,
    pub artifact_type: String,
    pub status: String,
    pub message: String,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub run_id: Option<String>,
    #[serde(default)]
    pub resource_type: Option<String>,
    #[serde(default)]
    pub resource_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationProgressEvent {
    pub artifact_type: String,
    pub group_id: String,
    pub run_id: String,
    pub completed_resources: i64,
    pub total_resources: i64,
    pub percentage: i64,
    pub last_completed_resource: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationSavedEvent {
    pub artifact_type: String,
    pub group_id: String,
    pub run_id: String,
    #[serde(default)]
    pub artifact_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAgentApiRequest {
    #[serde(default)]
    pub agent_id: Option<String>,
    #[serde(default)]
    pub draft_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAgentApiResponseInput {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub agent_exists: Option<bool>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub disabled_reason: Option<String>,
    #[serde(default)]
    pub draft_version: Option<i64>,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub basic_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub general_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub names: Option<AgentNameSection>,
    #[serde(default)]
    pub descriptions: Option<AgentDescriptionSection>,
    #[serde(default)]
    pub models: Option<AgentModelSection>,
    #[serde(default)]
    pub prompts: Option<AgentPromptSection>,
    #[serde(default)]
    pub instructions: Option<AgentInstructionSection>,
    #[serde(default)]
    pub flags: Option<AgentFlagSection>,
    #[serde(default)]
    pub departments: Option<AgentDepartmentSection>,
    #[serde(default)]
    pub tools: Option<AgentToolSection>,
    #[serde(default)]
    pub temperature_levels: Option<AgentTemperatureLevelSection>,
    #[serde(default)]
    pub reasoning_levels: Option<AgentReasoningLevelSection>,
    #[serde(default)]
    pub voices: Option<AgentVoiceSection>,
    #[serde(default)]
    pub qualities: Option<AgentQualitySection>,
    #[serde(default)]
    pub rubrics: Option<AgentRubricSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAgentApiResponseOutput {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub agent_exists: Option<bool>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub disabled_reason: Option<String>,
    #[serde(default)]
    pub draft_version: Option<i64>,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub basic_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub general_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub names: Option<AgentNameSection>,
    #[serde(default)]
    pub descriptions: Option<AgentDescriptionSection>,
    #[serde(default)]
    pub models: Option<AgentModelSection>,
    #[serde(default)]
    pub prompts: Option<AgentPromptSection>,
    #[serde(default)]
    pub instructions: Option<AgentInstructionSection>,
    #[serde(default)]
    pub flags: Option<AgentFlagSection>,
    #[serde(default)]
    pub departments: Option<AgentDepartmentSection>,
    #[serde(default)]
    pub tools: Option<AgentToolSection>,
    #[serde(default)]
    pub temperature_levels: Option<AgentTemperatureLevelSection>,
    #[serde(default)]
    pub reasoning_levels: Option<AgentReasoningLevelSection>,
    #[serde(default)]
    pub voices: Option<AgentVoiceSection>,
    #[serde(default)]
    pub qualities: Option<AgentQualitySection>,
    #[serde(default)]
    pub rubrics: Option<AgentRubricSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAgentDraftResponse {
    pub id: String,
    pub version: i64,
    pub created_at: String,
    pub generated: bool,
    pub mcp: bool,
    pub active: bool,
    pub group_id: String,
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
    pub rubric_ids: Option<Vec<String>>,
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
pub struct GetAttemptDetailResponseInput {
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
    pub available_continuation_options: Option<AvailableContinuationOptionsInput>,
    #[serde(default)]
    pub rubric_structure: Option<RubricStructureData>,
    #[serde(default)]
    pub training_id: Option<String>,
    #[serde(default)]
    pub chat_entry_id: Option<String>,
    #[serde(default)]
    pub resources: Option<AttemptResourcesInput>,
    #[serde(default)]
    pub entries: Option<AttemptEntriesInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAttemptDetailResponseOutput {
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
    pub available_continuation_options: Option<AvailableContinuationOptionsOutput>,
    #[serde(default)]
    pub rubric_structure: Option<RubricStructureData>,
    #[serde(default)]
    pub training_id: Option<String>,
    #[serde(default)]
    pub chat_entry_id: Option<String>,
    #[serde(default)]
    pub resources: Option<AttemptResourcesOutput>,
    #[serde(default)]
    pub entries: Option<AttemptEntriesOutput>,
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
    pub is_archived: bool,
    pub scenario_ids: Vec<String>,
    pub chat_entry_id: Option<String>,
    pub attempt_chat_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAuthApiRequest {
    #[serde(default)]
    pub auth_id: Option<String>,
    #[serde(default)]
    pub draft_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAuthApiResponseInput {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub auth_exists: Option<bool>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub disabled_reason: Option<String>,
    #[serde(default)]
    pub draft_version: Option<i64>,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub basic_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub names: Option<AuthNameSection>,
    #[serde(default)]
    pub descriptions: Option<AuthDescriptionSection>,
    #[serde(default)]
    pub flags: Option<AuthFlagSection>,
    #[serde(default)]
    pub protocols: Option<AuthProtocolSection>,
    #[serde(default)]
    pub slugs: Option<AuthSlugSection>,
    #[serde(default)]
    pub items: Option<AuthItemSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAuthApiResponseOutput {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub auth_exists: Option<bool>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub disabled_reason: Option<String>,
    #[serde(default)]
    pub draft_version: Option<i64>,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub basic_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub names: Option<AuthNameSection>,
    #[serde(default)]
    pub descriptions: Option<AuthDescriptionSection>,
    #[serde(default)]
    pub flags: Option<AuthFlagSection>,
    #[serde(default)]
    pub protocols: Option<AuthProtocolSection>,
    #[serde(default)]
    pub slugs: Option<AuthSlugSection>,
    #[serde(default)]
    pub items: Option<AuthItemSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAuthDraftResponse {
    pub id: String,
    pub version: i64,
    pub created_at: String,
    pub generated: bool,
    pub mcp: bool,
    pub active: bool,
    pub group_id: String,
    pub session_id: String,
    pub department_ids: Vec<String>,
    pub description_ids: Vec<String>,
    pub flag_ids: Vec<String>,
    pub item_ids: Vec<String>,
    pub name_ids: Vec<String>,
    pub profile_ids: Vec<String>,
    pub protocol_ids: Vec<String>,
    pub slug_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAuthDraftsApiResponse {
    #[serde(default)]
    pub entries: Option<Vec<GetAuthDraftResponse>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetChatDraftResponse {
    pub id: String,
    pub version: i64,
    pub created_at: String,
    pub generated: bool,
    pub mcp: bool,
    pub active: bool,
    pub group_id: String,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetChatDraftsApiResponse {
    #[serde(default)]
    pub entries: Option<Vec<GetChatDraftResponse>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetChatRequest {
    pub chat_entry_id: String,
    #[serde(default)]
    pub attempt_id: Option<String>,
    #[serde(default)]
    pub draft_id: Option<String>,
    #[serde(default)]
    pub description_search: Option<String>,
    #[serde(default)]
    pub persona_search: Option<String>,
    #[serde(default)]
    pub document_search: Option<String>,
    #[serde(default)]
    pub problem_statement_search: Option<String>,
    #[serde(default)]
    pub image_search: Option<String>,
    #[serde(default)]
    pub video_search: Option<String>,
    #[serde(default)]
    pub question_search: Option<String>,
    #[serde(default)]
    pub option_search: Option<String>,
    #[serde(default)]
    pub persona_show_selected: Option<bool>,
    #[serde(default)]
    pub document_show_selected: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetChatResponse {
    pub chat_entry_id: String,
    pub group_id: String,
    #[serde(default)]
    pub attempt_id: Option<String>,
    #[serde(default)]
    pub draft_version: Option<i64>,
    #[serde(default)]
    pub names: Option<ChatNameSection>,
    #[serde(default)]
    pub descriptions: Option<ChatDescriptionSection>,
    #[serde(default)]
    pub flags: Option<ChatFlagSection>,
    #[serde(default)]
    pub departments: Option<ChatDepartmentSection>,
    #[serde(default)]
    pub personas: Option<ChatPersonaSection>,
    #[serde(default)]
    pub documents: Option<ChatDocumentSection>,
    #[serde(default)]
    pub parameter_fields: Option<ChatParameterFieldSection>,
    #[serde(default)]
    pub scenarios: Option<ChatScenarioSection>,
    #[serde(default)]
    pub fields: Option<ChatFieldSection>,
    #[serde(default)]
    pub questions: Option<ChatQuestionSection>,
    #[serde(default)]
    pub options: Option<ChatOptionSection>,
    #[serde(default)]
    pub videos: Option<ChatVideoSection>,
    #[serde(default)]
    pub images: Option<ChatImageSection>,
    #[serde(default)]
    pub problem_statements: Option<ChatProblemStatementSection>,
    #[serde(default)]
    pub objectives: Option<ChatObjectiveSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCohortApiRequest {
    #[serde(default)]
    pub cohort_id: Option<String>,
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
    #[serde(default)]
    pub draft_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCohortApiResponseInput {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub cohort_exists: Option<bool>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub disabled_reason: Option<String>,
    #[serde(default)]
    pub draft_version: Option<i64>,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub basic_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub simulations_step_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub profiles_step_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub names: Option<CohortNameSection>,
    #[serde(default)]
    pub descriptions: Option<CohortDescriptionSection>,
    #[serde(default)]
    pub flags: Option<CohortFlagSection>,
    #[serde(default)]
    pub departments: Option<CohortDepartmentSection>,
    #[serde(default)]
    pub simulations: Option<CohortSimulationSection>,
    #[serde(default)]
    pub simulation_positions: Option<CohortSimulationPositionSection>,
    #[serde(default)]
    pub simulation_availability: Option<CohortSimulationAvailabilitySection>,
    #[serde(default)]
    pub profiles: Option<CohortProfileSection>,
    #[serde(default)]
    pub profile_personas: Option<CohortProfilePersonaSection>,
    #[serde(default)]
    pub personas: Option<Vec<GetPersonaResponse>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCohortApiResponseOutput {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub cohort_exists: Option<bool>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub disabled_reason: Option<String>,
    #[serde(default)]
    pub draft_version: Option<i64>,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub basic_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub simulations_step_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub profiles_step_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub names: Option<CohortNameSection>,
    #[serde(default)]
    pub descriptions: Option<CohortDescriptionSection>,
    #[serde(default)]
    pub flags: Option<CohortFlagSection>,
    #[serde(default)]
    pub departments: Option<CohortDepartmentSection>,
    #[serde(default)]
    pub simulations: Option<CohortSimulationSection>,
    #[serde(default)]
    pub simulation_positions: Option<CohortSimulationPositionSection>,
    #[serde(default)]
    pub simulation_availability: Option<CohortSimulationAvailabilitySection>,
    #[serde(default)]
    pub profiles: Option<CohortProfileSection>,
    #[serde(default)]
    pub profile_personas: Option<CohortProfilePersonaSection>,
    #[serde(default)]
    pub personas: Option<Vec<GetPersonaResponse>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCohortDraftResponse {
    pub id: String,
    pub version: i64,
    pub created_at: String,
    pub generated: bool,
    pub mcp: bool,
    pub active: bool,
    pub group_id: String,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCohortDraftsApiResponse {
    #[serde(default)]
    pub entries: Option<Vec<GetCohortDraftResponse>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDepartmentApiRequest {
    #[serde(default)]
    pub department_id: Option<String>,
    #[serde(default)]
    pub draft_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDepartmentApiResponseInput {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub department_exists: Option<bool>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub disabled_reason: Option<String>,
    #[serde(default)]
    pub draft_version: Option<i64>,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub basic_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub names: Option<DepartmentNameSection>,
    #[serde(default)]
    pub descriptions: Option<DepartmentDescriptionSection>,
    #[serde(default)]
    pub flags: Option<DepartmentFlagSection>,
    #[serde(default)]
    pub settings: Option<DepartmentSettingSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDepartmentApiResponseOutput {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub department_exists: Option<bool>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub disabled_reason: Option<String>,
    #[serde(default)]
    pub draft_version: Option<i64>,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub basic_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub names: Option<DepartmentNameSection>,
    #[serde(default)]
    pub descriptions: Option<DepartmentDescriptionSection>,
    #[serde(default)]
    pub flags: Option<DepartmentFlagSection>,
    #[serde(default)]
    pub settings: Option<DepartmentSettingSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDepartmentDraftResponse {
    pub id: String,
    pub version: i64,
    pub created_at: String,
    pub generated: bool,
    pub mcp: bool,
    pub active: bool,
    pub group_id: String,
    pub session_id: String,
    pub description_ids: Vec<String>,
    pub flag_ids: Vec<String>,
    pub name_ids: Vec<String>,
    pub profile_ids: Vec<String>,
    pub setting_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDepartmentDraftsApiResponse {
    #[serde(default)]
    pub entries: Option<Vec<GetDepartmentDraftResponse>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocumentApiRequest {
    #[serde(default)]
    pub document_id: Option<String>,
    #[serde(default)]
    pub draft_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocumentApiResponseInput {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub document_exists: Option<bool>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub disabled_reason: Option<String>,
    #[serde(default)]
    pub draft_version: Option<i64>,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub basic_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub content_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub names: Option<DocumentNameSection>,
    #[serde(default)]
    pub descriptions: Option<DocumentDescriptionSection>,
    #[serde(default)]
    pub flags: Option<DocumentFlagSection>,
    #[serde(default)]
    pub departments: Option<DocumentDepartmentSection>,
    #[serde(default)]
    pub fields: Option<DocumentFieldSection>,
    #[serde(default)]
    pub parameters: Option<DocumentParameterSection>,
    #[serde(default)]
    pub uploads: Option<DocumentUploadSection>,
    #[serde(default)]
    pub images: Option<DocumentImageSection>,
    #[serde(default)]
    pub texts: Option<DocumentTextSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocumentApiResponseOutput {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub document_exists: Option<bool>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub disabled_reason: Option<String>,
    #[serde(default)]
    pub draft_version: Option<i64>,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub basic_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub content_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub names: Option<DocumentNameSection>,
    #[serde(default)]
    pub descriptions: Option<DocumentDescriptionSection>,
    #[serde(default)]
    pub flags: Option<DocumentFlagSection>,
    #[serde(default)]
    pub departments: Option<DocumentDepartmentSection>,
    #[serde(default)]
    pub fields: Option<DocumentFieldSection>,
    #[serde(default)]
    pub parameters: Option<DocumentParameterSection>,
    #[serde(default)]
    pub uploads: Option<DocumentUploadSection>,
    #[serde(default)]
    pub images: Option<DocumentImageSection>,
    #[serde(default)]
    pub texts: Option<DocumentTextSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocumentDraftResponse {
    pub id: String,
    pub version: i64,
    pub created_at: String,
    pub generated: bool,
    pub mcp: bool,
    pub active: bool,
    pub group_id: String,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocumentDraftsApiResponse {
    #[serde(default)]
    pub entries: Option<Vec<GetDocumentDraftResponse>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetEvalApiRequest {
    #[serde(default)]
    pub eval_id: Option<String>,
    #[serde(default)]
    pub draft_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetEvalApiResponseInput {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub eval_exists: Option<bool>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub disabled_reason: Option<String>,
    #[serde(default)]
    pub draft_version: Option<i64>,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub basic_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub model_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub names: Option<EvalNameSection>,
    #[serde(default)]
    pub descriptions: Option<EvalDescriptionSection>,
    #[serde(default)]
    pub active_flags: Option<EvalFlagSection>,
    #[serde(default)]
    pub dynamic_flags: Option<EvalFlagSection>,
    #[serde(default)]
    pub groups_flags: Option<EvalFlagSection>,
    #[serde(default)]
    pub departments: Option<EvalDepartmentSection>,
    #[serde(default)]
    pub models: Option<EvalModelSection>,
    #[serde(default)]
    pub model_flags: Option<EvalModelFlagSection>,
    #[serde(default)]
    pub model_rubrics: Option<EvalModelRubricSection>,
    #[serde(default)]
    pub model_positions: Option<EvalModelPositionSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetEvalApiResponseOutput {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub eval_exists: Option<bool>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub disabled_reason: Option<String>,
    #[serde(default)]
    pub draft_version: Option<i64>,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub basic_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub model_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub names: Option<EvalNameSection>,
    #[serde(default)]
    pub descriptions: Option<EvalDescriptionSection>,
    #[serde(default)]
    pub active_flags: Option<EvalFlagSection>,
    #[serde(default)]
    pub dynamic_flags: Option<EvalFlagSection>,
    #[serde(default)]
    pub groups_flags: Option<EvalFlagSection>,
    #[serde(default)]
    pub departments: Option<EvalDepartmentSection>,
    #[serde(default)]
    pub models: Option<EvalModelSection>,
    #[serde(default)]
    pub model_flags: Option<EvalModelFlagSection>,
    #[serde(default)]
    pub model_rubrics: Option<EvalModelRubricSection>,
    #[serde(default)]
    pub model_positions: Option<EvalModelPositionSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetEvalDraftResponse {
    pub id: String,
    pub version: i64,
    pub created_at: String,
    pub generated: bool,
    pub mcp: bool,
    pub active: bool,
    pub group_id: String,
    pub session_id: String,
    pub department_ids: Vec<String>,
    pub description_ids: Vec<String>,
    pub flag_ids: Vec<String>,
    pub model_ids: Vec<String>,
    pub name_ids: Vec<String>,
    pub profile_ids: Vec<String>,
    pub rubric_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetEvalDraftsApiResponse {
    #[serde(default)]
    pub entries: Option<Vec<GetEvalDraftResponse>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFieldApiRequest {
    #[serde(default)]
    pub field_id: Option<String>,
    #[serde(default)]
    pub draft_id: Option<String>,
    #[serde(default)]
    pub descriptions_search: Option<String>,
    #[serde(default)]
    pub conditional_parameter_search: Option<String>,
    #[serde(default)]
    pub conditional_parameter_show_selected: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFieldApiResponseInput {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub field_exists: Option<bool>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub disabled_reason: Option<String>,
    #[serde(default)]
    pub draft_version: Option<i64>,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub basic_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub names: Option<FieldNameSection>,
    #[serde(default)]
    pub descriptions: Option<FieldDescriptionSection>,
    #[serde(default)]
    pub flags: Option<FieldFlagSection>,
    #[serde(default)]
    pub departments: Option<FieldDepartmentSection>,
    #[serde(default)]
    pub conditional_parameters: Option<FieldConditionalParameterSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFieldApiResponseOutput {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub field_exists: Option<bool>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub disabled_reason: Option<String>,
    #[serde(default)]
    pub draft_version: Option<i64>,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub basic_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub names: Option<FieldNameSection>,
    #[serde(default)]
    pub descriptions: Option<FieldDescriptionSection>,
    #[serde(default)]
    pub flags: Option<FieldFlagSection>,
    #[serde(default)]
    pub departments: Option<FieldDepartmentSection>,
    #[serde(default)]
    pub conditional_parameters: Option<FieldConditionalParameterSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFieldDraftResponse {
    pub id: String,
    pub version: i64,
    pub created_at: String,
    pub generated: bool,
    pub mcp: bool,
    pub active: bool,
    pub group_id: String,
    pub session_id: String,
    pub conditional_parameter_ids: Vec<String>,
    pub department_ids: Vec<String>,
    pub description_ids: Vec<String>,
    pub flag_ids: Vec<String>,
    pub name_ids: Vec<String>,
    pub profile_ids: Vec<String>,
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
pub struct GetGroupDetailRequest {
    pub group_id: String,
    #[serde(default)]
    pub message_limit: Option<i64>,
    #[serde(default)]
    pub message_offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetGroupDetailResponseInput {
    #[serde(default)]
    pub group_exists: Option<bool>,
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub group_name: Option<String>,
    #[serde(default)]
    pub total_message_count: Option<i64>,
    #[serde(default)]
    pub runs: Option<Vec<GroupDetailRunWithMessagesInput>>,
    #[serde(default)]
    pub models: Option<Vec<GroupDetailResourceItem>>,
    #[serde(default)]
    pub agents: Option<Vec<GroupDetailResourceItem>>,
    #[serde(default)]
    pub profiles: Option<Vec<GroupDetailResourceItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetGroupDetailResponseOutput {
    #[serde(default)]
    pub group_exists: Option<bool>,
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub group_name: Option<String>,
    #[serde(default)]
    pub total_message_count: Option<i64>,
    #[serde(default)]
    pub runs: Option<Vec<GroupDetailRunWithMessagesOutput>>,
    #[serde(default)]
    pub models: Option<Vec<GroupDetailResourceItem>>,
    #[serde(default)]
    pub agents: Option<Vec<GroupDetailResourceItem>>,
    #[serde(default)]
    pub profiles: Option<Vec<GroupDetailResourceItem>>,
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
pub struct GetHomeResponseInput {
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
    pub analytics: Option<AnalyticsFacetsInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetHomeResponseOutput {
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
    pub analytics: Option<AnalyticsFacetsOutput>,
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
    pub model_id: Option<String>,
    #[serde(default)]
    pub draft_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetModelApiResponseInput {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub model_exists: Option<bool>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub disabled_reason: Option<String>,
    #[serde(default)]
    pub draft_version: Option<i64>,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub basic_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub provider_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub features_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub names: Option<ModelNameSection>,
    #[serde(default)]
    pub descriptions: Option<ModelDescriptionSection>,
    #[serde(default)]
    pub values: Option<ModelValueSection>,
    #[serde(default)]
    pub providers: Option<ModelProviderSection>,
    #[serde(default)]
    pub flags: Option<ModelFlagSection>,
    #[serde(default)]
    pub departments: Option<ModelDepartmentSection>,
    #[serde(default)]
    pub modalities: Option<ModelModalitySection>,
    #[serde(default)]
    pub temperature_levels: Option<ModelTemperatureLevelSection>,
    #[serde(default)]
    pub pricing: Option<ModelPricingSection>,
    #[serde(default)]
    pub reasoning_levels: Option<ModelReasoningLevelSection>,
    #[serde(default)]
    pub qualities: Option<ModelQualitySection>,
    #[serde(default)]
    pub voices: Option<ModelVoiceSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetModelApiResponseOutput {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub model_exists: Option<bool>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub disabled_reason: Option<String>,
    #[serde(default)]
    pub draft_version: Option<i64>,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub basic_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub provider_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub features_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub names: Option<ModelNameSection>,
    #[serde(default)]
    pub descriptions: Option<ModelDescriptionSection>,
    #[serde(default)]
    pub values: Option<ModelValueSection>,
    #[serde(default)]
    pub providers: Option<ModelProviderSection>,
    #[serde(default)]
    pub flags: Option<ModelFlagSection>,
    #[serde(default)]
    pub departments: Option<ModelDepartmentSection>,
    #[serde(default)]
    pub modalities: Option<ModelModalitySection>,
    #[serde(default)]
    pub temperature_levels: Option<ModelTemperatureLevelSection>,
    #[serde(default)]
    pub pricing: Option<ModelPricingSection>,
    #[serde(default)]
    pub reasoning_levels: Option<ModelReasoningLevelSection>,
    #[serde(default)]
    pub qualities: Option<ModelQualitySection>,
    #[serde(default)]
    pub voices: Option<ModelVoiceSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetModelDraftResponse {
    pub id: String,
    pub version: i64,
    pub created_at: String,
    pub generated: bool,
    pub mcp: bool,
    pub active: bool,
    pub group_id: String,
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
    pub value_ids: Vec<String>,
    pub voice_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetModelDraftsApiResponse {
    #[serde(default)]
    pub entries: Option<Vec<GetModelDraftResponse>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetParameterApiRequest {
    #[serde(default)]
    pub parameter_id: Option<String>,
    #[serde(default)]
    pub draft_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetParameterApiResponseInput {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub parameter_exists: Option<bool>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub disabled_reason: Option<String>,
    #[serde(default)]
    pub draft_version: Option<i64>,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub basic_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub fields_step_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub names: Option<ParameterNameSection>,
    #[serde(default)]
    pub descriptions: Option<ParameterDescriptionSection>,
    #[serde(default)]
    pub flags: Option<ParameterFlagSection>,
    #[serde(default)]
    pub departments: Option<ParameterDepartmentSection>,
    #[serde(default)]
    pub fields: Option<ParameterFieldSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetParameterApiResponseOutput {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub parameter_exists: Option<bool>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub disabled_reason: Option<String>,
    #[serde(default)]
    pub draft_version: Option<i64>,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub basic_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub fields_step_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub names: Option<ParameterNameSection>,
    #[serde(default)]
    pub descriptions: Option<ParameterDescriptionSection>,
    #[serde(default)]
    pub flags: Option<ParameterFlagSection>,
    #[serde(default)]
    pub departments: Option<ParameterDepartmentSection>,
    #[serde(default)]
    pub fields: Option<ParameterFieldSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetParameterDraftResponse {
    pub id: String,
    pub version: i64,
    pub created_at: String,
    pub generated: bool,
    pub mcp: bool,
    pub active: bool,
    pub group_id: String,
    pub session_id: String,
    pub department_ids: Vec<String>,
    pub description_ids: Vec<String>,
    pub field_ids: Vec<String>,
    pub flag_ids: Vec<String>,
    pub name_ids: Vec<String>,
    pub profile_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetParameterDraftsApiResponse {
    #[serde(default)]
    pub entries: Option<Vec<GetParameterDraftResponse>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetParameterResponse {
    pub id: String,
    pub name: String,
    pub description: String,
    pub value: String,
    pub department_ids: Vec<String>,
    pub persona_parameter: bool,
    pub document_parameter: bool,
    pub scenario_parameter: bool,
    pub video_parameter: bool,
    pub field_ids: Vec<String>,
    pub created_at: String,
    pub active: bool,
    pub generated: bool,
    pub mcp: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPersonaApiRequest {
    #[serde(default)]
    pub persona_id: Option<String>,
    #[serde(default)]
    pub draft_id: Option<String>,
    #[serde(default)]
    pub color_search: Option<String>,
    #[serde(default)]
    pub icon_search: Option<String>,
    #[serde(default)]
    pub descriptions_search: Option<String>,
    #[serde(default)]
    pub instructions_search: Option<String>,
    #[serde(default)]
    pub parameter_field_search: Option<String>,
    #[serde(default)]
    pub parameter_ids: Option<Vec<String>>,
    #[serde(default)]
    pub color_show_selected: Option<bool>,
    #[serde(default)]
    pub icon_show_selected: Option<bool>,
    #[serde(default)]
    pub parameter_field_show_selected: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPersonaApiResponseInput {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub persona_exists: Option<bool>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub disabled_reason: Option<String>,
    #[serde(default)]
    pub draft_version: Option<i64>,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub basic_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub content_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub parameters_step_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub names: Option<PersonaNameSection>,
    #[serde(default)]
    pub descriptions: Option<PersonaDescriptionSection>,
    #[serde(default)]
    pub colors: Option<PersonaColorSection>,
    #[serde(default)]
    pub icons: Option<PersonaIconSection>,
    #[serde(default)]
    pub instructions: Option<PersonaInstructionSection>,
    #[serde(default)]
    pub flags: Option<PersonaFlagSection>,
    #[serde(default)]
    pub departments: Option<PersonaDepartmentSection>,
    #[serde(default)]
    pub parameter_fields: Option<PersonaParameterFieldSection>,
    #[serde(default)]
    pub examples: Option<PersonaExampleSection>,
    #[serde(default)]
    pub parameters: Option<PersonaParameterSection>,
    #[serde(default)]
    pub voices: Option<PersonaVoiceSection>,
    #[serde(default)]
    pub fields: Option<Vec<GetFieldResponse>>,
    #[serde(default)]
    pub resolved_parameter_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPersonaApiResponseOutput {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub persona_exists: Option<bool>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub disabled_reason: Option<String>,
    #[serde(default)]
    pub draft_version: Option<i64>,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub basic_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub content_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub parameters_step_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub names: Option<PersonaNameSection>,
    #[serde(default)]
    pub descriptions: Option<PersonaDescriptionSection>,
    #[serde(default)]
    pub colors: Option<PersonaColorSection>,
    #[serde(default)]
    pub icons: Option<PersonaIconSection>,
    #[serde(default)]
    pub instructions: Option<PersonaInstructionSection>,
    #[serde(default)]
    pub flags: Option<PersonaFlagSection>,
    #[serde(default)]
    pub departments: Option<PersonaDepartmentSection>,
    #[serde(default)]
    pub parameter_fields: Option<PersonaParameterFieldSection>,
    #[serde(default)]
    pub examples: Option<PersonaExampleSection>,
    #[serde(default)]
    pub parameters: Option<PersonaParameterSection>,
    #[serde(default)]
    pub voices: Option<PersonaVoiceSection>,
    #[serde(default)]
    pub fields: Option<Vec<GetFieldResponse>>,
    #[serde(default)]
    pub resolved_parameter_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPersonaDraftResponse {
    pub id: String,
    pub version: i64,
    pub created_at: String,
    pub generated: bool,
    pub mcp: bool,
    pub active: bool,
    pub group_id: String,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPersonaDraftsApiResponse {
    #[serde(default)]
    pub entries: Option<Vec<GetPersonaDraftResponse>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPersonaResponse {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub color: String,
    pub department_ids: Vec<String>,
    pub instructions: String,
    pub examples: Vec<String>,
    pub parameter_field_ids: Vec<String>,
    pub created_at: String,
    pub active: bool,
    pub generated: bool,
    pub mcp: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPracticeResponseInput {
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
    pub analytics: Option<AnalyticsFacetsInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPracticeResponseOutput {
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
    pub analytics: Option<AnalyticsFacetsOutput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetProfileApiRequest {
    #[serde(default)]
    pub target_profile_id: Option<String>,
    #[serde(default)]
    pub draft_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetProfileApiResponseInput {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub profile_exists: Option<bool>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub disabled_reason: Option<String>,
    #[serde(default)]
    pub draft_version: Option<i64>,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub profile_id: Option<String>,
    #[serde(default)]
    pub role: Option<String>,
    #[serde(default)]
    pub role_options: Option<Vec<String>>,
    #[serde(default)]
    pub basic_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub general_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub names: Option<ProfileNameSection>,
    #[serde(default)]
    pub emails: Option<ProfileEmailSection>,
    #[serde(default)]
    pub request_limits: Option<ProfileRequestLimitSection>,
    #[serde(default)]
    pub flags: Option<ProfileFlagSection>,
    #[serde(default)]
    pub departments: Option<ProfileDepartmentSection>,
    #[serde(default)]
    pub roles: Option<ProfileRoleSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetProfileApiResponseOutput {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub profile_exists: Option<bool>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub disabled_reason: Option<String>,
    #[serde(default)]
    pub draft_version: Option<i64>,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub profile_id: Option<String>,
    #[serde(default)]
    pub role: Option<String>,
    #[serde(default)]
    pub role_options: Option<Vec<String>>,
    #[serde(default)]
    pub basic_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub general_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub names: Option<ProfileNameSection>,
    #[serde(default)]
    pub emails: Option<ProfileEmailSection>,
    #[serde(default)]
    pub request_limits: Option<ProfileRequestLimitSection>,
    #[serde(default)]
    pub flags: Option<ProfileFlagSection>,
    #[serde(default)]
    pub departments: Option<ProfileDepartmentSection>,
    #[serde(default)]
    pub roles: Option<ProfileRoleSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetProfileContextApiRequest {
    #[serde(default)]
    pub department_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetProfileDraftResponse {
    pub id: String,
    pub version: i64,
    pub created_at: String,
    pub generated: bool,
    pub mcp: bool,
    pub active: bool,
    pub group_id: String,
    pub session_id: String,
    pub profile_ids: Vec<String>,
    pub department_ids: Vec<String>,
    pub email_ids: Vec<String>,
    pub flag_ids: Vec<String>,
    pub name_ids: Vec<String>,
    pub request_limit_ids: Vec<String>,
    pub role_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetProfileDraftsApiResponse {
    #[serde(default)]
    pub entries: Option<Vec<GetProfileDraftResponse>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetProviderApiRequest {
    #[serde(default)]
    pub provider_id: Option<String>,
    #[serde(default)]
    pub draft_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetProviderApiResponseInput {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub provider_exists: Option<bool>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub disabled_reason: Option<String>,
    #[serde(default)]
    pub draft_version: Option<i64>,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub basic_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub integrations_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub names: Option<ProviderNameSection>,
    #[serde(default)]
    pub descriptions: Option<ProviderDescriptionSection>,
    #[serde(default)]
    pub flags: Option<ProviderFlagSection>,
    #[serde(default)]
    pub departments: Option<ProviderDepartmentSection>,
    #[serde(default)]
    pub values: Option<ProviderValueSection>,
    #[serde(default)]
    pub endpoints: Option<ProviderEndpointSection>,
    #[serde(default)]
    pub keys: Option<ProviderKeySection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetProviderApiResponseOutput {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub provider_exists: Option<bool>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub disabled_reason: Option<String>,
    #[serde(default)]
    pub draft_version: Option<i64>,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub basic_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub integrations_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub names: Option<ProviderNameSection>,
    #[serde(default)]
    pub descriptions: Option<ProviderDescriptionSection>,
    #[serde(default)]
    pub flags: Option<ProviderFlagSection>,
    #[serde(default)]
    pub departments: Option<ProviderDepartmentSection>,
    #[serde(default)]
    pub values: Option<ProviderValueSection>,
    #[serde(default)]
    pub endpoints: Option<ProviderEndpointSection>,
    #[serde(default)]
    pub keys: Option<ProviderKeySection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetProviderDraftResponse {
    pub id: String,
    pub version: i64,
    pub created_at: String,
    pub generated: bool,
    pub mcp: bool,
    pub active: bool,
    pub group_id: String,
    pub session_id: String,
    pub department_ids: Vec<String>,
    pub description_ids: Vec<String>,
    pub endpoint_ids: Vec<String>,
    pub flag_ids: Vec<String>,
    pub key_ids: Vec<String>,
    pub name_ids: Vec<String>,
    pub profile_ids: Vec<String>,
    pub value_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetProviderDraftsApiResponse {
    #[serde(default)]
    pub entries: Option<Vec<GetProviderDraftResponse>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRubricApiRequest {
    #[serde(default)]
    pub rubric_id: Option<String>,
    #[serde(default)]
    pub draft_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRubricApiResponseInput {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub rubric_exists: Option<bool>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub disabled_reason: Option<String>,
    #[serde(default)]
    pub draft_version: Option<i64>,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub basic_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub content_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub names: Option<RubricNameSection>,
    #[serde(default)]
    pub descriptions: Option<RubricDescriptionSection>,
    #[serde(default)]
    pub flags: Option<RubricFlagSection>,
    #[serde(default)]
    pub departments: Option<RubricDepartmentSection>,
    #[serde(default)]
    pub points: Option<RubricPointsSection>,
    #[serde(default)]
    pub standard_groups: Option<RubricStandardGroupsSection>,
    #[serde(default)]
    pub standards: Option<RubricStandardsSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRubricApiResponseOutput {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub rubric_exists: Option<bool>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub disabled_reason: Option<String>,
    #[serde(default)]
    pub draft_version: Option<i64>,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub basic_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub content_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub names: Option<RubricNameSection>,
    #[serde(default)]
    pub descriptions: Option<RubricDescriptionSection>,
    #[serde(default)]
    pub flags: Option<RubricFlagSection>,
    #[serde(default)]
    pub departments: Option<RubricDepartmentSection>,
    #[serde(default)]
    pub points: Option<RubricPointsSection>,
    #[serde(default)]
    pub standard_groups: Option<RubricStandardGroupsSection>,
    #[serde(default)]
    pub standards: Option<RubricStandardsSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRubricDraftResponse {
    pub id: String,
    pub version: i64,
    pub created_at: String,
    pub generated: bool,
    pub mcp: bool,
    pub active: bool,
    pub group_id: String,
    pub session_id: String,
    pub department_ids: Vec<String>,
    pub description_ids: Vec<String>,
    pub flag_ids: Vec<String>,
    pub name_ids: Vec<String>,
    pub point_ids: Vec<String>,
    pub profile_ids: Vec<String>,
    pub standard_group_ids: Vec<String>,
    pub standard_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRubricDraftsApiResponse {
    #[serde(default)]
    pub entries: Option<Vec<GetRubricDraftResponse>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRunListViewResponseInput {
    #[serde(default)]
    pub items: Option<Vec<RunViewItem>>,
    #[serde(default)]
    pub total_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRunListViewResponseOutput {
    #[serde(default)]
    pub items: Option<Vec<RunViewItem>>,
    #[serde(default)]
    pub total_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetScenarioApiRequest {
    #[serde(default)]
    pub scenario_id: Option<String>,
    #[serde(default)]
    pub document_ids: Option<Vec<String>>,
    #[serde(default)]
    pub problem_statement_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_persona_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_document_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_parameter_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_field_ids: Option<Vec<String>>,
    #[serde(default)]
    pub persona_search: Option<String>,
    #[serde(default)]
    pub document_search: Option<String>,
    #[serde(default)]
    pub parameter_search: Option<String>,
    #[serde(default)]
    pub description_search: Option<String>,
    #[serde(default)]
    pub problem_statement_search: Option<String>,
    #[serde(default)]
    pub image_search: Option<String>,
    #[serde(default)]
    pub video_search: Option<String>,
    #[serde(default)]
    pub question_search: Option<String>,
    #[serde(default)]
    pub option_search: Option<String>,
    #[serde(default)]
    pub persona_show_selected: Option<bool>,
    #[serde(default)]
    pub document_show_selected: Option<bool>,
    #[serde(default)]
    pub parameter_show_selected: Option<bool>,
    #[serde(default)]
    pub field_show_selected_by_param: Option<Vec<ScenarioFieldParamFilter>>,
    #[serde(default)]
    pub draft_id: Option<String>,
    #[serde(default)]
    pub mcp: Option<bool>,
    #[serde(default)]
    pub parameter_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetScenarioApiResponseInput {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub scenario_exists: Option<bool>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub disabled_reason: Option<String>,
    #[serde(default)]
    pub draft_version: Option<i64>,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub basic_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub content_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub resolved_parameter_ids: Option<Vec<String>>,
    #[serde(default)]
    pub names: Option<ScenarioNameSection>,
    #[serde(default)]
    pub descriptions: Option<ScenarioDescriptionSection>,
    #[serde(default)]
    pub problem_statements: Option<ScenarioProblemStatementSection>,
    #[serde(default)]
    pub flags: Option<ScenarioFlagSection>,
    #[serde(default)]
    pub departments: Option<ScenarioDepartmentSection>,
    #[serde(default)]
    pub personas: Option<ScenarioPersonaSection>,
    #[serde(default)]
    pub documents: Option<ScenarioDocumentSection>,
    #[serde(default)]
    pub parameters: Option<ScenarioParameterSection>,
    #[serde(default)]
    pub parameter_fields: Option<ScenarioParameterFieldSection>,
    #[serde(default)]
    pub objectives: Option<ScenarioObjectiveSection>,
    #[serde(default)]
    pub images: Option<ScenarioImageSection>,
    #[serde(default)]
    pub videos: Option<ScenarioVideoSection>,
    #[serde(default)]
    pub questions: Option<ScenarioQuestionSection>,
    #[serde(default)]
    pub options: Option<ScenarioOptionSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetScenarioApiResponseOutput {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub scenario_exists: Option<bool>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub disabled_reason: Option<String>,
    #[serde(default)]
    pub draft_version: Option<i64>,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub basic_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub content_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub resolved_parameter_ids: Option<Vec<String>>,
    #[serde(default)]
    pub names: Option<ScenarioNameSection>,
    #[serde(default)]
    pub descriptions: Option<ScenarioDescriptionSection>,
    #[serde(default)]
    pub problem_statements: Option<ScenarioProblemStatementSection>,
    #[serde(default)]
    pub flags: Option<ScenarioFlagSection>,
    #[serde(default)]
    pub departments: Option<ScenarioDepartmentSection>,
    #[serde(default)]
    pub personas: Option<ScenarioPersonaSection>,
    #[serde(default)]
    pub documents: Option<ScenarioDocumentSection>,
    #[serde(default)]
    pub parameters: Option<ScenarioParameterSection>,
    #[serde(default)]
    pub parameter_fields: Option<ScenarioParameterFieldSection>,
    #[serde(default)]
    pub objectives: Option<ScenarioObjectiveSection>,
    #[serde(default)]
    pub images: Option<ScenarioImageSection>,
    #[serde(default)]
    pub videos: Option<ScenarioVideoSection>,
    #[serde(default)]
    pub questions: Option<ScenarioQuestionSection>,
    #[serde(default)]
    pub options: Option<ScenarioOptionSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetScenarioDraftResponse {
    pub id: String,
    pub version: i64,
    pub created_at: String,
    pub generated: bool,
    pub mcp: bool,
    pub active: bool,
    pub group_id: String,
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
pub struct GetSessionDetailResponseInput {
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
    pub groups: Option<Vec<ArtifactSessionGroupInput>>,
    #[serde(default)]
    pub timeline: Option<Vec<SessionTimelineItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSessionDetailResponseOutput {
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
    pub groups: Option<Vec<ArtifactSessionGroupOutput>>,
    #[serde(default)]
    pub timeline: Option<Vec<SessionTimelineItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSettingApiRequest {
    #[serde(default)]
    pub settings_id: Option<String>,
    #[serde(default)]
    pub color_search: Option<String>,
    #[serde(default)]
    pub draft_id: Option<String>,
    #[serde(default)]
    pub mcp: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSettingApiResponseInput {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub setting_exists: Option<bool>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub disabled_reason: Option<String>,
    #[serde(default)]
    pub draft_version: Option<i64>,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub names: Option<SettingNameSection>,
    #[serde(default)]
    pub descriptions: Option<SettingDescriptionSection>,
    #[serde(default)]
    pub colors: Option<SettingColorSection>,
    #[serde(default)]
    pub flags: Option<SettingFlagSection>,
    #[serde(default)]
    pub departments: Option<SettingDepartmentSection>,
    #[serde(default)]
    pub profiles: Option<SettingProfileSection>,
    #[serde(default)]
    pub auths: Option<SettingAuthSection>,
    #[serde(default)]
    pub provider_keys: Option<SettingProviderKeySection>,
    #[serde(default)]
    pub auth_item_keys: Option<SettingAuthItemKeySection>,
    #[serde(default)]
    pub systems: Option<SettingSystemSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSettingApiResponseOutput {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub setting_exists: Option<bool>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub disabled_reason: Option<String>,
    #[serde(default)]
    pub draft_version: Option<i64>,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub names: Option<SettingNameSection>,
    #[serde(default)]
    pub descriptions: Option<SettingDescriptionSection>,
    #[serde(default)]
    pub colors: Option<SettingColorSection>,
    #[serde(default)]
    pub flags: Option<SettingFlagSection>,
    #[serde(default)]
    pub departments: Option<SettingDepartmentSection>,
    #[serde(default)]
    pub profiles: Option<SettingProfileSection>,
    #[serde(default)]
    pub auths: Option<SettingAuthSection>,
    #[serde(default)]
    pub provider_keys: Option<SettingProviderKeySection>,
    #[serde(default)]
    pub auth_item_keys: Option<SettingAuthItemKeySection>,
    #[serde(default)]
    pub systems: Option<SettingSystemSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSettingDraftResponse {
    pub id: String,
    pub version: i64,
    pub created_at: String,
    pub generated: bool,
    pub mcp: bool,
    pub active: bool,
    pub group_id: String,
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
    pub profile_ids: Vec<String>,
    pub provider_key_ids: Vec<String>,
    pub threshold_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSettingDraftsApiResponse {
    #[serde(default)]
    pub entries: Option<Vec<GetSettingDraftResponse>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSimulationApiRequest {
    #[serde(default)]
    pub simulation_id: Option<String>,
    #[serde(default)]
    pub draft_id: Option<String>,
    #[serde(default)]
    pub scenario_search: Option<String>,
    #[serde(default)]
    pub filter_scenario_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSimulationApiResponseInput {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub simulation_exists: Option<bool>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub disabled_reason: Option<String>,
    #[serde(default)]
    pub draft_version: Option<i64>,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub basic_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub names: Option<SimulationNameSection>,
    #[serde(default)]
    pub descriptions: Option<SimulationDescriptionSection>,
    #[serde(default)]
    pub flags: Option<SimulationFlagSection>,
    #[serde(default)]
    pub departments: Option<SimulationDepartmentSection>,
    #[serde(default)]
    pub scenarios: Option<SimulationScenarioSection>,
    #[serde(default)]
    pub scenario_flags: Option<SimulationScenarioFlagSection>,
    #[serde(default)]
    pub scenario_positions: Option<SimulationScenarioPositionSection>,
    #[serde(default)]
    pub scenario_rubrics: Option<SimulationScenarioRubricSection>,
    #[serde(default)]
    pub scenario_time_limits: Option<SimulationScenarioTimeLimitSection>,
    #[serde(default)]
    pub rubrics: Option<Vec<SimulationRubric>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSimulationApiResponseOutput {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub simulation_exists: Option<bool>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub disabled_reason: Option<String>,
    #[serde(default)]
    pub draft_version: Option<i64>,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub basic_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub names: Option<SimulationNameSection>,
    #[serde(default)]
    pub descriptions: Option<SimulationDescriptionSection>,
    #[serde(default)]
    pub flags: Option<SimulationFlagSection>,
    #[serde(default)]
    pub departments: Option<SimulationDepartmentSection>,
    #[serde(default)]
    pub scenarios: Option<SimulationScenarioSection>,
    #[serde(default)]
    pub scenario_flags: Option<SimulationScenarioFlagSection>,
    #[serde(default)]
    pub scenario_positions: Option<SimulationScenarioPositionSection>,
    #[serde(default)]
    pub scenario_rubrics: Option<SimulationScenarioRubricSection>,
    #[serde(default)]
    pub scenario_time_limits: Option<SimulationScenarioTimeLimitSection>,
    #[serde(default)]
    pub rubrics: Option<Vec<SimulationRubric>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSimulationDraftResponse {
    pub id: String,
    pub version: i64,
    pub created_at: String,
    pub generated: bool,
    pub mcp: bool,
    pub active: bool,
    pub group_id: String,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSimulationDraftsApiResponse {
    #[serde(default)]
    pub entries: Option<Vec<GetSimulationDraftResponse>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSuiteRequest {
    pub test_id: String,
    #[serde(default)]
    pub draft_id: Option<String>,
    #[serde(default)]
    pub descriptions_search: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSuiteResponse {
    pub test_id: String,
    #[serde(default)]
    pub profile_has_access: Option<bool>,
    #[serde(default)]
    pub draft_version: Option<i64>,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub names: Option<SuiteNameSection>,
    #[serde(default)]
    pub descriptions: Option<SuiteDescriptionSection>,
    #[serde(default)]
    pub values: Option<SuiteValueSection>,
    #[serde(default)]
    pub flags: Option<SuiteFlagSection>,
    #[serde(default)]
    pub departments: Option<SuiteDepartmentSection>,
    #[serde(default)]
    pub keys: Option<SuiteKeySection>,
    #[serde(default)]
    pub endpoints: Option<SuiteEndpointSection>,
    #[serde(default)]
    pub modalities: Option<SuiteModalitySection>,
    #[serde(default)]
    pub temperature_levels: Option<SuiteTemperatureLevelSection>,
    #[serde(default)]
    pub pricing: Option<SuitePricingSection>,
    #[serde(default)]
    pub reasoning_levels: Option<SuiteReasoningLevelSection>,
    #[serde(default)]
    pub qualities: Option<SuiteQualitySection>,
    #[serde(default)]
    pub voices: Option<SuiteVoiceSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTestArtifactRequest {
    pub test_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTestArtifactResponseInput {
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
    pub status_summary: Option<TestStatusSummary>,
    #[serde(default)]
    pub show_controls: Option<bool>,
    #[serde(default)]
    pub current_invocation_id: Option<String>,
    #[serde(default)]
    pub has_runs_or_groups: Option<bool>,
    #[serde(default)]
    pub entries: Option<TestEntries>,
    #[serde(default)]
    pub resources: Option<TestResources>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTestArtifactResponseOutput {
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
    pub status_summary: Option<TestStatusSummary>,
    #[serde(default)]
    pub show_controls: Option<bool>,
    #[serde(default)]
    pub current_invocation_id: Option<String>,
    #[serde(default)]
    pub has_runs_or_groups: Option<bool>,
    #[serde(default)]
    pub entries: Option<TestEntries>,
    #[serde(default)]
    pub resources: Option<TestResources>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTestFeedbackResponse {
    pub feedback_id: String,
    pub grade_id: String,
    pub total: i64,
    pub feedback: String,
    pub total_points: i64,
    pub pass_points: i64,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTestGradeResponse {
    pub id: String,
    pub invocation_id: String,
    pub run_id: Option<String>,
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
pub struct GetTestInvocationGroupsResponse {
    pub id: String,
    pub test_invocation_id: String,
    pub created_at: String,
    pub updated_at: String,
    pub generated: bool,
    pub mcp: bool,
    pub active: bool,
    #[serde(default)]
    pub agent_ids: Option<Vec<String>>,
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
    pub run_agent_ids: Option<Vec<String>>,
    #[serde(default)]
    pub group_agent_ids: Option<Vec<String>>,
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
    pub agent_ids: Option<Vec<String>>,
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
    pub tool_id: Option<String>,
    #[serde(default)]
    pub draft_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetToolApiResponseInput {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub tool_exists: Option<bool>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub disabled_reason: Option<String>,
    #[serde(default)]
    pub draft_version: Option<i64>,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub basic_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub args_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub arg_positions_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub args_outputs_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub names: Option<ToolNameSection>,
    #[serde(default)]
    pub descriptions: Option<ToolDescriptionSection>,
    #[serde(default)]
    pub flags: Option<ToolFlagSection>,
    #[serde(default)]
    pub args: Option<ToolArgSection>,
    #[serde(default)]
    pub arg_positions: Option<ToolArgPositionSection>,
    #[serde(default)]
    pub args_outputs: Option<ToolArgOutputSection>,
    #[serde(default)]
    pub artifacts: Option<ToolArtifactSection>,
    #[serde(default)]
    pub operations: Option<ToolOperationSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetToolApiResponseOutput {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub tool_exists: Option<bool>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub disabled_reason: Option<String>,
    #[serde(default)]
    pub draft_version: Option<i64>,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub basic_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub args_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub arg_positions_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub args_outputs_show_ai_generate: Option<bool>,
    #[serde(default)]
    pub names: Option<ToolNameSection>,
    #[serde(default)]
    pub descriptions: Option<ToolDescriptionSection>,
    #[serde(default)]
    pub flags: Option<ToolFlagSection>,
    #[serde(default)]
    pub args: Option<ToolArgSection>,
    #[serde(default)]
    pub arg_positions: Option<ToolArgPositionSection>,
    #[serde(default)]
    pub args_outputs: Option<ToolArgOutputSection>,
    #[serde(default)]
    pub artifacts: Option<ToolArtifactSection>,
    #[serde(default)]
    pub operations: Option<ToolOperationSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetToolDraftResponse {
    pub id: String,
    pub version: i64,
    pub created_at: String,
    pub generated: bool,
    pub mcp: bool,
    pub active: bool,
    pub group_id: String,
    pub session_id: String,
    pub arg_position_ids: Vec<String>,
    pub arg_ids: Vec<String>,
    pub args_output_ids: Vec<String>,
    pub artifact_ids: Vec<String>,
    pub department_ids: Vec<String>,
    pub description_ids: Vec<String>,
    pub flag_ids: Vec<String>,
    pub name_ids: Vec<String>,
    pub operation_ids: Vec<String>,
    pub profile_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetToolDraftsApiResponse {
    #[serde(default)]
    pub entries: Option<Vec<GetToolDraftResponse>>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradeAttemptRequest {
    pub attempt_id: String,
    #[serde(default)]
    pub chat_id: Option<String>,
    #[serde(default)]
    pub resource_types: Option<Vec<String>>,
    #[serde(default)]
    pub user_instructions: Option<Vec<String>>,
    #[serde(default)]
    pub score: Option<i64>,
    #[serde(default)]
    pub passed: Option<bool>,
    #[serde(default)]
    pub time_taken: Option<i64>,
    #[serde(default)]
    pub feedbacks: Option<Vec<AttemptGradeFeedbackEntry>>,
    #[serde(default)]
    pub strengths: Option<Vec<AttemptGradeStrengthEntry>>,
    #[serde(default)]
    pub improvements: Option<Vec<AttemptGradeImprovementEntry>>,
    #[serde(default)]
    pub analyses: Option<Vec<AttemptGradeAnalysisEntry>>,
    #[serde(default)]
    pub highlights: Option<Vec<AttemptGradeHighlightEntry>>,
    #[serde(default)]
    pub replacements: Option<Vec<AttemptGradeReplacementEntry>>,
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
pub struct GroupDetailCallItem {
    pub id: String,
    pub created_at: String,
    #[serde(default)]
    pub template_name: Option<String>,
    #[serde(default)]
    pub file_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupDetailMessageItem {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub role: Option<String>,
    #[serde(default)]
    pub text_upload_ids: Option<Vec<String>>,
    #[serde(default)]
    pub audio_upload_ids: Option<Vec<String>>,
    #[serde(default)]
    pub image_upload_ids: Option<Vec<String>>,
    #[serde(default)]
    pub video_upload_ids: Option<Vec<String>>,
    #[serde(default)]
    pub file_upload_ids: Option<Vec<String>>,
    #[serde(default)]
    pub call_upload_ids: Option<Vec<String>>,
    #[serde(default)]
    pub calls: Option<Vec<GroupDetailCallItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupDetailResourceItem {
    #[serde(default)]
    pub model_id: Option<String>,
    #[serde(default)]
    pub agent_id: Option<String>,
    #[serde(default)]
    pub profile_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupDetailRunItem {
    pub id: String,
    pub created_at: String,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupDetailRunWithMessagesInput {
    pub run: GroupDetailRunItem,
    #[serde(default)]
    pub messages: Option<Vec<GroupDetailMessageItem>>,
    #[serde(default)]
    pub previous_context_start_index: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupDetailRunWithMessagesOutput {
    pub run: GroupDetailRunItem,
    #[serde(default)]
    pub messages: Option<Vec<GroupDetailMessageItem>>,
    #[serde(default)]
    pub previous_context_start_index: Option<i64>,
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
pub struct HealthResponseInput {
    #[serde(default)]
    pub views: Option<HealthViews>,
    #[serde(default)]
    pub total_count: Option<i64>,
    #[serde(default)]
    pub analytics: Option<AnalyticsFacetsInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponseOutput {
    #[serde(default)]
    pub views: Option<HealthViews>,
    #[serde(default)]
    pub total_count: Option<i64>,
    #[serde(default)]
    pub analytics: Option<AnalyticsFacetsOutput>,
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
pub struct HintEntryOutput {
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
pub struct ImageEntry {
    #[serde(default)]
    pub image_id: Option<String>,
    #[serde(default)]
    pub upload_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
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
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub simulation_filters: Option<Vec<String>>,
    #[serde(default)]
    pub target_profile_id: Option<String>,
    #[serde(default)]
    pub cohort_id: Option<String>,
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
pub struct LeaderboardResponseInput {
    #[serde(default)]
    pub sections: Option<LeaderboardSectionsInput>,
    #[serde(default)]
    pub resources: Option<LeaderboardResources>,
    #[serde(default)]
    pub analytics: Option<AnalyticsFacetsInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardResponseOutput {
    #[serde(default)]
    pub sections: Option<LeaderboardSectionsOutput>,
    #[serde(default)]
    pub resources: Option<LeaderboardResources>,
    #[serde(default)]
    pub analytics: Option<AnalyticsFacetsOutput>,
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
pub struct LeaderboardSectionsInput {
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
pub struct LeaderboardSectionsOutput {
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
    pub roles: Option<Vec<String>>,
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
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub can_duplicate: Option<bool>,
    #[serde(default)]
    pub can_delete: Option<bool>,
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
    pub total_count: Option<i64>,
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
    pub is_inactive: Option<bool>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub can_duplicate: Option<bool>,
    #[serde(default)]
    pub can_delete: Option<bool>,
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
pub struct ListDashboardRequest {
    #[serde(default)]
    pub start_date: Option<String>,
    #[serde(default)]
    pub end_date: Option<String>,
    #[serde(default)]
    pub cohort_ids: Option<Vec<String>>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub target_profile_id: Option<String>,
    #[serde(default)]
    pub practice: Option<bool>,
    #[serde(default)]
    pub scenario_ids: Option<Vec<String>>,
    #[serde(default)]
    pub infinite_mode: Option<bool>,
    #[serde(default)]
    pub show_archived: Option<bool>,
    #[serde(default)]
    pub sort_by: Option<String>,
    #[serde(default)]
    pub sort_order: Option<String>,
    #[serde(default)]
    pub page: Option<i64>,
    #[serde(default)]
    pub page_size: Option<i64>,
    #[serde(default)]
    pub simulation_search: Option<String>,
    #[serde(default)]
    pub scenario_search: Option<String>,
    #[serde(default)]
    pub profile_search: Option<String>,
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
    pub is_inactive: Option<bool>,
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
pub struct ListDepartmentApiResponse {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub departments: Option<Vec<ListDepartmentApiDepartment>>,
    #[serde(default)]
    pub total_count: Option<i64>,
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
    pub is_inactive: Option<bool>,
    #[serde(default)]
    pub num_scenarios: Option<i64>,
    #[serde(default)]
    pub active_scenario_count: Option<i64>,
    #[serde(default)]
    pub upload_id: Option<String>,
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
    pub total_count: Option<i64>,
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
    pub is_inactive: Option<bool>,
    #[serde(default)]
    pub is_dynamic: Option<bool>,
    #[serde(default)]
    pub use_groups: Option<bool>,
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
    pub total_count: Option<i64>,
    #[serde(default)]
    pub user_role: Option<String>,
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
    pub total_count: Option<i64>,
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
pub struct ListHomeRequest {
    #[serde(default)]
    pub sort_by: Option<String>,
    #[serde(default)]
    pub sort_order: Option<String>,
    #[serde(default)]
    pub page: Option<i64>,
    #[serde(default)]
    pub page_size: Option<i64>,
    #[serde(default)]
    pub simulation_search: Option<String>,
    #[serde(default)]
    pub scenario_search: Option<String>,
    #[serde(default)]
    pub scenario_ids: Option<Vec<String>>,
    #[serde(default)]
    pub infinite_mode: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListHomeResponse {
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
pub struct ListLeaderboardRequest {
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
    #[serde(default)]
    pub page_limit: Option<i64>,
    #[serde(default)]
    pub page_offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListLeaderboardResponse {
    #[serde(default)]
    pub data: Option<Vec<LeaderboardDataRow>>,
    #[serde(default)]
    pub resources: Option<LeaderboardResources>,
    #[serde(default)]
    pub total_count: Option<i64>,
    #[serde(default)]
    pub simulation_options: Option<Vec<FilterOption>>,
    #[serde(default)]
    pub profile_options: Option<Vec<FilterOption>>,
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
    pub total_count: Option<i64>,
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
    pub total_count: Option<i64>,
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
    pub total_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPracticeRequest {
    #[serde(default)]
    pub sort_by: Option<String>,
    #[serde(default)]
    pub sort_order: Option<String>,
    #[serde(default)]
    pub page: Option<i64>,
    #[serde(default)]
    pub page_size: Option<i64>,
    #[serde(default)]
    pub simulation_search: Option<String>,
    #[serde(default)]
    pub scenario_search: Option<String>,
    #[serde(default)]
    pub show_archived: Option<bool>,
    #[serde(default)]
    pub scenario_ids: Option<Vec<String>>,
    #[serde(default)]
    pub infinite_mode: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPracticeResponse {
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
    pub page: Option<i64>,
    #[serde(default)]
    pub page_size: Option<i64>,
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
    pub initials: Option<String>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub primary_department_id: Option<String>,
    #[serde(default)]
    pub requests_per_day: Option<i64>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub can_duplicate: Option<bool>,
    #[serde(default)]
    pub can_delete: Option<bool>,
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
    pub total_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListRecordRequest {
    pub target_profile_id: String,
    #[serde(default)]
    pub start_date: Option<String>,
    #[serde(default)]
    pub end_date: Option<String>,
    #[serde(default)]
    pub cohort_ids: Option<Vec<String>>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub practice: Option<bool>,
    #[serde(default)]
    pub scenario_ids: Option<Vec<String>>,
    #[serde(default)]
    pub infinite_mode: Option<bool>,
    #[serde(default)]
    pub show_archived: Option<bool>,
    #[serde(default)]
    pub sort_by: Option<String>,
    #[serde(default)]
    pub sort_order: Option<String>,
    #[serde(default)]
    pub page: Option<i64>,
    #[serde(default)]
    pub page_size: Option<i64>,
    #[serde(default)]
    pub simulation_search: Option<String>,
    #[serde(default)]
    pub scenario_search: Option<String>,
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
    pub total_count: Option<i64>,
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
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub can_delete: Option<bool>,
    #[serde(default)]
    pub can_duplicate: Option<bool>,
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
    pub is_inactive: Option<bool>,
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
    pub agent_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub creatable_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub total_count: Option<i64>,
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
    pub updated_at: Option<String>,
    #[serde(default)]
    pub can_edit: Option<bool>,
    #[serde(default)]
    pub can_duplicate: Option<bool>,
    #[serde(default)]
    pub can_delete: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageAttemptApiRequest {
    pub attempt_id: String,
    pub chat_id: String,
    pub message: String,
    #[serde(default)]
    pub parent_message_id: Option<String>,
    #[serde(default)]
    pub assistant_content: Option<String>,
    #[serde(default)]
    pub hints: Option<Vec<app__routes__attempt__message__HintEntry>>,
    #[serde(default)]
    pub contents: Option<Vec<app__routes__attempt__message__ContentEntry>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageAttemptApiResponse {
    pub chat_id: String,
    #[serde(default)]
    pub user_message_id: Option<String>,
    #[serde(default)]
    pub assistant_message_id: Option<String>,
    #[serde(default)]
    pub assistant_content: Option<String>,
    #[serde(default)]
    pub hints: Option<Vec<std::collections::HashMap<String, serde_json::Value>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageDataInput {
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
    pub contents: Option<Vec<app__infra__attempt__types__ContentEntry>>,
    #[serde(default)]
    pub feedbacks: Option<Vec<MessageFeedbackEntry>>,
    #[serde(default)]
    pub hints: Option<Vec<app__infra__attempt__types__HintEntry>>,
    #[serde(default)]
    pub parent_message_id: Option<String>,
    #[serde(default)]
    pub sibling_index: Option<i64>,
    #[serde(default)]
    pub sibling_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageDataOutput {
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
    pub contents: Option<Vec<ContentEntryOutput>>,
    #[serde(default)]
    pub feedbacks: Option<Vec<MessageFeedbackEntry>>,
    #[serde(default)]
    pub hints: Option<Vec<HintEntryOutput>>,
    #[serde(default)]
    pub parent_message_id: Option<String>,
    #[serde(default)]
    pub sibling_index: Option<i64>,
    #[serde(default)]
    pub sibling_count: Option<i64>,
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
pub struct ModelDepartmentSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelDescriptionSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub resource: Option<serde_json::Value>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelDraftFormState {
    pub flag_ids: Vec<String>,
    pub department_ids: Vec<String>,
    pub modality_ids: Vec<String>,
    pub pricing_ids: Vec<String>,
    pub provider_ids: Vec<String>,
    pub quality_ids: Vec<String>,
    pub reasoning_level_ids: Vec<String>,
    pub temperature_level_ids: Vec<String>,
    pub value_ids: Vec<String>,
    pub voice_ids: Vec<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelFieldError {
    pub field: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelFlagConfig {
    pub key: String,
    pub label: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub icon_id: Option<String>,
    #[serde(default)]
    pub flag_option_id: Option<String>,
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub generated: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelFlagSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<ModelFlagConfig>>,
    #[serde(default)]
    pub resources: Option<Vec<ModelFlagConfig>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelModalitySection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelNameSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub resource: Option<serde_json::Value>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelPricingSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelProviderSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub resource: Option<serde_json::Value>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelQualitySection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelReasoningLevelSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
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
pub struct ModelTemperatureLevelSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelValueSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub resource: Option<serde_json::Value>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelVoiceSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MvInfo {
    pub name: String,
    pub definition: String,
    pub columns: Vec<ColumnInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NextAttemptApiResponse {
    pub attempt_id: String,
    pub chat_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NextTestApiResponse {
    pub invocation_id: String,
    pub run_id: String,
    pub current_run: i64,
    pub total_runs: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectiveEntry {
    #[serde(default)]
    pub objective_id: Option<String>,
    #[serde(default)]
    pub objective: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationErrorEvent {
    pub message: String,
    #[serde(default)]
    pub error_type: Option<String>,
    #[serde(default)]
    pub artifact: Option<String>,
    #[serde(default)]
    pub operation: Option<String>,
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
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterDepartmentSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<ParameterDepartmentResource>>,
    #[serde(default)]
    pub resources: Option<Vec<ParameterDepartmentResource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterDescriptionResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterDescriptionSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub resource: Option<ParameterDescriptionResource>,
    #[serde(default)]
    pub resources: Option<Vec<ParameterDescriptionResource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterDraftFormState {
    pub flag_ids: Vec<String>,
    pub department_ids: Vec<String>,
    pub field_ids: Vec<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
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
    pub generated: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterFieldSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<ParameterFieldResource>>,
    #[serde(default)]
    pub resources: Option<Vec<ParameterFieldResource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterFlagConfig {
    pub key: String,
    pub label: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub icon_id: Option<String>,
    #[serde(default)]
    pub flag_option_id: Option<String>,
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub generated: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterFlagSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<ParameterFlagConfig>>,
    #[serde(default)]
    pub resources: Option<Vec<ParameterFlagConfig>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterNameResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterNameSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub resource: Option<ParameterNameResource>,
    #[serde(default)]
    pub resources: Option<Vec<ParameterNameResource>>,
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
    pub group_id: Option<String>,
    #[serde(default)]
    pub input_draft_id: Option<String>,
    #[serde(default)]
    pub expected_version: Option<i64>,
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
    pub model_ids: Option<Vec<String>>,
    #[serde(default)]
    pub tool_ids: Option<Vec<String>>,
    #[serde(default)]
    pub reasoning_level_ids: Option<Vec<String>>,
    #[serde(default)]
    pub temperature_level_ids: Option<Vec<String>>,
    #[serde(default)]
    pub voice_ids: Option<Vec<String>>,
    #[serde(default)]
    pub rubric_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchAgentDraftApiResponse {
    pub success: bool,
    pub draft_id: String,
    pub new_version: i64,
    pub message: String,
    #[serde(default)]
    pub form_state: Option<AgentDraftFormState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchAuthDraftApiRequest {
    #[serde(default)]
    pub input_draft_id: Option<String>,
    #[serde(default)]
    pub expected_version: Option<i64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub flag_id: Option<String>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub protocol_ids: Option<Vec<String>>,
    #[serde(default)]
    pub slug_ids: Option<Vec<String>>,
    #[serde(default)]
    pub item_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchAuthDraftApiResponse {
    pub success: bool,
    pub draft_id: String,
    pub new_version: i64,
    pub message: String,
    #[serde(default)]
    pub form_state: Option<AuthDraftFormState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchChatDraftApiRequest {
    #[serde(default)]
    pub input_draft_id: Option<String>,
    #[serde(default)]
    pub expected_version: Option<i64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub problem_statement: Option<String>,
    #[serde(default)]
    pub objectives: Option<Vec<String>>,
    #[serde(default)]
    pub images: Option<Vec<app__infra__chat__types__DraftImageValue>>,
    #[serde(default)]
    pub videos: Option<Vec<app__infra__chat__types__DraftVideoValue>>,
    #[serde(default)]
    pub questions: Option<Vec<app__infra__chat__types__DraftQuestionValue>>,
    #[serde(default)]
    pub options: Option<Vec<app__infra__chat__types__DraftOptionValue>>,
    #[serde(default)]
    pub name_ids: Option<Vec<String>>,
    #[serde(default)]
    pub description_ids: Option<Vec<String>>,
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
    pub problem_statement_ids: Option<Vec<String>>,
    #[serde(default)]
    pub question_ids: Option<Vec<String>>,
    #[serde(default)]
    pub scenario_ids: Option<Vec<String>>,
    #[serde(default)]
    pub video_ids: Option<Vec<String>>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchChatDraftApiResponse {
    pub success: bool,
    pub draft_id: String,
    pub new_version: i64,
    pub message: String,
    #[serde(default)]
    pub form_state: Option<ChatDraftFormState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchCohortDraftApiRequest {
    #[serde(default)]
    pub input_draft_id: Option<String>,
    #[serde(default)]
    pub expected_version: Option<i64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub flag_id: Option<String>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub simulation_ids: Option<Vec<String>>,
    #[serde(default)]
    pub profile_ids: Option<Vec<String>>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchCohortDraftApiResponse {
    pub success: bool,
    pub draft_id: String,
    pub new_version: i64,
    pub message: String,
    #[serde(default)]
    pub form_state: Option<CohortDraftFormState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchDepartmentDraftApiRequest {
    #[serde(default)]
    pub input_draft_id: Option<String>,
    #[serde(default)]
    pub expected_version: Option<i64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub flag_id: Option<String>,
    #[serde(default)]
    pub setting_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchDepartmentDraftApiResponse {
    pub success: bool,
    pub draft_id: String,
    pub new_version: i64,
    pub message: String,
    #[serde(default)]
    pub form_state: Option<DepartmentDraftFormState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchDocumentDraftApiRequest {
    #[serde(default)]
    pub input_draft_id: Option<String>,
    #[serde(default)]
    pub expected_version: Option<i64>,
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
    pub flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub image_ids: Option<Vec<String>>,
    #[serde(default)]
    pub parameter_field_ids: Option<Vec<String>>,
    #[serde(default)]
    pub parameter_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchDocumentDraftApiResponse {
    pub success: bool,
    pub draft_id: String,
    pub new_version: i64,
    pub message: String,
    #[serde(default)]
    pub form_state: Option<DocumentDraftFormState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchEvalDraftApiRequest {
    #[serde(default)]
    pub input_draft_id: Option<String>,
    #[serde(default)]
    pub expected_version: Option<i64>,
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
    pub model_ids: Option<Vec<String>>,
    #[serde(default)]
    pub rubric_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchEvalDraftApiResponse {
    pub success: bool,
    pub draft_id: String,
    pub new_version: i64,
    pub message: String,
    #[serde(default)]
    pub form_state: Option<EvalDraftFormState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchFieldDraftApiRequest {
    #[serde(default)]
    pub input_draft_id: Option<String>,
    #[serde(default)]
    pub expected_version: Option<i64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub flag_id: Option<String>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub conditional_parameter_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchFieldDraftApiResponse {
    pub success: bool,
    pub draft_id: String,
    pub new_version: i64,
    pub message: String,
    #[serde(default)]
    pub form_state: Option<FieldDraftFormState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchModelDraftApiRequest {
    #[serde(default)]
    pub input_draft_id: Option<String>,
    #[serde(default)]
    pub expected_version: Option<i64>,
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
    pub modality_ids: Option<Vec<String>>,
    #[serde(default)]
    pub pricing_ids: Option<Vec<String>>,
    #[serde(default)]
    pub provider_ids: Option<Vec<String>>,
    #[serde(default)]
    pub quality_ids: Option<Vec<String>>,
    #[serde(default)]
    pub reasoning_level_ids: Option<Vec<String>>,
    #[serde(default)]
    pub temperature_level_ids: Option<Vec<String>>,
    #[serde(default)]
    pub value_ids: Option<Vec<String>>,
    #[serde(default)]
    pub voice_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchModelDraftApiResponse {
    pub success: bool,
    pub draft_id: String,
    pub new_version: i64,
    pub message: String,
    #[serde(default)]
    pub form_state: Option<ModelDraftFormState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchParameterDraftApiRequest {
    #[serde(default)]
    pub input_draft_id: Option<String>,
    #[serde(default)]
    pub expected_version: Option<i64>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchParameterDraftApiResponse {
    pub success: bool,
    pub draft_id: String,
    pub new_version: i64,
    pub message: String,
    #[serde(default)]
    pub form_state: Option<ParameterDraftFormState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchPersonaDraftApiRequest {
    #[serde(default)]
    pub input_draft_id: Option<String>,
    #[serde(default)]
    pub expected_version: Option<i64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub instructions: Option<String>,
    #[serde(default)]
    pub instructions_id: Option<String>,
    #[serde(default)]
    pub examples: Option<Vec<String>>,
    #[serde(default)]
    pub example_ids: Option<Vec<String>>,
    #[serde(default)]
    pub color_id: Option<String>,
    #[serde(default)]
    pub icon_id: Option<String>,
    #[serde(default)]
    pub flag_id: Option<String>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub parameter_field_ids: Option<Vec<String>>,
    #[serde(default)]
    pub voice_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchPersonaDraftApiResponse {
    pub success: bool,
    pub draft_id: String,
    pub new_version: i64,
    pub message: String,
    pub form_state: DraftFormState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchProfileDraftApiRequest {
    #[serde(default)]
    pub input_draft_id: Option<String>,
    #[serde(default)]
    pub expected_version: Option<i64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub request_limit: Option<i64>,
    #[serde(default)]
    pub flag_id: Option<String>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub email_ids: Option<Vec<String>>,
    #[serde(default)]
    pub role_ids: Option<Vec<String>>,
    #[serde(default)]
    pub request_limit_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchProfileDraftApiResponse {
    pub success: bool,
    pub draft_id: String,
    pub new_version: i64,
    pub message: String,
    #[serde(default)]
    pub form_state: Option<ProfileDraftFormState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchProviderDraftApiRequest {
    #[serde(default)]
    pub input_draft_id: Option<String>,
    #[serde(default)]
    pub expected_version: Option<i64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub flag_id: Option<String>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub endpoint_ids: Option<Vec<String>>,
    #[serde(default)]
    pub key_ids: Option<Vec<String>>,
    #[serde(default)]
    pub value_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchProviderDraftApiResponse {
    pub success: bool,
    pub draft_id: String,
    pub new_version: i64,
    pub message: String,
    #[serde(default)]
    pub form_state: Option<ProviderDraftFormState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchRubricDraftApiRequest {
    #[serde(default)]
    pub input_draft_id: Option<String>,
    #[serde(default)]
    pub expected_version: Option<i64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub flag_id: Option<String>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub point_ids: Option<Vec<String>>,
    #[serde(default)]
    pub standard_group_ids: Option<Vec<String>>,
    #[serde(default)]
    pub standard_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchRubricDraftApiResponse {
    pub success: bool,
    pub draft_id: String,
    pub new_version: i64,
    pub message: String,
    #[serde(default)]
    pub form_state: Option<RubricDraftFormState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchScenarioDraftApiRequest {
    #[serde(default)]
    pub input_draft_id: Option<String>,
    #[serde(default)]
    pub expected_version: Option<i64>,
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
    pub images: Option<Vec<app__infra__scenario__types__DraftImageValue>>,
    #[serde(default)]
    pub image_ids: Option<Vec<String>>,
    #[serde(default)]
    pub videos: Option<Vec<app__infra__scenario__types__DraftVideoValue>>,
    #[serde(default)]
    pub video_ids: Option<Vec<String>>,
    #[serde(default)]
    pub questions: Option<Vec<app__infra__scenario__types__DraftQuestionValue>>,
    #[serde(default)]
    pub question_ids: Option<Vec<String>>,
    #[serde(default)]
    pub options: Option<Vec<app__infra__scenario__types__DraftOptionValue>>,
    #[serde(default)]
    pub option_ids: Option<Vec<String>>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchScenarioDraftApiResponse {
    pub success: bool,
    pub draft_id: String,
    pub new_version: i64,
    pub message: String,
    pub form_state: ScenarioDraftFormState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchSettingDraftApiRequest {
    #[serde(default)]
    pub input_draft_id: Option<String>,
    #[serde(default)]
    pub expected_version: Option<i64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub flag_id: Option<String>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub color_ids: Option<Vec<String>>,
    #[serde(default)]
    pub profile_ids: Option<Vec<String>>,
    #[serde(default)]
    pub auth_ids: Option<Vec<String>>,
    #[serde(default)]
    pub provider_key_ids: Option<Vec<String>>,
    #[serde(default)]
    pub auth_item_key_ids: Option<Vec<String>>,
    #[serde(default)]
    pub threshold_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchSettingDraftApiResponse {
    pub success: bool,
    pub draft_id: String,
    pub new_version: i64,
    pub message: String,
    #[serde(default)]
    pub form_state: Option<SettingDraftFormState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchSimulationDraftApiRequest {
    #[serde(default)]
    pub input_draft_id: Option<String>,
    #[serde(default)]
    pub expected_version: Option<i64>,
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
    pub scenario_ids: Option<Vec<String>>,
    #[serde(default)]
    pub scenario_flag_ids: Option<Vec<String>>,
    #[serde(default)]
    pub scenario_flags: Option<Vec<DraftScenarioFlagValue>>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchSimulationDraftApiResponse {
    pub success: bool,
    pub draft_id: String,
    pub new_version: i64,
    pub message: String,
    #[serde(default)]
    pub form_state: Option<SimulationDraftFormState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchToolDraftApiRequest {
    #[serde(default)]
    pub input_draft_id: Option<String>,
    #[serde(default)]
    pub expected_version: Option<i64>,
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
    pub arg_ids: Option<Vec<String>>,
    #[serde(default)]
    pub arg_position_ids: Option<Vec<String>>,
    #[serde(default)]
    pub args_output_ids: Option<Vec<String>>,
    #[serde(default)]
    pub artifact_ids: Option<Vec<String>>,
    #[serde(default)]
    pub operation_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchToolDraftApiResponse {
    pub success: bool,
    pub draft_id: String,
    pub new_version: i64,
    pub message: String,
    #[serde(default)]
    pub form_state: Option<ToolDraftFormState>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaColorSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub resource: Option<PersonaColorResource>,
    #[serde(default)]
    pub resources: Option<Vec<PersonaColorResource>>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaDepartmentSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<PersonaDepartmentResource>>,
    #[serde(default)]
    pub resources: Option<Vec<PersonaDepartmentResource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaDescriptionResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaDescriptionSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub resource: Option<PersonaDescriptionResource>,
    #[serde(default)]
    pub resources: Option<Vec<PersonaDescriptionResource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaEntry {
    #[serde(default)]
    pub id: Option<String>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaExampleSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<PersonaExampleResource>>,
    #[serde(default)]
    pub resources: Option<Vec<PersonaExampleResource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaFieldError {
    pub field: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaFlagConfig {
    pub key: String,
    pub label: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub icon_id: Option<String>,
    #[serde(default)]
    pub flag_option_id: Option<String>,
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub generated: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaFlagSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<PersonaFlagConfig>,
    #[serde(default)]
    pub resources: Option<Vec<PersonaFlagConfig>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaGenerationProgressEvent {
    #[serde(default)]
    pub artifact_type: Option<String>,
    #[serde(default)]
    pub resource_type: Option<String>,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub artifact_id: Option<String>,
    #[serde(default)]
    pub run_id: Option<String>,
    #[serde(default)]
    pub success: Option<bool>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub error_stage: Option<String>,
    #[serde(default)]
    pub tool_call_id: Option<String>,
    #[serde(default)]
    pub tool_name: Option<String>,
    #[serde(default)]
    pub arguments_delta: Option<String>,
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub example: Option<String>,
    #[serde(default)]
    pub template: Option<String>,
    #[serde(default)]
    pub hex_code: Option<String>,
    #[serde(default)]
    pub value: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub department_ids: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub setting_ids: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub field_id: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
    #[serde(default)]
    pub parameter_id: Option<String>,
    #[serde(default)]
    pub conditional_parameter_id: Option<String>,
    #[serde(default)]
    pub conditional_parameter_ids: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub voice: Option<String>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaIconSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub resource: Option<PersonaIconResource>,
    #[serde(default)]
    pub resources: Option<Vec<PersonaIconResource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaInstructionResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub template: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaInstructionSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub resource: Option<PersonaInstructionResource>,
    #[serde(default)]
    pub resources: Option<Vec<PersonaInstructionResource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaNameResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaNameSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub resource: Option<PersonaNameResource>,
    #[serde(default)]
    pub resources: Option<Vec<PersonaNameResource>>,
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
    pub generated: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaParameterFieldSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<PersonaParameterFieldResource>>,
    #[serde(default)]
    pub resources: Option<Vec<PersonaParameterFieldResource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaParameterSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<GetParameterResponse>>,
    #[serde(default)]
    pub resources: Option<Vec<GetParameterResponse>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaResultItem {
    pub success: bool,
    pub message: String,
    #[serde(default)]
    pub persona_id: Option<String>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaVoiceSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<PersonaVoiceResource>>,
    #[serde(default)]
    pub resources: Option<Vec<PersonaVoiceResource>>,
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
pub struct PricingDailyItemInput {
    pub date_key: String,
    #[serde(default)]
    pub model_id: Option<String>,
    #[serde(default)]
    pub total_cost: Option<serde_json::Value>,
    #[serde(default)]
    pub run_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingDailyItemOutput {
    pub date_key: String,
    #[serde(default)]
    pub model_id: Option<String>,
    #[serde(default)]
    pub total_cost: Option<String>,
    #[serde(default)]
    pub run_count: Option<i64>,
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
    pub agent_names: Option<Vec<String>>,
    #[serde(default)]
    pub model_names: Option<Vec<String>>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingResources {
    #[serde(default)]
    pub agents: Option<
        std::collections::HashMap<String, std::collections::HashMap<String, serde_json::Value>>,
    >,
    #[serde(default)]
    pub models: Option<
        std::collections::HashMap<String, std::collections::HashMap<String, serde_json::Value>>,
    >,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingResponseInput {
    #[serde(default)]
    pub daily: Option<Vec<PricingDailyItemInput>>,
    #[serde(default)]
    pub resources: Option<PricingResources>,
    #[serde(default)]
    pub total_count: Option<i64>,
    #[serde(default)]
    pub model_options: Option<Vec<FilterOption>>,
    #[serde(default)]
    pub agent_options: Option<Vec<FilterOption>>,
    #[serde(default)]
    pub analytics: Option<AnalyticsFacetsInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingResponseOutput {
    #[serde(default)]
    pub daily: Option<Vec<PricingDailyItemOutput>>,
    #[serde(default)]
    pub resources: Option<PricingResources>,
    #[serde(default)]
    pub total_count: Option<i64>,
    #[serde(default)]
    pub model_options: Option<Vec<FilterOption>>,
    #[serde(default)]
    pub agent_options: Option<Vec<FilterOption>>,
    #[serde(default)]
    pub analytics: Option<AnalyticsFacetsOutput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimaryPersonaPerformanceInput {
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
pub struct PrimaryPersonaPerformanceOutput {
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
pub struct PrimaryRubricHeatmapInput {
    #[serde(default)]
    pub matrices: Option<Vec<RubricHeatmapMatrixInput>>,
    #[serde(default)]
    pub valid_rubric_ids: Option<Vec<String>>,
    #[serde(default)]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimaryRubricHeatmapOutput {
    #[serde(default)]
    pub matrices: Option<Vec<RubricHeatmapMatrixOutput>>,
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
pub struct ProblemStatementEntry {
    #[serde(default)]
    pub problem_statement_id: Option<String>,
    #[serde(default)]
    pub problem_statement: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileContextApiResponse {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub role: Option<String>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub role_artifacts: Option<Vec<String>>,
    #[serde(default)]
    pub scoped_roles: Option<Vec<String>>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub primary_department_id: Option<String>,
    #[serde(default)]
    pub settings_id: Option<String>,
    #[serde(default)]
    pub theme: Option<ThemePrimitives>,
    #[serde(default)]
    pub session_id: Option<String>,
    #[serde(default)]
    pub is_emulation: Option<bool>,
    #[serde(default)]
    pub emulation_depth: Option<i64>,
    #[serde(default)]
    pub role_resources: Option<Vec<QGetProfileContextV4RoleResource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileDepartmentResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileDepartmentSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<ProfileDepartmentResource>>,
    #[serde(default)]
    pub resources: Option<Vec<ProfileDepartmentResource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileDraftFormState {
    pub department_ids: Vec<String>,
    pub email_ids: Vec<String>,
    pub role_ids: Vec<String>,
    pub request_limit_ids: Vec<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub flag_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileEmailResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileEmailSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<ProfileEmailResource>>,
    #[serde(default)]
    pub resources: Option<Vec<ProfileEmailResource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileFieldError {
    pub field: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileFlagConfig {
    pub key: String,
    pub label: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub icon_id: Option<String>,
    #[serde(default)]
    pub flag_option_id: Option<String>,
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub generated: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileFlagSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<ProfileFlagConfig>,
    #[serde(default)]
    pub resources: Option<Vec<ProfileFlagConfig>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileNameResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileNameSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub resource: Option<ProfileNameResource>,
    #[serde(default)]
    pub resources: Option<Vec<ProfileNameResource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileRequestLimitResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub requests_per_day: Option<i64>,
    #[serde(default)]
    pub generated: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileRequestLimitSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub resource: Option<ProfileRequestLimitResource>,
    #[serde(default)]
    pub resources: Option<Vec<ProfileRequestLimitResource>>,
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
    pub icon_value: Option<String>,
    #[serde(default)]
    pub color_hex: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileRoleSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<ProfileRoleResource>>,
    #[serde(default)]
    pub resources: Option<Vec<ProfileRoleResource>>,
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
pub struct ProviderDepartmentSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderDescriptionSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub resource: Option<serde_json::Value>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderDraftFormState {
    pub department_ids: Vec<String>,
    pub endpoint_ids: Vec<String>,
    pub key_ids: Vec<String>,
    pub value_ids: Vec<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub flag_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderEndpointSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub resource: Option<serde_json::Value>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderFieldError {
    pub field: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderFlagConfig {
    pub key: String,
    pub label: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub icon_id: Option<String>,
    #[serde(default)]
    pub flag_option_id: Option<String>,
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub generated: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderFlagSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<ProviderFlagConfig>>,
    #[serde(default)]
    pub resources: Option<Vec<ProviderFlagConfig>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderKeySection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub resource: Option<serde_json::Value>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderNameSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub resource: Option<serde_json::Value>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
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
pub struct ProviderValueSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub resource: Option<serde_json::Value>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
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
pub struct RecordRequest {
    pub target_profile_id: String,
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
    pub actor_profile_id: Option<String>,
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
    #[serde(default)]
    pub history_practice: Option<bool>,
    #[serde(default)]
    pub history_scenario_ids: Option<Vec<String>>,
    #[serde(default)]
    pub history_infinite_mode: Option<bool>,
    #[serde(default)]
    pub history_show_archived: Option<bool>,
    #[serde(default)]
    pub history_sort_by: Option<String>,
    #[serde(default)]
    pub history_sort_order: Option<String>,
    #[serde(default)]
    pub history_page: Option<i64>,
    #[serde(default)]
    pub history_page_size: Option<i64>,
    #[serde(default)]
    pub history_simulation_search: Option<String>,
    #[serde(default)]
    pub history_scenario_search: Option<String>,
    #[serde(default)]
    pub history_profile_search: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshResponse {
    pub success: bool,
    pub refreshed_views: Vec<String>,
    pub invalidated_tags: Vec<String>,
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
    pub roles: Option<Vec<String>>,
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
    pub analytics: Option<AnalyticsFacetsOutput>,
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
pub struct ResolveProblemRequest {
    pub problem_id: String,
    #[serde(default)]
    pub resolved: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolveProblemResponse {
    pub problem_id: String,
    pub resolved: bool,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceTypeItem {
    pub name: String,
    pub operation: String,
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
pub struct RubricDepartmentSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RubricDescriptionSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub resource: Option<serde_json::Value>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RubricDraftFormState {
    pub department_ids: Vec<String>,
    pub point_ids: Vec<String>,
    pub standard_group_ids: Vec<String>,
    pub standard_ids: Vec<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub flag_id: Option<String>,
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
pub struct RubricFlagConfig {
    pub key: String,
    pub label: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub icon_id: Option<String>,
    #[serde(default)]
    pub flag_option_id: Option<String>,
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub generated: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RubricFlagSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<RubricFlagConfig>>,
    #[serde(default)]
    pub resources: Option<Vec<RubricFlagConfig>>,
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
pub struct RubricHeatmapMatrixInput {
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
pub struct RubricHeatmapMatrixOutput {
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
pub struct RubricNameSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub resource: Option<serde_json::Value>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RubricPointsSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub resource: Option<serde_json::Value>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
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
pub struct RubricStandardGroupsSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RubricStandardsSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
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
pub struct RunTestApiResponse {
    pub test_id: String,
    pub invocation_id: String,
    pub run_id: String,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioDepartmentSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<ScenarioDepartment>>,
    #[serde(default)]
    pub resources: Option<Vec<ScenarioDepartment>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioDescriptionResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioDescriptionSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub resource: Option<ScenarioDescriptionResource>,
    #[serde(default)]
    pub resources: Option<Vec<ScenarioDescriptionResource>>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioDocumentSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<ScenarioDocument>>,
    #[serde(default)]
    pub resources: Option<Vec<ScenarioDocument>>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioFieldError {
    pub field: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioFieldParamFilter {
    #[serde(default)]
    pub parameter_id: Option<String>,
    #[serde(default)]
    pub show_selected: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioFlagConfig {
    pub key: String,
    pub label: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub icon_id: Option<String>,
    #[serde(default)]
    pub flag_option_id: Option<String>,
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub generated: Option<bool>,
    #[serde(default)]
    pub video_flag: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioFlagSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<ScenarioFlagConfig>>,
    #[serde(default)]
    pub resources: Option<Vec<ScenarioFlagConfig>>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioImageSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<ScenarioImage>>,
    #[serde(default)]
    pub resources: Option<Vec<ScenarioImage>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioNameResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioNameSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub resource: Option<ScenarioNameResource>,
    #[serde(default)]
    pub resources: Option<Vec<ScenarioNameResource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioObjective {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub objective: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioObjectiveSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<ScenarioObjective>>,
    #[serde(default)]
    pub resources: Option<Vec<ScenarioObjective>>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioOptionSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<ScenarioOption>>,
    #[serde(default)]
    pub resources: Option<Vec<ScenarioOption>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioParameter {
    #[serde(default)]
    pub parameter_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub document_parameter: Option<bool>,
    #[serde(default)]
    pub persona_parameter: Option<bool>,
    #[serde(default)]
    pub scenario_parameter: Option<bool>,
    #[serde(default)]
    pub video_parameter: Option<bool>,
    #[serde(default)]
    pub non_video_parameter: Option<bool>,
    #[serde(default)]
    pub conditional: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioParameterFieldSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<ScenarioField>>,
    #[serde(default)]
    pub resources: Option<Vec<ScenarioField>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioParameterSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<ScenarioParameter>>,
    #[serde(default)]
    pub resources: Option<Vec<ScenarioParameter>>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioPersonaSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<ScenarioPersona>>,
    #[serde(default)]
    pub resources: Option<Vec<ScenarioPersona>>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioProblemStatementSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub resource: Option<ScenarioProblemStatement>,
    #[serde(default)]
    pub resources: Option<Vec<ScenarioProblemStatement>>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioQuestionSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<ScenarioQuestion>>,
    #[serde(default)]
    pub resources: Option<Vec<ScenarioQuestion>>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioVideoSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<ScenarioVideo>>,
    #[serde(default)]
    pub resources: Option<Vec<ScenarioVideo>>,
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
    pub search: Option<String>,
    #[serde(default)]
    pub simulation_ids: Option<Vec<String>>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub practice: Option<bool>,
    #[serde(default)]
    pub is_archived: Option<bool>,
    #[serde(default)]
    pub infinite_mode: Option<bool>,
    #[serde(default)]
    pub start_date: Option<String>,
    #[serde(default)]
    pub end_date: Option<String>,
    #[serde(default)]
    pub simulation_search: Option<String>,
    #[serde(default)]
    pub department_search: Option<String>,
    #[serde(default)]
    pub page_size: Option<i64>,
    #[serde(default)]
    pub page_offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchAttemptApiResponse {
    #[serde(default)]
    pub actor_name: Option<String>,
    #[serde(default)]
    pub attempts: Option<Vec<SearchAttemptItem>>,
    #[serde(default)]
    pub simulation_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub department_filter: Option<ListFilterSection>,
    #[serde(default)]
    pub total_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchAttemptItem {
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
    pub department_id: Option<String>,
    #[serde(default)]
    pub cohort_id: Option<String>,
    #[serde(default)]
    pub practice: Option<bool>,
    #[serde(default)]
    pub infinite_mode: Option<bool>,
    #[serde(default)]
    pub num_chats: Option<i64>,
    #[serde(default)]
    pub is_archived: Option<bool>,
    #[serde(default)]
    pub scenario_ids: Option<Vec<String>>,
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
    pub text_upload_ids: Vec<String>,
    pub audio_upload_ids: Vec<String>,
    pub image_upload_ids: Vec<String>,
    pub video_upload_ids: Vec<String>,
    pub file_upload_ids: Vec<String>,
    pub call_upload_ids: Vec<String>,
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
    pub filter_agent_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_creatable: Option<Vec<String>>,
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
pub struct SecondarySkillPerformanceInput {
    #[serde(default)]
    pub packages: Option<Vec<SecondarySkillPackage>>,
    #[serde(default)]
    pub valid_rubric_ids: Option<Vec<String>>,
    #[serde(default)]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecondarySkillPerformanceOutput {
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
pub struct SettingAuthItemKeySection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingAuthSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingColorSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingDepartmentSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingDescriptionSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub resource: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingDraftFormState {
    pub department_ids: Vec<String>,
    pub color_ids: Vec<String>,
    pub profile_ids: Vec<String>,
    pub auth_ids: Vec<String>,
    pub provider_key_ids: Vec<String>,
    pub auth_item_key_ids: Vec<String>,
    pub threshold_ids: Vec<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub flag_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingFieldError {
    pub field: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingFlagConfig {
    pub key: String,
    pub label: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub icon_id: Option<String>,
    #[serde(default)]
    pub flag_option_id: Option<String>,
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub generated: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingFlagSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<SettingFlagConfig>,
    #[serde(default)]
    pub resources: Option<Vec<SettingFlagConfig>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingNameSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub resource: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingProfileSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingProviderKeySection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
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
pub struct SettingSystemSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationDepartmentSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<SimulationDepartment>>,
    #[serde(default)]
    pub resources: Option<Vec<SimulationDepartment>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationDescriptionResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationDescriptionSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub resource: Option<SimulationDescriptionResource>,
    #[serde(default)]
    pub resources: Option<Vec<SimulationDescriptionResource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationDraftFormState {
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationFieldError {
    pub field: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationFlagConfig {
    pub key: String,
    pub label: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub icon_id: Option<String>,
    #[serde(default)]
    pub flag_option_id: Option<String>,
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub generated: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationFlagSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<SimulationFlagConfig>>,
    #[serde(default)]
    pub resources: Option<Vec<SimulationFlagConfig>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationNameResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationNameSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub resource: Option<SimulationNameResource>,
    #[serde(default)]
    pub resources: Option<Vec<SimulationNameResource>>,
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
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub generated: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationScenarioFlagSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<SimulationScenarioFlag>>,
    #[serde(default)]
    pub resources: Option<Vec<SimulationScenarioFlag>>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationScenarioPositionSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<SimulationScenarioPosition>>,
    #[serde(default)]
    pub resources: Option<Vec<SimulationScenarioPosition>>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationScenarioRubricSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<SimulationScenarioRubric>>,
    #[serde(default)]
    pub resources: Option<Vec<SimulationScenarioRubric>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationScenarioSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<SimulationScenario>>,
    #[serde(default)]
    pub resources: Option<Vec<SimulationScenario>>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationScenarioTimeLimitSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<SimulationScenarioTimeLimit>>,
    #[serde(default)]
    pub resources: Option<Vec<SimulationScenarioTimeLimit>>,
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
pub struct StartAttemptApiResponse {
    pub attempt_id: String,
    #[serde(default)]
    pub chat_entry_id: Option<String>,
    #[serde(default)]
    pub attempt_chat_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartTestApiResponse {
    pub test_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StopAttemptApiResponse {
    pub chat_id: String,
    pub success: bool,
    #[serde(default)]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StopTestApiResponse {
    pub invocation_id: String,
    pub success: bool,
    #[serde(default)]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuiteDepartmentSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuiteDescriptionSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuiteEndpointSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuiteFlagSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuiteKeySection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuiteModalitySection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuiteNameSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuitePricingSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuiteQualitySection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuiteReasoningLevelSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuiteTemperatureLevelSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuiteValueSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuiteVoiceSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableInfo {
    pub name: String,
    pub columns: Vec<ColumnInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestAllCompleteEvent {
    pub invocation_id: String,
    pub total_runs: i64,
    #[serde(default)]
    pub success: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestEndAllPayload {
    pub test_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestEndPayload {
    pub test_id: String,
    pub test_invocation_id: String,
    pub run_id: String,
    #[serde(default)]
    pub grade: Option<bool>,
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
    pub groups: Option<Vec<GetTestInvocationGroupsResponse>>,
    #[serde(default)]
    pub grades: Option<Vec<GetTestGradeResponse>>,
    #[serde(default)]
    pub feedback: Option<Vec<GetTestFeedbackResponse>>,
    #[serde(default)]
    pub messages: Option<Vec<SearchMessageResponse>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestGroupPayload {
    pub test_id: String,
    pub test_invocation_id: String,
    #[serde(default)]
    pub prev_run_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestJoinPayload {
    pub invocation_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestJoinRequest {
    pub sid: String,
    pub invocation_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestJoinResponse {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestJoinedEvent {
    pub invocation_id: String,
    #[serde(default)]
    pub success: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestLeavePayload {
    pub invocation_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestLeaveRequest {
    pub sid: String,
    pub invocation_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestLeaveResponse {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestNextPayload {
    pub test_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestProgressEvent {
    pub invocation_id: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(default)]
    pub run_id: Option<String>,
    #[serde(default)]
    pub current_run: Option<i64>,
    #[serde(default)]
    pub total_runs: Option<i64>,
    #[serde(default)]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResources {
    #[serde(default)]
    pub evals: Option<
        std::collections::HashMap<String, std::collections::HashMap<String, serde_json::Value>>,
    >,
    #[serde(default)]
    pub rubrics: Option<
        std::collections::HashMap<String, std::collections::HashMap<String, serde_json::Value>>,
    >,
    #[serde(default)]
    pub agents: Option<
        std::collections::HashMap<String, std::collections::HashMap<String, serde_json::Value>>,
    >,
    #[serde(default)]
    pub models: Option<
        std::collections::HashMap<String, std::collections::HashMap<String, serde_json::Value>>,
    >,
    #[serde(default)]
    pub voices: Option<
        std::collections::HashMap<String, std::collections::HashMap<String, serde_json::Value>>,
    >,
    #[serde(default)]
    pub temperature_levels: Option<
        std::collections::HashMap<String, std::collections::HashMap<String, serde_json::Value>>,
    >,
    #[serde(default)]
    pub reasoning_levels: Option<
        std::collections::HashMap<String, std::collections::HashMap<String, serde_json::Value>>,
    >,
    #[serde(default)]
    pub modalities: Option<
        std::collections::HashMap<String, std::collections::HashMap<String, serde_json::Value>>,
    >,
    #[serde(default)]
    pub prompts: Option<
        std::collections::HashMap<String, std::collections::HashMap<String, serde_json::Value>>,
    >,
    #[serde(default)]
    pub instructions: Option<
        std::collections::HashMap<String, std::collections::HashMap<String, serde_json::Value>>,
    >,
    #[serde(default)]
    pub tools: Option<
        std::collections::HashMap<String, std::collections::HashMap<String, serde_json::Value>>,
    >,
    #[serde(default)]
    pub qualities: Option<
        std::collections::HashMap<String, std::collections::HashMap<String, serde_json::Value>>,
    >,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestRunCompleteEvent {
    pub invocation_id: String,
    pub run_id: String,
    pub current_run: i64,
    pub total_runs: i64,
    pub remaining_runs: i64,
    #[serde(default)]
    pub original_run_resource_id: Option<String>,
    #[serde(default)]
    pub tool_calls: Option<Vec<std::collections::HashMap<String, serde_json::Value>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestRunDeltaEvent {
    pub invocation_id: String,
    pub run_id: String,
    pub content: String,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestRunStartEvent {
    pub invocation_id: String,
    pub run_id: String,
    pub current_run: i64,
    pub total_runs: i64,
    pub created_at: String,
    #[serde(default)]
    pub original_run_resource_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestStartPayload {
    pub benchmark_id: String,
    #[serde(default)]
    pub infinite_mode: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestStartedEvent {
    pub test_id: String,
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
pub struct TestStoppedEvent {
    pub invocation_id: String,
    #[serde(default)]
    pub success: Option<bool>,
    #[serde(default)]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemePrimitives {
    #[serde(default)]
    pub primary: Option<String>,
    #[serde(default)]
    pub accent: Option<String>,
    #[serde(default)]
    pub background: Option<String>,
    #[serde(default)]
    pub surface: Option<String>,
    #[serde(default)]
    pub success: Option<String>,
    #[serde(default)]
    pub warning: Option<String>,
    #[serde(default)]
    pub error: Option<String>,
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
pub struct ToolArgOutputSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolArgPositionSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolArgSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolArtifactSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDescriptionSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub resource: Option<serde_json::Value>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDraftFormState {
    pub flag_ids: Vec<String>,
    pub department_ids: Vec<String>,
    pub arg_ids: Vec<String>,
    pub arg_position_ids: Vec<String>,
    pub args_output_ids: Vec<String>,
    pub artifact_ids: Vec<String>,
    pub operation_ids: Vec<String>,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolFieldError {
    pub field: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolFlagConfig {
    pub key: String,
    pub label: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub icon_id: Option<String>,
    #[serde(default)]
    pub flag_option_id: Option<String>,
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub generated: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolFlagSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<ToolFlagConfig>,
    #[serde(default)]
    pub resources: Option<Vec<ToolFlagConfig>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolNameSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub resource: Option<serde_json::Value>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolOperationSection {
    #[serde(default)]
    pub show: Option<bool>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub suggestions: Option<Vec<String>>,
    #[serde(default)]
    pub show_ai_generate: Option<bool>,
    #[serde(default)]
    pub tool_id: Option<String>,
    #[serde(default)]
    pub link_tool_id: Option<String>,
    #[serde(default)]
    pub current: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub resources: Option<Vec<serde_json::Value>>,
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
pub struct UnemulateProfileApiResponse {
    pub ok: bool,
    #[serde(default)]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAgentApiRequest {
    pub agents: Vec<UpdateAgentItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAgentApiResponseInput {
    pub results: Vec<AgentResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAgentApiResponseOutput {
    pub results: Vec<AgentResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAgentItem {
    pub agent_id: String,
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
    pub model_ids: Option<Vec<String>>,
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
    pub auths: Vec<UpdateAuthItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAuthApiResponseInput {
    pub results: Vec<AuthResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAuthApiResponseOutput {
    pub results: Vec<AuthResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAuthItem {
    pub auth_id: String,
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
    pub active_flag_id: Option<String>,
    #[serde(default)]
    pub active_flag: Option<bool>,
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
    pub cohorts: Vec<UpdateCohortItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCohortApiResponseInput {
    pub results: Vec<CohortResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCohortApiResponseOutput {
    pub results: Vec<CohortResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCohortItem {
    pub cohort_id: String,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub flag_id: Option<String>,
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
    pub is_inactive: Option<bool>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub simulations: Option<Vec<String>>,
    #[serde(default)]
    pub profiles: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDepartmentApiRequest {
    pub departments: Vec<UpdateDepartmentItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDepartmentApiResponseInput {
    pub results: Vec<DepartmentResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDepartmentApiResponseOutput {
    pub results: Vec<DepartmentResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDepartmentItem {
    pub department_id: String,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub active_flag_id: Option<String>,
    #[serde(default)]
    pub active_flag: Option<bool>,
    #[serde(default)]
    pub settings_ids: Option<Vec<String>>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDocumentApiRequest {
    pub documents: Vec<UpdateDocumentItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDocumentApiResponseInput {
    pub results: Vec<DocumentResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDocumentApiResponseOutput {
    pub results: Vec<DocumentResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDocumentItem {
    pub document_id: String,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub flag_id: Option<String>,
    #[serde(default)]
    pub is_inactive: Option<bool>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub field_ids: Option<Vec<String>>,
    #[serde(default)]
    pub upload_ids: Option<Vec<String>>,
    #[serde(default)]
    pub image_ids: Option<Vec<String>>,
    #[serde(default)]
    pub text_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateEvalApiRequest {
    pub evals: Vec<UpdateEvalItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateEvalApiResponseInput {
    pub results: Vec<EvalResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateEvalApiResponseOutput {
    pub results: Vec<EvalResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateEvalItem {
    pub eval_id: String,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFieldApiRequest {
    pub fields: Vec<UpdateFieldItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFieldApiResponseInput {
    pub results: Vec<FieldResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFieldApiResponseOutput {
    pub results: Vec<FieldResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFieldItem {
    pub field_id: String,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub flag_id: Option<String>,
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
    pub models: Vec<UpdateModelItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateModelApiResponseInput {
    pub results: Vec<ModelResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateModelApiResponseOutput {
    pub results: Vec<ModelResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateModelItem {
    pub model_id: String,
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
    pub provider_ids: Option<Vec<String>>,
    #[serde(default)]
    pub quality_ids: Option<Vec<String>>,
    #[serde(default)]
    pub reasoning_level_ids: Option<Vec<String>>,
    #[serde(default)]
    pub temperature_level_ids: Option<Vec<String>>,
    #[serde(default)]
    pub value_ids: Option<Vec<String>>,
    #[serde(default)]
    pub voice_ids: Option<Vec<String>>,
    #[serde(default)]
    pub model_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateParameterApiRequest {
    pub parameters: Vec<UpdateParameterItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateParameterApiResponseInput {
    pub results: Vec<ParameterResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateParameterApiResponseOutput {
    pub results: Vec<ParameterResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateParameterItem {
    pub parameter_id: String,
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
    pub personas: Vec<UpdatePersonaItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePersonaApiResponseInput {
    pub results: Vec<PersonaResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePersonaApiResponseOutput {
    pub results: Vec<PersonaResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePersonaItem {
    pub persona_id: String,
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
    pub active_flag_id: Option<String>,
    #[serde(default)]
    pub active_flag: Option<bool>,
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
    pub profiles: Vec<UpdateProfileItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProfileApiResponseInput {
    pub results: Vec<ProfileResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProfileApiResponseOutput {
    pub results: Vec<ProfileResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProfileItem {
    pub profile_id: String,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub request_limit_id: Option<String>,
    #[serde(default)]
    pub flag_id: Option<String>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub email_ids: Option<Vec<String>>,
    #[serde(default)]
    pub role_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProviderApiRequest {
    pub providers: Vec<UpdateProviderItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProviderApiResponseInput {
    pub results: Vec<ProviderResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProviderApiResponseOutput {
    pub results: Vec<ProviderResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProviderItem {
    pub provider_id: String,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub active_flag_id: Option<String>,
    #[serde(default)]
    pub active_flag: Option<bool>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub endpoint_ids: Option<Vec<String>>,
    #[serde(default)]
    pub key_ids: Option<Vec<String>>,
    #[serde(default)]
    pub value_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRubricApiRequest {
    pub rubrics: Vec<UpdateRubricItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRubricApiResponseInput {
    pub results: Vec<RubricResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRubricApiResponseOutput {
    pub results: Vec<RubricResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRubricItem {
    pub rubric_id: String,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub active_flag_id: Option<String>,
    #[serde(default)]
    pub active_flag: Option<bool>,
    #[serde(default)]
    pub department_ids: Option<Vec<String>>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub point_ids: Option<Vec<String>>,
    #[serde(default)]
    pub standard_group_ids: Option<Vec<String>>,
    #[serde(default)]
    pub standard_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateScenarioApiRequest {
    pub scenarios: Vec<UpdateScenarioItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateScenarioApiResponseInput {
    pub results: Vec<ScenarioResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateScenarioApiResponseOutput {
    pub results: Vec<ScenarioResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateScenarioItem {
    pub scenario_id: String,
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
    pub active_flag_id: Option<String>,
    #[serde(default)]
    pub objectives_enabled_flag_id: Option<String>,
    #[serde(default)]
    pub images_enabled_flag_id: Option<String>,
    #[serde(default)]
    pub video_enabled_flag_id: Option<String>,
    #[serde(default)]
    pub questions_enabled_flag_id: Option<String>,
    #[serde(default)]
    pub problem_statement_enabled_flag_id: Option<String>,
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
    pub active_flag: Option<bool>,
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
    pub settings: Vec<UpdateSettingItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSettingApiResponseInput {
    pub results: Vec<SettingResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSettingApiResponseOutput {
    pub results: Vec<SettingResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSettingItem {
    pub setting_id: String,
    #[serde(default)]
    pub name_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
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
    pub profile_ids: Option<Vec<String>>,
    #[serde(default)]
    pub auth_ids: Option<Vec<String>>,
    #[serde(default)]
    pub provider_key_ids: Option<Vec<String>>,
    #[serde(default)]
    pub auth_item_key_ids: Option<Vec<String>>,
    #[serde(default)]
    pub auth_item_value_ids: Option<Vec<String>>,
    #[serde(default)]
    pub system_ids: Option<Vec<String>>,
    #[serde(default)]
    pub threshold_ids: Option<Vec<String>>,
    #[serde(default)]
    pub setting_resource_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSimulationApiRequest {
    pub simulations: Vec<UpdateSimulationItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSimulationApiResponseInput {
    pub results: Vec<SimulationResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSimulationApiResponseOutput {
    pub results: Vec<SimulationResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSimulationItem {
    pub simulation_id: String,
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
    pub is_inactive: Option<bool>,
    #[serde(default)]
    pub is_practice: Option<bool>,
    #[serde(default)]
    pub departments: Option<Vec<String>>,
    #[serde(default)]
    pub scenarios: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateToolApiRequest {
    pub tools: Vec<UpdateToolItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateToolApiResponseInput {
    pub results: Vec<ToolResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateToolApiResponseOutput {
    pub results: Vec<ToolResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateToolItem {
    pub tool_id: String,
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
    pub artifact_ids: Option<Vec<String>>,
    #[serde(default)]
    pub operation_ids: Option<Vec<String>>,
    #[serde(default)]
    pub tool_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadResponse {
    pub upload_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsePreviousAttemptApiResponse {
    pub success: bool,
    #[serde(default)]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    pub loc: Vec<serde_json::Value>,
    pub msg: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoEntry {
    #[serde(default)]
    pub video_id: Option<String>,
    #[serde(default)]
    pub upload_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct app__infra__attempt__types__ContentEntry {
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
pub struct app__infra__attempt__types__HintEntry {
    #[serde(default)]
    pub hint: Option<String>,
    #[serde(default)]
    pub idx: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct app__infra__chat__types__DraftImageValue {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub upload_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct app__infra__chat__types__DraftOptionValue {
    pub option_text: String,
    #[serde(default)]
    pub question_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct app__infra__chat__types__DraftQuestionValue {
    pub question_text: String,
    #[serde(default)]
    pub time: Option<i64>,
    #[serde(default)]
    pub allow_multiple: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct app__infra__chat__types__DraftVideoValue {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub upload_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct app__infra__cohort__types__ExportCohortApiRequest {
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub filter_simulation_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_profile_ids: Option<Vec<String>>,
    #[serde(default)]
    pub filter_department_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct app__infra__scenario__types__DraftImageValue {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub upload_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct app__infra__scenario__types__DraftOptionValue {
    pub option_text: String,
    #[serde(default)]
    pub question_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct app__infra__scenario__types__DraftQuestionValue {
    pub question_text: String,
    #[serde(default)]
    pub time: Option<i64>,
    #[serde(default)]
    pub allow_multiple: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct app__infra__scenario__types__DraftVideoValue {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub upload_id: Option<String>,
    #[serde(default)]
    pub length_seconds: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct app__routes__attempt__message__ContentEntry {
    pub content: String,
    #[serde(default)]
    pub persona_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct app__routes__attempt__message__HintEntry {
    pub hint: String,
    #[serde(default)]
    pub message_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct app__routes__cohort__export__ExportCohortApiRequest {
    #[serde(default)]
    pub cohort_id: Option<String>,
}
