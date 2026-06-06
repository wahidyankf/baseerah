@git-pre-commit
Feature: Pre-commit hook orchestration

  As a developer
  I want rhino-cli git pre-commit to orchestrate all pre-commit checks
  So that code quality is enforced consistently before every commit

  Scenario: Running pre-commit outside a git repository fails
    Given the developer is outside a git repository
    When the developer runs rhino-cli git pre-commit
    Then the command exits with a failure code
    And the output mentions that a git repository was not found

  Scenario: Broken-link detection in step 7 reports per-link details
    Given staged markdown files contain a link to a non-existent target
    When the developer runs rhino-cli git pre-commit
    Then the command exits with a failure code
    And the stderr output identifies the source file containing the broken link
    And the stderr output identifies the line number of the broken link
    And the stderr output identifies the broken link target

  Scenario: staged-mermaid-blocks — staged malformed mermaid diagram blocks commit
    Given a staged markdown file under docs containing a mermaid diagram with a label exceeding the maximum length
    When the developer runs rhino-cli git pre-commit
    Then the command exits with a failure code
    And the output indicates a mermaid violation was found

  Scenario: staged-prose-heading-blocks — staged docs file with bad heading hierarchy blocks commit
    Given a staged markdown file under docs containing two H1 headings
    When the developer runs rhino-cli git pre-commit
    Then the command exits with a failure code
    And the output indicates a heading hierarchy violation was found

  Scenario: staged-skill-file-exempt — staged SKILL.md with bad heading hierarchy does not block commit
    Given a staged SKILL.md under .claude/skills with multiple H1 headings
    When the developer runs rhino-cli git pre-commit
    Then the heading hierarchy step does not block the commit for that file

  Scenario: link-step-honors-exclusions — staged plans/done broken link does not block commit
    Given a staged markdown file under plans/done containing a broken internal link
    When the developer runs rhino-cli git pre-commit
    Then the link validation step does not report a broken link for the plans/done file
