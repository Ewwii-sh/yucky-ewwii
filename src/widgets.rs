use crate::convert::ConvertContext;
use ewwii_plugin_api::shared_utils::{
    prop::{PropertyMap, Property}, 
    ast::WidgetNode
};
use yuck::config::attributes::Attributes;
use yuck::config::widget_definition::WidgetDefinition;
use yuck::config::widget_use::{WidgetUse, BasicWidgetUse};
use yuck::parser::ast::Ast;
use std::collections::HashMap;
use simplexpr::dynval::DynVal;
use simplexpr::ast::SimplExpr;
use eww_shared_util::{VarName, Span};

const BOX_NAME: &str = "box";
const LABEL_NAME: &str = "label";
const BUTTON_NAME: &str = "button";

pub fn widget_use_to_node(
    widget_use: &WidgetUse, 
    ctx: &ConvertContext,
) -> Result<WidgetNode, String> {
    match widget_use {
        WidgetUse::Basic(basic) => basic_widget_to_node(basic, ctx),
        WidgetUse::Loop(loop_use) => {
            widget_use_to_node(&loop_use.body, ctx)
        }
        WidgetUse::Children(_) => {
            Ok(WidgetNode::Tree(vec![]))
        }
    }
}

fn basic_widget_to_node(
    basic: &BasicWidgetUse,
    ctx: &ConvertContext,
) -> Result<WidgetNode, String> {
    let props = extract_props(&basic.attrs, &ctx.args);
    let children = basic.children
        .iter()
        .map(|child| widget_use_to_node(child, ctx))
        .collect::<Result<Vec<_>, _>>()?;

    match basic.name.as_str() {
        BOX_NAME => Ok(WidgetNode::Box { props, children }),
        LABEL_NAME => Ok(WidgetNode::Label { props }),
        BUTTON_NAME => Ok(WidgetNode::Button { props }),
        other => {
            if let Some(def) = ctx.defs.get(other) {
                let new_ctx = ConvertContext {
                    defs: ctx.defs,
                    args: basic.attrs.attrs
                        .iter()
                        .map(|(k, v)| {
                            let val = match &v.value {
                                Ast::SimplExpr(_, expr) => resolve_simpl_expr(expr, &ctx.args),
                                Ast::Symbol(_, s) => ctx.args.get(s).cloned().unwrap_or(s.clone()),
                                _ => format!("{}", v.value).trim_matches('"').to_string(),
                            };
                            (k.0.clone(), val)
                        })
                        .collect(),
                };
                println!("NEW CTX {:#?}", &new_ctx);
                widget_use_to_node(&def.widget, &new_ctx)
            } else {
                Err(format!("Unknown widget: {}", other))
            }
        }
    }
}

fn resolve_simpl_expr(expr: &SimplExpr, args: &HashMap<String, String>) -> String {
    let var_map: HashMap<VarName, DynVal> = args
        .iter()
        .map(|(k, v)| (VarName(k.clone()), DynVal(v.clone(), Span::DUMMY)))
        .collect();

    match expr.eval(&var_map) {
        Ok(DynVal(s, _)) => s,
        Err(_) => {
            // has unresolved refs (defvar/defpoll) — keep as-is for renderer
            format!("{}", expr)
        }
    }
}

fn extract_props(attrs: &Attributes, args: &HashMap<String, String>) -> PropertyMap {
    let mut map = PropertyMap::new();
    for (key, attr) in &attrs.attrs {
        let val = match &attr.value {
            Ast::SimplExpr(_, expr) => resolve_simpl_expr(expr, args),
            Ast::Symbol(_, s) => {
                args.get(s).cloned().unwrap_or(s.clone())
            }
            _ => format!("{}", attr.value).trim_matches('"').to_string(),
        };
        map.insert(key.0.clone(), val.into());
    }
    map
}