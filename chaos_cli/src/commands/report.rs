use anyhow::Result;
use colored::Colorize;
use std::path::PathBuf;

pub async fn execute(
    metrics_file: PathBuf,
    format: String,
    output: Option<PathBuf>,
    compare: Vec<PathBuf>,
) -> Result<()> {
    println!("{}", "=== Generate Report ===".bold().cyan());
    println!("Metrics file: {}", metrics_file.display());
    println!("Format: {}", format);

    // Load metrics
    let contents = tokio::fs::read_to_string(&metrics_file).await?;
    let result: chaos_scenarios::runner::ScenarioResult = serde_json::from_str(&contents)?;

    match format.as_str() {
        "cli" => {
            print_cli_report(&result);
        }
        "json" => {
            let json = serde_json::to_string_pretty(&result)?;
            if let Some(output_path) = output {
                tokio::fs::write(output_path, json).await?;
            } else {
                println!("{}", json);
            }
        }
        "markdown" => {
            let md = generate_markdown_report(&result);
            if let Some(output_path) = output {
                tokio::fs::write(output_path, md).await?;
            } else {
                println!("{}", md);
            }
        }
        "html" => {
            println!("{}", "HTML report generation not yet implemented".yellow());
        }
        _ => {
            anyhow::bail!("Unknown format: {}", format);
        }
    }

    if !compare.is_empty() {
        println!("\n{}", "Comparison mode not yet implemented".yellow());
    }

    Ok(())
}

fn print_cli_report(result: &chaos_scenarios::runner::ScenarioResult) {
    println!("\n{}", "=== Scenario Report ===".bold().green());
    println!("Scenario: {}", result.scenario_name.cyan());
    println!("Total Duration: {:?}", result.total_duration);
    println!("Total Injections: {}", result.total_injections);
    println!("Success Rate: {:.2}%", result.success_rate() * 100.0);

    println!("\n{}", "Phase Results:".bold());
    for phase in &result.phase_results {
        println!("  {} - Duration: {:?}, Injections: {}",
            phase.name.yellow(),
            phase.duration,
            phase.injection_count
        );
    }
}

fn generate_markdown_report(result: &chaos_scenarios::runner::ScenarioResult) -> String {
    format!(
        r#"# Chaos Test Report: {}

## Summary

- **Total Duration**: {:?}
- **Total Injections**: {}
- **Success Rate**: {:.2}%

## Phase Results

{}

## Conclusion

Test completed successfully.
"#,
        result.scenario_name,
        result.total_duration,
        result.total_injections,
        result.success_rate() * 100.0,
        result
            .phase_results
            .iter()
            .map(|p| format!("- **{}**: {:?} ({} injections)", p.name, p.duration, p.injection_count))
            .collect::<Vec<_>>()
            .join("\n")
    )
}
