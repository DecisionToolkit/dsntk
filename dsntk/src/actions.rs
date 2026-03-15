//! # Command-line actions

use crate::built_in_examples::*;
use crate::errors::{err_create_directory, err_save_file};
use antex::{ColorMode, StyledText, Text};
use clap::{arg, command, crate_version, Arg, ArgAction, ArgMatches, Command};
use dsntk_common::{Jsonify, Result};
use dsntk_feel::values::Value;
use dsntk_feel::FeelScope;
use dsntk_model::{DecisionTable, DmnElement, NamedElement};
use std::fs;
use std::path::Path;

/// Automatic color selection flag.
const COLOR_AUTO: &str = "auto";

/// Always color selection flag.
const COLOR_ALWAYS: &str = "always";

/// Color off-switching flag.
const COLOR_NEVER: &str = "never";

const COLORS: [&str; 3] = [COLOR_AUTO, COLOR_ALWAYS, COLOR_NEVER];

struct AppCommand {
  name: &'static str,
  about: &'static str,
  display_order: usize,
}

const COMMAND_SRV: AppCommand = AppCommand {
  name: "srv",
  about: "Run as a service",
  display_order: 1,
};

const COMMAND_EDM: AppCommand = AppCommand {
  name: "edm",
  about: "Evaluate DMN Model",
  display_order: 2,
};

const COMMAND_EDT: AppCommand = AppCommand {
  name: "edt",
  about: "Evaluate Decision Table",
  display_order: 3,
};

const COMMAND_EFE: AppCommand = AppCommand {
  name: "efe",
  about: "Evaluate FEEL Expression",
  display_order: 4,
};

const COMMAND_PDM: AppCommand = AppCommand {
  name: "pdm",
  about: "Parse DMN Model",
  display_order: 5,
};

const COMMAND_PDT: AppCommand = AppCommand {
  name: "pdt",
  about: "Parse Decision Table",
  display_order: 6,
};

const COMMAND_PFE: AppCommand = AppCommand {
  name: "pfe",
  about: "Parse FEEL Expression",
  display_order: 7,
};

const COMMAND_TDM: AppCommand = AppCommand {
  name: "tdm",
  about: "Test DMN Model",
  display_order: 8,
};

const COMMAND_TDT: AppCommand = AppCommand {
  name: "tdt",
  about: "Test Decision Table",
  display_order: 9,
};

const COMMAND_TFE: AppCommand = AppCommand {
  name: "tfe",
  about: "Test FEEL Expression",
  display_order: 10,
};

const COMMAND_XDM: AppCommand = AppCommand {
  name: "xdm",
  about: "eXport DMN Model",
  display_order: 11,
};

const COMMAND_XDT: AppCommand = AppCommand {
  name: "xdt",
  about: "eXport Decision Table",
  display_order: 12,
};

const COMMAND_XFE: AppCommand = AppCommand {
  name: "xfe",
  about: "eXport FEEL Expression",
  display_order: 13,
};

const COMMAND_EXS: AppCommand = AppCommand {
  name: "exs",
  about: "Save examples",
  display_order: 14,
};

/// Supported decision table input file formats.
enum DecisionTableFormat {
  /// Decision table defined as plain Unicode text.
  Unicode,
  /// Decision table defined as plain Markdown text.
  Markdown,
}

/// Command-line actions.
enum Action {
  /// Parse FEEL expression.
  ParseFeelExpression(
    /// Name of the file containing parsing context.
    String,
    /// Name of the file containing FEEL expression to be parsed.
    String,
    /// Requested color mode.
    ColorMode,
  ),
  /// Evaluate FEEL expression.
  EvaluateFeelExpression(
    /// Name of the file containing input data.
    String,
    /// Name of the file containing FEEL expression to be evaluated.
    String,
  ),
  /// Test FEEL expression.
  TestFeelExpression(
    /// Name of the file containing tests.
    String,
    /// Name of the file containing FEEL expression to be tested.
    String,
    /// Flag indicating if only test summary should be printed.
    bool,
    /// Requested color mode.
    ColorMode,
  ),
  /// Export FEEL expression to HTML.
  ExportFeelExpression(
    /// Name of the file containing parsing context.
    String,
    /// Name of the file containing FEEL expression to be exported.
    String,
    /// Output HTML file name.
    String,
  ),
  /// Parse decision table.
  ParseDecisionTable(
    /// Name of the file containing decision table definition to be parsed.
    String,
    /// Input format of the parsed decision table.
    DecisionTableFormat,
  ),
  /// Evaluate decision table.
  EvaluateDecisionTable(
    /// Name of the file containing input data.
    String,
    /// Name of the file containing decision table definition to be evaluated.
    String,
    /// Input format of the evaluated decision table.
    DecisionTableFormat,
  ),
  /// Test decision table.
  TestDecisionTable(
    /// Test file name.
    String,
    /// Name of the file containing decision table definition to be tested.
    String,
    /// Input format of the tested decision table.
    DecisionTableFormat,
    /// Flag indicating if only test summary will be printed.
    bool,
    /// Requested color mode.
    ColorMode,
  ),
  /// Export decision table.
  ExportDecisionTable(
    /// Decision table file name.
    String,
    /// Output HTML file name.
    String,
    /// Input format of the exported decision table.
    DecisionTableFormat,
  ),
  /// Parse DMN model.
  ParseDmnModel(
    /// Name of the file containing DMN model.
    String,
    /// Requested color mode.
    ColorMode,
  ),
  /// Evaluate DMN model.
  EvaluateDmnModel(
    /// Name of the file containing input data.
    String,
    /// Name of the file containing DMN model to be evaluated.
    String,
    /// Name of the invocable to be evaluated.
    String,
  ),
  /// Test DMN model.
  TestDmnModel(
    /// Test file name.
    String,
    /// Decision table file name.
    String,
    /// Invocable name.
    String,
    /// Flag indicating if only test summary will be printed.
    bool,
    /// Requested color mode.
    ColorMode,
  ),
  /// Export DMN model.
  ExportDmnModel(
    /// Name of the file containing DMN model.
    String,
    /// Output HTML file name.
    String,
  ),
  /// Starts as a service.
  StartService(
    /// Optional host name.
    Option<String>,
    /// Optional port number
    Option<String>,
    /// Optional directory containing models to be loaded on start.
    Vec<String>,
    /// Requested color mode.
    ColorMode,
    /// Flag indicating if more detailed information should be displayed during startup.
    bool,
  ),
  /// Save examples.
  SaveExamples(
    /// Directory where examples are saved.
    String,
  ),
  /// Do nothing, no action was specified.
  DoNothing,
}

