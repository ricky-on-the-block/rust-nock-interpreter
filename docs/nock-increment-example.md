# Understanding Nock: Incrementing a Number

This guide walks through a simple Nock example: incrementing the number 42. We'll use the Nock 4K definition to explain each step of the process.

## The Example

We want to evaluate the Nock expression: `[42 [4 0 1]]`

This should increment 42, resulting in 43.

## Nock Evaluation and the * Operator

Nock is a pure (stateless) function from noun to noun. In our pseudocode (and only in our pseudocode) we express this with the prefix operator `*`.

This means:

1. The Nock specification defines: `nock(a)  *a`
2. When we write `[42 [4 0 1]]`, it's implicitly a Nock computation. We make this explicit by writing `*[42 [4 0 1]]`.
3. The * can be read as "evaluate using Nock".

## Step-by-Step Evaluation

### Step 1: Identify the Subject and Formula

In Nock, computations are performed on a subject using a formula. "Nock maps a cell `[subject formula]` to a noun `product`."

- Input: `*[42 [4 0 1]]`
- Subject: `42`
- Formula: `[4 0 1]`

### Step 2: Apply the Rule for Nock Operator 4 (Increment)

The Nock 4K definition states: `*[a 4 b]  +*[a b]`
This means we need to:
1. Evaluate `*[a b]`
2. Increment the result

So our expression becomes: `+*[42 [0 1]]`

### Step 3: Apply the Rule for Nock Operator 0 (Slot)

Now we apply the rule for Nock operator 0: `*[a 0 b]  /[b a]` to evaluate `*[42 [0 1]]`

In our case:
- a = 42
- b = 1

So we're evaluating `/[1 42]`

### Step 4: Apply Tree Addressing

The tree addressing rule states that `/[1 a]  a` returns the entire subject `a`. So:

`/[1 42] = 42`

### Step 5: Increment the Result

Now that we've evaluated `*[42 [0 1]]` to 42, we need to increment it according to the rule for operator 4.

`+42 = 43`

### Step 6: Final Result

The evaluation is complete, and the result is 43.

## Summary of Steps

| Step | Expression      | Description                                          |
|:-----|:----------------|:-----------------------------------------------------|
| 1.   | `[42 [4 0 1]]`  | Apply nock function: `nock(a)  *a`                   |
| 2.   | `*[42 [4 0 1]]` | Apply rule for operator 4: `*[a 4 b]  +*[a b]`       |
| 3.   | `+*[42 [0 1]]`  | Apply rule for operator 0: `*[a 0 b]  /[b a]`        |
| 4.   | `+(/[1 42])`    | Resolving slot operation: `/[1 a]  a`                |
| 5.   | `+(42)`         | Apply increment: `+a  1+a`                           |
| 6.   | `43`            | `product` noun reached from input `[subject formula]`|

This example demonstrates how Nock uses its simple rules to perform computations, in this case incrementing the number 42 to produce 43.