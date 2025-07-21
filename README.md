# instructions-reflection

### log
transaction.log

### summary
```
top
+-- memo ( memo#t.1 )
+-- memo ( memo#t.2 )
+-- parent
    +-- CPI memo ( memo#p.1 )
    +-- CPI memo ( memo#p.2 )
    +-- CPI child
    |   +-- CPI memo ( memo#c.1 )
    |   +-- CPI memo ( memo#c.2 )
    |   +-- get_processed_sibling_instruction (GPSI_AT_CHILD)
    +-- get_processed_sibling_instruction (GPSI_AT_PARENT)

GPSI_AT_CHILD returns: memo#p.2, memo#p.1
GPSI_AT_PARENT returns: memo#t.2, memo#t.1
```

