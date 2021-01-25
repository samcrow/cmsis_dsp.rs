
/*
 * This file is used to determine the sizes of C types on the target ARM platform.
 *
 * Compile: arm-none-eabi-gcc -c -o data_types.o data_types.c
 * Inspect: arm-none-eabi-objdump -D data_types.o
 */

static unsigned int SIZEOF_SHORT = sizeof(short);
static unsigned int SIZEOF_INT = sizeof(int);
static unsigned int SIZEOF_LONG = sizeof(long);
static unsigned int SIZEOF_LONG_LONG = sizeof(long long);
