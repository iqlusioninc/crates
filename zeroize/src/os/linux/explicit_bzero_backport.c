/**
 * Backport of `explicit_bzero()` for Linux when using glibc versions earlier
 * than 2.2.5.
 */

#include <string.h>

#undef explicit_bzero

void explicit_bzero(void *dest, size_t n) {
    memset(dest, '\0', n);
    asm volatile ("" ::: "memory");
}
