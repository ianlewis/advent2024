# Copyright 2024 Ian Lewis
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

SHELL := /bin/bash
OUTPUT_FORMAT ?= $(shell if [ "${GITHUB_ACTIONS}" == "true" ]; then echo "github"; else echo ""; fi)
REPO_NAME = $(shell basename "$$(pwd)")

SOLUTIONS_REPO ?= ../advent-of-code-solutions

# The help command prints targets in groups. Help documentation in the Makefile
# uses comments with double hash marks (##). Documentation is printed by the
# help target in the order in appears in the Makefile.
#
# Make targets can be documented with double hash marks as follows:
#
#	target-name: ## target documentation.
#
# Groups can be added with the following style:
#
#	## Group name

.PHONY: help
help: ## Shows all targets and help from the Makefile (this message).
	@echo "$(REPO_NAME) Makefile"
	@echo "Usage: make [COMMAND]"
	@echo ""
	@grep --no-filename -E '^([/a-z.A-Z0-9_%-]+:.*?|)##' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = "(:.*?|)## ?"}; { \
			if (length($$1) > 0) { \
				printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2; \
			} else { \
				if (length($$2) > 0) { \
					printf "%s\n", $$2; \
				} \
			} \
		}'

package-lock.json:
	npm install

node_modules/.installed: package.json package-lock.json
	npm ci
	touch node_modules/.installed

.venv/bin/activate:
	python -m venv .venv

.venv/.installed: .venv/bin/activate requirements.txt
	./.venv/bin/pip install -r requirements.txt --require-hashes
	touch .venv/.installed

## Tools
#####################################################################

.PHONY: license-headers
license-headers: ## Update license headers.
	@set -euo pipefail; \
		files=$$( \
			git ls-files \
				'*.go' '**/*.go' \
				'*.ts' '**/*.ts' \
				'*.js' '**/*.js' \
				'*.py' '**/*.py' \
				'*.yaml' '**/*.yaml' \
				'*.yml' '**/*.yml' \
				'*.rs' '**/*.rs' \
		); \
		name=$$(git config user.name); \
		if [ "$${name}" == "" ]; then \
			>&2 echo "git user.name is required."; \
			>&2 echo "Set it up using:"; \
			>&2 echo "git config user.name \"John Doe\""; \
		fi; \
		for filename in $${files}; do \
			if ! ( head "$${filename}" | grep -iL "Copyright" > /dev/null ); then \
				autogen -i --no-code --no-tlc -c "$${name}" -l apache "$${filename}"; \
			fi; \
		done; \
		if ! ( head Makefile | grep -iL "Copyright" > /dev/null ); then \
			autogen -i --no-code --no-tlc -c "$${name}" -l apache Makefile; \
		fi;

.PHONY: format
format: md-format yaml-format ## Format all files

.PHONY: md-format
md-format: node_modules/.installed ## Format Markdown files.
	@set -euo pipefail; \
		files=$$( \
			git ls-files \
				'*.md' '**/*.md' \
				'*.markdown' '**/*.markdown' \
		); \
		npx prettier --write --no-error-on-unmatched-pattern $${files}

.PHONY: yaml-format
yaml-format: node_modules/.installed ## Format YAML files.
	@set -euo pipefail; \
		files=$$( \
			git ls-files \
				'*.yml' '**/*.yml' \
				'*.yaml' '**/*.yaml' \
		); \
		npx prettier --write --no-error-on-unmatched-pattern $${files}

## Linters
#####################################################################

.PHONY: lint
lint: yamllint actionlint markdownlint clippy ## Run all linters.