/// Executes command-line action.
pub async fn do_action() -> std::io::Result<()> {
  match get_cli_action() {
    Action::ParseFeelExpression(ctx_file_name, feel_file_name, cm) => {
      parse_feel_expression(&ctx_file_name, &feel_file_name, cm);
      Ok(())
    }
    Action::EvaluateFeelExpression(input_file_name, feel_file_name) => {
      evaluate_feel_expression(&input_file_name, &feel_file_name);
      Ok(())
    }
    Action::TestFeelExpression(test_file_name, feel_file_name, summary_only, cm) => {
      test_feel_expression(&test_file_name, &feel_file_name, summary_only, cm);
      Ok(())
    }
    Action::ExportFeelExpression(ctx_file_name, feel_file_name, html_file_name) => {
      export_feel_expression(&ctx_file_name, &feel_file_name, &html_file_name);
      Ok(())
    }
    Action::ParseDecisionTable(dectab_file_name, dectab_file_format) => {
      parse_decision_table(&dectab_file_name, dectab_file_format);
      Ok(())
    }
    Action::EvaluateDecisionTable(input_file_name, dectab_file_name, dectab_file_format) => {
      evaluate_decision_table(&input_file_name, &dectab_file_name, dectab_file_format);
      Ok(())
    }
    Action::TestDecisionTable(test_file_name, dectab_file_name, dectab_file_format, summary_only, cm) => {
      test_decision_table(&test_file_name, &dectab_file_name, dectab_file_format, summary_only, cm);
      Ok(())
    }
    Action::ExportDecisionTable(dectab_file_name, html_file_name, dectab_file_format) => {
      export_decision_table(&dectab_file_name, &html_file_name, dectab_file_format);
      Ok(())
    }
    Action::ParseDmnModel(dmn_file_name, cm) => {
      parse_dmn_model(&dmn_file_name, cm);
      Ok(())
    }
    Action::EvaluateDmnModel(dmn_file_name, ctx_file_name, invocable_name) => {
      evaluate_dmn_model(&dmn_file_name, &ctx_file_name, &invocable_name);
      Ok(())
    }
    Action::TestDmnModel(test_file_name, dmn_file_name, invocable_name, summary_only, cm) => {
      test_dmn_model(&test_file_name, &dmn_file_name, &invocable_name, summary_only, cm);
      Ok(())
    }
    Action::ExportDmnModel(dmn_file_name, html_file_name) => {
      export_dmn_model(&dmn_file_name, &html_file_name);
      Ok(())
    }
    Action::StartService(opt_host, opt_port, opt_dir, cm, verbose) => {
      // Start a REST API server.
      dsntk_server::start_server(opt_host, opt_port, opt_dir, cm, verbose).await
    }
    Action::SaveExamples(root_dir) => {
      // Save the examples in the specified root directory.
      save_builtin_examples(&root_dir);
      Ok(())
    }
    Action::DoNothing => {
      // No specific action was requested.
      Ok(())
    }
  }
}

