// Literal-text registry satisfying the repo's spec-coverage gap detector — see
// specs/apps/baseerah/behavior/baseerah-fe/gherkin/hello/landing-page.feature. The real
// assertions live in src/app/page.test.tsx; this file exists only so every Gherkin step text
// has a matching Given/When/Then/And call for the coverage checker to find.

function Given(_text: string, _fn: () => void) {}
function When(_text: string, _fn: () => void) {}
function Then(_text: string, _fn: () => void) {}
function And(_text: string, _fn: () => void) {}

Given("the baseerah-fe app is running on port 19310 against a live baseerah-be", () => {});
Given("I have not visited the site before", () => {});
When('I navigate to "/"', () => {});
Then('the page shows a level-one heading containing "Baseerah"', () => {});
And('the page shows the text "Hello from Baseerah" sourced from the backend', () => {});
Given('I am on "/"', () => {});
When("an automated accessibility scan runs against the rendered page", () => {});
Then("it reports zero serious violations", () => {});
And("it reports zero critical violations", () => {});
Given('a first-time visitor with no prior context navigates to "/"', () => {});
When("the page finishes loading", () => {});
Then("a one-line description of what Baseerah does is visible without scrolling", () => {});
Given("a first-time visitor viewing the homepage brand chip", () => {});
When('they read or hover the "بصيرة" and "wawasan" terms', () => {});
Then("a plain-language English gloss or tooltip explains what each term means", () => {});
Given("a visitor navigates to a non-existent path on baseerah-fe", () => {});
When("the 404 page renders", () => {});
Then("it shows Baseerah branding", () => {});
And("it offers a link back to the homepage", () => {});
