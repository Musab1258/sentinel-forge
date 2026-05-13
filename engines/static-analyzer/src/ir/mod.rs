use serde::Serialize;

use crate::ast::{FunctionMetadata, OperationSite, ParsedContract};

#[derive(Debug, Clone, Serialize)]
pub struct IRContract {
    pub file: String,
    pub functions: Vec<IRFunction>,
}

#[derive(Debug, Clone, Serialize)]
pub struct IRFunction {
    pub name: String,
    pub line: usize,
    pub is_public: bool,
    pub parameters: Vec<String>,
    pub operations: Vec<IROperation>,
    pub metadata: FunctionSummary,
}

#[derive(Debug, Clone, Serialize)]
pub struct FunctionSummary {
    pub has_non_env_params: bool,
    pub mutates_state: bool,
    pub requires_auth: bool,
    pub has_validation: bool,
    pub raw_arithmetic_count: usize,
    pub checked_arithmetic_count: usize,
    pub external_call_count: usize,
    pub loop_count: usize,
    pub possible_privileged: bool,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct IROperation {
    pub kind: IROperationKind,
    pub line: usize,
    pub detail: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
pub enum IROperationKind {
    RequireAuth,
    ReadStorage,
    WriteStorage,
    ExternalCall,
    Arithmetic,
    CheckedArithmetic,
    Loop,
    Validation,
    PrivilegedAction,
}

impl IRContract {
    pub fn from_contract(contract: ParsedContract) -> Self {
        Self {
            file: contract.file,
            functions: contract.functions.into_iter().map(IRFunction::from_metadata).collect(),
        }
    }
}

impl IRFunction {
    pub fn from_metadata(metadata: FunctionMetadata) -> Self {
        let FunctionMetadata {
            name,
            line,
            is_public,
            parameters,
            requires_auth_sites,
            storage_read_sites,
            storage_write_sites,
            external_call_sites,
            raw_arithmetic_sites,
            checked_arithmetic_sites,
            loop_sites,
            validation_sites,
            privileged_sites,
        } = metadata;

        let has_non_env_params = parameters.iter().any(|parameter| !is_env_parameter(parameter));
        let mut operations = Vec::new();

        operations.extend(map_sites(requires_auth_sites, IROperationKind::RequireAuth));
        operations.extend(map_sites(storage_read_sites, IROperationKind::ReadStorage));
        operations.extend(map_sites(storage_write_sites, IROperationKind::WriteStorage));
        operations.extend(map_sites(external_call_sites, IROperationKind::ExternalCall));
        operations.extend(map_sites(raw_arithmetic_sites, IROperationKind::Arithmetic));
        operations.extend(map_sites(
            checked_arithmetic_sites,
            IROperationKind::CheckedArithmetic,
        ));
        operations.extend(map_sites(loop_sites, IROperationKind::Loop));
        operations.extend(map_sites(validation_sites, IROperationKind::Validation));
        operations.extend(map_sites(privileged_sites, IROperationKind::PrivilegedAction));
        operations.sort_by_key(|operation| operation.line);

        let metadata = FunctionSummary {
            has_non_env_params,
            mutates_state: operations
                .iter()
                .any(|operation| operation.kind == IROperationKind::WriteStorage),
            requires_auth: operations
                .iter()
                .any(|operation| operation.kind == IROperationKind::RequireAuth),
            has_validation: operations
                .iter()
                .any(|operation| operation.kind == IROperationKind::Validation),
            raw_arithmetic_count: operations
                .iter()
                .filter(|operation| operation.kind == IROperationKind::Arithmetic)
                .count(),
            checked_arithmetic_count: operations
                .iter()
                .filter(|operation| operation.kind == IROperationKind::CheckedArithmetic)
                .count(),
            external_call_count: operations
                .iter()
                .filter(|operation| operation.kind == IROperationKind::ExternalCall)
                .count(),
            loop_count: operations
                .iter()
                .filter(|operation| operation.kind == IROperationKind::Loop)
                .count(),
            possible_privileged: name_has_privilege_hint(&name)
                || parameters.iter().any(|parameter| name_has_privilege_hint(parameter))
                || operations
                    .iter()
                    .any(|operation| operation.kind == IROperationKind::PrivilegedAction),
        };

        Self {
            name,
            line,
            is_public,
            parameters,
            operations,
            metadata,
        }
    }

    pub fn operation_lines(&self, kind: IROperationKind) -> Vec<usize> {
        self.operations
            .iter()
            .filter(|operation| operation.kind == kind)
            .map(|operation| operation.line)
            .collect()
    }

    pub fn trace(&self) -> Vec<String> {
        self.operations
            .iter()
            .map(|operation| format!("{:?}@{}", operation.kind, operation.line))
            .collect()
    }
}

fn map_sites(sites: Vec<OperationSite>, kind: IROperationKind) -> Vec<IROperation> {
    sites.into_iter()
        .map(|site| IROperation {
            kind: kind.clone(),
            line: site.line,
            detail: site.detail,
        })
        .collect()
}

fn is_env_parameter(parameter: &str) -> bool {
    matches!(parameter, "e" | "env" | "self")
}

fn name_has_privilege_hint(name: &str) -> bool {
    let lowered = name.to_ascii_lowercase();
    lowered.contains("admin")
        || lowered.contains("owner")
        || lowered.contains("role")
        || lowered.contains("privilege")
}
