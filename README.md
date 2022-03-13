<div align="center">
  <h1>Saas Auth Ssot</h1>
  <p>
    <strong>Single Source Of Truth authentication system for Speculare SAAS</strong>
  </p>
  <p>

[![Apache 2 License](https://img.shields.io/badge/license-Apache%202-blue.svg)](LICENSE)

  </p>
</div>

This project is intended to be used internally only, but if somebody finds a personal use case for this, feel free to use.

The goal of `saas-auth-ssot` is to have a service (API + database) that other Speculare services can use to authorize requests and users. This service will be responsible for:
- `Sign-{In/Up}` using `Magic Link` (no password)
- Keeping track of ownership of hosts being monitored
- Generating API Key for a newly created host
- Overall authorization/authentication across the service


Contributing
--------------------------

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.