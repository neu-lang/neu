# ADR-0055 Adversarial Review

Decision: approve. Rejecting raw IDs, missing records, and non-Int records
prevents backend lowering from treating an unrelated or forged type identity as
a valid runtime `Int`.
