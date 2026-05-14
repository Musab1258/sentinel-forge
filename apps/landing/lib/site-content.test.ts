import { describe, expect, it } from "vitest";

import {
  architecture,
  ecosystemFoundations,
  metrics,
  modules,
  readinessTracks,
  roadmap,
} from "./site-content";

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
    expect(roadmap[roadmap.length - 1]?.phase).toBe("Phase 8");
    expect(metrics).toHaveLength(3);
  });

  it("includes later-phase readiness surfaces", () => {
    expect(readinessTracks).toHaveLength(3);
    expect(readinessTracks.map((track) => track.phase)).toEqual([
      "Phase 6",
      "Phase 7",
      "Phase 8",
    ]);
    expect(ecosystemFoundations.length).toBeGreaterThanOrEqual(5);
  });
});
