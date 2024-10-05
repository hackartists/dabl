VERSION ?= v0.1.0
COMMIT ?= $(shell git rev-parse --short HEAD)
ENV ?= local
SERVICE ?= $(shell basename `git rev-parse --show-toplevel`)
BASE_DOMAIN ?= biyard.co


ifeq ("$(ENV)","prod")
	LOG_LEVEL ?= error
	DOMAIN ?= $(SERVICE).$(BASE_DOMAIN)
	REDIRECT_URI ?= https://$(DOMAIN)
	AWS_DYNAMODB_TABLE ?= $(SERVICE)-prod
endif

ifeq ("$(ENV)","dev")
	REDIRECT_URI ?= https://$(DOMAIN)
endif

DOMAIN ?= $(SERVICE).$(ENV).$(BASE_DOMAIN)
LOG_LEVEL ?= debug
REDIRECT_URI ?= http://localhost:8080
AWS_ACCESS_KEY_ID ?= $(shell aws configure get aws_access_key_id $(AWS_FLAG))
AWS_SECRET_ACCESS_KEY ?= $(shell aws configure get aws_secret_access_key $(AWS_FLAG))
AWS_REGION ?= $(shell aws configure get region)
AWS_DYNAMODB_TABLE ?= $(SERVICE)-dev
CDN_ID ?= $(shell aws cloudfront list-distributions --query "DistributionList.Items[*].{id:Id,test:AliasICPRecordals[?CNAME=='$(DOMAIN)']}" --output json |jq '. | map(select(.test | length > 0))[0] | .id' | tr -d \")
ACM_ID ?= $(shell aws acm list-certificates --query "CertificateSummaryList[*].{id:CertificateArn,domains:SubjectAlternativeNameSummaries}[?contains(domains,'dabl.biyard.co')].id" --output text --region us-east-1)
HOSTED_ZONE_ID ?= $(shell basename `aws route53 list-hosted-zones-by-name --dns-name biyard.co --query "HostedZones[0].Id" --output text`)

BUILD_ENV ?= LOG_LEVEL=$(LOG_LEVEL) REDIRECT_URI=$(REDIRECT_URI) AWS_DYNAMODB_TABLE=$(AWS_DYNAMODB_TABLE) VERSION=$(VERSION) COMMIT=$(COMMIT) ENV=$(ENV) SERVICE=$(SERVICE) TABLE_NAME=$(AWS_DYNAMODB_TABLE) DOMAIN=$(DOMAIN) AWS_ACCESS_KEY_ID=$(AWS_ACCESS_KEY_ID) AWS_SECRET_ACCESS_KEY=$(AWS_SECRET_ACCESS_KEY) AWS_REGION=$(AWS_REGION) CDN_ID=$(CDN_ID) ACM_ID=$(ACM_ID) HOSTED_ZONE_ID=$(HOSTED_ZONE_ID)

.PHONY: setup run
setup:
	cargo install dioxus-cli --version 0.6.0-alpha.2
	npm install -g aws-cdk tailwindcss

run: public/tailwind.css
	$(BUILD_ENV) dx serve -i false

build: clean public/tailwind.css
	$(BUILD_ENV) dx build --release

run-server: build
	dist/$(SERVICE)

build-lambda: clean public/tailwind.css
	$(BUILD_ENV) dx build --release --platform fullstack --server-feature lambda
	mv dist/$(SERVICE) dist/bootstrap

public/tailwind.css:
	tailwindcss -i ./input.css -o ./public/tailwind.css --minify

.ONESHELL: cdk-build cdk-deploy fixtures/cdk/node_modules
fixtures/cdk/node_modules:
	cd fixtures/cdk
	npm install

cdk-build: fixtures/cdk/node_modules
	cd fixtures/cdk
	$(BUILD_ENV) npm run build
	$(BUILD_ENV) cdk synth > /dev/null

cdk-deploy:
	cd fixtures/cdk
	yes | $(BUILD_ENV) cdk deploy --require-approval never $(AWS_FLAG)

clean:
	rm -rf dist public/tailwind.css

dist/public/members:
	cp -r public/members dist/public

dist/public/services:
	cp -r public/services dist/public

dup-assets:
	cp -r dist/public/*.css dist/public/*.avif dist/public/*.ico dist/public/assets/

deploy: build-lambda cdk-build cdk-deploy dup-assets s3-sync

s3-sync:
	aws s3 sync dist/public s3://$(DOMAIN) $(AWS_FLAG) --delete
	aws cloudfront create-invalidation --distribution-id $(CDN_ID) --paths "/*" $(AWS_FLAG) > /dev/null

run-api: build-lambda cdk-build sam-local-api

sam-local-api:
	sam local start-api -t ./fixtures/cdk/cdk.out/Stack.template.json
