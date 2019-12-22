# hla
HLA stands for High Level Assembler. It is a compiler/assembler which assembles a easy to read version of assembly into i386 assembly ready to run. This was made to make making operating system boot loaders easier to understand.

## Syntax

HLA is space insensitive. This means that this:

"$eax = 2"

is functionally the same as:

" $eax=   2  "

Though, it wouldn't be so bad to keep your code clean.

### Labels
Labels are essentially addresses in memory. There are three uses for labels.
* They can be used to expose symbols to the linker
* They can be used to jump to a certain point in your code
* They can be used as addresses to data

In HLA, you can create a label by writing the label name and putting a colon.

`label_name:`

If you would like to expose a label to the linker as a symbol, prepend an at sign (@). Generally, linkers require you to define a _start symbol, which is what global labels are usually used for.

`@_start:`

### Assigning values
Writing to registers, memory, or labels is incredibly easy in HLA. In order to write to a register, you must first specify what you're writing to is a register. This is done with the dollar symbol ($).

`$eax = 1`

The ladder sets the value of EAX as 1.

In order to write to memory, simply use the asterisk operator (*). Generally, the asterisk operator is used to say "the value at". So, `*0xdeadbeef` would mean "the value at 0xdeadbeef`. In addition to this, you cannot write raw values to memory. This is a limitation of assembly. You must use an intermediate register.

`*0xdeadbeef ($eax) = 0b11`

This first sets EAX is 3 (11 in binary), then sets the value at 0xdeadbeef to eax. If you only need to move the contents of a register into a memory space, the parenthesis may be ommited.

`*0xdeadbeef = $ebx`

Writing to labels is also very similar. Simply set the value at the label to whatever you want.

`*label_name = $ebp`

The same rule of intermediate registers applies with labels as well.

`*label_name ($ecx) = 69`


### Conditional Branching
Conditional branching in HLA is massively simplified compared to pure assembly branching. Simply put the label and the condition.

If you wanted to jump to "label_name" if a EAX equaled 0, for instance:

`goto label_name if ($eax == 0)`

Due to the restrictions of assembly, you may only compare registers to registers or registers to values. In addition, you may only have one comparision.

Here are the avalible comparisons:

Condition | Symbol
---|---
Equals | ==
Not Equals | !=
Greater than | >=
Less than | <=

You can do an unconditional jump by doing:

`goto label_name`

### Memory
Reserving buffers and storing constant data in HLA is very similar to how you would store data in other assemblers.

To reserve N amount of bytes:

`reserve[N]`

Example:

```asm
my_buffer:
reserve[40]
```

To set constant data:

`const data`

Example (the 0xa signifies the newline character):

```asm
my_string:
const "hello", 0xa
```

### Passing
There are many obscure instructions or instructions that are simple enough that they don't need to be abstracted by HLA. In order to use such instructions, you should "pass" them to NASM instead of letting HLA handle it.

To do this, simple prefix the instruction with an exclamation mark. For example:

`!hlt` or `!cli` or `!int 0x80`

## Compiling Code
The code HLA generates is for NASM, though it should work for any Intel syntax based assembler. The `make.sh` script can be used to compile the code HLA outputs.
