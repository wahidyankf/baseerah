@repo-config-data-driven
Feature: Repo-specific behaviour is data-driven from repo-config.yml

  As a maintainer keeping rhino-cli byte-identical across ose-public, ose-primer, and ose-infra
  I want every per-repo behaviour (env globs, domain-areas, ddd-areas) read from repo-config.yml
  So that the Rust source stays identical and only the per-repo data file differs

  Scenario: Repo-specific behaviour is data-driven, not hard-coded
    Given rhino-cli's repo-specific behaviour (env globs, domain/ddd areas)
    When rhino-cli runs
    Then it reads that behaviour from repo-config.yml, not from source hard-coded per repo
