build: generate .WAIT lint
	go build

lint:
	golangci-lint run ./...

solution_dirs:
	for i in $$(seq 25); do \
		mkdir -pv "solutions/day$$(printf "%02d" "$$i")"; \
	done

generate:
	go generate

clean:
	rm -f 2023

.PHONY: build lint solution_dirs generate clean
