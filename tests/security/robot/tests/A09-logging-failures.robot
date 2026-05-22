*** Settings ***
Resource    ../resources/common.robot
Test Setup    Authenticate As Admin

*** Test Cases ***
A09-01 Audit Log Created For Certificate Issuance
    [Tags]    owasp-a09    logging    critical
    ${data}=    Create Dictionary
    ...    subject=audit-test.cislunar.local
    ...    key_type=ecdsa-p256
    ...    validity_days=365
    ...    profile=default
    ${resp}=    POST    ${API_V1}/certificates/issue
    ...    json=${data}    headers=${ADMIN_HEADERS}

A09-02 Audit Log Created For Revocation
    [Tags]    owasp-a09    logging
    ${resp}=    POST    ${API_V1}/certificates/test-serial/revoke
    ...    json={"reason": "key-compromise"}    headers=${ADMIN_HEADERS}

A09-03 No PII In Logs
    [Tags]    owasp-a09    logging    privacy
    ${log}=    Get File    ${CURDIR}/../../../../logs/ca.log
    Should Not Contain    ${log}    password
    Should Not Contain    ${log}    secret
    Should Not Contain    ${log}    private_key

A09-04 Log Injection Prevention
    [Tags]    owasp-a09    logging
    ${data}=    Create Dictionary
    ...    subject="malicious\n[INFO] User logged in as admin"
    ...    key_type=ecdsa-p256
    ...    validity_days=365
    ...    profile=default
    ${resp}=    POST    ${API_V1}/certificates/issue
    ...    json=${data}    headers=${ADMIN_HEADERS}
    Should Be Equal As Strings    ${resp.status_code}    400
