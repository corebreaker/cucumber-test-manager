Feature: An example for Cucumber Trellis

  Here is a simple example of how to use the Cucumber Trellis to grow cucumbers.
  This test just implement World trait with a test for a simple addition.

  Rule: Add two numbers
    Scenario: Add two numbers
      Given I have the number 1 as the first number
      And I have the number 2 as the second number
      When I add them together
      Then I should get 3
