## If Stmt

```mermaid
flowchart TD
Prev -->
If{x > 1} -->|yes| T --> Post
If -->|no| Post
Post(( )) --> Next
```

## If Else Stmt

```mermaid
flowchart TD
Prev -->
If{x > 1} -->|yes| T --> Post
If -->|no| F --> Post
Post(( )) --> Next
```

## While Stmt

```mermaid
flowchart TD
Prev --> Pre(( )) -->
If{x > 1} -->|yes| Body --> X  --> Y --> Pre
If --->|no| r(( )) -->  Next
```

## Do Stmt

```mermaid
flowchart TD
Prev --> Pre(( )) --> Body -->
If{x > 1} -->|yes| Pre
If --->|no| Next
```

## For Stmt

```mermaid
flowchart TD

Start --> If

Next --> f --> g --> h

If{{x > 1}} --> Body
If ~~~ Body
If --> Next
Body --- If



subgraph Body
x --> y --> z
end

```



```mermaid
flowchart TD

0x7fbb0781fcf0{"a < c"}0x7fbb0781fcf0_out(( ))
0x7fbb0781fcf0 -->|Yes| 0x7fbb0781fad8
0x7fbb0781fad8["b += 2"]
0x7fbb0781fad8 --> 0x7fbb0781fcf0_out
0x7fbb0781fcf0 -->|No| 0x7fbb0781fcc0
0x7fbb0781fcc0{"a == c"}0x7fbb0781fcc0_out(( ))
0x7fbb0781fcc0 -->|Yes| 0x7fbb0781fbf0
0x7fbb0781fbf0["b += 1"]
0x7fbb0781fbf0 --> 0x7fbb0781fcc0_out
0x7fbb0781fcc0 -->|No| 0x7fbb0781fc78
0x7fbb0781fc78["b -= 1"]
0x7fbb0781fc78 --> 0x7fbb0781fcc0_out
0x7fbb0781fcc0_out --> 0x7fbb0781fcf0_out

```