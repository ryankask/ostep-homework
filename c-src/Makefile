CC = gcc
CFLAGS = -Wall -Wextra -std=c99

# Find all .c files in the current directory
SRCS = $(wildcard *.c)

# Generate target executable names (removing .c extension)
EXES = $(patsubst %.c,%,$(SRCS))
DEBUG_EXES = $(patsubst %,%.debug,$(EXES))

.PHONY: all clean debug

# The 'all' target builds all executables found
all: $(EXES)

# The 'debug' target builds all executables with debug info
debug: $(DEBUG_EXES)

# Pattern rule: How to build any executable '%' from its corresponding .c file '%.c'
%: %.c
	$(CC) $(CFLAGS) $< -o $@

# Pattern rule: How to build a debug executable '%.debug' from its corresponding .c file '%.c'
%.debug: %.c
	$(CC) $(CFLAGS) -g $< -o $@

# New: Pattern rule to run an executable
run-%: %
	./$<

# New: Pattern rule to run a debug executable
run-debug-%: %.debug
	./$<

# Clean up: remove all generated executables
clean:
	rm -f $(EXES) $(DEBUG_EXES)
