test:
	@rm -rf tests/test*.cdb
	@cargo test
	@rm -rf tests/test*.cdb

.PHONY: test
