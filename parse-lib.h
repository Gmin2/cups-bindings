#include <stdbool.h>
#include <stddef.h>
#include <sys/time.h>

#define MAX_JOBS 100
#define INVALID_JOB -1

struct cups_connection;

/* Connect to a printer.
 *
 * The `url` must be allocated by the caller, and must be valid at least until `cups_free` is called.
 * The returned `cups_connection` will be NULL if an error occurs. Check `status` to see the error.
 *
 * `status` will be set to 0 if the connection was successful, a positive integer if an IO error occurred, or -1 if the URL is invalid.
 * If an IO error occurs, the integer is compatible with all functions
 * in `errno.h` (e.g. it is valid to call `strerror` on it).
 */
struct cups_connection *cups_connect(const char *url, int *status);
/* Print a document.
 *
 * Returns 0 if printing was successful, or a positive (errno-compatible) integer if an IO error occurred.
 * The `data` is owned by the caller and will not be freed.
 */
int cups_print(struct cups_connection*, const char *data);
/* Disconnect from the printer and free all memory associated with the connection.
 *
 * There must not be any active jobs when this function is called.
 * You can get a list of active jobs with `cups_list` and cancel a job with `cups_cancel`.
 */
void cups_free(struct cups_connection*);