.PHONY: actionlint
actionlint: ## Runs the actionlint linter.
	@# NOTE: We need to ignore config files used in tests.
	@set -euo pipefail;\
		files=$$( \
			git ls-files \
				'.github/workflows/*.yml' \
				'.github/workflows/*.yaml' \
		); \
		if [ "$(OUTPUT_FORMAT)" == "github" ]; then \
			actionlint -format '{{range $$err := .}}::error file={{$$err.Filepath}},line={{$$err.Line}},col={{$$err.Column}}::{{$$err.Message}}%0A```%0A{{replace $$err.Snippet "\\n" "%0A"}}%0A```\n{{end}}' -ignore 'SC2016:' $${files}; \
		else \
			actionlint $${files}; \
		fi

.PHONY: markdownlint
markdownlint: node_modules/.installed ## Runs the markdownlint linter.
	@set -euo pipefail;\
		files=$$( \
			git ls-files \
				'*.md' '**/*.md' \
				'*.markdown' '**/*.markdown' \
		); \
		if [ "$(OUTPUT_FORMAT)" == "github" ]; then \
			exit_code=0; \
			while IFS="" read -r p && [ -n "$$p" ]; do \
				file=$$(echo "$$p" | jq -c -r '.fileName // empty'); \
				line=$$(echo "$$p" | jq -c -r '.lineNumber // empty'); \
				endline=$${line}; \
				message=$$(echo "$$p" | jq -c -r '.ruleNames[0] + "/" + .ruleNames[1] + " " + .ruleDescription + " [Detail: \"" + .errorDetail + "\", Context: \"" + .errorContext + "\"]"'); \
				exit_code=1; \
				echo "::error file=$${file},line=$${line},endLine=$${endline}::$${message}"; \
			done <<< "$$(npx markdownlint --dot --json $${files} 2>&1 | jq -c '.[]')"; \
			exit "$${exit_code}"; \
		else \
			npx markdownlint --dot $${files}; \
		fi

.PHONY: yamllint
yamllint: .venv/.installed ## Runs the yamllint linter.
	@set -euo pipefail;\
		extraargs=""; \
		files=$$( \
			git ls-files \
				'*.yml' '**/*.yml' \
				'*.yaml' '**/*.yaml' \
		); \
		if [ "$(OUTPUT_FORMAT)" == "github" ]; then \
			extraargs="-f github"; \
		fi; \
		.venv/bin/yamllint --strict -c .yamllint.yaml $${extraargs} $${files}

DAYS = day1 day2 day3 day4 day5 day6 day7 day8 day9 day10 day11 day12 day13 day14 day15 day16 day17 day18 day19 day20 day21 day22 day23 day24

.PHONY: clippy
clippy: ## Runs clippy linter.
	@set -euo pipefail; \
		exit_code=0; \
		for i in day*; do\
			if ! cargo clippy --manifest-path "$$i/Cargo.toml" -- -D warnings; then \
				exit_code=1; \
			fi; \
		done; \
		exit "$${exit_code}"; \

## Solutions
#####################################################################

all: $(DAYS) ## Build all solutions

define GEN_RULE
.PHONY: $1
$1: ## Make each day's solution binary.
	cargo build --manifest-path "$1/Cargo.toml"
endef

$(foreach var,$(DAYS),$(eval $(call GEN_RULE,$(var))))

## Tests
#####################################################################

unit-tests: ## Run unit tests.
	@set -euo pipefail;\
		for i in day*; do\
			cargo test --manifest-path "$$i/Cargo.toml";\
		done

integration-tests: ## Run integration tests.
	@set -euo pipefail;\
		for i in day*; do\
			cargo build --manifest-path "$${i}/Cargo.toml";\
			SOLUTION=$$("$${i}/target/debug/$${i}" < "$(SOLUTIONS_REPO)/2024/$${i}/input.txt");\
			EXPECTED="$$(cat "$(SOLUTIONS_REPO)/2024/$${i}/solution.txt")";\
			[ "$${SOLUTION}" == "$${EXPECTED}" ] || (echo "$${i} failed!" && exit 1);\
		done;


## Maintenance
#####################################################################

clean: ## Clean up build files.
	@set -euo pipefail;\
		for i in day*; do\
			rm -rf "$$i/target";\
		done;
