mod convert;
mod widgets;

use ewwii_plugin_api::{auto_plugin, PluginInfo, ConfigInfo, ParseFn, ParseFnExt};
use ewwii_plugin_api::shared_utils::ast::WidgetNode;
use yuck::config::TopLevel;
use yuck::parser::from_ast::FromAst;

auto_plugin!(
    MyPluginName,
    PluginInfo::new("ewwii.language.yuck", "0.1.0"),
    host,
    {
        host.log("Loading language: Yuck!");
        host.register_config_engine(
            ConfigInfo {
                extension: "yuck",
                main_file: "main.yuck",
            },
            ParseFn::new(|source, path| {
                match yuck::parser::parse_toplevel(0, source.to_string()) {
                    Ok((_span, ast_nodes)) => {
                        let top_levels: Vec<TopLevel> = ast_nodes
                            .into_iter()
                            .map(|ast| TopLevel::from_ast(ast)
                            .map_err(|e| e.to_string()))
                            .collect::<Result<Vec<_>, _>>()?;

                        let tree = convert::convert_to_widgetnode(top_levels)?;
                        Ok(tree)
                    }
                    Err(e) => {
                        eprintln!("Parsing error: {}", e);
                        Err(format!("Failed to parse yuck: {}", e))
                    }
                }
            })
        );
    }
);