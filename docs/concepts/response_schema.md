# LLM Response Schema

This document defines the structured output format that LLMs must follow when reporting security vulnerabilities in Parsentry.

## Overview

Parsentry enforces a strict JSON schema for LLM responses to ensure:
- Consistent vulnerability reporting
- Automated analysis and processing
- Actionable security findings
- Integration with CI/CD pipelines

## Schema Definition

### Complete JSON Structure

```json
{
  "scratchpad": "string - Analysis thought process and reasoning",
  "analysis": "string - Detailed vulnerability explanation",
  "poc": "string - Proof-of-concept exploit code",
  "confidence_score": "integer - Confidence level (0-10)",
  "vulnerability_types": ["Array of vulnerability type strings"],
  "context_code": [
    {
      "name": "string - Function/class/method name",
      "reason": "string - Why this code is vulnerable",
      "code_line": "string - Actual vulnerable code"
    }
  ]
}
```

### Field Specifications

#### 1. Scratchpad (Required)
- **Type**: String
- **Purpose**: Capture LLM's analysis process
- **Content**: Step-by-step reasoning, data flow analysis, security considerations
- **Example**:
```json
"scratchpad": "1. Identified user input from request parameter 'id'\n2. Traced flow into SQL query construction\n3. Found no parameterization\n4. Confirmed direct string concatenation\n5. Verified SQL injection vulnerability"
```

#### 2. Analysis (Required)
- **Type**: String
- **Purpose**: Comprehensive vulnerability explanation
- **Content**: 
  - Root cause analysis
  - Attack vectors
  - Potential impact
  - Business risk assessment
- **Example**:
```json
"analysis": "SQL injection vulnerability in get_user function. User input from 'id' parameter is directly concatenated into SQL query without sanitization. Attackers can inject malicious SQL to extract sensitive data, modify records, or execute database commands. Impact: Full database compromise possible."
```

#### 3. Proof of Concept (Required)
- **Type**: String
- **Purpose**: Demonstrate exploitability
- **Content**: Working exploit code or clear exploitation steps
- **Format**: Code snippets, curl commands, or step-by-step instructions
- **Example**:
```json
"poc": "curl 'http://example.com/user?id=1 OR 1=1--' \n# Returns all users\ncurl 'http://example.com/user?id=1 UNION SELECT password FROM admins--'\n# Extracts admin passwords"
```

#### 4. Confidence Score (Required)
- **Type**: Integer (0-10)
- **Purpose**: Indicate certainty of analysis
- **Scoring Guide**:
  - 0-3: Low confidence, requires verification
  - 4-6: Medium confidence, likely vulnerable
  - 7-9: High confidence, vulnerability confirmed
  - 10: Certain, with working PoC
- **Example**: `8`

#### 5. Vulnerability Types (Required)
- **Type**: Array of strings
- **Purpose**: Classify findings
- **Allowed Values**:
  - `"LFI"` - Local File Inclusion
  - `"RCE"` - Remote Code Execution
  - `"SSRF"` - Server-Side Request Forgery
  - `"AFO"` - Arbitrary File Operations
  - `"SQLI"` - SQL Injection
  - `"XSS"` - Cross-Site Scripting
  - `"IDOR"` - Insecure Direct Object Reference
- **Example**: `["SQLI", "IDOR"]`

#### 6. Context Code (Required)
- **Type**: Array of objects
- **Purpose**: Link findings to specific code
- **Object Structure**:
  - `name`: Identifier of vulnerable component
  - `reason`: Explanation of vulnerability relevance
  - `code_line`: Actual vulnerable code snippet
  - `path`: File path (added in current implementation)
- **Example**:
```json
"context_code": [
  {
    "name": "get_user",
    "reason": "Constructs SQL query with unsanitized input",
    "code_line": "query = \"SELECT * FROM users WHERE id = \" + request.params.id",
    "path": "src/database.py"
  },
  {
    "name": "execute_query", 
    "reason": "Executes vulnerable query",
    "code_line": "results = db.execute(query)",
    "path": "src/database.py"
  }
]
```

## Complete Response Example

```json
{
  "scratchpad": "1. Analyzed /profile endpoint\n2. 'filename' parameter is unvalidated\n3. Confirmed file path manipulation\n4. Directory traversal possible",
  "analysis": "Local File Inclusion vulnerability. Profile image upload feature doesn't properly validate 'filename' parameter, allowing relative paths (e.g., '../../../etc/passwd'). Attackers can read system files.",
  "poc": "curl -X POST 'http://example.com/profile' -F 'filename=../../../etc/passwd' -F 'image=@test.jpg'",
  "confidence_score": 9,
  "vulnerability_types": ["LFI"],
  "context_code": [
    {
      "name": "save_profile_image",
      "reason": "Performs file operations with unvalidated filename",
      "code_line": "file.save(os.path.join(UPLOAD_FOLDER, filename))",
      "path": "src/profile.py"
    }
  ]
}
```

## Validation Rules

1. **Required Fields Check**: All mandatory fields must exist with correct types
2. **Confidence Score Validation**: Must be within 0-10 range
3. **Vulnerability Types Validation**: Only allowed values are included
4. **Context Code Integrity**: Each code reference must contain name/reason/code_line
5. **PoC Validation**: Exploitation method must be clearly explained

## Error Handling

### Common Error Format

```json
{
  "error": true,
  "code": "Error code",
  "message": "Human-readable error message",
  "details": {
    "field": "Problematic field name",
    "requirement": "Unmet requirement"
  }
}
```

## Error Handling

### Error Codes

| Code | Description |
|-------|------|
| `SCHEMA_001` | Missing required field |
| `SCHEMA_002` | Field type mismatch |
| `SCHEMA_003` | Confidence score out of range |
| `SCHEMA_004` | Invalid vulnerability type |
| `SCHEMA_005` | Incomplete context code |
| `SCHEMA_006` | Insufficient PoC |

### Schema Validation

The schema is enforced by:
1. JSON schema specification in `response_json_schema()` function
2. genai client configuration with ChatOptions and JsonSpec
3. JSON parsing in `parse_json_response()`
4. Serde deserialization to `Response` struct
5. Type validation via Rust's type system
6. Confidence score normalization via `normalize_confidence_score()`

## Best Practices

1. **Scratchpad**: Clearly describe step-by-step reasoning
2. **Analysis**: Cover both technical details and business impact
3. **PoC**: Provide actually working exploit code
4. **Confidence**: Assign appropriate score (don't overestimate)
5. **Context Code**: Precisely identify code locations

## Integration Guide

### CI/CD Pipeline

1. Parse JSON response
2. Trigger warnings/failures based on confidence scores
3. Integrate results into security dashboard
4. Automatically generate tickets

### Developer Feedback

1. Directly link code references to IDE
2. Provide remediation guidance
3. Recommend relevant training resources

## Integration Notes

### CI/CD Pipeline
- Parse JSON response programmatically
- Fail builds on high confidence scores
- Generate reports from structured data
- Track vulnerability trends

### Security Tools
- Export to standard formats (SARIF, etc.)
- Integrate with issue trackers
- Feed into vulnerability databases
- Support automated remediation

## Future Extensions

1. **Remediation Suggestions**: Add automated fix recommendation field
2. **Impact Score**: Standardized impact assessment based on CVSS
3. **Trend Analysis**: Correlation with historical vulnerability data
4. **Team-Specific Classification**: Support organization-specific tagging
