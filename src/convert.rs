use yuck::config::{
    TopLevel,
    script_var_definition::{
        ScriptVarDefinition, 
        VarSource,
    },
    widget_definition::WidgetDefinition,
};
use yuck::parser::from_ast::FromAst;
use ewwii_plugin_api::shared_utils::ast::WidgetNode;
use ewwii_plugin_api::shared_utils::prop::{PropertyMap, Property};
use ewwii_plugin_api::shared_utils::variables::GlobalVar;
use std::collections::HashMap;
use crate::widgets;
use std::fs;

#[derive(Debug)]
pub struct ConvertContext<'a> {
    pub defs: &'a HashMap<String, WidgetDefinition>,
    pub args: HashMap<String, String>,
    pub vars: &'a Vec<GlobalVar>,
}

pub fn convert_to_widgetnode(top_levels: Vec<TopLevel>) -> Result<WidgetNode, String> {
    let mut tree: Vec<WidgetNode> = Vec::new();
    let mut global_var_defs: Vec<GlobalVar> = Vec::new();
    let mut widget_defs: HashMap<String, WidgetDefinition> = HashMap::new();

    for top_level in top_levels {
        match top_level {
            TopLevel::Include(inc) => {
                let source = fs::read_to_string(inc.path)
                    .expect("Should have been able to read the file");

                match yuck::parser::parse_toplevel(0, source) {
                    Ok((_span, ast_nodes)) => {
                        let top_levels: Vec<TopLevel> = ast_nodes
                            .into_iter()
                            .map(|ast| TopLevel::from_ast(ast)
                            .map_err(|e| {
                                errors::report_diag_error(source, path, &e);
                                e.to_string()
                            }))
                            .collect::<Result<Vec<_>, _>>()?;

                        let result = convert_to_widgetnode(top_levels)?;

                        if let WidgetNode::Tree(nested_vec) = result {
                            tree.extend(nested_vec);
                        }
                    }
                    Err(e) => {
                        // errors::report_parse_error(source, path, &e);
                        Err(format!("Failed to parse yuck: {}", e))
                    }
                }
            }
            TopLevel::VarDefinition(var_def) => {
                let global_var = GlobalVar {
                    name: var_def.name.0.clone(),
                    initial: Property::String(var_def.initial_value.0.clone()),
                    template: None,
                };
                global_var_defs.push(global_var);

                let mut props = PropertyMap::new();
                props.insert(
                    "cmd", 
                    Property::String(format!("echo '{}'", var_def.initial_value.0))
                );

                tree.push(WidgetNode::Listen {
                    var: var_def.name.0,
                    props,
                });
            }
            TopLevel::ScriptVarDefinition(script_var_def) => {
                match script_var_def {
                    ScriptVarDefinition::Poll(poll) => {
                        let mut props = PropertyMap::new();

                        let cmd = match &poll.command {
                            VarSource::Shell(_, cmd_str) => Property::String(cmd_str.clone()),
                            VarSource::Function(_) => {
                                eprintln!("[yuck] Function-based poll '{}' is not supported, skipping cmd", poll.name);
                                Property::None
                            }
                        };

                        props.insert("cmd", cmd);
                        props.insert("interval", Property::String(
                            format!("{}ms", poll.interval.as_millis())
                        ));

                        let initial = match poll.initial_value {
                            Some(i) => Property::String(i.to_string()),
                            None => Property::String(String::new())
                        };

                        props.insert("initial", initial.clone());

                        let global_var = GlobalVar {
                            name: poll.name.to_string(),
                            initial,
                            template: None,
                        };

                        tree.push(WidgetNode::Poll {
                            var: poll.name.to_string(),
                            props,
                        });

                        global_var_defs.push(global_var);
                    }
                    ScriptVarDefinition::Listen(listen) => {
                        let mut props = PropertyMap::new();

                        props.insert("cmd", Property::String(listen.command));
                        let initial = listen.initial_value.0;
                        props.insert("initial", Property::String(initial.clone()));

                        let global_var = GlobalVar {
                            name: listen.name.to_string(),
                            initial: Property::String(initial),
                            template: None,
                        };

                        tree.push(WidgetNode::Listen {
                            var: listen.name.to_string(),
                            props,
                        });

                        global_var_defs.push(global_var);
                    }
                }
            }
            TopLevel::WidgetDefinition(widget_def) => {
                widget_defs.insert(widget_def.name.clone(), widget_def);
            }
            TopLevel::WindowDefinition(window_def) => {
                let ctx = ConvertContext {
                    defs: &widget_defs,
                    args: HashMap::new(),
                    vars: &global_var_defs,
                };
                let node = widgets::widget_use_to_node(&window_def.widget, &ctx)?;

                tree.push(WidgetNode::DefWindow {
                    name: window_def.name,
                    props: PropertyMap::new(),
                    node: Box::new(node),
                });
            }
        }
    }

    Ok(WidgetNode::Tree(tree))
}