/// Parses CLI argument matches.
fn get_matches() -> ArgMatches {
  command!()
    // disable the built-in version flag
    .disable_version_flag(true)
    // handle the version flag in a custom way
    .arg(Arg::new("version").short('V').long("version").help("Print version").action(ArgAction::SetTrue))
    // pfe - Parse FEEL Expression
    .subcommand(
      Command::new(COMMAND_PFE.name)
        .about(COMMAND_PFE.about)
        .display_order(COMMAND_PFE.display_order)
        .arg(
          arg!(-c --color <WHEN>)
            .help("Control when colored output is used")
            .value_parser(COLORS)
            .action(ArgAction::Set)
            .display_order(1),
        )
        .arg(arg!(<CONTEXT_FILE>).help("File containing context for parsed FEEL expression").required(true).index(1))
        .arg(arg!(<FEEL_FILE>).help("File containing FEEL expression to be parsed").required(true).index(2)),
    )
    // efe - Evaluate FEEL Expression
    .subcommand(
      Command::new(COMMAND_EFE.name)
        .about(COMMAND_EFE.about)
        .display_order(COMMAND_EFE.display_order)
        .arg(arg!(<INPUT_FILE>).help("File containing input data for evaluated FEEL expression").required(true).index(1))
        .arg(arg!(<FEEL_FILE>).help("File containing FEEL expression to be evaluated").required(true).index(2)),
    )
    // tfe - Test FEEL Expression
    .subcommand(
      Command::new(COMMAND_TFE.name)
        .about(COMMAND_TFE.about)
        .display_order(COMMAND_TFE.display_order)
        .arg(
          arg!(--"summary")
            .help("Display only summary after completing all tests")
            .short('s')
            .action(ArgAction::SetTrue)
            .display_order(1),
        )
        .arg(
          arg!(--"color" <WHEN>)
            .help("Control when colored output is used")
            .short('c')
            .value_parser(COLORS)
            .action(ArgAction::Set)
            .display_order(2),
        )
        .arg(arg!(<TEST_FILE>).help("File containing test cases for tested FEEL expression").required(true).index(1))
        .arg(arg!(<FEEL_FILE>).help("File containing FEEL expression to be tested").required(true).index(2)),
    )
    // xfe - Export FEEL Expression
    .subcommand(
      Command::new(COMMAND_XFE.name)
        .about(COMMAND_XFE.about)
        .display_order(COMMAND_XFE.display_order)
        .arg(
          arg!(<INPUT_FILE>)
            .help("File containing input data for expression to be exported to HTML")
            .required(true)
            .index(1),
        )
        .arg(arg!(<FEEL_FILE>).help("File containing FEEL expression to be exported to HTML").required(true).index(2))
        .arg(arg!(<HTML_FILE>).help("Output HTML file").required(true).index(3)),
    )
    // pdm - Parse DMN Model
    .subcommand(
      Command::new(COMMAND_PDM.name)
        .about(COMMAND_PDM.about)
        .display_order(COMMAND_PDM.display_order)
        .arg(
          arg!(--"color" <WHEN>)
            .help("Control when colored output is used")
            .short('c')
            .value_parser(COLORS)
            .action(ArgAction::Set)
            .display_order(1),
        )
        .arg(arg!(<DMN_FILE>).help("File containing DMN model to be parsed").required(true).index(1)),
    )
    // edm - Evaluate DMN Model
    .subcommand(
      Command::new(COMMAND_EDM.name)
        .about(COMMAND_EDM.about)
        .display_order(COMMAND_EDM.display_order)
        .arg(
          arg!(--"invocable" <NAME>)
            .help("Name of the invocable (decision, bkm, decision service) to be evaluated")
            .short('i')
            .action(ArgAction::Set)
            .required(true)
            .display_order(1),
        )
        .arg(arg!(<INPUT_FILE>).help("File containing input data for evaluated DMN model").required(true).index(1))
        .arg(arg!(<DMN_FILE>).help("File containing DMN model to be evaluated").required(true).index(2)),
    )
    // tdm - Test DMN Model
    .subcommand(
      Command::new(COMMAND_TDM.name)
        .about(COMMAND_TDM.about)
        .display_order(COMMAND_TDM.display_order)
        .arg(
          arg!(--"invocable" <NAME>)
            .help("Name of the invocable to be tested")
            .short('i')
            .required(true)
            .action(ArgAction::Set)
            .display_order(1),
        )
        .arg(
          arg!(--"summary")
            .help("Display only summary after completing all tests")
            .short('s')
            .action(ArgAction::SetTrue)
            .display_order(2),
        )
        .arg(
          arg!(--"color" <WHEN>)
            .help("Control when colored output is used")
            .short('c')
            .value_parser(COLORS)
            .action(ArgAction::Set)
            .display_order(3),
        )
        .arg(arg!(<TEST_FILE>).help("File containing test cases for tested DMN model").required(true).index(1))
        .arg(arg!(<DMN_FILE>).help("File containing DMN model to be tested").required(true).index(2)),
    )
    // xdm - Export DMN Model
    .subcommand(
      Command::new(COMMAND_XDM.name)
        .about(COMMAND_XDM.about)
        .display_order(COMMAND_XDM.display_order)
        .arg(arg!(<DMN_FILE>).help("File containing DMN model to be exported to HTML").required(true).index(1))
        .arg(arg!(<HTML_FILE>).help("Output HTML file").required(true).index(2)),
    )
    // pdt - Parse Decision Table
    .subcommand(
      Command::new(COMMAND_PDT.name)
        .about(COMMAND_PDT.about)
        .display_order(COMMAND_PDT.display_order)
        .arg(
          arg!(--"markdown")
            .help("Markdown decision table")
            .short('m')
            .action(ArgAction::SetTrue)
            .display_order(1)
            .conflicts_with("unicode"),
        )
        .arg(
          arg!(--"unicode")
            .help("Unicode decision table (default)")
            .short('u')
            .action(ArgAction::SetTrue)
            .display_order(2)
            .conflicts_with("markdown"),
        )
        .arg(arg!(<DECTAB_FILE>).help("File containing decision table to be parsed").required(true).index(1)),
    )
    // edt - Evaluate Decision Table
    .subcommand(
      Command::new(COMMAND_EDT.name)
        .about(COMMAND_EDT.about)
        .display_order(COMMAND_EDT.display_order)
        .arg(
          arg!(--"markdown")
            .help("Markdown decision table")
            .short('m')
            .action(ArgAction::SetTrue)
            .display_order(1)
            .conflicts_with("unicode"),
        )
        .arg(
          arg!(--"unicode")
            .help("Unicode decision table (default)")
            .short('u')
            .action(ArgAction::SetTrue)
            .display_order(2)
            .conflicts_with("markdown"),
        )
        .arg(arg!(<INPUT_FILE>).help("File containing input data for evaluated decision table").required(true).index(1))
        .arg(arg!(<DECTAB_FILE>).help("File containing decision table to be evaluated").required(true).index(2)),
    )
    // tdt - Test Decision Table
    .subcommand(
      Command::new(COMMAND_TDT.name)
        .about(COMMAND_TDT.about)
        .display_order(COMMAND_TDT.display_order)
        .arg(
          arg!(--"markdown")
            .help("Markdown decision table")
            .short('m')
            .action(ArgAction::SetTrue)
            .display_order(1)
            .conflicts_with("unicode"),
        )
        .arg(
          arg!(--"unicode")
            .help("Unicode decision table (default)")
            .short('u')
            .action(ArgAction::SetTrue)
            .display_order(2)
            .conflicts_with("markdown"),
        )
        .arg(
          arg!(--"summary")
            .help("Display only summary after completing all tests")
            .short('s')
            .action(ArgAction::SetTrue)
            .display_order(3),
        )
        .arg(
          arg!(--"color" <WHEN>)
            .help("Control when colored output is used")
            .short('c')
            .value_parser(COLORS)
            .action(ArgAction::Set)
            .display_order(4),
        )
        .arg(arg!(<TEST_FILE>).help("File containing test cases for tested decision table").required(true).index(1))
        .arg(arg!(<DECTAB_FILE>).help("File containing FEEL expression to be tested").required(true).index(2)),
    )
    // xdt - Export Decision Table
    .subcommand(
      Command::new(COMMAND_XDT.name)
        .about(COMMAND_XDT.about)
        .display_order(COMMAND_XDT.display_order)
        .arg(
          arg!(--"markdown")
            .help("Markdown decision table")
            .short('m')
            .action(ArgAction::SetTrue)
            .display_order(1)
            .conflicts_with("unicode"),
        )
        .arg(
          arg!(--"unicode")
            .help("Unicode decision table (default)")
            .short('u')
            .action(ArgAction::SetTrue)
            .display_order(2)
            .conflicts_with("markdown"),
        )
        .arg(arg!(<DECTAB_FILE>).help("File containing decision table to be exported to HTML").required(true).index(1))
        .arg(arg!(<HTML_FILE>).help("Output HTML file").required(true).index(2)),
    )
    // srv - Run as a service
    .subcommand(
      Command::new(COMMAND_SRV.name)
        .about(COMMAND_SRV.about)
        .display_order(COMMAND_SRV.display_order)
        .arg(arg!(-H --host <HOST>).help("Host name").action(ArgAction::Set).display_order(1))
        .arg(arg!(-P --port <PORT>).help("Port number").action(ArgAction::Set).display_order(2))
        .arg(
          arg!(-D <DIR>)
            .help("Directory where DMN files are searched")
            .id("dir")
            .action(ArgAction::Append)
            .display_order(3),
        )
        .arg(
          arg!(--"verbose")
            .help("Displays model deployment details during startup")
            .short('v')
            .action(ArgAction::SetTrue)
            .display_order(4),
        )
        .arg(
          arg!(--"color" <WHEN>)
            .help("Control when colored output is used")
            .short('c')
            .value_parser(COLORS)
            .action(ArgAction::Set)
            .display_order(4),
        ),
    )
    // exs - Save examples
    .subcommand(
      Command::new(COMMAND_EXS.name)
        .about(COMMAND_EXS.about)
        .display_order(COMMAND_EXS.display_order)
        .arg(arg!(<DIR>).help("Directory where examples are saved").action(ArgAction::Set).required(true).index(1)),
    )
    .get_matches()
}

