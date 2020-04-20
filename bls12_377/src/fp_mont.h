#ifndef FPC_H
#define FPC_H

#include <stdint.h>

#if defined(__cplusplus) || defined(c_plusplus)
extern "C" {
#endif

// Separate multiplication and reduction functions
void fp_prod(uint32_t *, const uint32_t *, const uint32_t *);
void fp_redc(uint32_t *, uint32_t *);

// Combined multiply-reduce function
void fp_mulred(uint32_t *, const uint32_t *, const uint32_t *);

#if defined(__cplusplus) || defined(c_plusplus)
}
#endif

#endif // FPC_H
