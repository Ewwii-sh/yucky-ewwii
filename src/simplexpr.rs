use crate::convert::WidgetArgs;
use eww_shared_util::{Span, VarName};
use ewwii_plugin_api::shared_utils::{
    prop::Property,
    template::{TemplateExpr, TemplateOp},
    variables::GlobalVar,
};
use simplexpr::ast::{BinOp, SimplExpr};
use simplexpr::dynval::DynVal;
use std::collections::HashMap;

pub fn simpl_expr_to_template(expr: &SimplExpr) -> TemplateExpr {
    match expr {
        SimplExpr::Literal(DynVal(s, _)) => TemplateExpr::Literal(s.clone()),
        SimplExpr::VarRef(_, var) => TemplateExpr::Var(var.0.clone()),
        SimplExpr::Concat(_, parts) => {
            TemplateExpr::Concat(parts.iter().map(simpl_expr_to_template).collect())
        }
        SimplExpr::IfElse(_, cond, if_true, if_false) => TemplateExpr::IfElse {
            condition: Box::new(simpl_expr_to_template(cond)),
            if_true: Box::new(simpl_expr_to_template(if_true)),
            if_false: Box::new(simpl_expr_to_template(if_false)),
        },
        SimplExpr::BinOp(_, left, op, right) => TemplateExpr::BinOp {
            op: match op {
                BinOp::Plus => TemplateOp::Add,
                BinOp::Minus => TemplateOp::Sub,
                BinOp::Times => TemplateOp::Mul,
                BinOp::Div => TemplateOp::Div,
                BinOp::Equals => TemplateOp::Eq,
                BinOp::NotEquals => TemplateOp::NotEq,
                BinOp::GT => TemplateOp::Gt,
                BinOp::LT => TemplateOp::Lt,
                BinOp::GE => TemplateOp::Gte,
                BinOp::LE => TemplateOp::Lte,
                BinOp::And => TemplateOp::And,
                BinOp::Or => TemplateOp::Or,
                BinOp::Mod => TemplateOp::Mod,
                BinOp::Elvis => TemplateOp::Elvis,
                BinOp::RegexMatch => TemplateOp::RegexMatch,
            },
            left: Box::new(simpl_expr_to_template(left)),
            right: Box::new(simpl_expr_to_template(right)),
        },
        other => {
            let s = format!("{}", other);
            if let Some((var, key)) = parse_index_expr(&s) {
                TemplateExpr::Index {
                    expr: Box::new(TemplateExpr::Var(var)),
                    key: Box::new(TemplateExpr::Literal(key)),
                }
            } else {
                TemplateExpr::Literal(s)
            }
        }
    }
}

pub fn resolve_simpl_expr(
    expr: &SimplExpr,
    args: &HashMap<String, WidgetArgs>,
    vars: &Vec<GlobalVar>,
) -> Property {
    let var_map: HashMap<VarName, DynVal> = args
        .iter()
        .filter_map(|(k, v)| match v {
            WidgetArgs::String(s) => Some((VarName(k.clone()), DynVal(s.clone(), Span::DUMMY))),
            WidgetArgs::GlobalVar(g) => match &g.initial {
                Property::String(s) => Some((VarName(k.clone()), DynVal(s.clone(), Span::DUMMY))),
                _ => None,
            },
        })
        .collect();

    match expr.eval(&var_map) {
        Ok(DynVal(s, _)) => Property::String(s),
        Err(_) => {
            let var_refs = expr.collect_var_refs();

            if var_refs.is_empty() {
                return Property::String(format!("{}", expr));
            }

            // convert SimplExpr to TemplateExpr
            let template = simpl_expr_to_template(expr);

            if var_refs.len() == 1 && matches!(expr, SimplExpr::VarRef(..)) {
                let var_name = &var_refs[0].0;
                if let Some(global) = vars.iter().find(|v| &v.name == var_name) {
                    Property::GlobalVar(Box::new(GlobalVar {
                        name: global.name.clone(),
                        initial: global.initial.clone(),
                        template: None,
                    }))
                } else {
                    Property::String(format!("{}", expr))
                }
            } else {
                let primary_var = &var_refs[0].0;
                let global = vars.iter().find(|v| &v.name == primary_var);
                Property::GlobalVar(Box::new(GlobalVar {
                    name: primary_var.clone(),
                    initial: global.map(|g| g.initial.clone()).unwrap_or(Property::None),
                    template: Some(template),
                }))
            }
        }
    }
}

pub fn resolve_as_bool(
    expr: &SimplExpr,
    args: &HashMap<String, WidgetArgs>,
    vars: &Vec<GlobalVar>,
) -> Property {
    match resolve_simpl_expr(expr, args, vars) {
        Property::String(s) => match s.as_str() {
            "true" => Property::Bool(true),
            "false" => Property::Bool(false),
            _ => Property::String(s),
        },
        other => other,
    }
}

pub fn resolve_as_int(
    expr: &SimplExpr,
    args: &HashMap<String, WidgetArgs>,
    vars: &Vec<GlobalVar>,
) -> Property {
    match resolve_simpl_expr(expr, args, vars) {
        Property::String(s) => s.parse::<i64>().map(Property::Int).unwrap_or(Property::String(s)),
        other => other,
    }
}

fn parse_index_expr(s: &str) -> Option<(String, String)> {
    let s = s.trim();
    let bracket = s.find('[')?;
    if !s.ends_with(']') { return None; }
    let var = s[..bracket].trim().to_string();
    let key = s[bracket+1..s.len()-1].trim().trim_matches('"').to_string();
    if var.is_empty() || key.is_empty() { return None; }
    Some((var, key))
}