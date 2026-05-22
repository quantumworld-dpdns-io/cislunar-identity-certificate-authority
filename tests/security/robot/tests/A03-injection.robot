*** Settings ***
Resource    ../resources/common.robot
Test Setup    Authenticate As Admin

*** Test Cases ***
A03-01 SQL Injection In Subject Field
    [Tags]    owasp-a03    injection    critical
    ${data}=    Create Dictionary
    ...    subject="'; DROP TABLE certificates; --"
    ...    key_type=ecdsa-p256
    ...    validity_days=365
    ...    profile=default
    ${resp}=    POST    ${API_V1}/certificates/issue
    ...    json=${data}    headers=${ADMIN_HEADERS}
    Should Be Equal As Strings    ${resp.status_code}    400

A03-02 NoSQL Injection In Metadata
    [Tags]    owasp-a03    injection
    ${data}=    Create Dictionary
    ...    subject=test.cislunar.local
    ...    key_type=ecdsa-p256
    ...    validity_days=365
    ...    profile=default
    ...    extensions={"$gt": ""}
    ${resp}=    POST    ${API_V1}/certificates/issue
    ...    json=${data}    headers=${ADMIN_HEADERS}
    Should Be Equal As Strings    ${resp.status_code}    400

A03-03 Command Injection In Identity Name
    [Tags]    owasp-a03    injection    critical
    ${data}=    Create Dictionary
    ...    name="; rm -rf /;"
    ...    organization=test
    ...    asset_type=LunarRelay
    ${resp}=    POST    ${API_V1}/identities/register
    ...    json=${data}    headers=${ADMIN_HEADERS}
    Should Be Equal As Strings    ${resp.status_code}    400

A03-04 XSS In Subject Field
    [Tags]    owasp-a03    injection    xss
    ${data}=    Create Dictionary
    ...    subject="<script>alert('XSS')</script>"
    ...    key_type=ecdsa-p256
    ...    validity_days=365
    ...    profile=default
    ${resp}=    POST    ${API_V1}/certificates/issue
    ...    json=${data}    headers=${ADMIN_HEADERS}
    Should Be Equal As Strings    ${resp.status_code}    400
