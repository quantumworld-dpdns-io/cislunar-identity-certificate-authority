*** Settings ***
Resource    ../resources/common.robot

*** Test Cases ***
A05-01 No Default Credentials
    [Tags]    owasp-a05    misconfig    critical
    ${resp}=    POST    ${API_V1}/auth/login
    ...    json={"username": "admin", "password": "admin"}
    Should Not Be Equal As Strings    ${resp.status_code}    200

A05-02 No Verbose Error Messages
    [Tags]    owasp-a05    misconfig
    ${resp}=    POST    ${API_V1}/certificates/issue
    ...    json={"invalid": "data"}
    Should Not Contain    ${resp.text}    stack trace
    Should Not Contain    ${resp.text}    at line
    Should Not Contain    ${resp.text}    File "

A05-03 Security Headers Present
    [Tags]    owasp-a05    misconfig    headers
    ${resp}=    GET    ${CA_API}/health
    Should Contain    ${resp.headers}    X-Content-Type-Options
    Should Contain    ${resp.headers}    X-Frame-Options
    Should Contain    ${resp.headers}    X-XSS-Protection

A05-04 CORS Not Wildcard
    [Tags]    owasp-a05    misconfig    cors
    ${resp}=    OPTIONS    ${CA_API}/health
    ...    headers={"Origin": "https://evil.com"}
    Run Keyword And Continue On Failure
    ...    Should Not Be Equal    ${resp.headers.get('Access-Control-Allow-Origin')}    *

A05-05 Directory Listing Disabled
    [Tags]    owasp-a05    misconfig
    ${resp}=    GET    ${API_V1}/../   expected_status=404
