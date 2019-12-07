#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct PrivateKey PrivateKey;

typedef struct PublicKey PublicKey;

typedef struct Signature Signature;

bool aggregate_public_keys(const PublicKey *const *_in_public_keys,
                           int32_t _in_public_keys_len,
                           PublicKey **_out_public_key);

bool aggregate_public_keys_subtract(const PublicKey *_in_aggregated_public_key,
                                    const PublicKey *const *_in_public_keys,
                                    int32_t _in_public_keys_len,
                                    PublicKey **_out_public_key);

bool aggregate_signatures(const Signature *const *_in_signatures,
                          int32_t _in_signatures_len,
                          Signature **_out_signature);

bool deserialize_private_key(const uint8_t *_in_private_key_bytes,
                             int32_t _in_private_key_bytes_len,
                             PrivateKey **_out_private_key);

bool deserialize_public_key(const uint8_t *_in_public_key_bytes,
                            int32_t _in_public_key_bytes_len,
                            PublicKey **_out_public_key);

bool deserialize_signature(const uint8_t *_in_signature_bytes,
                           int32_t _in_signature_bytes_len,
                           Signature **_out_signature);

void destroy_private_key(PrivateKey *_private_key);

void destroy_public_key(PublicKey *_public_key);

void destroy_signature(Signature *_signature);

void free_vec(uint8_t *_bytes, int32_t _len);

bool generate_private_key(PrivateKey **out_private_key);

bool generate_signature(Signature **out_signature);

bool get_pubkey(uint64_t *in_private_key, uint8_t *out_public_key);

bool is_valid_key(const uint8_t *in_private_key);

bool private_key_to_public_key(const PrivateKey *in_private_key, PublicKey **out_public_key);

bool serialize_private_key(const PrivateKey *_in_private_key,
                           uint8_t **_out_bytes,
                           int32_t *_out_len);

bool serialize_public_key(const PublicKey *_in_public_key, uint8_t **_out_bytes, int32_t *_out_len);

bool serialize_signature(const Signature *_in_signature, uint8_t **_out_bytes, int32_t *_out_len);

bool sign_hash(uint64_t *in_private_key, uint8_t *in_hash, uint8_t *out_signature);

bool sign_message(uint64_t *in_private_key,
                  const uint8_t *in_message,
                  int32_t in_message_len,
                  const uint8_t *in_extra_data,
                  int32_t in_extra_data_len,
                  bool should_use_composite);

bool sign_pop(const PrivateKey *_in_private_key, Signature **_out_signature);

bool verify_pop(const PublicKey *_in_public_key,
                const Signature *_in_signature,
                bool *_out_verified);

bool verify_signature(const PublicKey *_in_public_key,
                      const uint8_t *_in_message,
                      int32_t _in_message_len,
                      const uint8_t *_in_extra_data,
                      int32_t _in_extra_data_len,
                      const Signature *_in_signature,
                      bool _should_use_composite,
                      bool *_out_verified);
