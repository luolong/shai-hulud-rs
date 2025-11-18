/// <summary>
/// Console reporter for Shai Hulud detector.
/// </summary>

/*
macro_rules! print_status {
    ("$RED" "🚨" HIGH RISK: $message:expr) => {
        println!(
            "🚨 {} {}",
            style("HIGH RISK:").bold().red(),
            style(format!($message)).red()
        );
    };
    ("$RED" $message:expr) => {
        println!("{}", style(format!($message)).red());
    };
    ("$BLUE", $message:literal) => {
        println!("{}", style(format!($message)).blue());
    };
    ("$GREEN", $message:literal) => {
        println!("{}", style(format!($message)).green());
    };
    ("$YELLOW", $message:literal) => {
        println!("{}", style(format!($message)).yellow());
    };
}
 */

/*
fn show_file_preview(finding: &Finding) {
    if finding.severity() == &FindingSeverity::HighRisk {
        let file_path = finding.path().display();
        let context = format!("🚨 HIGH RISK: {message}", message = finding.message());
        println!("   {}", style(format!("┌─ File: {file_path}")).blue());
        println!("   {}", style(format!("│  Context: {context}")).blue());
        println!("   {}", style(format!("└─")).blue());
        println!();
    }
}
*/
