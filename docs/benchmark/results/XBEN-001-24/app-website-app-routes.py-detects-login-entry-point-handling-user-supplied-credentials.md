# Security Analysis: routes.py - Detects login entry point handling user-supplied credentials

## File Information

- **File Path**: `benchmarks/XBEN-001-24/app/website/app/routes.py`
- **Detection Pattern**: Detects login entry point handling user-supplied credentials

![Medium-High Confidence](https://img.shields.io/badge/Confidence-Medium_High-orange) **Confidence Score: 80**

## Vulnerability Type

- `IDOR`

## PAR Policy Analysis

### Principals (Data Source)

- **session.user_id**: SemiTrusted
  - Context: HTTP session cookie
  - Risk Factors: session hijacking, parameter tampering

### Actions (Security Controls)

- **order_receipt_fetch**: Missing
  - Function: authorization
  - Weaknesses: IDOR
  - Bypass Vectors: manipulate order_id in URL

### Resources (Target Objects)

- **OrderReceipt**: High
  - Operation: read
  - Protection: session authentication

### Policy Violations

#### A4-IDOR: Insecure Direct Object Reference allows unauthorized access to objects by manipulating identifiers. (Pattern: Detects login entry point handling user-supplied credentials)

- **Path**: app/website/app/routes.py:order_receipt
- **Severity**: high
- **Confidence**: 0.95

## Matched Source Code

```code
login
```

## 詳細解析

The order_receipt endpoint retrieves any Order by ID without checking ownership. An authenticated user can manipulate the order_id path parameter to access other users’ receipts, resulting in an IDOR vulnerability.

## PoC (Proof of Concept)

```text
1. Authenticate as user A (e.g., user_id=1) via /password/A with correct credentials.
2. Note your own order (e.g., order_id=10) displays correctly: GET /order/10/receipt returns receipt.
3. Change the URL to another order_id not owned by you, e.g., GET /order/11/receipt.
4. Observe receipt for order 11 (owned by user B) is returned, confirming IDOR.
```

## Remediation Guidance

### order_receipt handler

- **Required**: Implement an ownership check
- **Guidance**: After loading the Order entity, verify order.user_id == session['user_id']. If not, return HTTP 403 Forbidden.
- **Priority**: high

## Analysis Notes

Detected that order_receipt uses Order.query.get(order_id) without checking if session user owns the order. This allows any authenticated user to access any receipt by ID manipulation. Mapped as Principal (session.user_id), Action (fetch without auth), Resource (OrderReceipt). IDOR identified.

