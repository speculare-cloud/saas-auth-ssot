<div align="center">
  <h1>Saas Auth Ssot</h1>
  <p>
    <strong>Single Source Of Truth authentication system for Speculare SAAS</strong>
  </p>
  <p>

[![Apache 2 License](https://img.shields.io/badge/license-Apache%202-blue.svg)](LICENSE)
[![CI](https://github.com/speculare-cloud/saas-auth-ssot/workflows/CI/badge.svg)](https://github.com/speculare-cloud/saas-auth-ssot/actions)

  </p>
</div>

This project is intended to be used internally only, but if somebody finds a personal use case for this, feel free to use.

The goal of `saas-auth-ssot` is to have a service (API + database) that other Speculare services can use to authorize requests and users. This service will be responsible for:
- `Sign-{In/Up}` using `Magic Link` (no password)
- Keeping track of ownership of hosts being monitored
- Generating API Key for a newly created host
- Overall authorization/authentication across the service

Server setup / Dev setup
--------------------------

- Install all build dependencies

```bash
$ sudo apt-get install cmake libssl-dev libpq-dev pkg-config build-essential
```

- Create a ssot.config file based on ssot.example.config

Generating JWT EC Keys
--------------------------

```bash
$ openssl ecparam -genkey -noout -name prime256v1 | openssl pkcs8 -topk8 -nocrypt -out ec-private.pem

$ openssl ec -in ec-private.pem -pubout -out ec-public.pem
```

Don't forgot to specify the path for those prime256v1 keys in your ssot.config.