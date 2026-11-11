---
description: 3D models: fewer larger parts, shared MeshIds, union triangle checks, anchoring, LevelOfDetail, texture sizes (256 minor, 512 standard).
type: checklist
status: draft
last_checked: 2026-11-09
sources: Research report (Nov 9 2026) C8; c-s-labs luckbound toolchain; 3daistudio free MCP servers
verify: Performance guidance from community references; verify each model in Studio Scene Analysis (triangles) and MicroProfiler.
---
# 3D model checklist

- [ ] Fewer, larger parts.
- [ ] MeshParts share a MeshId where possible (instanced, cheap).
- [ ] Unions checked for triangle count in Scene Analysis (they can be expensive).
- [ ] Static parts anchored.
- [ ] `Model.LevelOfDetail` set for static distant models (reported value name `SLIM`: UNVERIFIED, check the enum).
- [ ] Textures: 256x256 for minor objects, 512x512 standard. Memory scales with the square: 1024 is 4x a 512.
- [ ] Collision simplified where players never touch it.
- [ ] License recorded ([[10-Art-Environment/asset-sources-and-licenses]]); no Toolbox free models.
- [ ] Within [[14-QA-Optimization/performance-budgets]].
