use crate::convert::{ConvertContext, WidgetArgs};
use ewwii_plugin_api::shared_utils::{
    prop::{PropertyMap, Property}, 
    ast::WidgetNode,
    variables::GlobalVar,
};
use yuck::config::attributes::Attributes;
use yuck::config::widget_use::{WidgetUse, BasicWidgetUse};
use yuck::config::window_definition::WindowDefinition;
use yuck::parser::ast::Ast;
use std::collections::HashMap;
use heck::ToSnakeCase;
use crate::simplexpr::*;

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
                                        Property::GlobalVar(g) => WidgetArgs::GlobalVar(*g),
                                        Property::String(s) => WidgetArgs::String(s),
                                        Property::Int(i) => WidgetArgs::String(i.to_string()),
                                        Property::Float(f) => WidgetArgs::String(f.to_string()),
                                        Property::Bool(b) => WidgetArgs::String(b.to_string()),
                                        _ => WidgetArgs::String(format!("{}", v.value).trim_matches('"').to_string()),
                                    }
                                }
                                Ast::Symbol(_, s) => {
                                    if let Some(val) = ctx.args.get(s) {
                                        match val {
                                            WidgetArgs::String(s) => WidgetArgs::String(s.clone()),
                                            WidgetArgs::GlobalVar(g) => WidgetArgs::GlobalVar(g.clone()),
                                        }
                                    } else if let Some(global) = ctx.vars.iter().find(|v| &v.name == s) {
                                        WidgetArgs::GlobalVar(global.clone())
                                    } else {
                                        WidgetArgs::String(s.clone())
                                    }
                                }
                                _ => WidgetArgs::String(format!("{}", v.value).trim_matches('"').to_string()),
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

fn extract_props(
    attrs: &Attributes,
    args: &HashMap<String, WidgetArgs>,
    vars: &Vec<GlobalVar>,
) -> PropertyMap {
    let mut map = PropertyMap::new();
    for (key, attr) in &attrs.attrs {
        let prop = match &attr.value {
            Ast::SimplExpr(_, expr) => resolve_simpl_expr(expr, args, vars),
            Ast::Symbol(_, s) => {
                // check widget args first
                if let Some(val) = args.get(s) {
                    match val {
                        WidgetArgs::String(s) => Property::String(s.clone()),
                        WidgetArgs::GlobalVar(g) => Property::GlobalVar(Box::new(g.clone())),
                    }
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
        let snake_key = key.0.to_snake_case();
        map.insert(snake_key, prop);
    }
    map
}

pub fn window_def_to_props(window_def: &WindowDefinition, vars: &Vec<GlobalVar>) -> PropertyMap {
    let args: HashMap<String, WidgetArgs> = HashMap::new();
    let mut props = PropertyMap::new();

    // monitor
    if let Some(expr) = &window_def.monitor {
        props.insert("monitor".to_string(), resolve_as_int(expr, &args, vars));
    }

    // stacking
    if let Some(expr) = &window_def.stacking {
        props.insert("stacking".to_string(), resolve_simpl_expr(expr, &args, vars));
    }

    // resizable
    if let Some(expr) = &window_def.resizable {
        props.insert("resizable".to_string(), resolve_as_bool(expr, &args, vars));
    }

    // x11 backend options
    let x11 = &window_def.backend_options.x11;
    if let Some(expr) = &x11.sticky {
        props.insert("sticky".to_string(), resolve_as_bool(expr, &args, vars));
    }
    if let Some(expr) = &x11.wm_ignore {
        props.insert("wm_ignore".to_string(), resolve_as_bool(expr, &args, vars));
    }
    if let Some(expr) = &x11.window_type {
        props.insert("windowtype".to_string(), resolve_simpl_expr(expr, &args, vars));
    }
    if let Some(struts) = &x11.struts {
        let mut reserve = PropertyMap::new();
        if let Some(side) = &struts.side {
            reserve.insert("side".to_string(), resolve_simpl_expr(side, &args, vars));
        }
        reserve.insert("distance".to_string(), resolve_simpl_expr(&struts.distance, &args, vars));
        props.insert("reserve".to_string(), Property::Map(reserve));
    }

    // wayland backend options
    let wl = &window_def.backend_options.wayland;
    if let Some(expr) = &wl.exclusive {
        props.insert("exclusive".to_string(), resolve_as_bool(expr, &args, vars));
    }
    if let Some(expr) = &wl.focusable {
        props.insert("focusable".to_string(), resolve_simpl_expr(expr, &args, vars));
    }
    if let Some(expr) = &wl.namespace {
        props.insert("namespace".to_string(), resolve_simpl_expr(expr, &args, vars));
    }

    // geometry
    if let Some(geo) = &window_def.geometry {
        let mut geometry = PropertyMap::new();
        if let Some(expr) = &geo.offset.x {
            geometry.insert("x".to_string(), resolve_simpl_expr(expr, &args, vars));
        }
        if let Some(expr) = &geo.offset.y {
            geometry.insert("y".to_string(), resolve_simpl_expr(expr, &args, vars));
        }
        if let Some(expr) = &geo.size.x {
            geometry.insert("width".to_string(), resolve_simpl_expr(expr, &args, vars));
        }
        if let Some(expr) = &geo.size.y {
            geometry.insert("height".to_string(), resolve_simpl_expr(expr, &args, vars));
        }
        if let Some(expr) = &geo.anchor_point {
            geometry.insert("anchor".to_string(), resolve_simpl_expr(expr, &args, vars));
        }
        props.insert("geometry".to_string(), Property::Map(geometry));
    }

    props
}