*** Settings ***
Resource    ../resources/common.robot
Test Setup    Authenticate As Admin

*** Test Cases ***
A10-01 SSRF Protection On External URLs
    [Tags]    owasp-a10    ssrf    critical
    ${data}=    Create Dictionary
    ...    subject=ssrf-test.cislunar.local
    ...    key_type=ecdsa-p256
    ...    validity_days=365
    ...    profile=default
    ...    extensions=[{"oid": "1.3.6.1.4.1.311.21.1",
    ...                 "critical": false,
    ...                 "value": "http://169.254.169.254/latest/meta-data/"}]
    ${resp}=    POST    ${API_V1}/certificates/issue
    ...    json=${data}    headers=${ADMIN_HEADERS}
    Should Be Equal As Strings    ${resp.status_code}    400

A10-02 No Internal Network Scanning Via CA
    [Tags]    owasp-a10    ssrf    critical
    ${data}=    Create Dictionary
    ...    subject=internal-scan.cislunar.local
    ...    key_type=ecdsa-p256
    ...    validity_days=365
    ...    profile=default
    ...    extensions=[{"oid": "1.3.6.1.4.1.311.21.1",
    ...                 "critical": false,
    ...                 "value": "http://localhost:50051/certificates"}]
    ${resp}=    POST    ${API_V1}/certificates/issue
    ...    json=${data}    headers=${ADMIN_HEADERS}
    Should Be Equal As Strings    ${resp.status_code}    400

A10-3 DNS Rebinding Protection
    [Tags]    owasp-a10    ssrf
    ${data}=    Create Dictionary
    ...    subject=rebind.cislunar.local
    ...    key_type=ecdsa-p256
    ...    validity_days=365
    ...    profile=default
    ...    extensions=[{"oid": "1.3.6.1.4.1.311.21.1",
    ...                 "critical": false,
    ...                 "value": "http://1.1.1.1.internal"}]
    ${resp}=    POST    ${API_V1}/certificates/issue
    ...    json=${data}    headers=${ADMIN_HEADERS}
    Should Be Equal As Strings    ${resp.status_code}    400
