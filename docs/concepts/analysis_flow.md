# LLM Analysis Flow

This document describes how Parsentry utilizes large language models to perform security analysis of source code.

## Overview

The analysis process combines static code analysis with vulnerability detection using LLMs. The system first identifies potentially vulnerable files using security pattern matching, then performs in-depth analysis with LLMs to confirm and characterize vulnerabilities.

## Analysis Pipeline

### 1. File Discovery and Filtering

- Repository scans source files
- Files are filtered based on:
  - Language support (Rust, Python, JavaScript/TypeScript, Ruby, Go, Java, C/C++, Terraform)
  - Language-specific security risk patterns defined in `src/patterns/` directory
  - File size and complexity thresholds

### 2. Pattern-Based Risk Assessment

- Each file is evaluated against language-specific security patterns (PAR classification)
- Risk scores are calculated based on:
  - Principal (data source) patterns: Input sources, request handlers, environment variables, etc.
  - Action (operation) patterns: Data validation, sanitization, hashing, etc.
  - Resource patterns: File operations, database access, command execution, etc.
- Association with MITRE ATT&CK framework attack vectors

### 3. Code Context Construction

- Tree-sitter analyzes source code to extract:
  - Function/method definitions
  - Variable references and data flow
  - Import statements and dependencies
  - Comments and documentation
- Semantic information is used to build context around potential vulnerabilities

#### Improved Context Tracking

The PAR (Principal-Action-Resource) classification system has optimized context collection:

- **Principal patterns**: Use `find_references()` to track data flow forward
- **Action patterns**: Use `find_bidirectional()` to track data processing in both directions
- **Resource patterns**: Use `find_definition()` to track definitions backward
- **Attack vectors**: Threat classification based on MITRE ATT&CK tactics

This enables more accurate data flow analysis and better understanding of vulnerability contexts.

### 4. LLM Analysis

#### Initial Analysis

1. **Prompt Construction**:
   - System prompt containing security analysis guidelines
   - Complete source code of target file
   - Project context (if README summary exists)
   - Specific instructions for JSON format output

2. **LLM Request**:
   - API client sends request to selected model (OpenAI, Anthropic, etc.)
   - Model analyzes code for vulnerability patterns
   - Response includes vulnerability assessment

3. **Response Analysis**:
   - JSON response is validated against schema
   - Extracted fields include:
     - Identified vulnerability types
     - Detailed analysis
     - Proof-of-concept code
     - Confidence score
     - Remediation suggestions

#### Deep Vulnerability Analysis (Optional)

Additional analysis performed for high-risk vulnerabilities:

1. **Chain-of-Thought Analysis**:
   - LLM performs step-by-step reasoning about vulnerability root causes
   - Requests detailed explanation of attack vectors
   - Evaluates impact scope

2. **Remediation Guideline Generation**:
   - Specific code modification suggestions
   - Secure alternative solutions
   - Test case proposals

### 5. Results Aggregation

1. **Vulnerability Report Generation**:
   - Outputs vulnerability details in SARIF format
   - Includes:
     - File path and line numbers
     - Vulnerability type
     - Confidence score
     - Remediation guidance
     - Attack scenarios

2. **Project-Wide Risk Assessment**:
   - Visualizes vulnerability distribution and density
   - Calculates technical debt score
   - Generates prioritized remediation roadmap

## Key Components

1. **Parser Engine**:
   - Manages language-specific parsers
   - Generates AST and data flow graphs
   - Extracts code structure and dependencies

2. **Pattern Library**:
   - Language-specific security patterns
   - Risk assessment rules based on PAR classification
   - MITRE ATT&CK mapping

## Configuration

### Model Selection

Supported models:
- OpenAI: gpt-4, gpt-4-turbo, gpt-3.5-turbo
- Anthropic: claude-3-opus, claude-3-sonnet, claude-3-haiku
- Google: gemini-pro
- Groq: Fast inference models like llama
- Local models (compatible with Ollama etc.)

### Analysis Parameters

- **Max files**: Limit on number of files to analyze
- **Timeout**: API request timeout settings
- **Confidence threshold**: Minimum score for reporting
- **Pattern sensitivity**: Adjustment for pattern matching strictness

## Performance Considerations

1. **Parallel Processing**:
   - File analysis runs in parallel using worker pool
   - LLM requests are parallelized with rate limiting

2. **Caching**:
   - Caches analysis results to reduce re-analysis
   - Performs incremental analysis based on change detection

3. **Model Selection**:
   - Balance between accuracy and cost/speed

## Security Considerations

1. **Data Protection**:
   - Encrypts sensitive code during transmission
   - Securely manages API keys
   - Implements proper access control for analysis results

2. **Model Output Validation**:
   - Validates all LLM outputs
   - Filters malicious code suggestions
   - Monitors false positives/negatives