/// Checks the list of arguments passed from the command line
/// and returns an action related to a valid argument.
fn get_cli_action() -> Action {
  let matches = get_matches();
  // replaces built-in version flag with the custom handler
  if matches.get_flag("version") {
    // displays only the version number, without the name of the crate
    println!("{}", crate_version!());
    return Action::DoNothing;
  }
  match matches.subcommand() {
    // parse FEEL expression subcommand
    Some(("pfe", matches)) => {
      return Action::ParseFeelExpression(match_string(matches, "CONTEXT_FILE"), match_string(matches, "FEEL_FILE"), match_color(matches));
    }
    // evaluate FEEL expression subcommand
    Some(("efe", matches)) => {
      return Action::EvaluateFeelExpression(match_string(matches, "INPUT_FILE"), match_string(matches, "FEEL_FILE"));
    }
    // test FEEL expression subcommand
    Some(("tfe", matches)) => {
      return Action::TestFeelExpression(
        match_string(matches, "TEST_FILE"),
        match_string(matches, "FEEL_FILE"),
        match_summary(matches),
        match_color(matches),
      );
    }
    // export FEEL expression subcommand
    Some(("xfe", matches)) => {
      return Action::ExportFeelExpression(match_string(matches, "INPUT_FILE"), match_string(matches, "FEEL_FILE"), match_string(matches, "HTML_FILE"));
    }
    // parse decision table subcommand
    Some(("pdt", matches)) => {
      return Action::ParseDecisionTable(
        match_string(matches, "DECTAB_FILE"),
        match (match_unicode(matches), match_markdown(matches)) {
          (_, true) => DecisionTableFormat::Markdown,
          _ => DecisionTableFormat::Unicode,
        },
      );
    }
    // evaluate decision table subcommand
    Some(("edt", matches)) => {
      return Action::EvaluateDecisionTable(
        match_string(matches, "INPUT_FILE"),
        match_string(matches, "DECTAB_FILE"),
        match (match_unicode(matches), match_markdown(matches)) {
          (_, true) => DecisionTableFormat::Markdown,
          _ => DecisionTableFormat::Unicode,
        },
      );
    }
    // test decision table subcommand
    Some(("tdt", matches)) => {
      return Action::TestDecisionTable(
        match_string(matches, "TEST_FILE"),
        match_string(matches, "DECTAB_FILE"),
        match (match_unicode(matches), match_markdown(matches)) {
          (_, true) => DecisionTableFormat::Markdown,
          _ => DecisionTableFormat::Unicode,
        },
        match_summary(matches),
        match_color(matches),
      );
    }
    // export decision table subcommand
    Some(("xdt", matches)) => {
      return Action::ExportDecisionTable(
        match_string(matches, "DECTAB_FILE"),
        match_string(matches, "HTML_FILE"),
        match (match_unicode(matches), match_markdown(matches)) {
          (_, true) => DecisionTableFormat::Markdown,
          _ => DecisionTableFormat::Unicode,
        },
      );
    }
    // parse DMN model subcommand
    Some(("pdm", matches)) => {
      return Action::ParseDmnModel(match_string(matches, "DMN_FILE"), match_color(matches));
    }
    // evaluate DMN model subcommand
    Some(("edm", matches)) => {
      return Action::EvaluateDmnModel(match_string(matches, "INPUT_FILE"), match_string(matches, "DMN_FILE"), match_string(matches, "invocable"));
    }
    // test DMN model subcommand
    Some(("tdm", matches)) => {
      return Action::TestDmnModel(
        match_string(matches, "TEST_FILE"),
        match_string(matches, "DMN_FILE"),
        match_string(matches, "invocable"),
        match_summary(matches),
        match_color(matches),
      );
    }
    // export DMN model subcommand
    Some(("xdm", matches)) => {
      return Action::ExportDmnModel(match_string(matches, "DMN_FILE"), match_string(matches, "HTML_FILE"));
    }
    // start server subcommand
    Some(("srv", matches)) => {
      return Action::StartService(
        match_optional_string(matches, "host"),
        match_optional_string(matches, "port"),
        matches.get_many("dir").unwrap_or_default().cloned().collect(),
        match_color(matches),
        match_verbose(matches),
      );
    }
    // generate examples
    Some(("exs", matches)) => {
      return Action::SaveExamples(match_string(matches, "DIR"));
    }
    _ => {}
  }
  Text::default()
    .green()
    .bold()
    .s("dsntk")
    .clear()
    .s(" | ")
    .green()
    .s("DecisionToolkit")
    .clear()
    .s(" | ")
    .green()
    .s(crate_version!())
    .clear()
    .nl()
    .color_256(250)
    .s("Try '")
    .cyan()
    .s("dsntk --help")
    .color_256(250)
    .s("' to see all available commands.")
    .nl()
    .s("For more information, visit ")
    .cyan()
    .underline()
    .s("https://decision-toolkit.org")
    .cprintln();
  Action::DoNothing
}

