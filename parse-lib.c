#include <assert.h>
#include <errno.h>
// #include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "parse-lib.h"

/* In the real world, we would probably have something like this event loop below.
 * In fact I originally had that API. Then I realized that required you to implement your own Future and that would take too long; and anyway I'm not trying to judge your knowledge of async internals, just that you can do C FFI.
 */
//struct cups_connection {
//	struct cups_list job_ids;
//	struct {
//		struct timeval ready_at;
//		on_completion_t callback;
//	} jobs[MAX_JOBS];
//};

struct cups_connection {
	// malloc can return NULL if passed a size of 0.
	int unused;
};

struct cups_connection *cups_connect(const char *url, int *status) {
	if (memcmp(url, "http", 4)) {
		*status = -1;
		return NULL;
	}
	char *bad_printer = "http://bad-printer/";
	// puts(url);
	if (!strcmp(url, bad_printer)) {
		*status = ECONNREFUSED;
		return NULL;
	}
	return malloc(sizeof(struct cups_connection));
}

int cups_print(struct cups_connection *this, const char *_data) {
	(void)_data; /* unused */

	srandom((unsigned long)this);
	long r = random();
	// printf("rand: %ld\n", r);
	if (!(r%4))
		return EIO;
	return 0;
}

void cups_free(struct cups_connection *this) {
	free(this);
}
