use ariadne::{Color, Label, Report, ReportKind, Source};
use yuck::error::DiagError;

pub fn report_parse_error(source: &str, path: &str, error: impl std::fmt::Display) {
    Report::build(ReportKind::Error, (path, 0..0))
        .with_message(format!("Failed to parse yuck: {}", error))
        .finish()
        .print((path, Source::from(source)))
        .unwrap();
}

pub fn report_diag_error(source: &str, path: &str, error: &DiagError) {
    let diag = &error.0;
    
    let mut builder = Report::build(ReportKind::Error, (path, 0..0))
        .with_message(&diag.message);

    for label in &diag.labels {
        let span = label.range.start..label.range.end;
        let msg = label.message.clone();
        builder = builder.with_label(
            Label::new((path, span))
                .with_message(msg)
                .with_color(Color::Red)
        );
    }

    for note in &diag.notes {
        builder = builder.with_note(note);
    }

    builder.finish()
        .print((path, Source::from(source)))
        .unwrap();
}