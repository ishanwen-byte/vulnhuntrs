# LLM Analysis Context Construction

This document describes the LLM context construction methodology for accurate vulnerability detection by Parsentry.

## Overview

Accurate vulnerability detection requires understanding not only individual code segments but also comprehensive context including the execution environment. Parsentry improves LLM analysis accuracy through a multi-layered context architecture.

## Context Layers

### 1. Project-Level Context

#### README Analysis
- Automatically extracts and summarizes project README files
- Provides understanding of:
  - Project purpose and functionality
  - Technology stack and dependencies
  - Security considerations mentioned by developers
  - API endpoints and data flow

#### Repository Structure
- Maps project directory structure
- Identifies:
  - Entry points (main files, API routes)
  - Configuration files
  - Test directories
  - Third-party dependencies

#### Dependency Analysis
- Analyzes package manifests (package.json, Cargo.toml, requirements.txt, etc.)
- Identifies known vulnerable dependencies
- Maps dependency usage patterns

### 2. File-Level Context

#### Source Code Analysis
- Provides complete file content to LLM
- Preserves:
  - Original formatting and indentation
  - Comments and documentation
  - Import statements
  - Function/class structure

#### Semantic Information
Using Tree-sitter analysis:
- **Function definitions**: Names, parameters, return types
- **Variable declarations**: Scope and usage patterns
- **Control flow**: Conditional branches, loops, error handling
- **Data flow**: How user input propagates through the code

#### File Metadata
- Relative path within project
- File type and language
- Last modified time
- File size and complexity metrics

### 3. Code Relationship Context

#### Import Analysis
- Tracks imported modules and functions
- Maps cross-file dependencies
- Identifies external library usage

#### Function Call Graph
- Tracks function calls across files
- Identifies:
  - User input entry points
  - Data transformation points
  - Security-critical functions

#### Data Flow Tracking
- Tracks variables across function boundaries
- Tracks:
  - User input sources
  - Data transformations
  - Output destinations

## Context Construction Process

### 1. Initial Scan
```
Repository → File discovery → Language detection → Pattern matching
```

### 2. Analysis Phase
```
Source files → Tree-sitter AST → Semantic extraction → Relationship mapping
```

### 3. Context Assembly
```
Project information + File content + Semantic data → Structured context
```

## Context Optimization

### Relevance Filtering
- Focuses on security-related code sections
- Prioritizes:
  - Input processing functions
  - Database queries
  - File operations
  - Network requests
  - Authentication/authorization code

### Context Window Management
- Trims large files appropriately
- Preserves critical sections:
  - Function signatures
  - Security-critical operations
  - Error handling
  - Input validation

### Cross-Reference Enhancement
- Builds links between similar code patterns
- Automatically associates:
  - Same vulnerability patterns
  - Similar functionality
  - Common data flows

## Implementation Examples

### Tree-sitter Queries

Example queries for function definitions and references:
```scheme
; Definition query (definitions.scm)
(function_definition
  name: (identifier) @name
  body: (block)) @definition

; Reference query (references.scm)
(identifier) @reference
```

Currently supported languages: C, C++, Python, JavaScript, TypeScript, Java, Go, Rust, Ruby

### Context Template Structure

```rust
// Definition構造体
pub struct Definition {
    pub name: String,
    pub start_byte: usize,
    pub end_byte: usize, 
    pub source: String,
}

// Context構造体
pub struct Context {
    pub definitions: Vec<Definition>,
}
```

プロンプトで使用される実際のコンテキスト形式：
```
Context Definitions:

Function/Definition: function_name
Code:
function_body_source_code
```

## Best Practices

### 1. Comprehensive Coverage
- Include all relevant context layers
- Don't assume LLM knowledge about project-specific matters
- Explicitly provide security-related information

### 2. Noise Reduction
- Filter out irrelevant code (tests, documentation)
- Focus on executable code paths
- Prioritize user-facing functionality

### 3. Relationship Clarity
- Explicitly describe code relationships
- Highlight data flow paths
- Mark trust boundaries

## Future Enhancements

1. **Dynamic Analysis Integration**
   - Runtime behavior patterns
   - Actual data flow traces
   - Performance characteristics

2. **Historical Context**
   - Git history analysis
   - Previous vulnerability fixes
   - Code evolution patterns

3. **External Context**
   - CVE database integration
   - Security advisory correlations
   - Framework-specific vulnerabilities
