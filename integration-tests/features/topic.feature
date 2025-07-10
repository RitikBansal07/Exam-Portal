Feature: Topic API

  Scenario: Insert a new topic
    When I send a POST request to "/topic" with body:
      """
      { "topic": "Integration Tests" }
      """
    Then the response code should be 201

  Scenario: Get all topics
    When I send a GET request to "/topics"
    Then the response code should be 200
    And the response body should be a JSON array

  Scenario: Get topic by id
    When I send a GET request to "/1"
    Then the response code should be 200
    And the response body should contain "id"
