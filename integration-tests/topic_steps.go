package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"strings"

	"github.com/cucumber/godog"
)

var (
	resp     *http.Response
	respBody []byte
	baseURL  string = "http://localhost:8080" // default
)

func iSendARequestWithBody(method, path, body string) error {
	fullURL := baseURL + path

	client := &http.Client{}
	req, err := http.NewRequest(method, fullURL, bytes.NewBufferString(body))
	if err != nil {
		return fmt.Errorf("failed to create request: %w", err)
	}
	req.Header.Set("Content-Type", "application/json")

	resp, err = client.Do(req)
	if err != nil {
		return fmt.Errorf("request failed: %w", err)
	}
	defer resp.Body.Close()

	respBody, err = io.ReadAll(resp.Body)
	if err != nil {
		return fmt.Errorf("failed to read response: %w", err)
	}
	return nil
}

func iSendARequest(method, path string) error {
	return iSendARequestWithBody(method, path, "")
}

func theResponseCodeShouldBe(expected int) error {
	if resp.StatusCode != expected {
		return fmt.Errorf("expected status %d, got %d", expected, resp.StatusCode)
	}
	return nil
}

func theResponseBodyShouldContain(substring string) error {
	if !strings.Contains(string(respBody), substring) {
		return fmt.Errorf("response body does not contain %q", substring)
	}
	return nil
}

func theResponseBodyShouldBeAJSONArray() error {
	var arr []interface{}
	if err := json.Unmarshal(respBody, &arr); err != nil {
		return fmt.Errorf("response body is not a valid JSON array: %v", err)
	}
	return nil
}

func InitializeScenario(ctx *godog.ScenarioContext) {
	// baseURL is set in main test file (cucumber_test.go)
	ctx.Step(`^I send a (GET|POST) request to "([^"]*)"$`, iSendARequest)
	ctx.Step(`^I send a (GET|POST) request to "([^"]*)" with body:$`, iSendARequestWithBody)
	ctx.Step(`^the response code should be (\d+)$`, theResponseCodeShouldBe)
	ctx.Step(`^the response body should contain "([^"]*)"$`, theResponseBodyShouldContain)
	ctx.Step(`^the response body should be a JSON array$`, theResponseBodyShouldBeAJSONArray)
}
