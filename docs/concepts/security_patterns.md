# Security Patterns

This document explains the concept of pattern-based vulnerability detection system used by Parsentry.

## Overview

Security patterns are regex-based rules that efficiently filter potential vulnerabilities before LLM analysis. They support multiple programming languages and are used in combination with data flow analysis.

## PAR (Principal-Action-Resource) Classification

### Principal
Identifies authentication entities and trusted data sources.

**Examples in programming languages**:
- **Web framework**: HTTP request handlers, form data processing
- **File system**: File reading, configuration file parsing
- **Network**: API responses, external service calls
- **Environment**: Environment variables, command line arguments, database results

**Examples in IaC**:
- **AWS IAM**: Users, roles, accounts
- **Access permissions**: Policy attachments

### Action
Identifies operations representing data processing, validation, and security controls.

**Examples in programming languages**:
- **Data validation**: Schema validation, type checking, regex validation
- **Data sanitization**: HTML escaping, path normalization
- **Security controls**: Hashing, encryption, token validation

**Examples in IaC**:
- **AWS API operations**: s3:GetObject, ec2:DescribeInstances
- **Permission changes**: IAM policy updates

### Resource
Identifies final data destinations or dangerous operation targets.

**Examples in programming languages**:
- **Code execution**: `eval()`, `exec()`, dynamic code execution
- **Command execution**: Shell execution, process spawning
- **Database**: SQL query execution, NoSQL operations
- **File system**: File writing, path traversal
- **Network**: External HTTP requests, URL construction

**Examples in IaC**:
- **AWS services**: S3 buckets, EC2 instances, Lambda functions
- **Network resources**: VPCs, subnets, security groups

## Integration with Data Flow Analysis

Based on pattern matching results, appropriate context extraction is performed:

- **Principal match**: Forward tracking of data flow using `find_references()`
- **Action match**: Bidirectional tracking of data processing using `find_bidirectional()`
- **Resource match**: Backward tracking of data origins using `find_definition()`
- **Attack vectors**: Threat classification using MITRE ATT&CK framework tactic IDs

## Risk Scoring

Each pattern is assigned a risk score from 1-10:

- **1-3**: Low risk (informational)
- **4-6**: Medium risk (context-dependent)
- **7-9**: High risk (likely vulnerable)
- **10**: Critical (almost certainly vulnerable)

## Role in Analysis Pipeline

1. **File filtering**: Skip files below threshold
2. **Prioritization**: Prioritize high-risk files for LLM analysis
3. **Context enhancement**: Provide matched areas to LLM
4. **Vulnerability type hints**: Classification based on pattern categories

## Performance Optimization

- **Compile-time optimization**: Compile patterns at startup
- **Parallel processing**: Concurrent analysis of multiple files
- **Early termination**: Stop processing on high-confidence matches
- **Caching**: Reuse compiled patterns

## Configuration and Customization

Patterns are managed by language in the `src/patterns/` directory with the following structure:

```yaml
# 例: src/patterns/python.yml
principals:
  - pattern: "\\brequests\\."
    description: "HTTP requests library"
    attack_vector:
      - "T1071"  # Application Layer Protocol
      - "T1090"  # Proxy

actions:
  - pattern: "\\bhtml\\.escape\\s*\\("
    description: "HTML escaping action"
    attack_vector:
      - "T1055"  # Process Injection
      - "T1106"  # Native API

resources:
  - pattern: "\\bopen\\s*\\("
    description: "File operations resource"
    attack_vector:
      - "T1083"  # File and Directory Discovery
      - "T1005"  # Data from Local System
```

サポートされる言語ファイル:
- `python.yml`, `javascript.yml`, `typescript.yml`
- `rust.yml`, `java.yml`, `go.yml`, `ruby.yml`
- `c.yml`, `cpp.yml`
- `terraform.yml`, `kubernetes.yml`

## Future Extensions

- **Semantic analysis**: AST-based pattern matching
- **Machine learning integration**: Learning project-specific patterns
- **Interactive tuning**: Improvement through user feedback