/// Matches a mandatory string argument.
fn match_string(matches: &ArgMatches, name: &str) -> String {
  matches.get_one::<String>(name).unwrap().to_string()
}

/// Matches an optional string argument.
fn match_optional_string(matches: &ArgMatches, name: &str) -> Option<String> {
  matches.get_one::<String>(name).map(|value| value.to_string())
}

/// Matches color mode.
fn match_color(matches: &ArgMatches) -> ColorMode {
  matches.get_one::<String>("color").unwrap_or(&COLOR_AUTO.to_string()).into()
}

/// Matches summary flag.
fn match_summary(matches: &ArgMatches) -> bool {
  matches.get_flag("summary")
}

/// Matches Unicode flag.
fn match_unicode(matches: &ArgMatches) -> bool {
  matches.get_flag("unicode")
}

/// Matches Markdown flag.
fn match_markdown(matches: &ArgMatches) -> bool {
  matches.get_flag("markdown")
}

/// Matches verbosity flag.
fn match_verbose(matches: &ArgMatches) -> bool {
  matches.get_flag("verbose")
}

/// Parses `FEEL` expression loaded from file and prints the parsed `AST` to standard output.
fn parse_feel_expression(ctx_file_name: &str, feel_file_name: &str, cm: ColorMode) {
  match fs::read_to_string(feel_file_name) {
    Ok(feel_expression) => match fs::read_to_string(ctx_file_name) {
      Ok(context_definition) => match dsntk_evaluator::evaluate_context(&FeelScope::default(), &context_definition) {
        Ok(ctx) => match dsntk_feel_parser::parse_expression(&ctx.into(), &feel_expression, false) {
          Ok(node) => {
            println!("    AST:\n{}", node.tree(6, cm));
          }
          Err(reason) => eprintln!("parsing expression failed with reason: {reason}"),
        },
        Err(reason) => eprintln!("evaluating context failed with reason: {reason}"),
      },
      Err(reason) => eprintln!("loading context file `{ctx_file_name}` failed with reason: {reason}"),
    },
    Err(reason) => eprintln!("loading expression file `{feel_file_name}` failed with reason: {reason}"),
  }
}

