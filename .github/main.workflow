workflow "Test projects" {
  on = "push"
  resolves = "Test all projects"
}

action "Don't skip CI" {
  uses = "ffflorian/actions/skip-ci-check@master"
}

action "Test all projects" {
  uses = "./.github/actions/rust-test"
  needs = "Don't skip CI"
}
