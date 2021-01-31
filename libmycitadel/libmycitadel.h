#ifndef libmycitadel_h
#define libmycitadel_h

/* Generated with cbindgen:0.16.0 */

/* Warning, this file is autogenerated by cbindgen. Don't modify this manually. */

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define ERRNO_IO 1

#define ERRNO_RPC 2

#define ERRNO_NET 3

#define ERRNO_TRANSPORT 4

#define ERRNO_NOTSUPPORTED 5

#define ERRNO_STORAGE 6

#define ERRNO_SERVERFAIL 7

#define ERRNO_EMBEDDEDFAIL 8

#define ERRNO_CHAIN 100

#define ERRNO_JSON 101

typedef struct mycitadel_error_t {
        int errno;
        const char *message;
} mycitadel_error_t;

typedef struct mycitadel_client_t {
        void *_inner;
        struct mycitadel_error_t *last_error;
} mycitadel_client_t;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

bool mycitadel_is_ok(const struct mycitadel_client_t *self);

bool mycitadel_has_err(const struct mycitadel_client_t *self);

struct mycitadel_client_t *mycitadel_run_embedded(const char *chain,
                                                  const char *data_dir,
                                                  const char *electrum_server);

const char *mycitadel_list_assets(struct mycitadel_client_t *client);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif /* libmycitadel_h */