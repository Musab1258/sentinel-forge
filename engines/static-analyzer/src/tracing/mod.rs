use crate::ir::{IRFunction, IROperationKind};

pub fn build_trace(function: &IRFunction) -> Vec<String> {
    let mut trace = vec![format!("Function {}", function.name)];

    for operation in &function.operations {
        trace.push(match operation.kind {
            IROperationKind::RequireAuth => "RequireAuth".to_string(),
            IROperationKind::ReadStorage => "ReadStorage".to_string(),
            IROperationKind::WriteStorage => "WriteStorage".to_string(),
            IROperationKind::ExternalCall => "ExternalCall".to_string(),
            IROperationKind::Arithmetic => "Arithmetic".to_string(),
            IROperationKind::CheckedArithmetic => "CheckedArithmetic".to_string(),
            IROperationKind::Loop => "Loop".to_string(),
            IROperationKind::Validation => "Validation".to_string(),
            IROperationKind::PrivilegedAction => "PrivilegedAction".to_string(),
        });
    }

    trace
}
