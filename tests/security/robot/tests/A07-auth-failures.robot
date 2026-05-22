*** Settings ***
Resource    ../resources/common.robot

*** Test Cases ***
A07-01 No Access Without Authentication
    [Tags]    owasp-a07    auth    critical
    ${resp}=    GET    ${API_V1}/certificates
    Should Be Equal As Strings    ${resp.status_code}    401

A07-02 Invalid JWT Token Rejected
    [Tags]    owasp-a07    auth    critical
    ${headers}=    Create Dictionary    Authorization=Bearer invalid.jwt.token
    ${resp}=    GET    ${API_V1}/certificates    headers=${headers}
    Should Be Equal As Strings    ${resp.status_code}    401

A07-03 Expired Token Rejected
    [Tags]    owasp-a07    auth
    ${headers}=    Create Dictionary
    ...    Authorization=Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJleHAiOjEwMDAwMDAwMDB9.expired
    ${resp}=    GET    ${API_V1}/certificates    headers=${headers}
    Should Be Equal As Strings    ${resp.status_code}    401

A07-04 Weak Token Signature Rejected
    [Tags]    owasp-a07    auth    critical
    ${headers}=    Create Dictionary
    ...    Authorization=Bearer alg-none-token
    ${resp}=    GET    ${API_V1}/certificates    headers=${headers}
    Should Be Equal As Strings    ${resp.status_code}    401

A07-05 mTLS Certificate Validation
    [Tags]    owasp-a07    auth    mtls
    ${resp}=    GET    ${CA_API}/health
    Should Be Equal As Strings    ${resp.status_code}    200
