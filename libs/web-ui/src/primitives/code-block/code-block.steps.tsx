import path from "path";
import { loadFeature, describeFeature } from "@amiceli/vitest-cucumber";
import { render, screen, cleanup, fireEvent } from "@testing-library/react";
import { expect, vi } from "vitest";

import { CodeBlock } from "./code-block";
import codeBlockStories, {
  Copied as CodeBlockCopiedStory,
  Default as CodeBlockDefaultStory,
} from "./code-block.stories";

const feature = await loadFeature(
  path.resolve(__dirname, "../../../../../specs/libs/web-ui/behavior/gherkin/code-block/code-block.feature"),
);

/** Installs a mock `navigator.clipboard.writeText` (jsdom lacks it). Returns the spy. */
function stubClipboard(): ReturnType<typeof vi.fn> {
  const writeText = vi.fn(() => Promise.resolve());
  Object.defineProperty(navigator, "clipboard", { value: { writeText }, configurable: true });
  return writeText;
}

function getWrapper(container: HTMLElement): HTMLElement {
  const wrapper = container.querySelector('[data-slot="code-block"]');
  if (!(wrapper instanceof HTMLElement)) {
    throw new Error("code-block wrapper not found");
  }
  return wrapper;
}

// The @visual scenario is exercised by the Playwright Storybook baselines, not by this unit-level
// step binder, so it is excluded here.
describeFeature(
  feature,
  ({ Scenario }) => {
    Scenario("The code block renders its highlighted children and a copy button", ({ Given, When, Then, And }) => {
      let childPresent = false;
      let copyButtonPresent = false;

      Given("a CodeBlock rendered with code text and a highlighted <pre> child", () => {
        // mount + inspection happen together in the When step
      });

      When("the component mounts", () => {
        cleanup();
        stubClipboard();
        const { container } = render(
          <CodeBlock code="print('hi')">
            <pre data-testid="highlighted">print(&apos;hi&apos;)</pre>
          </CodeBlock>,
        );
        childPresent = screen.queryByTestId("highlighted") !== null;
        copyButtonPresent = getWrapper(container).querySelector('[data-slot="code-block-copy"]') !== null;
      });

      Then("the highlighted child is present", () => {
        expect(childPresent).toBe(true);
      });

      And("a copy button is present within the code-block wrapper", () => {
        expect(copyButtonPresent).toBe(true);
      });
    });

    Scenario("Copying from the code block yields the verbatim multi-line source", ({ Given, When, Then }) => {
      let writeText: ReturnType<typeof vi.fn>;
      const code = [
        "local ok = pcall(fn)   -- => runs inner fn",
        "error({ code = 42 })   -- => any Lua value",
        "print(err.code)        -- => err IS the table",
      ].join("\n");

      Given("a CodeBlock whose code prop is a three-line annotated snippet with trailing comments", () => {
        // render + click happen together in the When step
      });

      When("the user clicks the code block's copy button", () => {
        cleanup();
        writeText = stubClipboard();
        const { container } = render(
          <CodeBlock code={code}>
            <pre>highlighted</pre>
          </CodeBlock>,
        );
        const button = getWrapper(container).querySelector('[data-slot="code-block-copy"]');
        fireEvent.click(button as HTMLElement);
      });

      Then("the clipboard receives the snippet byte-for-byte including every annotation and newline", () => {
        // Compared against the in-process value handed to writeText (pre-clipboard), per
        // tech-docs.md's Windows \r\n caveat.
        expect(writeText).toHaveBeenCalledWith(code);
      });
    });

    Scenario("The code block establishes its own positioning context", ({ Given, When, Then }) => {
      let dataSlot: string | null = null;
      let className = "";

      Given("a CodeBlock is rendered", () => {
        // render + inspection happen together in the When step
      });

      When("its wrapper is inspected", () => {
        cleanup();
        stubClipboard();
        const { container } = render(
          <CodeBlock code="x">
            <pre>x</pre>
          </CodeBlock>,
        );
        const wrapper = getWrapper(container);
        dataSlot = wrapper.getAttribute("data-slot");
        className = wrapper.className;
      });

      Then('the wrapper is a relatively-positioned element carrying data-slot "code-block"', () => {
        expect(dataSlot).toBe("code-block");
        expect(className).toContain("relative");
      });
    });

    // The pixel comparison itself is the Playwright Storybook baseline
    // (`libs/web-ui/e2e/components.visual.ts`), which is not a CI-gated unit target; this
    // `@visual`-tagged binder is skipped at runtime (see `excludeTags` below) and exists so the
    // spec-coverage checker sees the scenario bound. Its bodies smoke-check that the resting +
    // copied stories the baselines capture actually exist, so a story rename can't silently strand
    // the visual cases.
    Scenario("The code block renders correctly in light and dark themes", ({ Given, When, Then }) => {
      Given("the CodeBlock stories are loaded in Storybook", () => {
        expect(codeBlockStories.title).toBe("Primitives/CodeBlock");
      });

      When("the resting and copied stories are captured in light and dark themes", () => {
        // Capture happens in Playwright against the light default and the `&globals=theme:dark`
        // global; here we assert the two captured stories are defined.
        expect(CodeBlockDefaultStory).toBeDefined();
        expect(CodeBlockCopiedStory).toBeDefined();
      });

      Then("each screenshot matches its committed visual baseline", () => {
        // The committed PNG baselines live beside `components.visual.ts`; the byte comparison is a
        // Playwright concern, not a jsdom one.
        expect(codeBlockStories.component).toBe(CodeBlock);
      });
    });
  },
  { excludeTags: ["visual"] },
);
