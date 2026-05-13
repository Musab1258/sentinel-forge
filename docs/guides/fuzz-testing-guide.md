# Fuzz Testing Guide

Fuzzing is not implemented yet, but contributors can prepare for it in a disciplined way.

## What to contribute now

- small fixture corpora
- documented invariants
- examples of dangerous state transitions
- reproducibility requirements for future harnesses

## Principles

- every failure must shrink to something understandable
- every interesting input should be reproducible
- hostile inputs should run inside isolated workflows

## Suggested invariant categories

- authorization must gate privileged actions
- state transitions must preserve documented assumptions
- arithmetic paths must remain within safe bounds
- repeated interactions must not create resource abuse patterns

## Relationship to static analysis

Fuzzing should complement detectors by testing dynamic behavior that static rules can only approximate.
