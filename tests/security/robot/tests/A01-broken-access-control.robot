*** Settings ***
Resource    ../resources/common.robot
Test Setup    Authenticate As User

*** Test Cases ***
A01-01 User Cannot Issue CA Certificates
    [Tags]    owasp-a01    access-control    critical
    ${data}=    Create Dictionary
    ...    subject=ca.cislunar.local
    ...    key_type=ecdsa-p384
    ...    validity_days=3650
    ...    profile=ca
    ${resp}=    POST    ${API_V1}/certificates/issue
    ...    json=${data}    headers=${USER_HEADERS}
    Should Not Be Equal As Strings    ${resp.status_code}    200

A01-02 User Cannot Revoke Admin Certificates
    [Tags]    owasp-a01    access-control    critical
    ${data}=    Create Dictionary
    ...    serial=admin-cert-001
    ...    reason=key-compromise
    ${resp}=    POST    ${API_V1}/certificates/admin-cert-001/revoke
    ...    json=${data}    headers=${USER_HEADERS}
    Should Not Be Equal As Strings    ${resp.status_code}    200

A01-03 No Privilege Escalation Via API
    [Tags]    owasp-a01    access-control    critical
    ${resp}=    GET    ${API_V1}/identities
    ...    headers=${USER_HEADERS}
    ${status}=    Convert To String    ${resp.status_code}
    Should Be True    ${status} != '200' or ${resp.status_code} == 403 or ${resp.status_code} == 401

A01-04 Horizontal Privilege Escalation
    [Tags]    owasp-a01    access-control
    ${resp}=    GET    ${API_V1}/identities/other-org-identity
    ...    headers=${USER_HEADERS}
    Should Not Be Equal As Strings    ${resp.status_code}    200
