Feature: crane-be PDF to markdown HTTP endpoint

  @unit
  Scenario: crane-be converts a PDF to markdown over HTTP using the fake adapter
    Given crane-be is configured with the fake media adapter
    When a client sends POST /media/pdf-to-md with sample PDF bytes
    Then the response status is 200
    And the response body contains the canned markdown output

  @integration @e2e
  Scenario: crane-be converts a real PDF to markdown over HTTP using the real adapter
    Given crane-be is configured with the real PdfPig/Tesseract adapter
    When a client sends POST /media/pdf-to-md with a real sample PDF
    Then the response status is 200
    And the response body contains markdown extracted from the PDF

  @unit @e2e
  Scenario: crane-be rejects an empty request body
    Given the crane-be service is running on its configured port
    When a client sends POST /media/pdf-to-md with an empty body
    Then the response status is 400
    And the response body indicates the PDF payload was missing

  @unit @e2e
  Scenario: crane-be rejects a non-PDF payload
    Given the crane-be service is running on its configured port
    When a client sends POST /media/pdf-to-md with bytes that are not a PDF
    Then the response status is 422
    And the response body indicates the payload could not be parsed as a PDF

  @e2e
  Scenario: crane-be returns markdown with the text/markdown content type
    Given the crane-be service is running on its configured port
    When a client sends POST /media/pdf-to-md with a real sample PDF
    Then the response status is 200
    And the response Content-Type is text/markdown
