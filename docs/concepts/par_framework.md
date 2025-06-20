# PAR Framework (Principal-Action-Resource)

## Overview

The PAR (Principal-Action-Resource) framework is an analysis model that systematically classifies vulnerabilities by organizing three key components of security-related operations:

- **Principal**: The entity performing the operation (user, service, process)
- **Action**: The operation being performed (read, write, execute, authenticate)
- **Resource**: The target of access (file, database, network endpoint, memory)

## Origins and Concept

The PAR framework builds upon existing security analysis concepts while providing a more structured approach:

### Taint Tracking Model
Traditional static analysis tools track data flow through taint tracking:
- **Sources**: Entry points for untrusted data
- **Sinks**: Points where data can cause security issues
- **Validation**: Sanitization and checks between sources and sinks

The PAR framework extends this model by:
- **Principal** encompassing traditional sources and controlling entities
- **Action** including validation/sanitization operations and actual execution operations
- **Resource** generalizing sinks to include all types of targets and system resources

### Influence from Cedar Language
Amazon Cedar's authorization language provides a policy framework based on:
- **Principal**: Entity making the request
- **Action**: Requested operation
- **Resource**: Target of the operation

Parsentry's PAR framework applies these concepts to vulnerability analysis:
- Cedar focuses on authorization decisions for permitted operations
- PAR focuses on security analysis to identify potential vulnerabilities
- Both provide systematic reasoning methods for security relationships

## Significance of the PAR Framework

Building upon these foundational concepts, the PAR framework provides a more systematic approach than traditional vulnerability scanners:

1. **Comprehensive Coverage**: Ensures analysis of all aspects of security operations
2. **Context Awareness**: Considers relationships between actors, operations, and targets
3. **Scalable Classification**: Works across different languages and technology stacks
4. **Risk Assessment**: Enables prioritization based on principal privileges and resource sensitivity

## Detailed PAR Components

### Principal
Entities that initiate or control operations:
- **User accounts** (authenticated, anonymous, privileged users)
- **Service accounts** (database users, API keys, service principals)
- **Processes** (system processes, application threads)
- **External systems** (third-party APIs, microservices)

### Action
Operations being performed:
- **Data operations** (read, write, update, delete)
- **Authentication** (login, logout, token generation)
- **Authorization** (permission checks, role assignments)
- **System operations** (file I/O, network calls, process execution)
- **Cryptographic operations** (encryption, signing, hashing)

### Resource
Target resources of operations:
- **Data stores** (databases, caches, queues)
- **File systems** (configuration files, logs, temporary files)
- **Network resources** (API endpoints, sockets, services)
- **Memory** (heap, stack, shared memory)
- **Cryptographic assets** (keys, certificates, tokens)

## Analysis Process

Vulnerability analysis using the PAR framework follows these steps:

1. **Principal Identification**: Enumerate all operational entities in the code
2. **Action Mapping**: Identify operations each principal can perform
3. **Resource Classification**: Catalog resources targeted by operations
4. **Relationship Analysis**: Evaluate security relationships between Principal-Action-Resource
5. **Risk Assessment**: Score potential risks for each relationship

## Implementation Examples

### Web Application Authentication

```python
# Principal: Unauthenticated user
user = get_unauthenticated_user()

# Action: Login attempt
try_login(user, password)

# Resource: User authentication table
db.execute("SELECT * FROM users WHERE username = ?", (user,))
```

## Analysis Framework Comparison

### Traditional Taint Tracking:
- **Source**: `userData.email` (untrusted input)
- **Sink**: `innerHTML` (DOM injection point)
- **Validation**: Present but insufficient - regex bypass possible

### Cedar-Style Policy Analysis:
- **Principal**: Web application service
- **Action**: DOM manipulation
- **Resource**: User interface
- **Policy**: "Application can update UI" (authorized but unsafe implementation)

### PAR Framework Analysis:
- **Principal**: User input via web interface (untrusted source)
- **Action**: Flawed regex email validation (vulnerable validation implementation)
  - Missing `^` and `$` anchors in regex
  - Allows valid email within malicious string
  - Creates false trust boundary
- **Resource**: DOM via `innerHTML` (XSS execution context)

**PAR Advantage**: Captures trust relationships (user input vs. application) and analyzes security action quality, providing more comprehensive analysis than traditional approaches alone.

## PAR Patterns Across Contexts

### JavaScript/Python/Ruby Libraries and Applications
**Library Level:**
- **P**: Function arguments, configuration objects, imported modules
- **A**: Data processing, validation functions, crypto operations
- **R**: Return values, file system, network endpoints

**Web Application Level:**
- **P**: User sessions, API clients, service accounts
- **A**: HTTP requests, database queries, authentication
- **R**: User data, configuration files, external APIs

**PAR Advantage**: Unlike traditional scanners that focus on user input flows, PAR can analyze security issues in library code, utility functions, and internal APIs regardless of the application context.

### Infrastructure as Code (Terraform)
- **P**: Cloud services, deployment pipelines, operators
- **A**: Resource provisioning, permission grants, network configuration
- **R**: Cloud resources, security groups, IAM policies

### System Programming (C/C++/Rust)
- **P**: Processes, threads, system users
- **A**: Memory allocation, file I/O, system calls
- **R**: Memory regions, file systems, hardware interfaces

### Framework Agnostic Analysis
Traditional vulnerability scanners typically require:
- Web application context (HTTP requests, form inputs)
- User input as vulnerability sources
- Application-specific sinks (responses, database writes)

PAR framework enables analysis of:
- **Standalone libraries** without web context
- **Internal APIs and utilities** 
- **Framework-independent code**
- **Microservices and serverless functions**
- **CLI tools and system utilities**

This versatility allows PAR-based analysis to identify vulnerabilities in reusable components that might be missed by application-focused scanners.

## Integration with LLM Analysis

The PAR framework guides LLM prompts to ensure comprehensive analysis:

1. **Structured Prompts**: Each analysis request includes PAR context
2. **Systematic Coverage**: Ensures all three dimensions are evaluated
3. **Consistent Classification**: Provides uniform vulnerability categorization
4. **Risk Scoring**: Enables PAR-based severity assessment