/// Evaluates `FEEL` expression loaded from file and prints the result to standard output.
fn evaluate_feel_expression(ctx_file_name: &str, feel_file_name: &str) {
  match fs::read_to_string(feel_file_name) {
    Ok(textual_expression) => match fs::read_to_string(ctx_file_name) {
      Ok(context_definition) => match dsntk_evaluator::evaluate_context(&FeelScope::default(), &context_definition) {
        Ok(ctx) => match dsntk_feel_parser::parse_expression(&ctx.clone().into(), &textual_expression, false) {
          Ok(ast_root_node) => {
            println!("{}", dsntk_evaluator::evaluate(&ctx.into(), &ast_root_node));
          }
          Err(reason) => eprintln!("parsing expression failed with reason: {reason}"),
        },
        Err(reason) => eprintln!("evaluating context failed with reason: {reason}"),
      },
      Err(reason) => eprintln!("loading context file `{ctx_file_name}` failed with reason: {reason}"),
    },
    Err(reason) => eprintln!("loading expression file `{feel_file_name}` failed with reason: {reason}"),
  }
}

/// Tests `FEEL` expression loaded from file and prints the test result to standard output.
fn test_feel_expression(test_file_name: &str, feel_file_name: &str, summary_only: bool, cm: ColorMode) {
  match fs::read_to_string(feel_file_name) {
    Ok(feel_file_content) => match fs::read_to_string(test_file_name) {
      Ok(test_file_content) => match dsntk_evaluator::prepare_test_cases(&test_file_content) {
        Ok(test_cases) => {
          let mut passed = 0_usize;
          let mut failed = 0_usize;
          for (test_no, (input_data, expected)) in test_cases.iter().enumerate() {
            let scope = input_data.clone().into();
            match dsntk_feel_parser::parse_expression(&scope, &feel_file_content, false) {
              Ok(node) => {
                let actual = dsntk_evaluator::evaluate(&scope, &node);
                display_test_case_result(&actual, expected, &test_no, &mut passed, &mut failed, summary_only, cm);
              }
              Err(reason) => eprintln!("parsing expression failed with reason: {reason}"),
            }
          }
          display_test_summary(passed, failed, summary_only, cm);
        }
        Err(reason) => eprintln!("evaluation of test cases failed with reason: {reason}"),
      },
      Err(reason) => eprintln!("loading test file `{test_file_name}` failed with reason: {reason}"),
    },
    Err(reason) => eprintln!("loading expression file `{feel_file_name}` failed with reason: {reason}"),
  }
}

/// Exports `FEEL` expression loaded from file to HTML output file.
fn export_feel_expression(_ctx_file_name: &str, _feel_file_name: &str, html_file_name: &str) {
  let _ = fs::write(html_file_name, "not implemented yet\n");
}

/// Parses decision table loaded from text file.
fn parse_decision_table(dectab_file_name: &str, dectab_file_format: DecisionTableFormat) {
  match fs::read_to_string(dectab_file_name) {
    Ok(dtb_file_content) => {
      let recognition_result = match dectab_file_format {
        DecisionTableFormat::Unicode => dsntk_recognizer::from_unicode(&dtb_file_content, true),
        DecisionTableFormat::Markdown => dsntk_recognizer::from_markdown(&dtb_file_content, true),
      };
      match recognition_result {
        Ok(_) => {}
        Err(reason) => eprintln!("ERROR: {reason}"),
      }
    }
    Err(reason) => eprintln!("loading decision table file `{dectab_file_name}` failed with reason: {reason}"),
  }
}

/// Evaluates context and decision table loaded from files.
fn evaluate_decision_table(input_file_name: &str, dectab_file_name: &str, dectab_file_format: DecisionTableFormat) {
  let input_file_content = match fs::read_to_string(input_file_name) {
    Ok(input_file_content) => input_file_content,
    Err(reason) => {
      eprintln!("loading input file `{input_file_name}` failed with reason: {reason}");
      return;
    }
  };
  let input_data = match dsntk_evaluator::evaluate_context(&FeelScope::default(), &input_file_content) {
    Ok(input_data) => input_data,
    Err(reason) => {
      eprintln!("evaluating input data failed with reason: {reason}");
      return;
    }
  };
  let dtb_file_content = match fs::read_to_string(dectab_file_name) {
    Ok(dtb_file_content) => dtb_file_content,
    Err(reason) => {
      eprintln!("loading input file `{dectab_file_name}` failed with reason: {reason}");
      return;
    }
  };
  let recognition_result = match dectab_file_format {
    DecisionTableFormat::Unicode => dsntk_recognizer::from_unicode(&dtb_file_content, false),
    DecisionTableFormat::Markdown => dsntk_recognizer::from_markdown(&dtb_file_content, false),
  };
  let recognized_decision_table = match recognition_result {
    Ok(decision_table) => decision_table,
    Err(reason) => {
      eprintln!("building decision table failed with reason: {reason}");
      return;
    }
  };
  let scope = input_data.into();
  let evaluator = match dsntk_evaluator::build_decision_table_evaluator(&scope, &recognized_decision_table.into()) {
    Ok(evaluator) => evaluator,
    Err(reason) => {
      eprintln!("building decision table evaluator failed with reason: {reason}");
      return;
    }
  };
  let result = evaluator(&scope);
  println!("{}", result.jsonify());
}

