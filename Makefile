# InventivAgents — repeatable local commands (GNU Make + bash).
# Discovery: make help
# Same behavior: ./scripts/dev/dev.sh <command>

.DEFAULT_GOAL := help

DEV := ./scripts/dev/dev.sh
WITH := ./scripts/dev/with-env.sh

# ---------------------------------------------------------------------------
# Help
# ---------------------------------------------------------------------------
.PHONY: help
help:
	@cat scripts/dev/make-help.txt

# ---------------------------------------------------------------------------
# Simple lifecycle verbs (aliases + small wrappers)
# ---------------------------------------------------------------------------
.PHONY: build release release-build start stop delete test test-unit fmt lint clean

# Debug build (.env loaded for consistency if build scripts ever need it).
build:
	@$(WITH) cargo build

# Alias: many teams use "make release" for a release binary (not deploy).
release: release-build

# Release artifact locally. Optional: TAG=v1.2.0 prints a reminder to record the version in git.
release-build:
	@$(WITH) cargo build --release
	@if [ -n "$(TAG)" ]; then printf '\nHint: record this release in git, e.g.:\n  git tag -a %s -m "Release %s" && git push origin %s\n\n' "$(TAG)" "$(TAG)" "$(TAG)"; fi

# Infra up + best-effort migrations (good "start my day" command).
start: ready

stop: down

# Wipes local Docker volumes for this project — destructive.
delete: reset

test:
	@$(DEV) test

test-unit: test-lib

fmt:
	@$(WITH) cargo fmt --all

lint:
	@$(WITH) cargo clippy --all-targets -- -D warnings

clean:
	cargo clean

# ---------------------------------------------------------------------------
# dev.sh passthrough (precise names)
# ---------------------------------------------------------------------------
.PHONY: doctor env up down migrate reset ready test-lib run run-rel check full cargo

doctor:
	@$(DEV) doctor

env:
	@$(DEV) env

up:
	@$(DEV) up

down:
	@$(DEV) down

migrate:
	@$(DEV) migrate

reset:
	@$(DEV) reset

ready:
	@$(DEV) ready

test-lib:
	@$(DEV) test-lib

run:
	@$(DEV) run $(ARGS)

run-rel:
	@$(DEV) run-rel $(ARGS)

check:
	@$(DEV) check

full:
	@$(DEV) full

cargo:
	@test -n "$(ARGS)" || (echo 'Set ARGS, e.g. make cargo ARGS="test --test agents_api"' >&2; exit 1)
	@$(WITH) cargo $(ARGS)

# ---------------------------------------------------------------------------
# Deploy stubs (replace bodies with gh / kubectl / your CD tool)
# ---------------------------------------------------------------------------
REF ?= latest

.PHONY: deploy deploy-staging deploy-prod

deploy:
	@echo "Usage:" >&2
	@echo "  make deploy-staging REF=<git-ref|image-tag>   (default REF=$(REF))" >&2
	@echo "  make deploy-prod    REF=<git-ref|image-tag>" >&2
	@echo "Wire these targets to your pipeline (immutable artifact + promotion). See scripts/dev/make-help.txt." >&2
	@exit 1

deploy-staging:
	@echo ">>> deploy-staging (stub): REF=$(REF)" >&2
	@echo "    Replace this target with your CD step (e.g. gh workflow run, kubectl, Scaleway)." >&2

deploy-prod:
	@echo ">>> deploy-prod (stub): REF=$(REF)" >&2
	@echo "    Production must use an approved pipeline; do not rely on Make alone for secrets/audit." >&2
