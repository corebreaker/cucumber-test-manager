Feature: An example for Cucumber Trellis

  Here is a simple example of how to use the Cucumber Trellis to grow cucumbers.
  This test just implement World trait with a test for a simple addition.

  Rule: Add two numbers
    Scenario: Add two numbers
      Given we have the number `1` as the first number
        And we have the number `2` as the second number
      When we add them together
      Then we should get `3` as the result
