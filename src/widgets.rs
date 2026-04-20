use crate::convert::ConvertContext;
use ewwii_plugin_api::shared_utils::{
    prop::{PropertyMap, Property}, 
    ast::WidgetNode,
    variables::GlobalVar,
    template::{TemplateExpr, TemplateOp},
};
use yuck::config::attributes::Attributes;
use yuck::config::widget_use::{WidgetUse, BasicWidgetUse};
use yuck::parser::ast::Ast;
use std::collections::HashMap;
use simplexpr::dynval::DynVal;
use simplexpr::ast::{SimplExpr, BinOp};
use eww_shared_util::{VarName, Span};

const BOX_NAME: &str = "box";
const LABEL_NAME: &str = "label";
const BUTTON_NAME: &str = "button";
const IMAGE_NAME: &str = "image";
const INPUT_NAME: &str = "input";
const PROGRESS_NAME: &str = "progress";
const COMBOBOXTEXT_NAME: &str = "combo-box-text";
const SCALE_NAME: &str = "scale";
const CHECKBOX_NAME: &str = "checkbox";
const EXPANDER_NAME: &str = "expander";
const REVEALER_NAME: &str = "revealer";
const SCROLL_NAME: &str = "scroll";
const OVERLAY_NAME: &str = "overlay";
const STACK_NAME: &str = "stack";
const COLORBUTTON_NAME: &str = "color-button";
const COLORCHOOSER_NAME: &str = "color-chooser";
const CIRCULARPROGRESS_NAME: &str = "circular-progress";
const GRAPH_NAME: &str = "graph";
const TRANSFORM_NAME: &str = "transform";
const EVENTBOX_NAME: &str = "eventbox";
const TOOLTIP_NAME: &str = "tooltip";

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
    let props = extract_props(&basic.attrs, &ctx.args, &ctx.vars);
    let children = basic.children
        .iter()
        .map(|child| widget_use_to_node(child, ctx))
        .collect::<Result<Vec<_>, _>>()?;

    match basic.name.as_str() {
        LABEL_NAME => Ok(WidgetNode::Label { props }),
        BUTTON_NAME => Ok(WidgetNode::Button { props }),
        BOX_NAME => Ok(WidgetNode::Box { props, children }),
        IMAGE_NAME => Ok(WidgetNode::Image { props }),
        INPUT_NAME => Ok(WidgetNode::Input { props }),
        PROGRESS_NAME => Ok(WidgetNode::Progress { props }),
        COMBOBOXTEXT_NAME => Ok(WidgetNode::ComboBoxText { props }),
        SCALE_NAME => Ok(WidgetNode::Scale { props }),
        CHECKBOX_NAME => Ok(WidgetNode::Checkbox { props }),
        EXPANDER_NAME => Ok(WidgetNode::Expander { props, children }),
        REVEALER_NAME => Ok(WidgetNode::Revealer { props, children }),
        SCROLL_NAME => Ok(WidgetNode::Scroll { props, children }),
        OVERLAY_NAME => Ok(WidgetNode::OverLay { props, children }),
        STACK_NAME => Ok(WidgetNode::Stack { props, children }),
        COLORBUTTON_NAME => Ok(WidgetNode::ColorButton { props }),
        COLORCHOOSER_NAME => Ok(WidgetNode::ColorChooser { props }),
        CIRCULARPROGRESS_NAME => Ok(WidgetNode::CircularProgress { props }),
        GRAPH_NAME => Ok(WidgetNode::Graph { props }),
        TRANSFORM_NAME => Ok(WidgetNode::Transform { props }),
        EVENTBOX_NAME => Ok(WidgetNode::EventBox { props, children }),
        TOOLTIP_NAME => Ok(WidgetNode::ToolTip { props, children }),
        other => {
            if let Some(def) = ctx.defs.get(other) {
                let new_ctx = ConvertContext {
                    defs: ctx.defs,
                    args: basic.attrs.attrs
                        .iter()
                        .map(|(k, v)| {
                            let val = match &v.value {
                                Ast::SimplExpr(_, expr) => {
                                    match resolve_simpl_expr(expr, &ctx.args, &ctx.vars) {
                                        Property::String(s) => s,
                                        Property::Int(i) => i.to_string(),
                                        Property::Float(f) => f.to_string(),
                                        Property::Bool(b) => b.to_string(),
                                        _ => format!("{}", v.value).trim_matches('"').to_string(),
                                    }
                                }
                                Ast::Symbol(_, s) => {
                                    if let Some(val) = ctx.args.get(s) {
                                        val.clone()
                                    } else if let Some(global) = ctx.vars.iter().find(|v| &v.name == s) {
                                        match &global.initial {
                                            Property::String(s) => s.clone(),
                                            _ => String::new(),
                                        }
                                    } else {
                                        s.clone()
                                    }
                                }
                                _ => format!("{}", v.value).trim_matches('"').to_string(),
                            };
                            (k.0.clone(), val)
                        })
                        .collect(),
                    vars: ctx.vars,
                };
                widget_use_to_node(&def.widget, &new_ctx)
            } else {
                Err(format!("Unknown widget: {}", other))
            }
        }
    }
}

fn simpl_expr_to_template(expr: &SimplExpr) -> TemplateExpr {
    match expr {
        SimplExpr::Literal(DynVal(s, _)) => TemplateExpr::Literal(s.clone()),
        SimplExpr::VarRef(_, var) => TemplateExpr::Var(var.0.clone()),
        SimplExpr::Concat(_, parts) => TemplateExpr::Concat(
            parts.iter().map(simpl_expr_to_template).collect()
        ),
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
        other => TemplateExpr::Literal(format!("{}", other)),
    }
}

fn resolve_simpl_expr(
    expr: &SimplExpr,
    args: &HashMap<String, String>,
    vars: &Vec<GlobalVar>,
) -> Property {
    let var_map: HashMap<VarName, DynVal> = args
        .iter()
        .map(|(k, v)| (VarName(k.clone()), DynVal(v.clone(), Span::DUMMY)))
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

fn extract_props(
    attrs: &Attributes,
    args: &HashMap<String, String>,
    vars: &Vec<GlobalVar>,
) -> PropertyMap {
    let mut map = PropertyMap::new();
    for (key, attr) in &attrs.attrs {
        let prop = match &attr.value {
            Ast::SimplExpr(_, expr) => resolve_simpl_expr(expr, args, vars),
            Ast::Symbol(_, s) => {
                // check widget args first
                if let Some(val) = args.get(s) {
                    Property::String(val.clone())
                // then check global vars
                } else if let Some(global) = vars.iter().find(|v| &v.name == s) {
                    Property::GlobalVar(Box::new(global.clone()))
                } else {
                    Property::String(s.clone())
                }
            }
            _ => Property::String(
                format!("{}", attr.value).trim_matches('"').to_string()
            ),
        };
        map.insert(key.0.clone(), prop);
    }
    map
}