*** Settings ***
Resource    ../resources/common.robot

*** Test Cases ***
A06-01 Known Vulnerability Database Exists
    [Tags]    owasp-a06    components    critical
    ${result}=    Run Process    cargo-deny    check    advisories
    ...    cwd=${CURDIR}/../../../../src/rust
    Should Be Equal As Integers    ${result.rc}    0

A06-02 Go Dependencies Audited
    [Tags]    owasp-a06    components
    ${result}=    Run Process    govulncheck    ./...
    ...    cwd=${CURDIR}/../../../../src/go
    Should Be Equal As Integers    ${result.rc}    0

A06-03 Python Dependencies Audited
    [Tags]    owasp-a06    components
    ${result}=    Run Process    pip-audit
    ...    cwd=${CURDIR}/../../../../src/python
    Should Be Equal As Integers    ${result.rc}    0

A06-04 Docker Image Has No Critical CVEs
    [Tags]    owasp-a06    components    docker
    ${result}=    Run Process    trivy    image    --severity    CRITICAL
    ...    cislunar-ca-engine:latest
    Should Be Equal As Integers    ${result.rc}    0
