# Security policy

## Reporting a vulnerability

Report a vulnerability in private, through **Report a vulnerability** on the
[Security tab](https://github.com/linouxis9/oxirush-ngap/security) of the
repository. Please do not open a public issue for it.

Say what the input is and what happens: the octets of the PDU, the call that
was made on them, and the version or the commit of the crate. Octets that make
the crate panic, loop, or take time or memory out of proportion to their size
are a vulnerability. So are octets that the crate reads as another message
than they are, a message that it writes as octets which it reads back as
another, and a key that `Debug` or `Display` prints.

## Supported versions

The latest version on crates.io receives the fixes.