/// Tests decision table loaded from file.
fn test_decision_table(test_file_name: &str, dectab_file_name: &str, dectab_file_format: DecisionTableFormat, summary_only: bool, cm: ColorMode) {
  let dtb_file_content = match fs::read_to_string(dectab_file_name) {
    Ok(dtb_file_content) => dtb_file_content,
    Err(reason) => {
      eprintln!("loading decision table file `{dectab_file_name}` failed with reason: {reason}");
      return;
    }
  };
  let recognition_result = match dectab_file_format {
    DecisionTableFormat::Unicode => dsntk_recognizer::from_unicode(&dtb_file_content, false),
    DecisionTableFormat::Markdown => dsntk_recognizer::from_markdown(&dtb_file_content, false),
  };
  let decision_table: DecisionTable = match recognition_result {
    Ok(decision_table) => decision_table.into(),
    Err(reason) => {
      eprintln!("building decision table failed with reason: {reason}");
      return;
    }
  };
  let test_file_content = match fs::read_to_string(test_file_name) {
    Ok(test_file_content) => test_file_content,
    Err(reason) => {
      eprintln!("loading test file `{test_file_name}` failed with reason: {reason}");
      return;
    }
  };
  let test_cases = match dsntk_evaluator::prepare_test_cases(&test_file_content) {
    Ok(test_cases) => test_cases,
    Err(reason) => {
      eprintln!("evaluating test file failed with reason: {reason}");
      return;
    }
  };
  let mut passed = 0_usize;
  let mut failed = 0_usize;
  for (test_no, (input_data, expected)) in test_cases.iter().enumerate() {
    let scope = input_data.clone().into();
    let evaluator = match dsntk_evaluator::build_decision_table_evaluator(&scope, &decision_table) {
      Ok(evaluator) => evaluator,
      Err(reason) => {
        eprintln!("building decision table evaluator failed with reason: {reason}");
        return;
      }
    };
    let actual = evaluator(&scope);
    display_test_case_result(&actual, expected, &test_no, &mut passed, &mut failed, summary_only, cm);
  }
  display_test_summary(passed, failed, summary_only, cm);
}

/// Exports decision table loaded from text file to HTML output file.
fn export_decision_table(dectab_file_name: &str, html_file_name: &str, dectab_file_format: DecisionTableFormat) {
  match fs::read_to_string(dectab_file_name) {
    Ok(dtb_file_content) => {
      let recognition_result = match dectab_file_format {
        DecisionTableFormat::Unicode => dsntk_recognizer::from_unicode(&dtb_file_content, false),
        DecisionTableFormat::Markdown => dsntk_recognizer::from_markdown(&dtb_file_content, false),
      };
      match recognition_result {
        Ok(recognized_decision_table) => {
          let html_output = dsntk_gendoc::decision_table_to_html(&recognized_decision_table.into());
          if let Err(reason) = fs::write(html_file_name, html_output) {
            println!("writing output HTML file `{html_file_name}` failed with reason: {reason}")
          }
        }
        Err(reason) => eprintln!("ERROR: {reason}"),
      }
    }
    Err(reason) => eprintln!("loading decision table file `{dectab_file_name}` failed with reason: {reason}"),
  }
}

/// Parses DMN model loaded from XML file and prints ASCII report.
fn parse_dmn_model(dmn_file_name: &str, cm: ColorMode) {
  match fs::read_to_string(dmn_file_name) {
    Ok(dmn_file_content) => match dsntk_model::from_xml(&dmn_file_content) {
      Ok(definitions) => {
        dsntk_gendoc::print_model(definitions, cm);
      }
      Err(reason) => eprintln!("parsing model file failed with reason: {reason}"),
    },
    Err(reason) => eprintln!("loading model file `{dmn_file_name}` failed with reason: {reason:?}"),
  }
}

/// Evaluates DMN model loaded from XML file.
fn evaluate_dmn_model(input_file_name: &str, dmn_file_name: &str, invocable_name: &str) {
  match fs::read_to_string(dmn_file_name) {
    Ok(dmn_file_content) => match fs::read_to_string(input_file_name) {
      Ok(input_file_content) => match dsntk_evaluator::evaluate_context(&FeelScope::default(), &input_file_content) {
        Ok(input_data) => match dsntk_model::from_xml(&dmn_file_content) {
          Ok(definitions) => {
            let model_namespace = definitions.namespace().to_string();
            let model_name = definitions.name().to_string();
            match dsntk_evaluator::ModelEvaluator::new(&[definitions]) {
              Ok(model_evaluator) => {
                let result = model_evaluator.evaluate_invocable(&model_namespace, &model_name, invocable_name, &input_data);
                println!("{}", result.jsonify())
              }
              Err(reason) => eprintln!("building model evaluator failed with reason: {reason}"),
            }
          }
          Err(reason) => eprintln!("parsing model failed with reason: {reason}"),
        },
        Err(reason) => eprintln!("evaluating input data failed with reason: {reason}"),
      },
      Err(reason) => eprintln!("loading input data file `{input_file_name}` failed with reason: {reason}"),
    },
    Err(reason) => eprintln!("loading model file `{dmn_file_name}` failed with reason: {reason}"),
  }
}

