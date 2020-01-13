#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

bool get_pubkey(uint64_t *in_private_key, uint8_t *out_public_key);

bool is_valid_key(const uint8_t *in_private_key);

bool sign_hash(uint64_t *in_private_key, uint8_t *in_hash, uint8_t *out_signature);
