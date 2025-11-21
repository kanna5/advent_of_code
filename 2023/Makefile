build: lint
	go build -v -trimpath -ldflags='-s -w'

lint:
	golangci-lint run ./...

solution_dirs:
	for i in $$(seq 25); do \
		mkdir -pv "solutions/day$$(printf "%02d" "$$i")"; \
	done

generate:
	go generate

fmt:
	go fmt ./...

clean:
	rm -f 2023
	rm -f ./*.{png,svg,dot}

.PHONY: build lint solution_dirs generate fmt clean 
