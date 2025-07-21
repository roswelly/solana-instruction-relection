# instructions-reflection

### log
transaction.log

### summary
```
top
+-- memo ( memo#t.1 )
+-- memo ( memo#t.2 )
+-- parent
|   +-- CPI memo ( memo#p.1 )
|   +-- CPI memo ( memo#p.2 )
|   +-- CPI child
|   |   +-- CPI memo ( memo#c.1 )
|   |   +-- CPI memo ( memo#c.2 )
|   |   +-- get_processed_sibling_instruction (GPSI_AT_CHILD1)
|   +-- CPI child
|   |   +-- CPI memo ( memo#c.1 )
|   |   +-- CPI memo ( memo#c.2 )
|   |   +-- get_processed_sibling_instruction (GPSI_AT_CHILD2)
|   +-- get_processed_sibling_instruction (GPSI_AT_PARENT1)
+-- parent
    +-- CPI memo ( memo#p.1 )
    +-- CPI memo ( memo#p.2 )
    +-- CPI child
    |   +-- CPI memo ( memo#c.1 )
    |   +-- CPI memo ( memo#c.2 )
    |   +-- get_processed_sibling_instruction (GPSI_AT_CHILD3)
    +-- CPI child
    |   +-- CPI memo ( memo#c.1 )
    |   +-- CPI memo ( memo#c.2 )
    |   +-- get_processed_sibling_instruction (GPSI_AT_CHILD4)
    +-- get_processed_sibling_instruction (GPSI_AT_PARENT2)

GPSI_AT_CHILD1 returns: memo#p.2, memo#p.1
GPSI_AT_CHILD2 returns: child, memo#p.2, memo#p.1
GPSI_AT_PARENT1 returns: memo#t.2, memo#t.1

GPSI_AT_CHILD3 returns: memo#p.2, memo#p.1
GPSI_AT_CHILD4 returns: child, memo#p.2, memo#p.1
GPSI_AT_PARENT2 returns: parent, memo#t.2, memo#t.1
```
