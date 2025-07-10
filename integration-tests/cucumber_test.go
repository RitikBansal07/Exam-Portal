package main

import (
	"os"
	"testing"

	"github.com/cucumber/godog"
)

func TestBDD(t *testing.T) {
	env := "local"
	if len(os.Args) > 1 {
		arg := os.Args[len(os.Args)-1]
		if arg == "deploy" || arg == "local" {
			env = arg
		}
	}

	switch env {
	case "deploy":
		baseURL = "https://api.yourdomain.com" // ✅ change to your actual deploy URL
	default:
		baseURL = "http://localhost:8080"
	}

	opts := godog.Options{
		Format: "pretty",
		Paths:  []string{"features"},
		Strict: true,
	}

	suite := godog.TestSuite{
		Name:                 "topic-api",
		ScenarioInitializer:  InitializeScenario,
		Options:              &opts,
	}

	if suite.Run() != 0 {
		t.Fail()
	}
}
