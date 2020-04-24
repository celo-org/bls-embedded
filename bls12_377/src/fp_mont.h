#ifndef FPC_H
#define FPC_H

#include <stdint.h>

#if defined(__cplusplus) || defined(c_plusplus)
extern "C" {
#endif

// Montgomery reduction function

void fp_redc(uint32_t *, uint32_t *);

#if defined(__cplusplus) || defined(c_plusplus)
}
#endif

#endif // FPC_H
