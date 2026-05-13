use anyhow::Result;
use quote::ToTokens;
use syn::spanned::Spanned;
use syn::visit::{self, Visit};
use syn::{Expr, ExprBinary, ExprCall, ExprLoop, ExprMacro, ExprMethodCall, ExprWhile, FnArg, ItemFn, Macro};

#[derive(Debug, Clone)]
pub struct OperationSite {
    pub line: usize,
    pub detail: String,
}

#[derive(Debug, Clone)]
pub struct ParsedContract {
    pub file: String,
    pub functions: Vec<FunctionMetadata>,
}

#[derive(Debug, Clone)]
pub struct FunctionMetadata {
    pub name: String,
    pub line: usize,
    pub is_public: bool,
    pub parameters: Vec<String>,
    pub requires_auth_sites: Vec<OperationSite>,
    pub storage_read_sites: Vec<OperationSite>,
    pub storage_write_sites: Vec<OperationSite>,
    pub external_call_sites: Vec<OperationSite>,
    pub raw_arithmetic_sites: Vec<OperationSite>,
    pub checked_arithmetic_sites: Vec<OperationSite>,
    pub loop_sites: Vec<OperationSite>,
    pub validation_sites: Vec<OperationSite>,
    pub privileged_sites: Vec<OperationSite>,
}

pub fn parse_contract(file: &str, source: &str) -> Result<ParsedContract> {
    let ast = syn::parse_file(source)?;
    let mut functions = Vec::new();

    for item in ast.items {
        if let syn::Item::Fn(item_fn) = item {
            functions.push(parse_function(item_fn));
        }
    }

    Ok(ParsedContract {
        file: file.to_string(),
        functions,
    })
}

fn parse_function(item_fn: ItemFn) -> FunctionMetadata {
    let mut visitor = FunctionVisitor::default();
    visitor.visit_block(&item_fn.block);

    FunctionMetadata {
        name: item_fn.sig.ident.to_string(),
        line: item_fn.sig.ident.span().start().line,
        is_public: matches!(item_fn.vis, syn::Visibility::Public(_)),
        parameters: item_fn
            .sig
            .inputs
            .iter()
            .filter_map(parameter_name)
            .collect(),
        requires_auth_sites: visitor.requires_auth_sites,
        storage_read_sites: visitor.storage_read_sites,
        storage_write_sites: visitor.storage_write_sites,
        external_call_sites: visitor.external_call_sites,
        raw_arithmetic_sites: visitor.raw_arithmetic_sites,
        checked_arithmetic_sites: visitor.checked_arithmetic_sites,
        loop_sites: visitor.loop_sites,
        validation_sites: visitor.validation_sites,
        privileged_sites: visitor.privileged_sites,
    }
}

fn parameter_name(arg: &FnArg) -> Option<String> {
    match arg {
        FnArg::Receiver(_) => Some("self".to_string()),
        FnArg::Typed(typed) => match typed.pat.as_ref() {
            syn::Pat::Ident(ident) => Some(ident.ident.to_string()),
            _ => None,
        },
    }
}

#[derive(Default)]
struct FunctionVisitor {
    requires_auth_sites: Vec<OperationSite>,
    storage_read_sites: Vec<OperationSite>,
    storage_write_sites: Vec<OperationSite>,
    external_call_sites: Vec<OperationSite>,
    raw_arithmetic_sites: Vec<OperationSite>,
    checked_arithmetic_sites: Vec<OperationSite>,
    loop_sites: Vec<OperationSite>,
    validation_sites: Vec<OperationSite>,
    privileged_sites: Vec<OperationSite>,
}

impl<'ast> Visit<'ast> for FunctionVisitor {
    fn visit_expr_method_call(&mut self, node: &'ast ExprMethodCall) {
        let method = node.method.to_string();
        let line = node.span().start().line;

        if method == "require_auth" {
            self.requires_auth_sites.push(site(line, method.clone()));
        }

        if is_storage_write(&method) {
            self.storage_write_sites.push(site(line, method.clone()));
        }

        if is_storage_read(&method) {
            self.storage_read_sites.push(site(line, method.clone()));
        }

        if is_external_call(&method) {
            self.external_call_sites.push(site(line, method.clone()));
        }

        if is_checked_arithmetic(&method) {
            self.checked_arithmetic_sites.push(site(line, method.clone()));
        }

        if is_validation_name(&method) {
            self.validation_sites.push(site(line, method.clone()));
        }

        if is_privileged_name(&method) {
            self.privileged_sites.push(site(line, method));
        }

        visit::visit_expr_method_call(self, node);
    }

    fn visit_expr_call(&mut self, node: &'ast ExprCall) {
        if let Expr::Path(path) = node.func.as_ref() {
            let detail = path.path.to_token_stream().to_string().replace(' ', "");
            let name = path
                .path
                .segments
                .last()
                .map(|segment| segment.ident.to_string())
                .unwrap_or_default();
            let line = node.span().start().line;

            if is_storage_write_path(&detail, &name) {
                self.storage_write_sites.push(site(line, detail.clone()));
            }

            if is_storage_read_path(&detail, &name) {
                self.storage_read_sites.push(site(line, detail.clone()));
            }

            if is_external_call(&name) {
                self.external_call_sites.push(site(line, detail.clone()));
            }

            if is_validation_name(&name) {
                self.validation_sites.push(site(line, detail.clone()));
            }

            if is_privileged_name(&name) {
                self.privileged_sites.push(site(line, detail));
            }
        }

        visit::visit_expr_call(self, node);
    }

