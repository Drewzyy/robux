---
description: How Roblox animations are made and played: Animation Editor, Animator, priorities, and keeping them cheap.
type: reference
status: draft
last_checked: 2026-11-11
sources: https://create.roblox.com/docs/animation/editor
verify: UNVERIFIED: written from model knowledge; check against https://create.roblox.com/docs/animation/editor
tags: [animation-vfx, reference]
---
# Animation basics

## Making an animation
- Built in Studio's **Animation Editor** on a rig (R15 character or any model with Motor6D joints).
- An animation is saved to Roblox as an asset and referenced by `rbxassetid://<id>` in an `Animation` object. Saving the asset is Andrew's step; agents write the request and the code that plays it.

## Playing it
- Load through the `Animator` inside the Humanoid or AnimationController: `animator:LoadAnimation(anim)` returns an `AnimationTrack`; call `:Play()`, `:Stop()`, adjust speed and weight.
- Load each track once and reuse it. Loading on every play leaks tracks.
- Play character animations from the client that owns the character; they replicate to others.

## Priorities
Core < Idle < Movement < Action (and Action2 to Action4 on newer rigs). A higher priority overrides lower ones on the joints it touches. Attacks and emotes use Action so walking does not cancel them.

## Cheap juice without new assets
Most polish needs no animation asset at all: `TweenService` on size, position, transparency, and colour; camera shake by small CFrame offsets; UI pops with UIScale tweens. Prefer these on phones.
