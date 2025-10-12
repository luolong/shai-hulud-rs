use console::style;

use crate::probe::{Finding, FindingSeverity};

macro_rules! print_status {
    ("$RED", $message:expr) => {
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

/// Generate comprehensive security report with risk stratification and findings
///
pub(crate) fn generate_report(findings: &[Finding], paranoid: bool) -> eros::Result<()> {
    println!();
    print_status!("$BLUE", "==============================================");
    if paranoid {
        print_status!("$BLUE", "  SHAI-HULUD + PARANOID SECURITY REPORT");
    } else {
        print_status!("$BLUE", "      SHAI-HULUD DETECTION REPORT");
    }
    print_status!("$BLUE", "==============================================");
    println!();

    let mut high_risk = 0usize;
    let mut medium_risk = 0usize;

    let mut low_risk_findings: Vec<Finding> = Vec::with_capacity(findings.len());

    for finding in findings {
        match finding.severity() {
            FindingSeverity::HighRisk => high_risk += 1,
            FindingSeverity::MediumRisk => medium_risk += 1,
            FindingSeverity::LowRisk => low_risk_findings.push(finding.clone()),
        }
    }

    let total_issues = high_risk + medium_risk;
    let low_risk_count = low_risk_findings.len();

    print_status!("$BLUE", "==============================================");
    if total_issues == 0 {
        print_status!(
            "$GREEN",
            "✅ No indicators of Shai-Hulud compromise detected."
        );
        print_status!(
            "$GREEN",
            "Your system appears clean from this specific attack."
        );

        if low_risk_count > 0 {
            println!();
            print_status!("$BLUE", "ℹ️  LOW RISK FINDINGS (informational only):");
            for finding in low_risk_findings {
                println!("   - {finding}");
            }
            println!(
                "   {}",
                style("NOTE: These are likely legitimate framework code or dependencies.").blue()
            );
        }
    } else {
        print_status!("$RED", "🔍 SUMMARY:");
        print_status!("$RED", "   High Risk Issues: {high_risk}");
        print_status!("$YELLOW", "   Medium Risk Issues: {medium_risk}");
        if low_risk_count > 0 {
            print_status!("$BLUE", "   Low Risk (informational): {low_risk_count}");
        }
        print_status!("$BLUE", "   Total Critical Issues: {total_issues}");
        println!();

        print_status!("$YELLOW", "⚠️  IMPORTANT:");
        print_status!(
            "$YELLOW",
            "   - High risk issues likely indicate actual compromise"
        );
        print_status!(
            "$YELLOW",
            "   - Medium risk issues require manual investigation"
        );
        print_status!(
            "$YELLOW",
            "   - Low risk issues are likely false positives from legitimate code"
        );
        if paranoid {
            print_status!(
                "$YELLOW",
                "   - Issues marked (PARANOID) are general security checks, not Shai-Hulud specific"
            );
        }
        print_status!("$YELLOW", "   - Consider running additional security scans");
        print_status!(
            "$YELLOW",
            "   - Review your npm audit logs and package history"
        );
    }

    println!(
        "{}",
        style("==============================================").blue()
    );

    Ok(())
}