    fn visit_expr_binary(&mut self, node: &'ast ExprBinary) {
        let line = node.span().start().line;
        let detail = node.to_token_stream().to_string();

        if is_arithmetic_op(&node.op) {
            self.raw_arithmetic_sites.push(site(line, detail.clone()));
        }

        if is_validation_binary(&node.op) {
            self.validation_sites.push(site(line, detail));
        }

        visit::visit_expr_binary(self, node);
    }

    fn visit_expr_if(&mut self, node: &'ast syn::ExprIf) {
        self.validation_sites
            .push(site(node.if_token.span().start().line, "if guard".to_string()));
        visit::visit_expr_if(self, node);
    }

    fn visit_expr_match(&mut self, node: &'ast syn::ExprMatch) {
        self.validation_sites.push(site(
            node.match_token.span().start().line,
            "match guard".to_string(),
        ));
        visit::visit_expr_match(self, node);
    }

    fn visit_expr_macro(&mut self, node: &'ast ExprMacro) {
        record_macro_validation(&mut self.validation_sites, &node.mac);
        visit::visit_expr_macro(self, node);
    }

    fn visit_macro(&mut self, node: &'ast Macro) {
        record_macro_validation(&mut self.validation_sites, node);
        visit::visit_macro(self, node);
    }

    fn visit_expr_for_loop(&mut self, node: &'ast syn::ExprForLoop) {
        self.loop_sites
            .push(site(node.for_token.span.start().line, "for loop".to_string()));
        visit::visit_expr_for_loop(self, node);
    }

    fn visit_expr_loop(&mut self, node: &'ast ExprLoop) {
        push_loop_site(&mut self.loop_sites, node, "loop");
        visit::visit_expr_loop(self, node);
    }

    fn visit_expr_while(&mut self, node: &'ast ExprWhile) {
        push_while_site(&mut self.loop_sites, node, "while loop");
        visit::visit_expr_while(self, node);
    }
}

fn site(line: usize, detail: String) -> OperationSite {
    OperationSite { line, detail }
}

fn push_loop_site(target: &mut Vec<OperationSite>, node: &ExprLoop, detail: &str) {
    target.push(site(node.loop_token.span.start().line, detail.to_string()));
}

fn push_while_site(target: &mut Vec<OperationSite>, node: &ExprWhile, detail: &str) {
    target.push(site(node.while_token.span.start().line, detail.to_string()));
}

fn is_storage_write(name: &str) -> bool {
    matches!(name, "set" | "update" | "remove" | "insert" | "extend_ttl")
}

fn is_storage_read(name: &str) -> bool {
    matches!(name, "get" | "has" | "contains" | "try_get")
}

fn is_storage_write_path(detail: &str, name: &str) -> bool {
    detail.contains("storage::") || matches!(name, "set_admin" | "set_owner" | "set_role")
        || name.starts_with("set_")
        || name.starts_with("update_")
        || name.starts_with("remove_")
}

fn is_storage_read_path(detail: &str, name: &str) -> bool {
    detail.contains("storage::")
        && (name.starts_with("get_") || name.starts_with("load_") || name == "get")
}

fn is_external_call(name: &str) -> bool {
    matches!(name, "invoke_contract" | "try_invoke_contract" | "call")
}

fn is_checked_arithmetic(name: &str) -> bool {
    matches!(
        name,
        "checked_add"
            | "checked_sub"
            | "checked_mul"
            | "checked_div"
            | "checked_rem"
            | "saturating_add"
            | "saturating_sub"
            | "saturating_mul"
    )
}

fn is_validation_name(name: &str) -> bool {
    !is_checked_arithmetic(name)
        && (name.starts_with("validate")
        || name.starts_with("ensure")
        || name.starts_with("check")
        || name.starts_with("verify"))
}

fn is_privileged_name(name: &str) -> bool {
    let lowered = name.to_ascii_lowercase();
    lowered.contains("admin")
        || lowered.contains("owner")
        || lowered.contains("role")
        || lowered.contains("privilege")
}

fn is_arithmetic_op(op: &syn::BinOp) -> bool {
    matches!(
        op,
        syn::BinOp::Add(_)
            | syn::BinOp::Sub(_)
            | syn::BinOp::Mul(_)
            | syn::BinOp::Div(_)
            | syn::BinOp::Rem(_)
    )
}

fn is_validation_binary(op: &syn::BinOp) -> bool {
    matches!(
        op,
        syn::BinOp::Eq(_)
            | syn::BinOp::Ne(_)
            | syn::BinOp::Lt(_)
            | syn::BinOp::Le(_)
            | syn::BinOp::Gt(_)
            | syn::BinOp::Ge(_)
    )
}

fn record_macro_validation(target: &mut Vec<OperationSite>, mac: &Macro) {
    let name = mac
        .path
        .segments
        .last()
        .map(|segment| segment.ident.to_string())
        .unwrap_or_default();

    if matches!(name.as_str(), "assert" | "assert_eq" | "assert_ne") {
        target.push(site(mac.span().start().line, format!("{name}!")));
    }
}
