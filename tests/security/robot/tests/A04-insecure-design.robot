*** Settings ***
Resource    ../resources/common.robot
Test Setup    Authenticate As Admin

*** Test Cases ***
A04-01 Rate Limiting On Certificate Issuance
    [Tags]    owasp-a04    design    critical
    FOR    ${i}    IN RANGE    0    10
        ${data}=    Create Dictionary
        ...    subject=ratelimit-${i}.cislunar.local
        ...    key_type=ecdsa-p256
        ...    validity_days=365
        ...    profile=default
        ${resp}=    POST    ${API_V1}/certificates/issue
        ...    json=${data}    headers=${ADMIN_HEADERS}
    END
    Should Be Equal As Strings    ${resp.status_code}    429

A04-02 Mass Assignment Protection
    [Tags]    owasp-a04    design
    ${data}=    Create Dictionary
    ...    subject=test.cislunar.local
    ...    key_type=ecdsa-p256
    ...    validity_days=365
    ...    profile=default
    ...    is_admin=true
    ...    role=superuser
    ${resp}=    POST    ${API_V1}/certificates/issue
    ...    json=${data}    headers=${ADMIN_HEADERS}
    Should Be Equal As Strings    ${resp.status_code}    400

A04-03 Certificate Count Limit Per Identity
    [Tags]    owasp-a04    design
    ${data}=    Create Dictionary
    ...    name=max-certs-test
    ...    organization=test-org
    ...    asset_type=LunarRelay
    ${resp}=    POST    ${API_V1}/identities/register
    ...    json=${data}    headers=${ADMIN_HEADERS}

A04-4 Validation Of Certificate Profiles
    [Tags]    owasp-a04    design
    ${data}=    Create Dictionary
    ...    subject=nonexistent-profile.cislunar.local
    ...    key_type=ecdsa-p256
    ...    validity_days=365
    ...    profile=nonexistent-profile
    ${resp}=    POST    ${API_V1}/certificates/issue
    ...    json=${data}    headers=${ADMIN_HEADERS}
    Should Be Equal As Strings    ${resp.status_code}    404
