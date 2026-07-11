# ADR-0056 Adversarial Review

Decision: approve. Numeric MIR IDs and hidden side tables are rejected because
they can alias or drift across module and declaration boundaries. Structured
identity is required before object symbol derivation.
