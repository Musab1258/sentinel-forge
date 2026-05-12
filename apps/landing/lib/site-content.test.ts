import { describe, expect, it } from "vitest";

import { architecture, metrics, modules, roadmap } from "./site-content";

describe("site content", () => {
  it("covers the core security modules", () => {
    expect(modules).toHaveLength(6);
    expect(modules.map((module) => module.name)).toContain("Static Analysis Engine");
  });

  it("keeps the architecture narrative intact", () => {
    expect(architecture[0]?.stage).toBe("Ingest");
    expect(architecture[architecture.length - 1]?.stage).toBe("Integrate");
  });

  it("surfaces roadmap and project metrics", () => {
    expect(roadmap[0]?.phase).toBe("Phase 1");
    expect(metrics).toHaveLength(3);
  });
});

