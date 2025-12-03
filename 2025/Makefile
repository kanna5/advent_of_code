build: lint
	go build -v -trimpath -ldflags='-s -w'

lint:
	golangci-lint run ./...

generate:
	go generate

fmt:
	go fmt ./...

input:
	bash ../scripts/get_input.sh

clean:
	rm -f 2025
	rm -f ./*.{png,svg,dot}

.PHONY: build lint generate fmt input clean 