/// Tests DMN model loaded from XML file.
fn test_dmn_model(test_file_name: &str, dmn_file_name: &str, invocable_name: &str, summary_only: bool, cm: ColorMode) {
  let dmn_file_content = match fs::read_to_string(dmn_file_name) {
    Ok(dmn_file_content) => dmn_file_content,
    Err(reason) => {
      eprintln!("loading model file `{dmn_file_name}` failed with reason: {reason}");
      return;
    }
  };
  let definitions = match dsntk_model::from_xml(&dmn_file_content) {
    Ok(definitions) => definitions,
    Err(reason) => {
      eprintln!("parsing model file failed with reason: {reason}");
      return;
    }
  };
  let model_namespace = definitions.namespace().to_string();
  let model_name = definitions.name().to_string();
  let model_evaluator = match dsntk_evaluator::ModelEvaluator::new(&[definitions]) {
    Ok(model_evaluator) => model_evaluator,
    Err(reason) => {
      eprintln!("building model evaluator failed with reason: {reason}");
      return;
    }
  };
  let test_file_content = match fs::read_to_string(test_file_name) {
    Ok(test_file_content) => test_file_content,
    Err(reason) => {
      eprintln!("loading test file `{test_file_name}` failed with reason: {reason}");
      return;
    }
  };
  let test_cases = match dsntk_evaluator::prepare_test_cases(&test_file_content) {
    Ok(test_cases) => test_cases,
    Err(reason) => {
      eprintln!("evaluating test file failed with reason: {reason}");
      return;
    }
  };
  let mut passed = 0_usize;
  let mut failed = 0_usize;
  for (test_no, (input_data, expected)) in test_cases.iter().enumerate() {
    let actual = model_evaluator.evaluate_invocable(&model_namespace, &model_name, invocable_name, input_data);
    display_test_case_result(&actual, expected, &test_no, &mut passed, &mut failed, summary_only, cm);
  }
  display_test_summary(passed, failed, summary_only, cm);
}

/// Exports DMN model loaded from `XML` file to `HTML` output file.
fn export_dmn_model(dmn_file_name: &str, html_file_name: &str) {
  match fs::read_to_string(dmn_file_name) {
    Ok(dmn_file_content) => match dsntk_model::from_xml(&dmn_file_content) {
      Ok(definitions) => {
        let html_output = dsntk_gendoc::dmn_model_to_html(&definitions);
        if let Err(reason) = fs::write(html_file_name, html_output) {
          println!("writing output HTML file `{html_file_name}` failed with reason: {reason}")
        }
      }
      Err(reason) => eprintln!("ERROR: {reason}"),
    },
    Err(reason) => eprintln!("loading model file `{dmn_file_name}` failed with reason: {reason}"),
  }
}

/// Saves built-in examples in specified directory.
fn save_builtin_examples(root_dir: &str) {
  // Utility function for creating (sub)directories.
  let create_dir = |root_dir: &str, child_dir: &str| -> Result<()> {
    let root_path = Path::new(root_dir);
    let child_path = Path::new(child_dir);
    let path = root_path.join(child_path);
    fs::create_dir_all(&path).map_err(|e| err_create_directory(&path.to_string_lossy(), &e.to_string()))
  };
  // Utility function for saving the file content.
  let write_file = |root_dir: &str, child_dir: &str, contents: &str| -> Result<()> {
    let root_path = Path::new(root_dir);
    let child_path = Path::new(child_dir);
    let path = root_path.join(child_path);
    fs::write(&path, contents).map_err(|e| err_save_file(&path.to_string_lossy(), &e.to_string()))
  };
  // Utility function for saving all examples.
  let save_examples = |root_dir: &str| -> Result<()> {
    // Save example decision model.
    create_dir(root_dir, "dm")?;
    write_file(root_dir, "dm/dm.ctx", EXAMPLE_DM_CTX)?;
    write_file(root_dir, "dm/dm.dmn", EXAMPLE_DM)?;
    // Save example decision table.
    create_dir(root_dir, "dt")?;
    write_file(root_dir, "dt/dt.ctx", EXAMPLE_DT_CTX)?;
    write_file(root_dir, "dt/dt.dtb", EXAMPLE_DT)?;
    // Save example FEEL expression.
    create_dir(root_dir, "fe")?;
    write_file(root_dir, "fe/fe.ctx", EXAMPLE_FE_CTX)?;
    write_file(root_dir, "fe/fe.feel", EXAMPLE_FE)?;
    Ok(())
  };
  // Try to save the examples.
  match save_examples(root_dir) {
    Ok(()) => {
      // Display the summary message.
      //TODO display saved directory tree
    }
    Err(reason) => eprintln!("{}", reason),
  }
}

/// Utility function for displaying test case result.
fn display_test_case_result(actual: &Value, expected: &Value, test_no: &usize, passed: &mut usize, failed: &mut usize, summary_only: bool, cm: ColorMode) {
  if dsntk_evaluator::evaluate_equals(actual, expected) {
    *passed += 1;
    if !summary_only {
      Text::new(cm).s("test ").s(test_no + 1).space().dots(3).space().green().s("ok").cprintln();
    }
  } else {
    *failed += 1;
    if !summary_only {
      Text::new(cm).s("test ").s(test_no + 1).space().dots(3).space().red().s("FAILED").cprintln();
      Text::new(cm).s("    expected: ").green().s(expected).cprintln();
      Text::new(cm).s("      actual: ").red().s(actual).cprintln();
    }
  }
}

/// Utility function for displaying test summary.
fn display_test_summary(passed: usize, failed: usize, summary_only: bool, cm: ColorMode) {
  if !summary_only {
    println!();
  }
  let mut text = Text::new(cm).s("test result: ");
  if failed > 0 {
    text = text.red().s("FAILED");
  } else {
    text = text.green().s("ok");
  }
  text.clear().dot().space().s(passed).s(" passed; ").s(failed).s(" failed.").nl().println();
}
