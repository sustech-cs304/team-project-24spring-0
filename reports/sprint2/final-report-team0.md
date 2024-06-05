# Moras - Sprint2

## 1. Metrics
script: [statics.sh](../../scripts/stastics.sh)
| Lines of Code | Number of packages/modules | Number of source files | Number of dependencies |
| ------------- | -------------------------- | ---------------------- | ---------------------- |
| 21259         | 2                          | 138                    | 34                     |

## 2. Documentation
Documentation for end users: [https://rosswasd.github.io/team-project-24spring-0/](https://rosswasd.github.io/team-project-24spring-0/)

Documentation for developers: [https://sustech-cs304.github.io/team-project-24spring-0/moras/](https://sustech-cs304.github.io/team-project-24spring-0/moras/)

## 3. Tests
- tools: `cargo tarpaulin`
- tasks: `cargo tarpaulin --out html`
- source code: [tests folder](../../src-tauri/src/tests), [ci test task](../../.github/workflows/build.yml), [coverage report script](../../scripts/report.sh)
- coverage report: [coverage report](https://sustech-cs304.github.io/team-project-24spring-0/report#src)

## 4. Build
- tools: `cargo`, `npm`, `github CI/CD`
- frameworks: `tauri`
- tasks:
  - CI ([build.yaml](../../.github/workflows/build.yml))
    - format check: `cargo fmt -- --check`, `npm run format-check`
    - generate test coverage report: [report.sh](../../scripts/report.sh) (`cargo tarpaulin --out Html`)
    - run tests: `cargo test` 
    - generate documentation and test reports: `cargo doc --no-deps`
    - deploy documentation and report to GitHub Pages
  - CD ([release.yaml](../../.github/workflows/release.yml))
    - build for multiple platforms: `tauri-apps/tauri-action@v0` (`cargo tauri build`)
    - deploy executable to GitHub
- executable: [releases page](https://github.com/sustech-cs304/team-project-24spring-0/releases)
- buildfile: [back end](../../src-tauri/Cargo.toml), [front end](../../src-ui/package.json), [github CI](../../.github/workflows/build.yml), [github CD](../../.github/workflows/release.yml)

## 5. Deployment
This is a desktop application without any online services, so there is no need for deployment. However, we use GitHub CD to automatically build our application for multiple platforms. [release.yaml](../../.github/workflows/release.yml)