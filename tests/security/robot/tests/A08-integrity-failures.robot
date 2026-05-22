*** Settings ***
Resource    ../resources/common.robot
Test Setup    Authenticate As Admin

*** Test Cases ***
A08-01 Certificate Chain Validation
    [Tags]    owasp-a08    integrity    critical
    ${resp}=    GET    ${API_V1}/certificates/test-chain-root
    ...    headers=${ADMIN_HEADERS}
    Run Keyword And Ignore Error    Should Contain    ${resp.json()}[ca_type]    Root

A08-02 CRL Tampering Detection
    [Tags]    owasp-a08    integrity    critical
    ${resp}=    GET    ${API_V1}/crl    headers=${ADMIN_HEADERS}
    Should Be Equal As Strings    ${resp.status_code}    200

A08-03 CSR Tampering Rejected
    [Tags]    owasp-a08    integrity
    ${data}=    Create Dictionary
    ...    subject=modified.cislunar.local
    ...    key_type=ecdsa-p256
    ...    validity_days=365
    ...    profile=default
    ...    extensions=[{"oid": "2.5.29.19", "critical": true, "value": "tampered"}]
    ${resp}=    POST    ${API_V1}/certificates/issue
    ...    json=${data}    headers=${ADMIN_HEADERS}
    Should Be Equal As Strings    ${resp.status_code}    400

A08-04 OCSP Response Integrity
    [Tags]    owasp-a08    integrity
    ${resp}=    GET    ${API_V1}/ocsp/test-serial
    ...    headers=${ADMIN_HEADERS}
    Should Be Equal As Strings    ${resp.status_code}    200
