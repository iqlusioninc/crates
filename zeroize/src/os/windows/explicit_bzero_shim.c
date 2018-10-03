/**
 * `explicit_bzero()` shim for invoking `SecureZeroMemory` on Windows, which
 * is provided as a macro.
 */

#include <windows.h>
#include <wincrypt.h>

void explicit_bzero(void *dest, size_t n) {
    SecureZeroMemory(dest, n);
}
