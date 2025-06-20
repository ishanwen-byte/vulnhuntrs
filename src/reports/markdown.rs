use crate::response::Response;

pub fn to_markdown(response: &Response) -> String {
    let mut md = String::new();
    
    // Enhanced title with file and pattern information
    let title = if let (Some(file_path), Some(pattern)) = (&response.file_path, &response.pattern_description) {
        format!("# Security Analysis: {} - {}", 
            file_path.split('/').last().unwrap_or(file_path), 
            pattern)
    } else if let Some(file_path) = &response.file_path {
        format!("# Security Analysis: {}", 
            file_path.split('/').last().unwrap_or(file_path))
    } else {
        "# Security Analysis Report".to_string()
    };
    md.push_str(&title);
    md.push_str("\n\n");

    // File information section
    if let Some(file_path) = &response.file_path {
        md.push_str("## File Information\n\n");
        md.push_str(&format!("- **File Path**: `{}`\n", file_path));
        if let Some(pattern) = &response.pattern_description {
            md.push_str(&format!("- **Detected Pattern**: {}\n", pattern));
        }
        md.push_str("\n");
    }

    let confidence_badge = match response.confidence_score {
        90..=100 => "![High Confidence](https://img.shields.io/badge/Confidence-High-red)",
        70..=89 => "![Medium-High Confidence](https://img.shields.io/badge/Confidence-Medium_High-orange)",
        50..=69 => "![Medium Confidence](https://img.shields.io/badge/Confidence-Medium-yellow)",
        30..=49 => "![Medium-Low Confidence](https://img.shields.io/badge/Confidence-Medium_Low-green)",
        _ => "![Low Confidence](https://img.shields.io/badge/Confidence-Low-blue)",
    };
    md.push_str(&format!(
        "{} **Confidence Score: {}**\n\n",
        confidence_badge, response.confidence_score
    ));

    if !response.vulnerability_types.is_empty() {
        md.push_str("## Vulnerability Types\n\n");
        for vuln_type in &response.vulnerability_types {
            md.push_str(&format!("- `{:?}`\n", vuln_type));
        }
        md.push('\n');
    }

    md.push_str("## PAR Policy Analysis\n\n");

    if !response.par_analysis.principals.is_empty() {
        md.push_str("### Principals (データ源)\n\n");
        for principal in &response.par_analysis.principals {
            md.push_str(&format!(
                "- **{}**: {:?}\n",
                principal.identifier, principal.trust_level
            ));
            md.push_str(&format!("  - Context: {}\n", principal.source_context));
            md.push_str(&format!(
                "  - Risk Factors: {}\n",
                principal.risk_factors.join(", ")
            ));
        }
        md.push('\n');
    }

    if !response.par_analysis.actions.is_empty() {
        md.push_str("### Actions \n");
        for action in &response.par_analysis.actions {
            md.push_str(&format!(
                "- **{}**: {:?}\n",
                action.identifier, action.implementation_quality
            ));
            md.push_str(&format!("  - Function: {}\n", action.security_function));
            md.push_str(&format!(
                "  - Weaknesses: {}\n",
                action.detected_weaknesses.join(", ")
            ));
            md.push_str(&format!(
                "  - Bypass Vectors: {}\n",
                action.bypass_vectors.join(", ")
            ));
        }
        md.push('\n');
    }

    if !response.par_analysis.resources.is_empty() {
        md.push_str("### Resources (操作対象)\n\n");
        for resource in &response.par_analysis.resources {
            md.push_str(&format!(
                "- **{}**: {:?}\n",
                resource.identifier, resource.sensitivity_level
            ));
            md.push_str(&format!("  - Operation: {}\n", resource.operation_type));
            md.push_str(&format!(
                "  - Protection: {}\n",
                resource.protection_mechanisms.join(", ")
            ));
        }
        md.push('\n');
    }

    if !response.par_analysis.policy_violations.is_empty() {
        md.push_str("### Policy Violations\n\n");
        for violation in &response.par_analysis.policy_violations {
            md.push_str(&format!(
                "#### {}: {}\n\n",
                violation.rule_id, violation.rule_description
            ));
            md.push_str(&format!("- **Path**: {}\n", violation.violation_path));
            md.push_str(&format!("- **Severity**: {}\n", violation.severity));
            md.push_str(&format!(
                "- **Confidence**: {:.2}\n\n",
                violation.confidence
            ));
        }
    }

    // Source code sections
    if let Some(matched_code) = &response.matched_source_code {
        if !matched_code.trim().is_empty() {
            md.push_str("## Matched Source Code\n\n");
            md.push_str("```code\n");
            md.push_str(matched_code);
            md.push_str("\n```\n\n");
        }
    }

    md.push_str("## Detailed Analysis\n\n");
    md.push_str(&response.analysis);
    md.push_str("\n\n");

    if !response.poc.is_empty() {
        md.push_str("## Proof of Concept\n\n");
        md.push_str("```text\n");
        md.push_str(&response.poc);
        md.push_str("\n```\n\n");
    }

    if !response.remediation_guidance.policy_enforcement.is_empty() {
        md.push_str("## Remediation Guidance\n\n");
        for remediation in &response.remediation_guidance.policy_enforcement {
            md.push_str(&format!("### {}\n\n", remediation.component));
            md.push_str(&format!(
                "- **Required**: {}\n",
                remediation.required_improvement
            ));
            md.push_str(&format!(
                "- **Guidance**: {}\n",
                remediation.specific_guidance
            ));
            md.push_str(&format!("- **Priority**: {}\n\n", remediation.priority));
        }
    }

    if !response.scratchpad.is_empty() {
        md.push_str("## Analysis Notes\n\n");
        md.push_str(&response.scratchpad);
        md.push_str("\n\n");
    }

    md
}