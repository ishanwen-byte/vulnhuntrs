# LLM Prompt Documentation

This document describes the prompt engineering approach used by Parsentry to guide LLMs in performing accurate security analysis.

## Prompt Architecture

Parsentry uses a multi-layered prompt system designed to:
1. Establish security analysis context
2. Guide systematic vulnerability discovery
3. Ensure consistent and actionable output
4. Enable vulnerability-specific deep dives

## Core Prompts

### 1. System Prompt

**Purpose**: Establish the LLM's role as a security researcher and define output requirements

**Key Components**:
- Role definition: "You are a professional security researcher"
- Focus areas:
  - Input validation and sanitization
  - Authentication and authorization
  - Data processing and storage
  - Network operations
  - Command execution
  - File operations
- Strict JSON output schema enforcement
- Focus only on exploitable vulnerabilities

**Output Schema**:
```json
{
  "scratchpad": "Analysis thought process",
  "analysis": "Detailed vulnerability explanation", 
  "poc": "Proof of concept code",
  "confidence_score": 0-10,
  "vulnerability_types": ["LFI", "RCE", "SSRF", "AFO", "SQLI", "XSS", "IDOR"],
  "context_code": [
    {
      "name": "function/method name",
      "reason": "why this code is vulnerable",
      "code_line": "actual vulnerable code",
      "path": "file path"
    }
  ]
}
```

### 2. System Prompt (Implemented)

**Actual System Prompt**:
```
You are a security vulnerability analyzer. You must reply with exactly one JSON object that matches this schema: { "scratchpad": string, "analysis": string, "poc": string, "confidence_score": integer, "vulnerability_types": array of strings, "context_code": array of objects with { "name": string, "reason": string, "code_line": string } }. Do not include any explanatory text outside the JSON object.
```

**Guidelines**:
- Analyze code vulnerabilities as a security researcher
- Focus on input validation and sanitization
- Verify authentication and authorization
- Check for data handling and leaks
- Identify command injection possibilities
- Detect path traversal vulnerabilities

### 3. Initial Analysis Prompt

**Purpose**: Perform broad security sweep of provided code

**Analysis Areas**:
- **Input processing**: User input, form data, API parameters
- **Authentication**: Session management, token validation
- **File operations**: Path traversal, file inclusion risks
- **Database queries**: SQL injection vulnerabilities
- **Command execution**: OS command injection
- **Network operations**: SSRF, request forgery

**Context Integration**:
- Include project summary
- Full source code of target files
- File paths and project structure hints

### 4. Vulnerability-Specific Prompts (Implemented)

Each vulnerability type has dedicated bypass techniques defined:

#### Local File Inclusion (LFI)
- Path traversal sequences(../../)
- URL encoding
- Null byte injection

#### Remote Code Execution (RCE)
- Shell metacharacters for command injection
- Python execution vectors
- Deserialization attacks

#### SQL Injection (SQLI)
- UNION-based injection
- Boolean-based blind injection
- Time-based blind injection

#### Cross-Site Scripting (XSS)
- HTML entity encoding bypass
- JavaScript template injection
- DOM-based XSS vectors

#### Server-Side Request Forgery (SSRF)
- DNS rebinding
- IP address encoding tricks
- Redirect chain

#### Arbitrary File Operations (AFO)
- Directory traversal sequences
- Following symbolic links
- Race conditions

#### Insecure Direct Object Reference (IDOR)
- Parameter tampering
- Horizontal privilege escalation
- Predictable resource paths

### 5. Analysis Guidelines

**Methodology Instructions**:
1. **Data flow tracing**: Track user input from entry to processing
2. **Trust boundary analysis**: Identify where validation occurs
3. **Impact assessment**: Determine exploitability and severity
4. **Bypass consideration**: Think like an attacker
5. **Confidence rating**: Based on code clarity and exploitability

**Quality Standards**:
- Report only exploitable vulnerabilities
- Provide specific code references
- Include working proof of concept
- Clearly explain attack scenarios
- Suggest practical remediation

### 6. Evaluation Prompt

**Purpose**: Rank multiple vulnerability candidates and determine priority

**Evaluation Criteria**:
1. **Exploitability**: Attack difficulty
2. **Impact**: Potential system damage
3. **Confidence**: Analysis certainty
4. **Prevalence**: Common occurrence rate
5. **Detectability**: Static analysis tool detection likelihood

**Output Format**:
```json
{
  "vulnerability": "XSS",
  "exploitability": 8,
  "impact": 7,
  "confidence": 9,
  "prevalence": 9,
  "detectability": 6,
  "priority": "High"
}
```

## Prompt Engineering Best Practices

1. **Clear Role Definition**:
   - Specify exact expertise
   - Define expected behaviors

2. **Structured Output**:
   - Enforce JSON schema
   - Prevent deviations

3. **Progressive Complexity**:
   - Start with general scan
   - Focus on specific vulnerabilities

4. **Context Integration**:
   - Include relevant code
   - Provide project structure

5. **Quality Control**:
   - Minimize false positives
   - Focus on exploitability

6. **Iterative Improvement**:
   - Learn from false positives
   - Incorporate new attack patterns

## Advanced Techniques

1. **Multi-Layer Analysis**:
   - Analyze at different abstraction levels
   - Combine code flow and semantics

2. **Context Expansion**:
   - Include relevant dependencies
   - Consider project-wide security context

3. **Meta-Prompting**:
   - Prompts that optimize other prompts
   - Based on analysis quality metrics

4. **Ensemble Prompting**:
   - Combine multiple expert roles
   - Analyze vulnerabilities from different perspectives

5. **Adaptive Prompting**:
   - Adjust prompts based on code context
   - Include language/framework-specific hints

### Chain-of-Thought Prompting
"scratchpad" field facilitates step-by-step reasoning:
```
1. Identify input sources
2. Trace data flow
3. Discover validation gaps
4. Construct exploit
5. Verify exploitability
```

### Few-Shot Examples
Provide vulnerability examples in prompts:
```
# Vulnerable:
query = f"SELECT * FROM users WHERE id = {user_input}"

# Exploit:
user_input = "1 OR 1=1--"
```

### Negative Examples
Clarify what not to report:
```
- Theoretical vulnerabilities without PoC
- Issues requiring privileged access
- Best practice violations without security impact
```

## Implementation Examples

### Python Web Application Analysis
```python
# System Prompt
expert_role = "Python Web Security Specialist"
focus_areas = ["SQL Injection", "XSS", "Authentication Bypass"]

# Initial Analysis Prompt
analyze_code("""
@app.route('/search')
def search():
    query = request.args.get('q')
    cursor.execute(f"SELECT * FROM products WHERE name LIKE '%{query}%'")
    return render_template('results.html', results=cursor.fetchall())
""")
```

### Node.js Authentication Analysis
```javascript
// Vulnerability-Specific Prompt
detect_vulnerability({
  code: """
  app.post('/login', (req, res) => {
    const user = users.find(u => u.email === req.body.email);
    if(user && user.password === req.body.password) {
      req.session.userId = user.id;
    }
  });
  """,
  vulnerability_types: ["Authentication", "Session Management"]
});
```

## Conclusion

Parsentry's prompt design features:
- **Hierarchical**: System → Initial → Specific
- **Context-aware**: Integrates code and project structure
- **Structured**: Strict JSON output
- **Iterative**: Continuous improvement

This approach enables deeper, context-aware vulnerability detection compared to traditional static analysis tools.
