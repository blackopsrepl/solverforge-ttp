# Changelog

All notable changes to this project will be documented in this file. See [Conventional Commits](https://conventionalcommits.org) for commit guidelines.

## [0.3.1](https://github.com/blackopsrepl/solverforge-ttp/compare/v0.3.0...v0.3.1) (2026-11-20)


### Documentation

* measure parallel evaluation against sequential and auto on 0.31.2 ([54aebd7](https://github.com/blackopsrepl/solverforge-ttp/commit/54aebd7d1b2671a49843a28ee9e892d4647e12ea))


## [0.3.0](https://github.com/blackopsrepl/solverforge-ttp/compare/v0.2.0...v0.3.0) (2026-11-20)


### ⚠ BREAKING CHANGES

* **runtime:** constraint analysis entries carry weightMultiplier instead of weight, and selector telemetry drops its six conflictRepair* fields.

### Features

* **runtime:** target SolverForge 0.31.2 ([5cde067](https://github.com/blackopsrepl/solverforge-ttp/commit/5cde0673cc0286f13a8cc5e6d8206cc05c3f6cee))


### Documentation

* publish the 120 s sweep on SolverForge 0.31.2 ([a577b9e](https://github.com/blackopsrepl/solverforge-ttp/commit/a577b9e5e8421ae229c8dcbf8fd35beaccc8bfbc))


### Code Refactoring

* **constraints:** keep the module root to declarations and re-exports ([1d1df76](https://github.com/blackopsrepl/solverforge-ttp/commit/1d1df767c4793a7e85093618c182f2e6d69940fd))


## [0.2.0](https://github.com/blackopsrepl/solverforge-ttp/compare/v0.1.0...v0.2.0) (2026-10-27)


### ⚠ BREAKING CHANGES

* **runtime:** telemetry.phase is replaced by telemetry.phases, one entry per started phase.

### Features

* **runtime:** target SolverForge 0.23.0 ([6041795](https://github.com/blackopsrepl/solverforge-ttp/commit/6041795fca81e7714670700d3178b8c79ef53607))
* **solver:** evaluate local-search steps on eight threads ([5204cd2](https://github.com/blackopsrepl/solverforge-ttp/commit/5204cd2ccd736cf62be6454024a6ca7cd41f6cc2))


### Bug Fixes

* **sweep:** send SECONDS as each job's solve budget ([9edadb2](https://github.com/blackopsrepl/solverforge-ttp/commit/9edadb24e4fb0c9fab6d621560740136f8401177))


## 0.1.0 (2026-10-19)


### Features

* **data:** embed RobinX NL benchmark instances with the published golden schedule ([fca544b](https://github.com/blackopsrepl/solverforge-ttp/commit/fca544b4c7e62e91ea610616217681ed61933571))
* model the classic TTP(3) as a SolverForge constraint program ([235c163](https://github.com/blackopsrepl/solverforge-ttp/commit/235c163f87c9df89879e90784d237684783c1665))
* **solver:** diversified late acceptance + best-of-step breadth for feasibility ([22fc768](https://github.com/blackopsrepl/solverforge-ttp/commit/22fc768f7cc3ef1180ea2b074c6b9dd2cabc0b4a))


### Bug Fixes

* **scripts:** parse SolverForge score strings; never report infeasible runs as gaps ([4c8d819](https://github.com/blackopsrepl/solverforge-ttp/commit/4c8d819c8cf6f00204d415e4290ad6108d3288e7))
