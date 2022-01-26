# 🛡️ Security Policy

The following document succinctly defines some informal baselines and procedures
for the information security management processes relevant to PackSquash. In no
event there is any warranty, express or implied, about any particular action for
any security vulnerability being done.

## Scope

This security policy applies to PackSquash and its sister projects, namely the
PackSquash GitHub Action.

Any security report must be about a potential security vulnerability. A security
vulnerability is an implementation detail that allows some security threat to
happen. A security threat is an event that might cause loss, damage or misuse of
information assets. The information assets include the computer PackSquash runs
on, the networks it is connected to, or any data stored on it.

Security vulnerabilities are a serious matter. They are **not** mere user or
environmental errors that might have undesirable consequences, neither they are
known limitations about some features (for example, the pack protection feature
is known to be able to be circumvented due to fundamental limits on how it
works). Please think twice about reporting a security vulnerability. Keeping the
noise to the minimum helps us focus on matters worth our attention.

## Supported Versions

For practical reasons, we will only accept security reports that affect the
latest stable releases or the latest unstable builds. In other words, we will not
backport any changes to older versions of our software. This might change in the
future or under exceptional circumstances.

Any change to address some security vulnerability will be part of the next
release. Releases are published at our discretion, considering the impact and
likelihood of its exploitation and other project management factors.

## Reporting a Vulnerability

To report a security vulnerability, you must first send a direct, **private**
message to the project lead over Discord, _AlexTMjugador#5124_. Further
communication via other channels might be negotiated afterward.

At our discretion, we will try to review your report and notify you about its
resolution. Accepted security vulnerabilities will be responsibly disclosed via
GitHub's security advisories feature, and maybe other security advisory
databases too (RustSec, etc.). Responsible disclosure will happen after the
vulnerability has been resolved, and we deem that its most egregious impacts have
been mitigated.
