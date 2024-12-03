DAY ?= $(shell date '+%d')

.PHONY: solve
solve:
	cargo solve ${DAY}

.PHONY: time
time:
	cargo time ${DAY}

.PHONY: init
init:
	cargo scaffold ${DAY} --download
