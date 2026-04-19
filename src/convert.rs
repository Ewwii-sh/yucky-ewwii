use yuck::config::{
    TopLevel, Include, var_definition::VarDefinition,
    script_var_definition::{ScriptVarDefinition, PollScriptVar, ListenScriptVar},
    widget_definition::WidgetDefinition,
    window_definition::WindowDefinition,
};
use yuck::parser::from_ast::FromAst;
use ewwii_plugin_api::shared_utils::ast::WidgetNode;
use ewwii_plugin_api::shared_utils::prop::{PropertyMap, Property};
use std::fs;

pub fn convert_to_widgetnode(top_levels: Vec<TopLevel>) -> Result<WidgetNode, String> {
    let mut tree: Vec<WidgetNode> = Vec::new();

    for top_level in top_levels {
        match top_level {
            TopLevel::Include(inc) => {
                let source = fs::read_to_string(inc.path)
                    .expect("Should have been able to read the file");

                match yuck::parser::parse_toplevel(0, source) {
                    Ok((_span, ast_nodes)) => {
                        let top_levels: Vec<TopLevel> = ast_nodes
                            .into_iter()
                            .map(|ast| TopLevel::from_ast(ast).expect("Invalid yuck syntax"))
                            .collect();

                        let result = convert_to_widgetnode(top_levels)?;

                        if let WidgetNode::Tree(nested_vec) = result {
                            tree.extend(nested_vec);
                        }
                    }
                    Err(e) => {
                        eprintln!("Parsing error: {}", e);
                        return Err(format!("Failed to parse yuck: {}", e))
                    }
                }
            }
            TopLevel::VarDefinition(var_def) => {}
            TopLevel::ScriptVarDefinition(script_var_def) => {
                match script_var_def {
                    ScriptVarDefinition::Poll(poll) => {
                        let mut props = PropertyMap::new();
                        // props.insert("cmd", Property::String());
                        props.insert("interval", Property::String(
                            format!("{}ms", poll.interval.as_millis())
                        ));
                        if let Some(dynval) = poll.initial_value {
                            props.insert("initial", Property::String(dynval.0));
                        }

                        tree.push(WidgetNode::Poll {
                            var: poll.name.to_string(),
                            props,
                        });
                    }
                    ScriptVarDefinition::Listen(listen) => {
                        let mut props = PropertyMap::new();

                        // props.insert("cmd", Property::String());
                        props.insert("initial", Property::String(listen.initial_value.0));

                        tree.push(WidgetNode::Listen {
                            var: listen.name.to_string(),
                            props,
                        });
                    }
                }
            }
            TopLevel::WidgetDefinition(widget_def) => {}
            TopLevel::WindowDefinition(window_def) => {}
        }
    }

    Ok(WidgetNode::Tree(vec![]))
}