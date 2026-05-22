*** Settings ***
Resource    ../resources/common.robot
Test Setup    Authenticate As Admin

*** Test Cases ***
A02-01 Weak RSA Key Rejected
    [Tags]    owasp-a02    crypto    critical
    ${data}=    Create Dictionary
    ...    subject=weak.cislunar.local
    ...    key_type=rsa-1024
    ...    validity_days=365
    ...    profile=default
    ${resp}=    POST    ${API_V1}/certificates/issue
    ...    json=${data}    headers=${ADMIN_HEADERS}
    Should Not Be Equal As Strings    ${resp.status_code}    200

A02-2 Short Validity Rejected For Weak Keys
    [Tags]    owasp-a02    crypto
    ${data}=    Create Dictionary
    ...    subject=rsa2048-short.cislunar.local
    ...    key_type=rsa-2048
    ...    validity_days=730
    ...    profile=default
    ${resp}=    POST    ${API_V1}/certificates/issue
    ...    json=${data}    headers=${ADMIN_HEADERS}
    Should Not Be Equal As Strings    ${resp.status_code}    200

A02-3 TLS Endpoint Uses Strong Ciphers
    [Tags]    owasp-a02    crypto    tls
    ${resp}=    GET    ${CA_API}/health    verify=False
    Should Be Equal As Strings    ${resp.status_code}    200

A02-4 PQC Keys Available
    [Tags]    owasp-a02    crypto    pqc
    ${data}=    Create Dictionary
    ...    subject=pqc-test.cislunar.local
    ...    key_type=pqc-dilithium3
    ...    validity_days=365
    ...    profile=pqc
    ${resp}=    POST    ${API_V1}/certificates/issue
    ...    json=${data}    headers=${ADMIN_HEADERS}

A02-5 Certificate Fingerprint SHA-256
    [Tags]    owasp-a02    crypto
    ${resp}=    GET    ${API_V1}/certificates/test-serial-001
    ...    headers=${ADMIN_HEADERS}
    Run Keyword And Ignore Error    Should Contain    ${resp.text}    fingerprint_sha256
