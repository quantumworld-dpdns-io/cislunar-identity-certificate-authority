*** Settings ***
Library           RequestsLibrary
Library           Collections
Library           OperatingSystem
Library           String

*** Variables ***
${CA_API}         %{CA_API=http://localhost:8080}
${API_V1}         ${CA_API}/api/v1
${ADMIN_TOKEN}    test-admin-token
${USER_TOKEN}     test-user-token

*** Keywords ***
Create API Session
    [Arguments]    ${alias}=ca    ${url}=${CA_API}
    Create Session    ${alias}    ${url}    verify=False

Authenticate As Admin
    Create API Session
    ${headers}=    Create Dictionary    Authorization=Bearer ${ADMIN_TOKEN}
    Set Suite Variable    ${ADMIN_HEADERS}    ${headers}

Authenticate As User
    Create API Session
    ${headers}=    Create Dictionary    Authorization=Bearer ${USER_TOKEN}
    Set Suite Variable    ${USER_HEADERS}    ${headers}

Issue Test Certificate
    [Arguments]    ${subject}=test.cislunar.local
    ${data}=    Create Dictionary
    ...    subject=${subject}
    ...    key_type=ecdsa-p256
    ...    validity_days=365
    ...    profile=default
    ${resp}=    POST    ${API_V1}/certificates/issue
    ...    json=${data}    headers=${ADMIN_HEADERS}
    RETURN    ${resp}

Health Check Should Pass
    ${resp}=    GET    ${CA_API}/health
    Should Be Equal As Strings    ${resp.status_code}    200
    Should Be Equal    ${resp.json()}[status]    ok
